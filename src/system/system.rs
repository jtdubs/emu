use crate::component::hd44780::HD44780;
use crate::component::mos6502::{Bus, MOS6502};
use crate::component::mos6522::{Ports, MOS6522};
use crate::component::snes_controller::SNESController;
use crate::component::RAM;

pub trait System {
    type BusType: Bus;
    type PortsType: Ports;

    fn is_halted(&self) -> bool;
    fn get_cpu(&self) -> &MOS6502<Self::BusType>;
    fn get_display(&mut self) -> Option<&mut HD44780>;
    fn get_ram(&self) -> &RAM;
    fn get_controller(&mut self) -> Option<&mut SNESController>;
    fn get_peripheral_controller(&self) -> Option<&MOS6522<Self::PortsType>>;
    fn peek(&mut self, addr: u16) -> u8;
    fn cycle(&mut self);
}
