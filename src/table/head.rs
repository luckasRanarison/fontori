use crate::{
    sfnt::types::{FWord, Fixed, LongDateTime},
    utils::reader::{ReadIsize, ReadUsize, TryFromStream},
};
use std::io::{self, Read};

#[derive(Debug)]
pub struct Head {
    pub version: Fixed,
    pub font_revision: Fixed,
    pub check_sum_adjustement: u32,
    pub magic_number: u32,
    pub flags: u16,
    pub units_per_em: u16,
    pub created: LongDateTime,
    pub modified: LongDateTime,
    pub x_min: FWord,
    pub y_min: FWord,
    pub x_max: FWord,
    pub y_max: FWord,
    pub mac_style: u16,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: i16,
    pub index_to_loc_format: i16,
    pub glyph_data_format: i16,
}

impl TryFromStream for Head {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        Ok(Self {
            version: Fixed::try_from_stream(stream)?,
            font_revision: Fixed::try_from_stream(stream)?,
            check_sum_adjustement: stream.read_u32()?,
            magic_number: stream.read_u32()?,
            flags: stream.read_u16()?,
            units_per_em: stream.read_u16()?,
            created: LongDateTime::try_from_stream(stream)?,
            modified: LongDateTime::try_from_stream(stream)?,
            x_min: FWord::try_from_stream(stream)?,
            y_min: FWord::try_from_stream(stream)?,
            x_max: FWord::try_from_stream(stream)?,
            y_max: FWord::try_from_stream(stream)?,
            mac_style: stream.read_u16()?,
            lowest_rec_ppem: stream.read_u16()?,
            font_direction_hint: stream.read_i16()?,
            index_to_loc_format: stream.read_i16()?,
            glyph_data_format: stream.read_i16()?,
        })
    }
}
