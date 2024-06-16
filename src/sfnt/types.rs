use crate::utils::reader::{ReadIsize, ReadUsize, TryFromStream};
use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
pub struct ShortFrac(u16);

impl ShortFrac {
    pub fn to_decimal(self) -> f32 {
        let integer_part = (self.0 >> 14) as f32;
        let fractional_part = (self.0 & 0x3FFF) as f32;
        integer_part + (fractional_part / 16384.)
    }
}

impl TryFromStream for ShortFrac {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        stream.read_u16().map(Self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Fixed(u16, u16);

impl Fixed {
    pub fn to_decimal(self) -> f32 {
        let integer_part = self.0 as f32;
        let fractional_part = self.1 as f32;
        integer_part + (fractional_part / 65536.)
    }
}

impl TryFromStream for Fixed {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        stream
            .read_u32()
            .map(|value| Self((value >> 16) as u16, (value & 0xFFFF) as u16))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LongDateTime(i64);

impl LongDateTime {
    pub fn value(self) -> i64 {
        self.0
    }
}

impl TryFromStream for LongDateTime {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        stream.read_i64().map(Self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FWord(i16);

impl FWord {
    pub fn value(self) -> i16 {
        self.0
    }
}

impl TryFromStream for FWord {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        stream.read_i16().map(Self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UFWord(u16);

impl UFWord {
    pub fn value(self) -> u16 {
        self.0
    }
}

impl TryFromStream for UFWord {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        stream.read_u16().map(Self)
    }
}
