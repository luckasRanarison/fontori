use crate::{
    error::Error,
    utils::{bincode::decode, types::Seq},
};
use bincode::Decode;
use std::{
    io::{self, Read, Seek},
    mem,
};

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

pub trait TryFromBytes: Sized {
    fn try_from_bytes(bytes: &[u8]) -> Option<Self>;
}

impl TryFromBytes for u8 {
    fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.first().cloned()
    }
}

impl TryFromBytes for u16 {
    fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().ok().map(u16::from_be_bytes)
    }
}

impl TryFromBytes for i16 {
    fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.try_into().ok().map(i16::from_be_bytes)
    }
}

pub trait ReadSeq {
    fn read_seq<T>(&mut self, length: usize) -> io::Result<Seq<T>>
    where
        T: TryFromBytes;
}

impl<T> ReadSeq for T
where
    T: Read,
{
    fn read_seq<U>(&mut self, length: usize) -> io::Result<Seq<U>>
    where
        U: TryFromBytes,
    {
        let size = mem::size_of::<T>();
        let mut buffer = vec![0; length * size];
        self.read_exact(&mut buffer)?;

        Ok(buffer
            .chunks_exact(size)
            .flat_map(U::try_from_bytes)
            .collect::<_>())
    }
}
