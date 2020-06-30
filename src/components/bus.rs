pub enum BusOperation {
    Read(u16),
    Write(u16, u8),
    Peek(u16)
}

pub trait BusArbiter {
    fn bus(&mut self, op : BusOperation) -> u8;
}