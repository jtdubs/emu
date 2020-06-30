use log::debug;
use std::cell::Cell;
use std::fmt;

#[derive(Debug)]
pub enum Port {
    A,
    B,
}

#[derive(Debug)]
enum Interrupts {
    T1 = 0x40,
}


pub enum PortOperation {
    Read(Port),
    Write(Port, u8),
    Peek(Port),
}

pub trait PortArbiter {
    fn port(&mut self, op: PortOperation) -> u8;
}

#[allow(dead_code)]
pub struct W65C22 {
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
    pub ifr: Cell<u8>,
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
            ifr: Cell::new(0),
            ier: 0,
        }
    }

    fn set_interrupt(&self, i: Interrupts) {
        debug!("Set interrupt: {:?}", i);
        let mut ifr = self.ifr.get();
        ifr |= i as u8;
        ifr |= 0x80;
        self.ifr.replace(ifr);
    }

    fn clear_interrupt(&self, i: Interrupts) {
        debug!("Clear interrupt: {:?}", i);
        let mut ifr = self.ifr.get();
        ifr &= !(i as u8);
        if ifr == 0x80 {
            ifr = 0;
        }
        self.ifr.replace(ifr);
    }

    pub fn cycle(&mut self) -> bool {
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

        (self.ifr.get() & self.ier) != 0
    }

    pub fn peek(&self, addr: u16, ports: &mut impl PortArbiter) -> u8 {
        match addr {
            0x0 => (self.orb & self.ddrb) | (ports.port(PortOperation::Peek(Port::B)) & !self.ddrb),
            0x1 => {
                unimplemented!();
            }
            0x2 => self.ddrb,
            0x3 => self.ddra,
            0x4 => (self.t1c & 0x00ff) as u8,
            0x5 => (self.t1c >> 8) as u8,
            0x6 => (self.t1l & 0x00ff) as u8,
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
            0xD => self.ifr.get(),
            0xE => self.ier,
            0xF => {
                (self.ora & self.ddra)
                    | (ports.port(PortOperation::Peek(Port::A)) & !self.ddra)
            }
            _ => panic!("attempt to access invalid W65C22 register: {}", addr),
        }
    }

    pub fn read(&self, addr: u16, ports: &mut impl PortArbiter) -> u8 {
        let data = match addr {
            0x0 => (self.orb & self.ddrb) | (ports.port(PortOperation::Read(Port::B)) & !self.ddrb),
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
            0xD => self.ifr.get(),
            0xE => self.ier,
            0xF => {
                (self.ora & self.ddra)
                    | (ports.port(PortOperation::Read(Port::A)) & !self.ddra)
            }
            _ => panic!("attempt to access invalid W65C22 register: {}", addr),
        };
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    pub fn write(&mut self, addr: u16, data: u8, ports: &mut impl PortArbiter) {
        debug!("W @ {:04x} = {:02x}", addr, data);
        match addr {
            0x0 => {
                self.orb = data & self.ddrb;
                ports.port(PortOperation::Write(Port::B, self.orb));
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
                *self.ifr.get_mut() &= !data;
            }
            0xE => {
                self.ier = data;
            }
            0xF => {
                self.ora = data & self.ddra;
                ports.port(PortOperation::Write(Port::A, self.ora));
            }
            _ => panic!("attempt to access invalid W65C22 register: {}", addr),
        }
    }
}
