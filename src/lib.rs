extern crate i2cdev;
#[macro_use] extern crate log;

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

/// The `EqualizerMode` type denotes different equalizer modes.
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

/// The `PlayStatus` type describes the state of the mp3 trigger.
#[derive(Debug, PartialEq)]
pub enum PlayStatus {
    Playing,
    Stopped,
    Unknown,
}

/// The `CardStatus` type indicates if an SD card is present.
#[derive(Debug, PartialEq)]
pub enum CardStatus {
    Good,
    Bad,
}

/// This object communicates with a Sparkfun Qwiic MP3 Trigger over i2c.
#[derive(Debug)]
pub struct QwiicMP3Trigger<T: I2CDevice + Sized> {
    i2cdev: T,
}

impl<T> QwiicMP3Trigger<T>
    where T: I2CDevice + Sized
{
    /// Constructs a new `QwiicMP3Trigger<T>`.
    pub fn new(i2cdev: T) -> Result<QwiicMP3Trigger<T>, T::Error> {
        Ok(QwiicMP3Trigger { i2cdev: i2cdev })
    }

    /// Tests communication.
    pub fn ping(&mut self) -> Result<(), T::Error> {
        debug!("ping()");
        self.send_data(&[COMMAND_GET_VERSION])
    }

    /// Returns the number of songs on the SD card, including trigger songs.
    pub fn get_song_count(&mut self) -> Result<u8, T::Error> {
        debug!("get_song_count()");
        self.send_data(&[COMMAND_GET_SONG_COUNT])?;
        read_delay();
        Ok(self.read_response()?[0])
    }

    /// Returns the version of the Qwiic MP3 Trigger firmware.
    pub fn get_version(&mut self) -> Result<String, T::Error> {
        debug!("get_version()");
        self.send_data(&[COMMAND_GET_VERSION])?;
        read_delay();
        let r = self.read_response()?;
        Ok(format!("{}.{}", r[0], r[1]).clone())
    }

    /// Plays a track based on the sorting rules of the WT2003S.
    pub fn play_track(&mut self, track: u8) -> Result<(), T::Error> {
        debug!("play_track({})", track);
        self.send_data(&[COMMAND_PLAY_TRACK, track])
    }

    /// Plays a track based on the filename.  `3` will play a file matching `F003***.mp3`.
    pub fn play_filenumber(&mut self, filenumber: u8) -> Result<(), T::Error> {
        debug!("play_filenumber({})", filenumber);
        self.send_data(&[COMMAND_PLAY_FILENUMBER, filenumber])
    }

    /// Plays the next track based on the sorting rules of the WT2003S.
    pub fn play_next(&mut self) -> Result<(), T::Error> {
        debug!("play_next()");
        self.send_data(&[COMMAND_PLAY_NEXT])
    }

    /// Plays the previous track based on the sorting rules of the WT2003S.
    pub fn play_previous(&mut self) -> Result<(), T::Error> {
        debug!("play_previous()");
        self.send_data(&[COMMAND_PLAY_PREVIOUS])
    }

    /// Stops playing.  Note that this may result in buzzing because the audio output it
    /// not disabled.  You may wish to play a long silence instead.
    pub fn stop(&mut self) -> Result<(), T::Error> {
        debug!("stop()");
        self.send_data(&[COMMAND_STOP])
    }

    /// Pauses playing.  Note that this may result in buzzing because the audio output it
    /// not disabled.  You may wish to play a long silence instead.
    pub fn pause(&mut self) -> Result<(), T::Error> {
        debug!("pause()");
        self.send_data(&[COMMAND_PAUSE])
    }

    /// Returns the play status.
    pub fn get_play_status(&mut self) -> Result<PlayStatus, T::Error> {
        debug!("get_play_status()");
        self.send_data(&[COMMAND_GET_PLAY_STATUS])?;
        read_delay();
        match self.read_response()?[0] {
            1 => Ok(PlayStatus::Playing),
            2 => Ok(PlayStatus::Stopped),
            _ => Ok(PlayStatus::Unknown),
        }
    }

    /// Checks the status of the SD card.
    pub fn get_card_status(&mut self) -> Result<CardStatus, T::Error> {
        debug!("get_card_status()");
        self.send_data(&[COMMAND_GET_CARD_STATUS])?;
        read_delay();
        match self.read_response()?[0] {
            0 => Ok(CardStatus::Bad),
            1 => Ok(CardStatus::Good),
            _ => Ok(CardStatus::Bad),
        }
    }

    /// Returns the name of the currently playing song.  May return `unable to parse`.
    pub fn get_song_name(&mut self) -> Result<String, T::Error> {
        debug!("get_song_name()");
        self.send_data(&[COMMAND_GET_SONG_NAME])?;
        read_delay();
        let string = String::from_utf8(self.read_response()?.to_vec());
        match string {
            Ok(s) => Ok(s),
            Err(_) => Ok(String::from("unable to parse")),
        }
    }

    /// Set the equalizer mode.
    pub fn set_eq(&mut self, mode: EqualizerMode) -> Result<(), T::Error> {
        debug!("set_eq({:?})", mode);
        self.send_data(&[COMMAND_SET_EQ, mode as u8])
    }

    /// Set the volume between 0 (off) to 31 (max).
    pub fn set_volume(&mut self, level: u8) -> Result<(), T::Error> {
        debug!("set_volume({})", level);
        self.send_data(&[COMMAND_SET_VOLUME, level])
    }

    /// Set the i2c address of the device.
    pub fn set_address(&mut self, address: u8) -> Result<(), T::Error> {
        debug!("set_address({})", address);
        self.send_data(&[COMMAND_SET_ADDRESS, address])
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), T::Error> {
        trace!("Sending: {:?}", data);
        self.i2cdev.write(data)?;
        trace!("Sent");
        Ok(())
    }

    fn read_response(&mut self) -> Result<[u8; 8], T::Error> {
        let mut buf: [u8; 8] = [0; 8];
        self.i2cdev.read(&mut buf)?;
        trace!("Reading: {:?}", buf);
        Ok(buf)
    }
}
