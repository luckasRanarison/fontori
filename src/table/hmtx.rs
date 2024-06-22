use crate::{
    error::Error,
    sfnt::types::FWord,
    table::{tags, Table},
    utils::{reader::ReadSeq, types::Seq},
};
use bincode::{Decode, Encode};
use std::{
    collections::BTreeMap,
    io::{Read, Seek},
};

#[derive(Debug, Encode)]
pub struct Hmtx {
    pub h_metrics: Seq<LongHorMetric>,
    pub left_side_bearing: Seq<FWord>,
}

impl Hmtx {
    pub fn try_from_params<T>(tables: &BTreeMap<u32, Table>, stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let num_glyphs = match &tables[&tags::MAXP] {
            Table::Maxp(maxp) => maxp.num_glyphs as usize,
            _ => unreachable!(), // should be safe at this point
        };

        let num_of_long_hor_metrics = match &tables[&tags::HHEA] {
            Table::Hhea(hhea) => hhea.num_of_long_hor_metrics as usize,
            _ => unreachable!(),
        };

        let h_metrics = stream.read_seq(num_of_long_hor_metrics)?;
        let remainder = num_glyphs - num_of_long_hor_metrics;
        let left_side_bearing = stream.read_seq(remainder)?;

        Ok(Self {
            h_metrics,
            left_side_bearing,
        })
    }
}

#[derive(Debug, Encode, Decode)]
pub struct LongHorMetric {
    pub advance_width: u16,
    pub left_side_bearing: i16,
}
