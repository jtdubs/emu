use log::debug;
use std::fs;

pub struct ROM {
    mem: Vec<u8>,
}

impl ROM {
    pub fn load(path: &str) -> ROM {
        ROM {
            mem: fs::read(path).unwrap(),
        }
    }

    pub fn peek(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn read(&self, addr: u16) -> u8 {
        let data = self.mem[addr as usize];
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    pub fn write(&mut self, _addr: u16, _data: u8) {
        panic!("attempted write to ROM");
    }
}
