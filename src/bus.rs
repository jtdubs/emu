use log::debug;

pub trait BusMember {
    fn read(&self, addr : u16) -> u8;
    fn write(&mut self, addr : u16, data : u8);
}

pub struct Bus {
    members : Vec<(u16, u16, Box<dyn BusMember>)>
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            members: Vec::new()
        }
    }

    pub fn add_member(&mut self, addr_mask: u16, addr_val: u16, member: Box<dyn BusMember>) {
        self.members.push((addr_mask, addr_val, member));
    }

    pub fn read(&self, addr : u16) -> u8 {
        debug!("R @ {:04x}", addr);

        let mut selected_members = self.members.iter().filter(move |&(mask, val, _)| { (addr & mask) == *val });
        match selected_members.next() {
            None => { panic!("no bus member responded to addr: {:04x}", addr); }
            Some((mask, _, member)) => {
                match selected_members.next() {
                    None => {
                        let data = member.read(addr & !mask);
                        data
                    }
                    _    => { panic!("multiple bus members responded to addr: {:04x}", addr); }
                }
            }
        }
    }

    pub fn write(&mut self, addr : u16, data : u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);

        let mut selected_members = self.members.iter_mut().filter(|(mask, val, _)| { (addr & mask) == *val });
        match selected_members.next() {
            None => { panic!("no bus member responded to addr: {:04x}", addr); }
            Some((mask, _, member)) => {
                match selected_members.next() {
                    None => { member.write(addr & !*mask, data) }
                    _    => { panic!("multiple bus members responded to addr: {:04x}", addr); }
                }
            }
        }
    }
}
