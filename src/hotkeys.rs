use anyhow::Result;
use thiserror::Error;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        RegisterHotKey, HOT_KEY_MODIFIERS, MOD_SHIFT, VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP,
    },
    WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY},
};

#[derive(Error, Debug)]
#[error("unable to bind the required hotkey")]
pub(crate) struct HotKeyBindError;

#[derive(Error, Debug)]
#[error("the quit message was received")]
pub(crate) struct QuitError;

#[derive(Debug)]
pub(crate) enum HotKey {
    VolumeUp = 1,
    VolumeDown = 2,
    VolumeUpFine = 3,
    VolumeDownfine = 4,
    Mute = 5,
}

pub(crate) struct HotKeys {}

impl HotKeys {
    pub fn new() -> Result<Self> {
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
                    modifiers.unwrap_or(HOT_KEY_MODIFIERS::default()),
                    key.0 as _,
                )
            };
            if !result.as_bool() {
                return Err(HotKeyBindError.into());
            }
        }

        Ok(HotKeys {})
    }

    pub fn receive(&self) -> Result<HotKey> {
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
            let hotkey = match msg.wParam.0 as _ {
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
}
