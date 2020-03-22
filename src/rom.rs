use log::debug;
use std::fs;
use crate::bus::BusMember;

pub struct ROM {
    mem : Vec<u8>
}

impl ROM {
    pub fn new(size : usize) -> ROM {
        let mut storage = Vec::new();
        storage.resize(size, 0u8);
        ROM { mem: storage }
    }

    pub fn load(path : &str) -> ROM {
        ROM { mem: fs::read(path).unwrap() }
    }

    pub fn map(&mut self, base_addr : u16, path : &str) {
        debug!("MAP @ {:04x} = {}", base_addr, path);
        fs::read(path).unwrap().iter().enumerate().for_each(move |(ix, &val)| { self.mem[(base_addr as usize) + ix] = val })
    }
}

impl BusMember for ROM {
    fn read(&self, addr : u16) -> u8 {
        let data = self.mem[addr as usize];
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    fn write(&mut self, _addr : u16, _data : u8) {
        panic!("attempted write to ROM");
    }
}
