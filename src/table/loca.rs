use crate::{
    error::Error,
    table::{tags::Tag, FontTable, GetFontTable},
    utils::{reader::ReadSeq, types::Seq},
};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use std::{collections::BTreeMap, io::Read};

#[derive(Debug, Encode)]
pub struct Loca {
    pub offsets: Seq<LocaOffset>,
}

impl Loca {
    pub fn try_from_params<T>(
        tables: &BTreeMap<Tag, FontTable>,
        stream: &mut T,
    ) -> Result<Self, Error>
    where
        T: Read,
    {
        let head = tables.head()?;
        let maxp = tables.maxp()?;

        let loc_format = head.index_to_loc_format;
        let length = maxp.num_glyphs as usize + 1;

        let offsets: Seq<_> = match loc_format {
            0 => stream
                .read_seq(length)?
                .into_iter()
                .map(LocaOffset::Short)
                .collect(),
            _ => stream
                .read_seq(length)?
                .into_iter()
                .map(LocaOffset::Long)
                .collect(),
        };

        Ok(Self { offsets })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LocaOffset {
    Short(u16),
    Long(u32),
}

impl Encode for LocaOffset {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            LocaOffset::Short(value) => value.encode(encoder),
            LocaOffset::Long(value) => value.encode(encoder),
        }
    }
}
