use crate::{
    error::Error,
    utils::{
        bincode::decode_from_reader,
        reader::{ReadSeq, TryFromStream},
        types::Seq,
    },
};
use bincode::Encode;
use std::io::{Read, Seek};

#[derive(Debug, Encode)]
pub struct Format6 {
    pub format: u16,
    pub length: u16,
    pub language: u16,
    pub first_code: u16,
    pub entry_count: u16,
    pub glyph_index_array: Seq<u16>,
}

impl TryFromStream for Format6 {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let length = decode_from_reader(stream)?;
        let language = decode_from_reader(stream)?;
        let first_code = decode_from_reader(stream)?;
        let entry_count = decode_from_reader(stream)?;
        let glyph_index_array = stream.read_seq(entry_count as usize)?;

        Ok(Self {
            format: 6,
            length,
            language,
            first_code,
            entry_count,
            glyph_index_array,
        })
    }
}
