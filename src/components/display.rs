use std::fmt;
use std::sync::Mutex;
use std::rc::Rc;
use log::debug;

use crate::components::clock;
use crate::components::periph;

#[derive(Debug)]
pub enum State {
    Idle,
    Busy(usize),
}

#[derive(Debug)]
pub enum RegisterSelector {
    Instruction = 0,
    Data = 1,
}

pub struct HD44780U {
    state: State,
    addr: u8,
    buffer: Vec<u8>
}

impl fmt::Debug for HD44780U {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HD44780U")
            .field("state", &self.state)
            .field("addr", &self.addr)
            .field("line1", &String::from_utf8_lossy(&self.buffer[..40]))
            .field("line2", &String::from_utf8_lossy(&self.buffer[40..]))
            .finish()
    }
}

impl HD44780U {
    pub fn new() -> HD44780U {
        let mut buffer = Vec::new();
        buffer.resize(80, ' ' as u8);

        HD44780U {
            // state: State::Busy(15000),
            state: State::Busy(150),
            addr: 0,
            buffer: buffer
        }
    }

    pub fn read(&self, addr: RegisterSelector) -> u8 {
        match addr {
            RegisterSelector::Instruction => {
                let mut result = self.addr;
                if let State::Busy(_) = self.state {
                    result |= 0x80;
                }
                debug!("R {:?} = {:02x}", addr, result);
                result
            }
            RegisterSelector::Data => {
                let result = self.buffer[self.addr as usize];
                debug!("R {:?} = {:02x}", addr, result);
                result
            }
        }
    }

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


#[derive(Debug)]
pub struct HD44780UAdapter {
    a_cache : u8,
    b_cache : u8,
    dsp : Option<Rc<Mutex<HD44780U>>>,
}

const RS : u8 = 0x20;
const RW : u8 = 0x40;
const E : u8 = 0x80;

impl HD44780UAdapter {
    pub fn new() -> HD44780UAdapter {
        HD44780UAdapter {
            a_cache: 0,
            b_cache: 0,
            dsp: None
        }
    }

    pub fn attach(&mut self, dsp: Rc<Mutex<HD44780U>>) {
        self.dsp = Some(dsp);
    }
}

fn get_control(a: u8) -> (RegisterSelector, bool, bool) {
    (if a & RS == RS { RegisterSelector::Data } else { RegisterSelector::Instruction },
        a & RW == RW,
        a & E == E)
}

impl periph::Attachment for HD44780UAdapter {
    fn read(&self, p : periph::Port) -> u8 {
        debug!("R {:?}", p);

        if let Some(dsp) = &self.dsp {
            match p {
                periph::Port::A => {
                    0u8
                }
                periph::Port::B => {
                    dsp.lock().unwrap().read(get_control(self.a_cache).0)
                }
            }
        } else {
            0u8
        }
    }

    fn write(&mut self, p : periph::Port, val: u8) {
        debug!("W {:?} = {:02x}", p, val);

        if let Some(dsp) = &self.dsp {
            match p {
                periph::Port::A => {
                    match (get_control(self.a_cache).2, get_control(val).2) {
                        (true, false) => {
                            dsp.lock().unwrap().write(get_control(val).0, self.b_cache);
                        }
                        _ => { }
                    }
                    self.a_cache = val;
                }
                periph::Port::B => {
                    self.b_cache = val;
                }
            }
        }
    }
}
