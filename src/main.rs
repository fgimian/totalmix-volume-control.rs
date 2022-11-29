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
use egui_glow::EguiGlow;
use glow::{Context, HasContext};
use glutin::ContextBuilder;
use gui::VolumeControlApp;
use hotkeys::HotKey;
use manager::Manager;
use parking_lot::Mutex;
use std::{
    net::SocketAddrV4,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};
use system_tray::{
    icon::Icon,
    menu::{menu_event_receiver, AboutMetadata, Menu, MenuItem, PredefinedMenuItem},
    tray_event_receiver, TrayIconBuilder,
};
use windows::Win32::UI::WindowsAndMessaging::MSG;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::windows::{EventLoopBuilderExtWindows, WindowBuilderExtWindows},
    window::WindowBuilder,
};

#[derive(Debug)]
pub enum UserEvent {
    RequestRepaint,
}

fn main() {
    // Register global hotkeys.
    hotkeys::register().unwrap();

    // Create the system tray.
    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Exit", true, None);
    tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("TotalMix Volume Control".to_string()),
                version: Some("1.0.0".to_string()),
                authors: Some(vec!["Fotis Gimian".to_string()]),
                license: Some("MIT or Apache 2.0".to_string()),
                website: Some("https://github.com/fgimian/totalmix-volume-control".to_string()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &quit_item,
    ]);

    let icon = Icon::from_resource(1, None).unwrap();
    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("TotalMix OSC connection active")
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = menu_event_receiver();
    let tray_channel = tray_event_receiver();

    // Create the volume manager.
    let sender = UdpSender::new(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 7002)).unwrap();
    let receiver =
        UdpReceiver::bind(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 9002)).unwrap();
    let manager = Arc::new(Manager::new(sender, receiver));

    // Create the event loop.
    let (hotkey_sender, hotkey_receiver) = mpsc::channel();
    let event_loop = EventLoopBuilder::with_user_event()
        .with_msg_hook(move |msg| {
            let msg = unsafe { &*(msg.cast::<MSG>()) };
            hotkeys::receive(msg).map_or(false, |hotkey| {
                hotkey_sender.send(hotkey).unwrap();
                true
            })
        })
        .build();
    let event_loop_proxy = Arc::new(Mutex::new(event_loop.create_proxy()));

    // Create the window and OpenGL context.
    let window_builder = WindowBuilder::new()
        .with_title("TotalMix Volume Control")
        .with_always_on_top(true)
        .with_decorations(false)
        .with_skip_taskbar(true)
        .with_drag_and_drop(false)
        .with_resizable(false)
        .with_transparent(true)
        .with_position(LogicalPosition { x: 40.0, y: 40.0 })
        .with_inner_size(LogicalSize {
            width: 165,
            height: 165,
        })
        .with_visible(false);

    let gl_window = unsafe {
        ContextBuilder::new()
            .with_depth_buffer(0)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };

    let gl = unsafe { Context::from_loader_function(|s| gl_window.get_proc_address(s)) };
    let gl = Arc::new(gl);

    // Create the egui glow integration.
    let mut egui_glow = EguiGlow::new(&event_loop, Arc::clone(&gl));

    // Create the application.
    let mut app = {
        let manager = Arc::clone(&manager);
        VolumeControlApp::new(&egui_glow.egui_ctx, manager)
    };

    // Create the thread that will receive volume changes from the device.
    {
        let manager = Arc::clone(&manager);
        thread::Builder::new()
            .name("receiver".to_string())
            .spawn(move || {
                manager.request_volume().unwrap();
                loop {
                    manager.recieve_volume().unwrap();
                }
            })
            .unwrap();
    }

    // Create the thread that will send volume changes to the device.
    {
        let manager = Arc::clone(&manager);
        let event_loop_proxy = Arc::clone(&event_loop_proxy);
        thread::Builder::new()
            .name("sender".to_string())
            .spawn(move || loop {
                let hotkey = hotkey_receiver.recv().unwrap();
                match hotkey {
                    HotKey::VolumeUp => manager.increase_volume().unwrap(),
                    HotKey::VolumeDown => manager.decrease_volume().unwrap(),
                    HotKey::VolumeUpFine => manager.increase_volume_fine().unwrap(),
                    HotKey::VolumeDownfine => manager.decrease_volume_fine().unwrap(),
                    HotKey::Mute => manager.toggle_dim().unwrap(),
                };
                event_loop_proxy
                    .lock()
                    .send_event(UserEvent::RequestRepaint)
                    .unwrap();
            })
            .unwrap();
    }

    // Run the event loop.
    let mut visible = false;
    event_loop.run(move |event, _target, control_flow| {
        let mut redraw = |restart| {
            let repaint_after = egui_glow.run(gl_window.window(), |egui_ctx| {
                app.draw(egui_ctx, restart);
            });

            let quit = false;
            *control_flow = if quit {
                ControlFlow::Exit
            } else if repaint_after.is_zero() {
                gl_window.window().request_redraw();
                ControlFlow::Poll
            } else if let Some(repaint_after_instant) = Instant::now().checked_add(repaint_after) {
                ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                ControlFlow::Wait
            };

            unsafe {
                gl.clear_color(0.0, 0.0, 0.0, 0.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
            }

            egui_glow.paint(gl_window.window());
            gl_window.swap_buffers().unwrap();

            // Set the window to visible when it's ready to avoid a flash of white.
            if !visible {
                gl_window.window().set_visible(true);
                visible = true;

                // Allow the mouse cursor to pass through the window.
                // (this must be set after the window is made visible)
                gl_window.window().set_cursor_hittest(false).unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            Event::RedrawEventsCleared if cfg!(windows) => redraw(false),
            Event::RedrawRequested(_) if !cfg!(windows) => redraw(false),
            Event::UserEvent(UserEvent::RequestRepaint) => redraw(true),

            Event::WindowEvent { event, .. } => {
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }

                if let WindowEvent::Resized(physical_size) = &event {
                    gl_window.resize(*physical_size);
                } else if let WindowEvent::ScaleFactorChanged { new_inner_size, .. } = &event {
                    gl_window.resize(**new_inner_size);
                }

                if egui_glow.on_event(&event) {
                    gl_window.window().request_redraw();
                }
            }
            Event::LoopDestroyed => {
                egui_glow.destroy();
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                gl_window.window().request_redraw();
            }

            _ => (),
        }

        if let Ok(menu_event) = menu_channel.try_recv() {
            println!("Menu Channel = {:?}", menu_event);
            if menu_event.id == quit_item.id() {
                tray_icon.take();
                *control_flow = ControlFlow::Exit;
                println!("Exiting from tray");
            }
        }

        if let Ok(tray_event) = tray_channel.try_recv() {
            println!("Tray Channel = {:?}", tray_event);
        }
    });
}
