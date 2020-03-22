use log::debug;
use std::fs;

pub struct Memory {
    pub mem : [u8; 65536]
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: [0; 65536] }
    }

    pub fn map(&mut self, base_addr : u16, path : &str) {
        debug!("MAP @ {:04x} = {}", base_addr, path);
        fs::read(path).unwrap().iter().enumerate().for_each(move |(ix, &val)| { self.mem[(base_addr as usize) + ix] = val })
    }

    pub fn read(&self, addr : u16) -> u8 {
        let data = self.mem[addr as usize];
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    pub fn write(&mut self, addr : u16, data : u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);
        self.mem[addr as usize] = data;
    }
}
