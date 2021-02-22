//extern crate rouille;
use serde::Deserialize;
extern crate serde_json;

//use self::rouille::Response;
use crate::keyboard_controller::{ RGBL };
//use self::rouille::input::json::JsonError;
use crate::animation_controller::AnimationController;
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use rand::Rng;

pub struct CSController {
}

#[derive(Deserialize, Clone)]
struct Auth {
    token: String,
}

#[derive(Deserialize, Clone)]
struct CSRequest {
    auth: Auth,
    player: Player,
}

#[derive(Deserialize, Clone)]
struct Player {
    state: PlayerState,
}

#[derive(Deserialize, Clone)]
struct PlayerState {
    health:  u8,
    armor: u8,
    helmet: bool,
    flashed: u8,
    smoked: u8,
    money: u32,
    burning: u32,
    round_kills: u32,
    round_killhs: u32,
    equip_value: u32,
}

static mut LAST_PLAYER_STATE: PlayerState = PlayerState {
    health: 0,
    armor: 0,
    helmet: false,
    flashed: 0,
    smoked: 0,
    money: 0,
    burning: 0,
    round_kills: 0,
    round_killhs: 0,
    equip_value: 0
};

static mut RECENTLY_FLASHED: bool = false;

impl CSController {
    pub fn start(animation_controller: &mut AnimationController) {
        // rouille::start_server("0.0.0.0:3000", move |request| {
        //     match rouille::input::json_input::<CSRequest>(request) {
        //         Err(e) => {
        //             match e {
        //                 JsonError::BodyAlreadyExtracted => { println!("body already extracted") }
        //                 JsonError::WrongContentType => { println!("Wrong content type") }
        //                 JsonError::IoError(e) => { println!("{}", e) }
        //                 JsonError::ParseError(e) => { println!("{}", e) }
        //             }
        //         },
        //         Ok(r) => unsafe {
        //             if r.player.state.health < lastPlayerState.health {
        //                 //animation_controller.fade_out(RGBL{r: 255, g: 0, b: 0, l: 3}, 0);
        //             }
        //             lastPlayerState = r.player.state;
        //         }
        //     }
        //     Response::text("Ok")
        // });
        let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            CSController::handle_connection(stream, animation_controller);
        }
    }

    fn handle_connection(mut stream: TcpStream, animation_controller: &mut AnimationController) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let req_body = String::from_utf8_lossy(&buffer[..]).to_string();
        let json_r: Vec<&str> = req_body.split("\r\n\r\n").collect();
        let json = json_r[1].trim_matches(char::from(0));
        if json.len() <= 0 { return; }
        let req = serde_json::from_str::<CSRequest>(&*json);
        match req {
            Ok(p) => unsafe {
                if p.player.state.health <= 0 {
                    animation_controller.keyboard_controller.send_rgbl(RGBL {r: 255, g: 0, b: 0, l: 3});
                    return;
                }
                if p.player.state.health < LAST_PLAYER_STATE.health {
                    animation_controller.fade_out(RGBL {r: 255, g: 0, b: 0, l: 3}, 0);
                }
                if p.player.state.flashed > 0 && !RECENTLY_FLASHED {
                    animation_controller.fade_out( RGBL {r: 255, g: 255, b: 80, l: 0}, 10);
                    RECENTLY_FLASHED = true;
                }else {
                    if p.player.state.flashed > 0 {
                        RECENTLY_FLASHED = false;
                    }
                }
                if LAST_PLAYER_STATE.equip_value < p.player.state.equip_value {
                    animation_controller.fade_out( RGBL {r: 0, g: 0, b: 255, l: 3}, 1);
                }
                if p.player.state.round_killhs > LAST_PLAYER_STATE.round_killhs {
                    animation_controller.flash(RGBL {r: 0, g: 0, b: 0, l: 3}, RGBL {r: 0, g: 255, b: 0, l: 0}, 100, 10);
                } else if p.player.state.round_kills > LAST_PLAYER_STATE.round_kills {
                    animation_controller.flash(RGBL {r: 0, g: 0, b: 0, l: 3}, RGBL {r: 0, g: 255, b: 0, l: 3}, 100, 3);
                }
                if p.player.state.burning > 0 {
                    animation_controller.keyboard_controller.send_rgbl(RGBL {r: 255, g: 40, b: 0, l: 3});
                } else {
                    animation_controller.keyboard_controller.send_rgbl( RGBL {r: 0, g: 0, b: 0, l: 3});
                }
                LAST_PLAYER_STATE = p.player.state;
            },
            Err(_e) => {
                let mut rng = rand::thread_rng();
                let r = rng.gen_range(0..105);
                animation_controller.keyboard_controller.send_rgbl( RGBL {r: 0, g: 0, b: 150 + r, l: 3} );
                //println!("{}\nRequest: {}", e, json.len());
                // Request without player
            }
        }
    }
}