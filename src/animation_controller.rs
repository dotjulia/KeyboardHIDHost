use crate::keyboard_controller::{KeyboardController, RGBL};
use std::thread::sleep;
use std::time::Duration;

pub struct AnimationController {
    pub keyboard_controller: KeyboardController
}

impl AnimationController {
    pub fn new(keyboard_controller: KeyboardController) -> AnimationController {
        AnimationController { keyboard_controller }
    }

    /**
     * Flashes from from to to state
     * After finished, keyboard remains in from state
     */
    pub fn flash(&mut self, from: RGBL, to: RGBL, delay_ms: u64, count: u16) {
        self.keyboard_controller.send_rgbl(from);
        for i in 0..count {
            sleep(Duration::from_millis(delay_ms));
            if i%2==0 {
                self.keyboard_controller.send_rgbl(to);
            } else {
                self.keyboard_controller.send_rgbl(from);
            }
        }
    }

    #[allow(dead_code)]
    pub fn fade_flash(&mut self, to: RGBL, delay_ms: u64, count: u16) {
        self.keyboard_controller.send_rgbl(RGBL { r: 0, g: 0, b: 0, l: 3});
        for _i in 0..count {
                self.fade_in(to, delay_ms);
                self.fade_out(to, delay_ms);
        }
    }

    pub fn fade_out(&mut self, from: RGBL, delay_ms: u64) {
        for i in 0..256 {
            let l = if from.l == 3 { 3 } else { match i {
                0..=85 => 0,
                86..=170 => 1,
                171..=255 => 2,
                _ => 3,
            }};
            let rgbl = RGBL {r: (from.r as f32 * ((255.0-i as f32)/255.0)) as u8, g: (from.g as f32 * ((255.0-i as f32)/255.0)) as u8, b: (from.b as f32 * ((255.0-i as f32)/255.0)) as u8, l};
            self.keyboard_controller.send_rgbl(rgbl);
            if delay_ms > 0 {
                sleep(Duration::from_millis(delay_ms));
            }
        }
        self.keyboard_controller.send_rgbl(RGBL {r: 0, g: 0, b: 0, l: 3});
    }

    pub fn fade_in(&mut self, to: RGBL, delay_ms: u64) {
        for i in 0..256 {
            let l = if to.l == 3 { 3 } else { match i {
                0..=85 => 3,
                86..=170 => 2,
                171..=255 => 1,
                _ => 0,
            }};
            let rgbl = RGBL {r: (to.r as f32 * ((i as f32)/255.0)) as u8, g: (to.g as f32 * ((i as f32)/255.0)) as u8, b: (to.b as f32 * ((i as f32)/255.0)) as u8, l};
            self.keyboard_controller.send_rgbl(rgbl);
            if delay_ms > 0 {
                sleep(Duration::from_millis(delay_ms));
            }
        }
        self.keyboard_controller.send_rgbl(to);
    }
}