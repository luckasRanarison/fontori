use bincode::{
    config::{self, BigEndian, Configuration, Fixint},
    error::{DecodeError, EncodeError},
    Decode, Encode,
};
use std::io::Read;

const BINCODE_CONFIG: Configuration<BigEndian, Fixint> = config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub fn decode_from_reader<T, U>(reader: &mut U) -> Result<T, DecodeError>
where
    T: Decode,
    U: Read,
{
    bincode::decode_from_std_read(reader, BINCODE_CONFIG)
}

pub fn decode_from_slice<T>(slice: &[u8]) -> Result<T, DecodeError>
where
    T: Decode,
{
    bincode::decode_from_slice(slice, BINCODE_CONFIG).map(|(value, _)| value)
}

pub fn encode_to_vec<T: Encode>(value: T) -> Result<Vec<u8>, EncodeError> {
    bincode::encode_to_vec(value, BINCODE_CONFIG)
}
