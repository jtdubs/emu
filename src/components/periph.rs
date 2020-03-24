use log::{debug,info};
use std::fmt;
use std::rc::Rc;
use std::sync::Mutex;

use crate::components::clock;
use crate::components::cpu;

#[derive(Debug)]
pub enum Port { A, B }

pub trait Attachment {
    fn read(&mut self, p : Port) -> u8;
    fn write(&mut self, p : Port, val : u8);
}

#[allow(dead_code)]
pub struct W65C22 {
    port_a: Option<Rc<Mutex<dyn Attachment>>>,
    port_b: Option<Rc<Mutex<dyn Attachment>>>,
    orb: u8,
    ora: u8,
    ddrb: u8,
    ddra: u8,
    t1c: u16,
    t1l: u16,
    t2c: u16,
    sr: u8,
    acr: u8,
    pcr: u8,
    ifr: u8,
    ier: u8,
}

impl fmt::Debug for W65C22 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("W65C22")
            .field("orb", &self.orb)
            .field("ora", &self.ora)
            .field("ddrb", &self.ddrb)
            .field("ddra", &self.ddra)
            .finish()
    }
}

impl W65C22 {
    pub fn new() -> W65C22 {
        W65C22 {
            port_a: None,
            port_b: None,
            orb: 0,
            ora: 0,
            ddrb: 0,
            ddra: 0,
            t1c: 0,
            t1l: 0,
            t2c: 0,
            sr: 0,
            acr: 0,
            pcr: 0,
            ifr: 0,
            ier: 0,
        }
    }

    #[allow(dead_code)]
    pub fn attach_a(&mut self, device: Rc<Mutex<dyn Attachment>>) {
        self.port_a = Some(device);
    }

    #[allow(dead_code)]
    pub fn attach_b(&mut self, device: Rc<Mutex<dyn Attachment>>) {
        self.port_b = Some(device);
    }
}

enum Interrupts {
    T1 = 0x40
}

impl clock::Attachment for W65C22 {
    fn cycle(&mut self) {
        if self.t1c > 0 {
            self.t1c -= 1;
        } else {
            info!("T1 Fire");
            self.ifr |= Interrupts::T1 as u8;
            match self.acr >> 6 {
                0 => {
                    // single-shot mode
                    // nothing to do
                }
                1 => {
                    // free-run mode
                    // reset timer to latched value
                    self.t1c = self.t1l;
                }
                2 => {
                    unimplemented!("timer1 - one shot pb7 mode");
                }
                3 => {
                    unimplemented!("timer1 - square wave mode");
                }
                _ => {
                    panic!("impossible value for self.acr");
                }
            }
            // TODO: if T1 is set in IER, raise the interrupt
        }
    }
}

impl cpu::Attachment for W65C22 {
    fn read(&mut self, addr: u16) -> u8 {
        let data = match addr {
            0x0 => {
                if let Some(b) = &self.port_b {
                    (self.orb & self.ddrb) | (b.lock().unwrap().read(Port::B) & !self.ddrb)
                } else {
                    self.orb
                }
            }
            0x1 => {
                unimplemented!();
            }
            0x2 => self.ddrb,
            0x3 => self.ddra,
            0x4 => {
                self.ifr &= !(Interrupts::T1 as u8);
                (self.t1c & 0x00ff) as u8
            }
            0x5 => {
                (self.t1c >> 8) as u8
            }
            0x6 => {
                (self.t1l & 0x00ff) as u8
            }
            0x7 => {
                (self.t1l >> 8) as u8
            }
            0x8 => {
                unimplemented!("W65C22 - Read T2C_L");
            }
            0x9 => {
                unimplemented!("W65C22 - Read T2C_H");
            }
            0xA => {
                unimplemented!("W65C22 - Read SR");
            }
            0xB => {
                unimplemented!("W65C22 - Read ACR");
            }
            0xC => {
                unimplemented!("W65C22 - Read PCR");
            }
            0xD => {
                unimplemented!("W65C22 - Read IFR");
            }
            0xE => {
                unimplemented!("W65C22 - Read IER");
            }
            0xF => {
                if let Some(a) = &self.port_a {
                    (self.ora & self.ddra) | (a.lock().unwrap().read(Port::A) & !self.ddra)
                } else {
                    self.ora
                }
            }
            _ => panic!("attempt to access invalid W65C22 register: {}", addr),
        };
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    fn write(&mut self, addr: u16, data: u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);
        match addr {
            0x0 => {
                self.orb = data & self.ddrb;
                if let Some(b) = &mut self.port_b {
                    b.lock().unwrap().write(Port::B, self.orb);
                }
            }
            0x1 => {
                unimplemented!("W65C22 - Access to ORA w/ handshake");
            }
            0x2 => {
                self.ddrb = data;
            }
            0x3 => {
                self.ddra = data;
            }
            0x4 => {
                self.t1l = (self.t1l & 0xff00) | (data as u16);
            }
            0x5 => {
                self.t1l = (self.t1l & 0x00ff) | ((data as u16) << 8);
                self.t1c = self.t1l;
                self.ifr &= !(Interrupts::T1 as u8);
            }
            0x6 => {
                self.t1l = (self.t1l & 0xff00) | (data as u16);
            }
            0x7 => {
                self.t1l = (self.t1l & 0x00ff) | ((data as u16) << 8);
                self.ifr &= !(Interrupts::T1 as u8);
            }
            0x8 => {
                unimplemented!("W65C22 - Write T2C_H");
            }
            0x9 => {
                unimplemented!("W65C22 - Write T2C_H");
            }
            0xA => {
                unimplemented!("W65C22 - Write SR");
            }
            0xB => {
                self.acr = data;
            }
            0xC => {
                unimplemented!("W65C22 - Write PCR");
            }
            0xD => {
                unimplemented!("W65C22 - Write IFR");
            }
            0xE => {
                self.ier = data;
            }
            0xF => {
                self.ora = data & self.ddra;
                if let Some(a) = &mut self.port_a {
                    a.lock().unwrap().write(Port::A, self.ora);
                }
            }
            _ => panic!("attempt to access invalid W65C22 register: {}", addr),
        }
    }
}
