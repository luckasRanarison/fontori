use std::io::Read;

use bincode::{
    config::{self, BigEndian, Configuration, Fixint},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    Decode, Encode,
};

const BINCODE_CONFIG: Configuration<BigEndian, Fixint> = config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub fn decode<T: Decode, U: Read>(reader: &mut U) -> Result<T, DecodeError> {
    bincode::decode_from_std_read(reader, BINCODE_CONFIG)
}

pub fn encode<T: Encode>(value: T) -> Result<Vec<u8>, EncodeError> {
    bincode::encode_to_vec(value, BINCODE_CONFIG)
}

pub type Padding<const N: usize> = [u8; N];

#[derive(Debug)]
pub struct Seq<T>(Vec<T>);

impl<T> Seq<T> {
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> Encode for Seq<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for item in &self.0 {
            item.encode(encoder)?;
        }

        Ok(())
    }
}

impl<T> FromIterator<T> for Seq<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T> From<Vec<T>> for Seq<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}
