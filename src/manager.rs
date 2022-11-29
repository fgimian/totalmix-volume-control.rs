use crate::{
    comms::{Receiver, Sender},
    floats,
};
use anyhow::Result;
use parking_lot::Mutex;
use rosc::{OscMessage, OscPacket, OscType};
use std::fmt;
use thiserror::Error;

const VOLUME_OSC_ADDR: &str = "/1/mastervolume";
const VOLUME_DECIBELS_OSC_ADDR: &str = "/1/mastervolumeVal";
const DIM_OSC_ADDR: &str = "/1/mainDim";

#[derive(Error, Debug)]
#[error("increment must be greater than 0 and no more than 0.1")]
struct IncrementRangeError;

#[derive(Error, Debug)]
#[error("fine increment must be greater than 0 and no more than 0.05")]
struct FineIncrementRangeError;

#[derive(Error, Debug)]
#[error("max volume must be no more than 1.0")]
struct MaxVolumeRangeError;

pub struct Manager<S: Sender, R: Receiver> {
    increment: f32,
    fine_increment: f32,
    max_volume: f32,
    volume: Mutex<f32>,
    volume_db: Mutex<Option<String>>,
    dim: Mutex<f32>,
    sender: S,
    receiver: R,
}

impl<S: Sender, R: Receiver> fmt::Debug for Manager<S, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VolumeManager")
            // .field("increment", &self.increment)
            // .field("fine_increment", &self.fine_increment)
            // .field("max_volume", &self.max_volume)
            .field("volume", &self.volume())
            .field("volume_db", &self.volume_db())
            .field("dimmed", &self.dimmed())
            .finish()
    }
}

impl<S: Sender, R: Receiver> Manager<S, R> {
    pub const fn new(sender: S, receiver: R) -> Self {
        Self {
            increment: 0.02,
            fine_increment: 0.01,
            max_volume: 1.0,
            volume: Mutex::new(-1.0),
            volume_db: Mutex::new(None),
            dim: Mutex::new(-1.0),
            sender,
            receiver,
        }
    }

    // pub fn increment(&mut self, increment: f32) -> Result<&Self> {
    //     if !(0.0..0.10).contains(&increment) {
    //         return Err(IncrementRangeError.into());
    //     }
    //     self.increment = increment;
    //     Ok(self)
    // }

    // pub fn fine_increment(&mut self, fine_increment: f32) -> Result<&Self> {
    //     if !(0.0..0.05).contains(&fine_increment) {
    //         return Err(FineIncrementRangeError.into());
    //     }
    //     self.fine_increment = fine_increment;
    //     Ok(self)
    // }

    // pub fn max_volume(&mut self, max_volume: f32) -> Result<&Self> {
    //     if !(0.0..1.0).contains(&max_volume) {
    //         return Err(MaxVolumeRangeError.into());
    //     }
    //     self.max_volume = max_volume;
    //     Ok(self)
    // }

    pub fn volume(&self) -> f32 {
        let volume = self.volume.lock();
        *volume
    }

    pub fn volume_db(&self) -> Option<String> {
        let volume_db = self.volume_db.lock();
        (*volume_db).clone()
    }

    pub fn dimmed(&self) -> bool {
        floats::roughly_eq(self.dim(), 1.0)
    }

    fn dim(&self) -> f32 {
        let dim = self.dim.lock();
        *dim
    }

    pub fn initialized(&self) -> bool {
        floats::roughly_ne(self.volume(), -1.0)
            && self.volume_db().is_some()
            && floats::roughly_ne(self.dim(), -1.0)
    }

    pub fn request_volume(&self) -> Result<()> {
        self.send(VOLUME_OSC_ADDR, -1.0)?;
        self.send(DIM_OSC_ADDR, -1.0)
    }

    pub fn recieve_volume(&self) -> Result<bool> {
        let packet = self.receiver.receive()?;
        let mut received = false;

        if let OscPacket::Bundle(bundle) = packet {
            for packet in bundle.content {
                if let OscPacket::Message(message) = packet {
                    match message.addr.as_str() {
                        VOLUME_OSC_ADDR => {
                            if let Some(OscType::Float(received_volume)) = message.args.first() {
                                let mut volume = self.volume.lock();
                                *volume = *received_volume;
                                received = true;
                            }
                        }
                        VOLUME_DECIBELS_OSC_ADDR => {
                            if let Some(OscType::String(received_volume_db)) = message.args.first()
                            {
                                let mut volume_db = self.volume_db.lock();
                                *volume_db = Some((*received_volume_db).clone());
                                received = true;
                            }
                        }
                        DIM_OSC_ADDR => {
                            if let Some(OscType::Float(received_dim)) = message.args.first() {
                                let mut dim = self.dim.lock();
                                *dim = *received_dim;
                                received = true;
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }

        Ok(received)
    }

    pub fn increase_volume(&self) -> Result<bool> {
        self.increase_volume_by_increment(self.increment)
    }

    pub fn increase_volume_fine(&self) -> Result<bool> {
        self.increase_volume_by_increment(self.fine_increment)
    }

    pub fn decrease_volume(&self) -> Result<bool> {
        self.decrease_volume_by_increment(self.increment)
    }

    pub fn decrease_volume_fine(&self) -> Result<bool> {
        self.decrease_volume_by_increment(self.fine_increment)
    }

    pub fn toggle_dim(&self) -> Result<bool> {
        if !self.initialized() {
            return Ok(false);
        }

        let mut dim = self.dim.lock();
        let new_dim = if floats::roughly_eq(*dim, 1.0) {
            0.0
        } else {
            1.0
        };
        self.send(DIM_OSC_ADDR, 1.0)?;
        *dim = new_dim;

        Ok(true)
    }

    fn send(&self, addr: &str, value: f32) -> Result<()> {
        let packet = OscPacket::Message(OscMessage {
            addr: addr.to_string(),
            args: vec![OscType::Float(value)],
        });
        self.sender.send(&packet)
    }

    fn increase_volume_by_increment(&self, increment: f32) -> Result<bool> {
        if !self.initialized() {
            return Ok(false);
        }

        let mut volume = self.volume.lock();
        let mut new_volume = *volume + increment;
        if new_volume >= self.max_volume {
            new_volume = self.max_volume;
        }

        if floats::roughly_eq(new_volume, *volume) {
            return Ok(false);
        }

        self.send(VOLUME_OSC_ADDR, new_volume)?;
        *volume = new_volume;

        Ok(true)
    }

    fn decrease_volume_by_increment(&self, increment: f32) -> Result<bool> {
        if !self.initialized() {
            return Ok(false);
        }

        let mut volume = self.volume.lock();
        let mut new_volume = *volume - increment;
        if new_volume < 0.0 {
            new_volume = 0.0;
        }

        if floats::roughly_eq(new_volume, *volume) {
            return Ok(false);
        }

        self.send(VOLUME_OSC_ADDR, new_volume)?;
        *volume = new_volume;

        Ok(true)
    }
}
