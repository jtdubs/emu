use log::debug;

use crate::components::clock;
use crate::components::periph;

#[derive(Debug)]
pub enum State {
    Idle,
    Busy(usize),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RegisterSelector {
    Instruction = 0,
    Data = 1,
}

#[derive(Debug)]
pub struct HD44780U {
    state: State,
    addr: u8,
}

impl HD44780U {
    pub fn new() -> HD44780U {
        HD44780U {
            state: State::Busy(15000),
            addr: 0,
        }
    }

    #[allow(dead_code)]
    pub fn read(&self, addr: RegisterSelector) -> u8 {
        let mut data = self.addr;
        if let State::Busy(_) = self.state {
            data |= 0x80;
        }
        debug!("R {:?} = {:02x}", addr, data);
        data
    }

    #[allow(dead_code)]
    pub fn write(&mut self, addr: RegisterSelector, val: u8) {
        debug!("W {:?} = {:02x}", addr, val);
        unimplemented!();
    }
}

impl clock::Attachment for HD44780U {
    fn cycle(&mut self) {
        debug!("DSP: {:x?}", self);

        self.state = match self.state {
            State::Idle => State::Idle,
            State::Busy(0) => State::Idle,
            State::Busy(c) => State::Busy(c - 1),
        };
    }
}

impl periph::Attachment for HD44780U {
    fn read(&self) -> u8 {
        unimplemented!();
    }

    fn write(&self, _val: u8) {
        unimplemented!();
    }
}
