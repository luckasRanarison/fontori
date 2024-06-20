use crate::{
    error::Error,
    utils::{bincode::decode, types::Seq},
};
use bincode::Decode;
use std::io::{self, Read, Seek};

pub trait TryFromStream: Sized {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek;
}

impl<T> TryFromStream for T
where
    T: Decode,
{
    fn try_from_stream<R: Read>(stream: &mut R) -> Result<Self, Error> {
        Ok(decode(stream)?)
    }
}

pub trait ReadSeq {
    fn read_u8_seq(&mut self, length: usize) -> io::Result<Seq<u8>>;
    fn read_u16_seq(&mut self, length: usize) -> io::Result<Seq<u16>>;
    fn read_i16_seq(&mut self, length: usize) -> io::Result<Seq<i16>>;
}

impl<T> ReadSeq for T
where
    T: Read,
{
    fn read_u8_seq(&mut self, length: usize) -> io::Result<Seq<u8>> {
        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }

    fn read_u16_seq(&mut self, length: usize) -> io::Result<Seq<u16>> {
        let mut buffer = vec![0; length * 2];
        self.read_exact(&mut buffer)?;
        let values = buffer
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        Ok(values)
    }

    fn read_i16_seq(&mut self, length: usize) -> io::Result<Seq<i16>> {
        let mut buffer = vec![0; length * 2];
        self.read_exact(&mut buffer)?;
        let values = buffer
            .chunks_exact(2)
            .map(|c| i16::from_be_bytes([c[0], c[1]]))
            .collect();
        Ok(values)
    }
}
