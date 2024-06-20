use crate::{
    error::Error,
    sfnt::types::FWord,
    table::{tags, Table},
    utils::{
        bincode::{decode, Seq},
        reader::ReadVec,
    },
};
use bincode::{Decode, Encode};
use std::{collections::BTreeMap, io::Read};

#[derive(Debug, Encode)]
pub struct Hmtx {
    pub h_metrics: Seq<LongHorMetric>,
    pub left_side_bearing: Seq<FWord>,
}

impl Hmtx {
    pub fn try_from_params<T: Read>(
        tables: &BTreeMap<u32, Table>,
        stream: &mut T,
    ) -> Result<Self, Error> {
        let num_glyphs = match tables.get(&tags::MAXP) {
            Some(Table::Maxp(maxp)) => Ok(maxp.num_glyphs),
            _ => Err(Error::MissingDependency("maxp".to_owned())),
        }?;
        let num_of_long_hor_metrics = match tables.get(&tags::HHEA) {
            Some(Table::Hhea(hhea)) => Ok(hhea.num_of_long_hor_metrics),
            _ => Err(Error::MissingDependency("hhea".to_owned())),
        }?;

        let h_metrics = (0..num_of_long_hor_metrics)
            .map(|_| decode::<LongHorMetric, _>(stream))
            .collect::<Result<_, _>>()?;

        let remainder = (num_glyphs - num_of_long_hor_metrics) as usize;
        let left_side_bearing = stream.read_i16_vec(remainder)?.into();

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
