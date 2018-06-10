# rust-qwiic-mp3-trigger

[![Build status](https://img.shields.io/travis/querry43/rust-qwiic-mp3-trigger.svg)](https://travis-ci.org/querry43/rust-qwiic-mp3-trigger)
[![License](https://img.shields.io/github/license/querry43/rust-qwiic-mp3-trigger.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/qwiic-mp3-trigger.svg)](https://crates.io/crates/qwiic-mp3-trigger)
[![Documentation](https://docs.rs/qwiic-mp3-trigger/badge.svg)](https://docs.rs/qwiic-mp3-trigger)

A rust crate for the Sparkfun Qwiic MP3 Trigger (https://www.sparkfun.com/products/14714).

## Synopsys

```rust,no_run
extern crate i2cdev;
extern crate qwiic_mp3_trigger;

use i2cdev::linux::LinuxI2CDevice;

fn main() {
    let i2cdev = LinuxI2CDevice::new("/dev/i2c-1", 0x37).unwrap();
    let mut mp3_trigger = qwiic_mp3_trigger::QwiicMP3Trigger::new(i2cdev).unwrap();

    mp3_trigger.ping().unwrap();

    let version: String = mp3_trigger.get_version().unwrap();
    let song_count: u8 = mp3_trigger.get_song_count().unwrap();
    let card_status: qwiic_mp3_trigger::CardStatus = mp3_trigger.get_card_status().unwrap();

    mp3_trigger.play_track(1).unwrap();
    mp3_trigger.play_filenumber(1).unwrap();
    mp3_trigger.play_next().unwrap();
    mp3_trigger.play_previous().unwrap();

    mp3_trigger.stop().unwrap();
    mp3_trigger.pause().unwrap();

    let song_name: String = mp3_trigger.get_song_name().unwrap();
    let play_status: qwiic_mp3_trigger::PlayStatus = mp3_trigger.get_play_status().unwrap();

    mp3_trigger.set_eq(qwiic_mp3_trigger::EqualizerMode::Bass).unwrap();
    mp3_trigger.set_volume(31).unwrap();

    mp3_trigger.set_address(0x88).unwrap();
}
```

## Configuring i2c

The raspberry pi is a good deal faster than the attiny85 in this module.  Communication is flakey unless you slow down the rate.  Add the following to `/boot/config.txt` and reboot.

```
dtparam=i2c_baudrate=50000
```

## Debugging

This crate uses the log crate for `debug!` and `trace!` logging.  Enable it by initializing a logger such as [simple_logger](https://github.com/borntyping/rust-simple_logger).
