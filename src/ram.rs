use log::debug;
use std::fs;
use crate::bus::BusMember;

pub struct RAM {
    mem : Vec<u8>
}

impl RAM {
    pub fn new(size : usize) -> RAM {
        let mut storage = Vec::new();
        storage.resize(size, 0u8);
        RAM { mem: storage }
    }

    pub fn map(&mut self, base_addr : u16, path : &str) {
        debug!("MAP @ {:04x} = {}", base_addr, path);
        fs::read(path).unwrap().iter().enumerate().for_each(move |(ix, &val)| { self.mem[(base_addr as usize) + ix] = val })
    }
}

impl BusMember for RAM {
    fn read(&self, addr : u16) -> u8 {
        let data = self.mem[addr as usize];
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    fn write(&mut self, addr : u16, data : u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);
        self.mem[addr as usize] = data;
    }
}