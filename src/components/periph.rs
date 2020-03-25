use log::debug;
use std::fmt;
use std::rc::Rc;
use std::sync::Mutex;

use crate::components::clock;
use crate::components::cpu;

#[derive(Debug)]
pub enum Port {
    A,
    B,
}

pub trait Attachment {
    fn peek(&self, p: Port) -> u8;
    fn read(&mut self, p: Port) -> u8;
    fn write(&mut self, p: Port, val: u8);
}

#[allow(dead_code)]
pub struct W65C22 {
    pub port_a: Vec<(u8, Rc<Mutex<dyn Attachment>>)>,
    pub port_b: Vec<(u8, Rc<Mutex<dyn Attachment>>)>,
    pub orb: u8,
    pub ora: u8,
    pub ddrb: u8,
    pub ddra: u8,
    pub t1c: u16,
    pub t1l: u16,
    pub t2c: u16,
    pub sr: u8,
    pub acr: u8,
    pub pcr: u8,
    pub ifr: u8,
    pub ier: u8,
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
            port_a: Vec::new(),
            port_b: Vec::new(),
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

    pub fn attach_a(&mut self, mask: u8, device: Rc<Mutex<dyn Attachment>>) {
        self.port_a.push((mask, device));
    }

    pub fn attach_b(&mut self, mask: u8, device: Rc<Mutex<dyn Attachment>>) {
        self.port_b.push((mask, device));
    }

    fn set_interrupt(&mut self, i: Interrupts) {
        debug!("Set interrupt: {:?}", i);
        self.ifr |= i as u8;
        self.ifr |= 0x80;
    }

    fn clear_interrupt(&mut self, i: Interrupts) {
        debug!("Clear interrupt: {:?}", i);
        self.ifr &= !(i as u8);
        if self.ifr == 0x80 {
            self.ifr = 0;
        }
    }
}

#[derive(Debug)]
enum Interrupts {
    T1 = 0x40,
}

impl clock::Attachment for W65C22 {
    fn cycle(&mut self) {
        if self.t1c > 0 {
            self.t1c -= 1;
        } else {
            self.set_interrupt(Interrupts::T1);
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
    fn peek(&self, addr: u16) -> u8 {
        match addr {
            0x0 => {
                (self.orb & self.ddrb)
                    | (self
                        .port_b
                        .iter()
                        .map(|(mask, device)| device.lock().unwrap().peek(Port::B) & *mask)
                        .fold(0, |a, b| a | b)
                        & !self.ddrb)
            }
            0x1 => {
                unimplemented!();
            }
            0x2 => self.ddrb,
            0x3 => self.ddra,
            0x4 => {
                (self.t1c & 0x00ff) as u8
            }
            0x5 => (self.t1c >> 8) as u8,
            0x6 => {
                (self.t1l & 0x00ff) as u8
            }
            0x7 => (self.t1l >> 8) as u8,
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
            0xD => self.ifr,
            0xE => self.ier,
            0xF => {
                (self.ora & self.ddra)
                    | (self
                        .port_a
                        .iter()
                        .map(|(mask, device)| device.lock().unwrap().peek(Port::A) & *mask)
                        .fold(0, |a, b| a | b)
                        & !self.ddra)
            }
            _ => panic!("attempt to access invalid W65C22 register: {}", addr),
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        let data = match addr {
            0x0 => {
                (self.orb & self.ddrb)
                    | (self
                        .port_b
                        .iter_mut()
                        .map(|(mask, device)| device.lock().unwrap().read(Port::B) & *mask)
                        .fold(0, |a, b| a | b)
                        & !self.ddrb)
            }
            0x1 => {
                unimplemented!();
            }
            0x2 => self.ddrb,
            0x3 => self.ddra,
            0x4 => {
                self.clear_interrupt(Interrupts::T1);
                (self.t1c & 0x00ff) as u8
            }
            0x5 => (self.t1c >> 8) as u8,
            0x6 => {
                self.clear_interrupt(Interrupts::T1);
                (self.t1l & 0x00ff) as u8
            }
            0x7 => (self.t1l >> 8) as u8,
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
            0xD => self.ifr,
            0xE => self.ier,
            0xF => {
                (self.ora & self.ddra)
                    | (self
                        .port_a
                        .iter_mut()
                        .map(|(mask, device)| device.lock().unwrap().read(Port::A) & *mask)
                        .fold(0, |a, b| a | b)
                        & !self.ddra)
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
                let val = self.orb;
                self.port_b.iter_mut().for_each(|(mask, device)| {
                    device.lock().unwrap().write(Port::B, val & *mask);
                });
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
                self.clear_interrupt(Interrupts::T1);
            }
            0x6 => {
                self.t1l = (self.t1l & 0xff00) | (data as u16);
            }
            0x7 => {
                self.t1l = (self.t1l & 0x00ff) | ((data as u16) << 8);
                self.clear_interrupt(Interrupts::T1);
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
                self.ifr &= !data;
            }
            0xE => {
                self.ier = data;
            }
            0xF => {
                self.ora = data & self.ddra;
                let val = self.ora;
                self.port_a.iter_mut().for_each(|(mask, device)| {
                    device.lock().unwrap().write(Port::A, val & *mask);
                });
            }
            _ => panic!("attempt to access invalid W65C22 register: {}", addr),
        }
    }

    fn has_interrupt(&self) -> bool {
        self.ifr & self.ier != 0
    }
}
