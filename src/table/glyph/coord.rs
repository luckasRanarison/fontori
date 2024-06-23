use bincode::{enc::Encoder, error::EncodeError, Encode};

#[derive(Debug, Clone, Copy)]
pub enum Coord {
    Int8(i8),
    Int16(i16),
    UInt8(u8),
    UInt16(i16),
}

impl Encode for Coord {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Coord::Int8(value) => value.encode(encoder),
            Coord::Int16(value) => value.encode(encoder),
            Coord::UInt8(value) => value.encode(encoder),
            Coord::UInt16(value) => value.encode(encoder),
        }
    }
}
