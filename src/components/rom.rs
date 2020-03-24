use log::debug;
use std::fs;

use crate::components::cpu;

pub struct ROM {
    mem: Vec<u8>,
}

impl ROM {
    pub fn load(path: &str) -> ROM {
        ROM {
            mem: fs::read(path).unwrap(),
        }
    }
}

impl cpu::Attachment for ROM {
    fn read(&mut self, addr: u16) -> u8 {
        let data = self.mem[addr as usize];
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    fn write(&mut self, _addr: u16, _data: u8) {
        panic!("attempted write to ROM");
    }

    fn has_interrupt(&self) -> bool {
        false
    }
}
