use std::fmt;
use log::debug;

use crate::cpu;

pub trait PortAttachment {
    // clock cycle
    fn step(&mut self);

    // port read/write
    fn read(&self) -> u8;
    fn write(&self, val : u8);
}

pub struct W65C22 {
    port_a : Option<Box<dyn PortAttachment>>,
    port_b : Option<Box<dyn PortAttachment>>,
    orb    : u8,
    ora    : u8,
    ddrb   : u8,
    ddra   : u8,
    t1c_l  : u8,
    t1c_h  : u8,
    t1l_l  : u8,
    t1l_h  : u8,
    t2c_l  : u8,
    t2c_h  : u8,
    sr     : u8,
    acr    : u8,
    pcr    : u8,
    ifr    : u8,
    ier    : u8,
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
            orb:    0,
            ora:    0,
            ddrb:   0,
            ddra:   0,
            t1c_l:  0,
            t1c_h:  0,
            t1l_l:  0,
            t1l_h:  0,
            t2c_l:  0,
            t2c_h:  0,
            sr:     0,
            acr:    0,
            pcr:    0,
            ifr:    0,
            ier:    0
        }
    }

    #[allow(dead_code)]
    pub fn attach_a(&mut self, device: Box<dyn PortAttachment>) {
        self.port_a = Some(device);
    }

    #[allow(dead_code)]
    pub fn attach_b(&mut self, device: Box<dyn PortAttachment>) {
        self.port_b = Some(device);
    }
}

impl cpu::Attachment for W65C22 {
    fn step(&mut self) {
        debug!("PER: {:x?}", self);

        if let Some(a) = &mut self.port_a {
            a.step();
        }
        if let Some(b) = &mut self.port_b {
            b.step();
        }
    }

    fn read(&self, addr : u16) -> u8 {
        let data = match addr {
            0x0 => {
                if let Some(b) = &self.port_b {
                    (self.orb & self.ddrb) | (b.read() & !self.ddrb)
                } else {
                    self.orb
                }
            }
            0x1 => {
                if let Some(a) = &self.port_a {
                    (self.ora & self.ddra) | (a.read() & !self.ddra)
                } else {
                    self.ora
                }
            }
            0x2 => { self.ddrb  }
            0x3 => { self.ddra  }
            0x4 => { self.t1c_l }
            0x5 => { self.t1c_h }
            0x6 => { self.t1l_l }
            0x7 => { self.t1l_h }
            0x8 => { self.t2c_l }
            0x9 => { self.t2c_h }
            0xA => { self.sr    }
            0xB => { self.acr   }
            0xC => { self.pcr   }
            0xD => { self.ifr   }
            0xE => { self.ier   }
            0xF => { self.ora   }
            _   => { panic!("attempt to access invalid W65C22 register: {}", addr) }
        };
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    fn write(&mut self, addr : u16, data : u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);
        match addr {
            0x0 => {
                self.orb = data & self.ddrb;
                if let Some(b) = &mut self.port_b {
                    b.write(self.orb);
                }
            }
            0x1 => {
                self.ora = data & self.ddra;
                if let Some(a) = &mut self.port_a {
                    a.write(self.ora);
                }
            }
            0x2 => { self.ddrb  = data; }
            0x3 => { self.ddra  = data; }
            0x4 => { self.t1c_l = data; }
            0x5 => { self.t1c_h = data; }
            0x6 => { self.t1l_l = data; }
            0x7 => { self.t1l_h = data; }
            0x8 => { self.t2c_l = data; }
            0x9 => { self.t2c_h = data; }
            0xA => { self.sr    = data; }
            0xB => { self.acr   = data; }
            0xC => { self.pcr   = data; }
            0xD => { self.ifr   = data; }
            0xE => { self.ier   = data; }
            0xF => { self.ora   = data; }
            _   => { panic!("attempt to access invalid W65C22 register: {}", addr) }
        }
    }
}
