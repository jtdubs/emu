use log::{debug, info};
use std::fmt;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum State {
    Idle,
    Busy(usize),
}

#[derive(Debug)]
pub enum RegisterSelector {
    Instruction = 0,
    Data = 1,
}

pub struct HD44780U {
    pub state: State,
    pub addr: u8,
    pub line1: Vec<u8>,
    pub line2: Vec<u8>,
    pub charset: Vec<char>,
    pub updated: bool,
    pub e: bool,
}

impl fmt::Debug for HD44780U {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (line1, line2) = self.get_output();

        f.debug_struct("HD44780U")
            .field("state", &self.state)
            .field("addr", &self.addr)
            .field("line1", &line1)
            .field("line2", &line2)
            .finish()
    }
}

impl HD44780U {
    pub fn new() -> HD44780U {
        let mut line1 = Vec::new();
        line1.resize(40, ' ' as u8);

        let mut line2 = Vec::new();
        line2.resize(40, ' ' as u8);

        let charset = vec![
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '!',
            '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2',
            '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C',
            'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
            'U', 'V', 'W', 'X', 'Y', 'Z', '[', '¥', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e',
            'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
            'w', 'x', 'y', 'z', '{', '|', '}', '→', '←', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '｡', '｢', '｣', '､', '･', 'ｦ', 'ｧ', 'ｨ', 'ｩ',
            'ｪ', 'ｫ', 'ｭ', 'ｪ', 'ｮ', 'ｯ', 'ｰ', 'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ', 'ｸ', 'ｹ', 'ｺ',
            'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ', 'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ', 'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ', 'ﾊ', 'ﾋ',
            'ﾌ', 'ﾍ', 'ﾎ', 'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ', 'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 'ﾛ', 'ﾜ',
            'ﾝ', 'ﾞ', 'ﾟ', 'α', 'ä', 'β', 'ε', 'μ', 'δ', 'ρ', 'g', '√', '¹', 'ϳ', '×', '¢', '£',
            'ñ', 'ö', 'p', 'q', 'θ', '∞', 'Ω', 'ü', '∑', 'π', 'x', 'y', '子', '万', '円', '÷', ' ',
            '█',
        ];

        HD44780U {
            // state: State::Busy(15000),
            state: State::Busy(150),
            addr: 0,
            line1: line1,
            line2: line2,
            charset: charset,
            updated: false,
            e: false,
        }
    }

    pub fn peek(&self, rs: RegisterSelector, rw: bool, _e: bool) -> u8 {
        if !rw {
            panic!("attempt to read display without read bit set");
        }

        match rs {
            RegisterSelector::Instruction => {
                let mut result = self.addr;
                if let State::Busy(_) = self.state {
                    result |= 0x80;
                }
                result
            }
            RegisterSelector::Data => {
                let offset = (self.addr & 0x3F) as usize;
                if self.addr & 0x40 == 0x00 {
                    self.line1[offset]
                } else {
                    self.line2[offset]
                }
            }
        }
    }

    pub fn read(&self, rs: RegisterSelector, rw: bool, _e: bool) -> u8 {
        if !rw {
            panic!("attempt to read display without read bit set");
        }
        
        match rs {
            RegisterSelector::Instruction => {
                let mut result = self.addr;
                if let State::Busy(_) = self.state {
                    result |= 0x80;
                }
                debug!("R {:?} = {:02x}", rs, result);
                result
            }
            RegisterSelector::Data => {
                let offset = (self.addr & 0x3F) as usize;
                let result = if self.addr & 0x40 == 0x00 {
                    self.line1[offset]
                } else {
                    self.line2[offset]
                };
                debug!("R {:?} = {:02x}", rs, result);
                result
            }
        }
    }

    pub fn write(&mut self, rs: RegisterSelector, rw: bool, e: bool, val: u8) {
        info!("W {:?} = {:02x}", rs, val);
        
        let last_e = self.e;
        self.e = e;
        
        if rw {
            return;
        }

        // falling edge triggers write
        if last_e && !self.e {
            match rs {
                RegisterSelector::Instruction => {
                    if val & 0x80 == 0x80 {
                        // set ddram addr
                        self.addr = (val & 0x7f) % 80;
                    } else if val & 0x40 == 0x40 {
                        // set cgram addr
                    } else if val & 0x20 == 0x20 {
                        // function set
                    } else if val & 0x10 == 0x10 {
                        // cursor or display shift
                    } else if val & 0x08 == 0x08 {
                        // display on/off
                    } else if val & 0x04 == 0x04 {
                        // entry mode set
                    } else if val & 0x02 == 0x02 {
                        // return home
                    } else if val & 0x01 == 0x01 {
                        // clear display
                        self.addr = 0;
                        self.line1.iter_mut().for_each(|x| *x = ' ' as u8);
                        self.line2.iter_mut().for_each(|x| *x = ' ' as u8);
                    }
                    self.state = State::Busy(37);
                }
                RegisterSelector::Data => {
                    let offset = (self.addr & 0x3F) as usize;
    
                    if self.addr & 0x40 == 0x00 {
                        self.line1[offset] = val;
                    } else {
                        self.line2[offset] = val;
                    };
    
                    self.addr += 1;
                    if self.addr & 0x40 == 0x00 {
                        if self.addr > 40 {
                            self.addr = 0x40;
                        }
                    } else {
                        if self.addr > (0x40 + 40) {
                            self.addr = 0x00;
                        }
                    }
    
                    self.state = State::Busy(37);
                    self.updated = true;
                }
            }
        }
    }

    pub fn get_output(&self) -> (String, String) {
        (
            String::from_iter(self.line1[..16].iter().map(|c| self.charset[*c as usize])),
            String::from_iter(self.line2[..16].iter().map(|c| self.charset[*c as usize])),
        )
    }

    pub fn get_updated(&mut self) -> bool {
        let result = self.updated;
        self.updated = false;
        result
    }

    pub fn cycle(&mut self) {
        debug!("DSP: {:x?}", self);

        self.state = match self.state {
            State::Idle => State::Idle,
            State::Busy(0) => State::Idle,
            State::Busy(c) => State::Busy(c - 1),
        };
    }
}