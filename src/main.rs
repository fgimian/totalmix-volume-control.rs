#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // clippy::expect_used,
    // clippy::unwrap_used
)]
mod comms;
mod config;
mod floats;
mod gui;
mod hotkeys;
mod manager;

use comms::{UdpReceiver, UdpSender};
use eframe::NativeOptions;
use egui::{pos2, vec2};
use gui::VolumeControlApp;
use hotkeys::HotKey;
use manager::Manager;
use std::{
    net::SocketAddrV4,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

fn main() {
    let native_options = NativeOptions {
        always_on_top: true,
        decorated: false,
        drag_and_drop_support: false,
        initial_window_pos: Some(pos2(40.0, 40.0)),
        initial_window_size: Some(vec2(165.0, 165.0)),
        resizable: false,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(
        "TotalMix Volume Control",
        native_options,
        Box::new(|cc| {
            let sender =
                UdpSender::new(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 7002)).unwrap();
            let receiver =
                UdpReceiver::bind(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 9002)).unwrap();
            let manager = Arc::new(Manager::new(sender, receiver));
            let ui_trigger = Arc::new(AtomicBool::new(false));

            // Create the thread that will receive volume changes from the device.
            {
                let manager = Arc::clone(&manager);
                let ctx = cc.egui_ctx.clone();
                thread::Builder::new()
                    .name("receiver".to_string())
                    .spawn(move || {
                        manager.request_volume().unwrap();
                        loop {
                            if manager.recieve_volume().unwrap() {
                                ctx.request_repaint();
                            }
                        }
                    })
                    .unwrap();
            }

            // Create the thread that will receive hotkeys and update the volume.  It will also send a
            // message to the main GUI thread.
            {
                let manager = Arc::clone(&manager);
                let ui_trigger = Arc::clone(&ui_trigger);
                thread::Builder::new()
                    .name("hotkeys".to_string())
                    .spawn(move || {
                        hotkeys::register().unwrap();
                        loop {
                            let hotkey = hotkeys::receive().unwrap();
                            ui_trigger.store(true, Ordering::SeqCst);
                            match hotkey {
                                HotKey::VolumeUp => manager.increase_volume().unwrap(),
                                HotKey::VolumeDown => manager.decrease_volume().unwrap(),
                                HotKey::VolumeUpFine => manager.increase_volume_fine().unwrap(),
                                HotKey::VolumeDownfine => manager.decrease_volume_fine().unwrap(),
                                HotKey::Mute => manager.toggle_dim().unwrap(),
                            };
                        }
                    })
                    .unwrap();
            }

            Box::new(VolumeControlApp::new(cc, manager, ui_trigger))
        }),
    );
}
