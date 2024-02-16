#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Standard(u32),
    Extended(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub id: Identifier,
    pub bus: u8, // TODO: Add enum to also support things like "vcan0"
    pub data: Vec<u8>,
    // TODO: Add timestamp, can-fd, rtr, dlc
}

impl From<u32> for Identifier {
    fn from(id: u32) -> Identifier {
        if id <= 0x7ff {
            Identifier::Standard(id)
        } else {
            Identifier::Extended(id)
        }
    }
}

impl Into<u32> for Identifier {
    fn into(self) -> u32 {
        match self {
            Identifier::Standard(id) => id,
            Identifier::Extended(id) => id,
        }
    }
}

pub trait CanAdapter {
    fn send(&mut self, frames: &[Frame]) -> Result<(), crate::error::Error>;
    fn recv(&mut self) -> Result<Vec<Frame>, crate::error::Error>;
}