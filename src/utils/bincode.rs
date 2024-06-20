use bincode::{
    config::{self, BigEndian, Configuration, Fixint},
    error::{DecodeError, EncodeError},
    Decode, Encode,
};
use std::io::Read;

const BINCODE_CONFIG: Configuration<BigEndian, Fixint> = config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub fn decode<T: Decode, U: Read>(reader: &mut U) -> Result<T, DecodeError> {
    bincode::decode_from_std_read(reader, BINCODE_CONFIG)
}

pub fn encode<T: Encode>(value: T) -> Result<Vec<u8>, EncodeError> {
    bincode::encode_to_vec(value, BINCODE_CONFIG)
}
