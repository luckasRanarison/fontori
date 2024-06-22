use crate::{
    error::Error,
    utils::{
        bincode::decode_from_reader,
        reader::{ReadSeq, TryFromStream},
        types::{Padding, Seq},
    },
};
use bincode::Encode;
use std::io::{Read, Seek};

#[derive(Debug, Encode)]
pub struct Format4 {
    pub format: u16,
    pub length: u16,
    pub language: u16,
    pub seg_count_x2: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub end_code: Seq<u16>,
    pub __: Padding<2>, // reserved
    pub start_code: Seq<u16>,
    pub id_delta: Seq<u16>,
    pub id_range_offset: Seq<u16>,
    pub glyph_index_array: Seq<u16>,
}

impl TryFromStream for Format4 {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let start_pos = stream.stream_position()? - 2; // two bytes from the format
        let length = decode_from_reader(stream)?;
        let language = decode_from_reader(stream)?;
        let seg_count_x2 = decode_from_reader(stream)?;
        let search_range = decode_from_reader(stream)?;
        let entry_selector = decode_from_reader(stream)?;
        let range_shift = decode_from_reader(stream)?;
        let seg_count = (seg_count_x2 as usize) / 2;
        let end_code = stream.read_seq(seg_count)?;
        let __ = decode_from_reader(stream)?;
        let start_code = stream.read_seq(seg_count)?;
        let id_delta = stream.read_seq(seg_count)?;
        let id_range_offset = stream.read_seq(seg_count)?;
        let current_pos = stream.stream_position()?;
        let remaining_bytes = length - (current_pos - start_pos) as u16;
        let glypth_count = remaining_bytes as usize / 2;
        let glyph_index_array = stream.read_seq(glypth_count)?;

        Ok(Self {
            format: 4,
            length,
            language,
            seg_count_x2,
            search_range,
            entry_selector,
            range_shift,
            end_code,
            __,
            start_code,
            id_delta,
            id_range_offset,
            glyph_index_array,
        })
    }
}
