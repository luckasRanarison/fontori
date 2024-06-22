use crate::{
    error::Error,
    utils::{
        bincode::decode_from_reader,
        reader::{ReadSeq, TryFromStream},
        types::{Padding, Seq},
    },
};
use bincode::{Decode, Encode};
use std::io::{Read, Seek};

#[derive(Debug, Encode)]
pub struct Format12 {
    pub format: u16,
    pub __: Padding<2>,
    pub length: u32,
    pub language: u32,
    pub n_groups: u32,
    pub groups: Seq<Format12Group>,
}

impl TryFromStream for Format12 {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let __ = decode_from_reader(stream)?;
        let length = decode_from_reader(stream)?;
        let language = decode_from_reader(stream)?;
        let n_groups = decode_from_reader(stream)?;
        let groups = stream.read_seq(n_groups as usize)?;

        Ok(Self {
            format: 12,
            __,
            length,
            language,
            n_groups,
            groups,
        })
    }
}

#[derive(Debug, Encode, Decode)]
pub struct Format12Group {
    pub start_char_code: u32,
    pub end_char_code: u32,
    pub start_glyph_code: u32,
}
