use crate::{
    sfnt::types::{FWord, Fixed, UFWord},
    utils::bincode::Padding,
};
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
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
    pub __: Padding<8>,
    pub metric_data_format: i16,
    pub num_of_long_hor_metrics: u16,
}
