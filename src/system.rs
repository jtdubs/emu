use crate::components::*;

pub trait System {
    type BusType : Bus;
    type PortsType : Ports;

    fn is_halted(&self) -> bool;
    fn get_cpu(&self) -> &W65C02S<Self::BusType>;
    fn get_display(&mut self) -> Option<&mut HD44780U>;
    fn get_ram(&self) -> &RAM;
    fn get_controller(&mut self) -> Option<&mut SNESController>;
    fn get_peripheral_controller(&self) -> Option<&W65C22<Self::PortsType>>;
    fn peek(&mut self, addr: u16) -> u8;
    fn cycle(&mut self);
}