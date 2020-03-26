use crate::components::periph;
use log::{debug, info};

#[derive(Debug)]
pub struct SNESController {}

impl SNESController {
    pub fn new() -> SNESController {
        SNESController {}
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'w' => {
                info!("on_key: UP");
            }
            's' => {
                info!("on_key: DOWN");
            }
            'a' => {
                info!("on_key: LEFT");
            }
            'd' => {
                info!("on_key: RIGHT");
            }
            'j' => {
                info!("on_key: A");
            }
            'k' => {
                info!("on_key: B");
            }
            'l' => {
                info!("on_key: SELECT");
            }
            ';' => {
                info!("on_key: START");
            }
            _ => {}
        }
    }
}

impl periph::Attachment for SNESController {
    fn peek(&self, _p: periph::Port) -> u8 {
        // 0xff
        0x00
    }

    fn read(&mut self, _p: periph::Port) -> u8 {
        // 0xff
        0x00
    }

    fn write(&mut self, _p: periph::Port, _val: u8) {}
}
