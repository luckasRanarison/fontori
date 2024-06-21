use crate::{
    error::Error,
    utils::{
        bincode::{decode_from_reader, decode_from_slice},
        types::Seq,
    },
};
use bincode::Decode;
use std::{
    io::{Read, Seek},
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
        Ok(decode_from_reader(stream)?)
    }
}

pub trait ReadSeq {
    fn read_seq<T>(&mut self, length: usize) -> Result<Seq<T>, Error>
    where
        T: Decode;
}

impl<T> ReadSeq for T
where
    T: Read,
{
    fn read_seq<U>(&mut self, length: usize) -> Result<Seq<U>, Error>
    where
        U: Decode,
    {
        let size = mem::size_of::<U>();
        let mut buffer = vec![0; length * size];

        self.read_exact(&mut buffer)?;

        let results = buffer
            .chunks_exact(size)
            .map(decode_from_slice)
            .collect::<Result<_, _>>()?;

        Ok(results)
    }
}
