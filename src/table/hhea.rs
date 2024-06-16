use crate::{
    sfnt::types::{FWord, Fixed, UFWord},
    utils::reader::{ReadIsize, ReadUsize, TryFromStream},
};
use std::io::{self, Read};

#[derive(Debug)]
pub struct Hhea {
    pub version: Fixed,
    pub ascent: FWord,
    pub descent: FWord,
    pub line_gap: FWord,
    pub advance_width_max: UFWord,
    pub min_left_side_bearing: FWord,
    pub min_right_side_bearing: FWord,
    pub x_max_extent: FWord,
    pub carret_slope_rise: i16,
    pub carret_slope_run: i16,
    pub carret_offset: FWord,
    pub _reserved1: i16,
    pub _reserved2: i16,
    pub _reserved3: i16,
    pub _reserved4: i16,
    pub metric_data_format: i16,
    pub num_of_long_hor_metrics: u16,
}

impl TryFromStream for Hhea {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        Ok(Self {
            version: Fixed::try_from_stream(stream)?,
            ascent: FWord::try_from_stream(stream)?,
            descent: FWord::try_from_stream(stream)?,
            line_gap: FWord::try_from_stream(stream)?,
            advance_width_max: UFWord::try_from_stream(stream)?,
            min_left_side_bearing: FWord::try_from_stream(stream)?,
            min_right_side_bearing: FWord::try_from_stream(stream)?,
            x_max_extent: FWord::try_from_stream(stream)?,
            carret_slope_rise: stream.read_i16()?,
            carret_slope_run: stream.read_i16()?,
            carret_offset: FWord::try_from_stream(stream)?,
            _reserved1: stream.read_i16()?,
            _reserved2: stream.read_i16()?,
            _reserved3: stream.read_i16()?,
            _reserved4: stream.read_i16()?,
            metric_data_format: stream.read_i16()?,
            num_of_long_hor_metrics: stream.read_u16()?,
        })
    }
}
