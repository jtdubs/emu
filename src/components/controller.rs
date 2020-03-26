use crate::components::periph;
use log::{debug, info};

pub enum Buttons {
    A = 0x01,
    B = 0x02,
    Select = 0x04,
    Start = 0x08,
    Up = 0x10,
    Down = 0x20,
    Left = 0x40,
    Right = 0x80
}

#[derive(Debug)]
pub struct SNESController {
}

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

    fn read(&mut self, p: periph::Port) -> u8 {
        info!("R {:?}", p);
        // A B SEL STA U D L R
        // Zero == pressed
        0x00
    }

    fn write(&mut self, p: periph::Port, val: u8) {
        info!("W {:?} = {:?}", p, val);
    }
}

// W A = 2
// W A = 0
// R A // A
// R A
// W A = 4
// W A = 0
// R A // B
// R A
// W A = 4
// W A = 0
// R A // SEL
// R A
// W A = 4
// W A = 0
// R A // STA
// R A
// W A = 4
// W A = 0
// R A // UP
// R A
// W A = 4
// W A = 0
// R A // DOWN
// R A
// W A = 4
// W A = 0
// R A // LET
// R A
// W A = 4
// W A = 0
// R A // RIGHT
// R A
// W A = 4
// W A = 0
// W A = 0
// W A = 0
// W A = 0
