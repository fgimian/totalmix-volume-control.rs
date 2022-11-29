use anyhow::Result;
use thiserror::Error;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        RegisterHotKey, MOD_SHIFT, VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP,
    },
    WindowsAndMessaging::{MSG, WM_HOTKEY},
};

#[derive(Error, Debug)]
#[error("unable to bind the required hotkey")]
pub struct HotKeyBindError;

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

pub const fn receive(msg: &MSG) -> Option<HotKey> {
    if msg.message != WM_HOTKEY {
        return None;
    }

    match msg.wParam.0 {
        1 => Some(HotKey::VolumeUp),
        2 => Some(HotKey::VolumeDown),
        3 => Some(HotKey::VolumeUpFine),
        4 => Some(HotKey::VolumeDownfine),
        5 => Some(HotKey::Mute),
        _ => None,
    }
}
