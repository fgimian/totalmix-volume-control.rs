use anyhow::Result;
use thiserror::Error;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        RegisterHotKey, MOD_SHIFT, VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP,
    },
    WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY},
};

#[derive(Error, Debug)]
#[error("unable to bind the required hotkey")]
pub struct HotKeyBindError;

#[derive(Error, Debug)]
#[error("the quit message was received")]
pub struct QuitError;

#[derive(Debug)]
pub enum HotKey {
    VolumeUp = 1,
    VolumeDown = 2,
    VolumeUpFine = 3,
    VolumeDownfine = 4,
    Mute = 5,
}

pub fn register() -> Result<()> {
    for (id, modifiers, key) in [
        (HotKey::VolumeUp, None, VK_VOLUME_UP),
        (HotKey::VolumeDown, None, VK_VOLUME_DOWN),
        (HotKey::VolumeUpFine, Some(MOD_SHIFT), VK_VOLUME_UP),
        (HotKey::VolumeDownfine, Some(MOD_SHIFT), VK_VOLUME_DOWN),
        (HotKey::Mute, None, VK_VOLUME_MUTE),
    ] {
        let result = unsafe {
            RegisterHotKey(
                None,
                id as i32,
                modifiers.unwrap_or_default(),
                u32::from(key.0),
            )
        };
        if !result.as_bool() {
            return Err(HotKeyBindError.into());
        }
    }

    Ok(())
}

pub fn receive() -> Result<HotKey> {
    loop {
        let mut msg = MSG::default();
        let result = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        if !result.as_bool() {
            return Err(QuitError.into());
        }

        if msg.message != WM_HOTKEY {
            continue;
        }

        // TODO: Can I make this nicer?
        let hotkey = match msg.wParam.0 {
            1 => Some(HotKey::VolumeUp),
            2 => Some(HotKey::VolumeDown),
            3 => Some(HotKey::VolumeUpFine),
            4 => Some(HotKey::VolumeDownfine),
            5 => Some(HotKey::Mute),
            _ => None,
        };

        if let Some(hotkey) = hotkey {
            return Ok(hotkey);
        }
    }
}
