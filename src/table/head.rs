use crate::{
    sfnt::types::{FWord, Fixed, LongDateTime},
    utils::bincode::Padding,
};
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
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
    __: Padding<2>,
}
