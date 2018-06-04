extern crate i2cdev;

use i2cdev::core::I2CDevice;
use std::{thread, time};

const COMMAND_STOP: u8 = 0x00;
const COMMAND_PLAY_TRACK: u8 = 0x01;
const COMMAND_PLAY_FILENUMBER: u8 = 0x02;
const COMMAND_PAUSE: u8 = 0x03;
const COMMAND_PLAY_NEXT: u8 = 0x04;
const COMMAND_PLAY_PREVIOUS: u8 = 0x05;
const COMMAND_SET_EQ: u8 = 0x06;
const COMMAND_SET_VOLUME: u8 = 0x07;
const COMMAND_GET_SONG_COUNT: u8 = 0x08;
const COMMAND_GET_SONG_NAME: u8 = 0x09;
const COMMAND_GET_PLAY_STATUS: u8 = 0x0A;
const COMMAND_GET_CARD_STATUS: u8 = 0x0B;
const COMMAND_GET_VERSION: u8 = 0x0C;
const COMMAND_SET_ADDRESS: u8 = 0xC7;

fn read_delay() {
    thread::sleep(time::Duration::from_millis(300));
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum EqualizerMode {
    Normal = 0,
    Pop = 1,
    Rock = 2,
    Jazz = 3,
    Classical = 4,
    Bass = 5,
}

#[derive(Debug, PartialEq)]
pub enum PlayStatus {
    Playing,
    Stopped,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum CardStatus {
    Good,
    Bad,
}

#[derive(Debug)]
pub struct QwiicMP3Trigger<T: I2CDevice + Sized> {
    i2cdev: T,
}

impl<T> QwiicMP3Trigger<T>
    where T: I2CDevice + Sized
{
    pub fn new(i2cdev: T) -> Result<QwiicMP3Trigger<T>, T::Error> {
        Ok(QwiicMP3Trigger { i2cdev: i2cdev })
    }

    pub fn ping(&mut self) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_GET_VERSION])
    }

    pub fn get_song_count(&mut self) -> Result<u8, T::Error> {
        self.send_data(&[COMMAND_GET_SONG_COUNT])?;
        read_delay();
        Ok(self.read_response()?[0])
    }

    pub fn get_version(&mut self) -> Result<String, T::Error> {
        self.send_data(&[COMMAND_GET_VERSION])?;
        read_delay();
        let r = self.read_response()?;
        Ok(format!("{}.{}", r[0], r[1]).clone())
    }

    pub fn play_track(&mut self, track: u8) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_PLAY_TRACK, track])
    }

    pub fn play_filenumber(&mut self, filenumber: u8) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_PLAY_FILENUMBER, filenumber])
    }

    pub fn play_next(&mut self) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_PLAY_NEXT])
    }

    pub fn play_previous(&mut self) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_PLAY_PREVIOUS])
    }

    pub fn stop(&mut self) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_STOP])
    }

    pub fn pause(&mut self) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_PAUSE])
    }

    pub fn get_play_status(&mut self) -> Result<PlayStatus, T::Error> {
        self.send_data(&[COMMAND_GET_PLAY_STATUS])?;
        read_delay();
        match self.read_response()?[0] {
            1 => Ok(PlayStatus::Playing),
            2 => Ok(PlayStatus::Stopped),
            _ => Ok(PlayStatus::Unknown),
        }
    }

    pub fn get_card_status(&mut self) -> Result<CardStatus, T::Error> {
        self.send_data(&[COMMAND_GET_CARD_STATUS])?;
        read_delay();
        match self.read_response()?[0] {
            0 => Ok(CardStatus::Bad),
            1 => Ok(CardStatus::Good),
            _ => Ok(CardStatus::Bad),
        }
    }

    pub fn get_song_name(&mut self) -> Result<String, T::Error> {
        self.send_data(&[COMMAND_GET_SONG_NAME])?;
        read_delay();
        let string = String::from_utf8(self.read_response()?.to_vec());
        match string {
            Ok(s) => Ok(s),
            Err(_) => Ok(String::from("unable to parse")),
        }
    }

    pub fn set_eq(&mut self, mode: EqualizerMode) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_SET_EQ, mode as u8])
    }

    /// Volume can be 0 (off) to 31 (max)
    pub fn set_volume(&mut self, level: u8) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_SET_VOLUME, level])
    }

    pub fn set_address(&mut self, address: u8) -> Result<(), T::Error> {
        self.send_data(&[COMMAND_SET_ADDRESS, address])
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), T::Error> {
        println!("Sending: {:?}", data);
        self.i2cdev.write(data)?;
        println!("Sent");
        Ok(())
    }

    fn read_response(&mut self) -> Result<[u8; 8], T::Error> {
        let mut buf: [u8; 8] = [0; 8];
        self.i2cdev.read(&mut buf)?;
        println!("Reading: {:?}", buf);
        Ok(buf)
    }
}
