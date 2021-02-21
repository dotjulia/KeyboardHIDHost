extern crate hidapi;

use hidapi::HidApi;
use self::hidapi::{HidDevice};
use std::fmt::{Display, Formatter, Error};

const VID: u16 = 0x7432;
const PID: u16 = 0x0658;
const USAGE_PAGE: u16 = 0xFF60;
const USAGE: u16 = 0x61;

pub struct KeyboardController {
    keyboard: HidDevice,
    pub last_state: RGBL,
}

#[derive(Clone, Copy)]
pub struct RGBL {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub l: u8,
}

impl Display for RGBL {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[r: {}, g: {}, b: {}, l: {}]", self.r, self.g, self.b, self.l)
    }
}

impl KeyboardController {
    fn find_keyboard() -> Result<HidDevice, &'static str>{
        return match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    if device.vendor_id() == VID && device.product_id() == PID && device.usage() == USAGE && device.usage_page() == USAGE_PAGE {
                        match api.open_path(device.path()) {
                            Ok(hd) => {
                                return Ok(hd);
                            },
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            },
                        };
                        return Err("Failed to open found device path");
                    }
                }
                Err("Device not found!")
            },
            Err(e) => {
                println!("HidError: {}", e);
                Err("HidError")
            },
        }
    }

    pub fn send_rgbl(&mut self, rgbl: RGBL) {
        // report id, r, g, b, light level (3 = off, 0 = on)
        self.last_state = rgbl;
        match self.keyboard.write(&[0, rgbl.r, rgbl.g, rgbl.b, rgbl.l]) {
            Err(e) => {
                println!("Error: {}", e);
            }
            _ => {}
        };
    }

    pub fn new() -> Result<KeyboardController, &'static str> {
        match KeyboardController::find_keyboard() {
            Ok(hd) => {
                Ok(KeyboardController { keyboard: hd, last_state: RGBL {r: 0, g: 0, b: 0, l: 3} })
            },
            Err(e) => {
                Err(e)
            },
        }
    }
}