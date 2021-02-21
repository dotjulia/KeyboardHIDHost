use crate::keyboard_controller::{KeyboardController};
use crate::animation_controller::AnimationController;
use crate::cs_controller::CSController;

mod keyboard_controller;
mod cs_controller;
mod animation_controller;

fn main() {
    let keyboard_controller: KeyboardController = match KeyboardController::new() {
        Ok(kc) => kc,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        },
    };
    let mut animation_controller: AnimationController = AnimationController::new(keyboard_controller);
    CSController::start(&mut animation_controller);
}
