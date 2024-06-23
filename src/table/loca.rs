use crate::{
    error::Error,
    table::{tags::Tag, FontTable, GetFontTable},
    utils::{reader::ReadSeq, types::Seq},
};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use std::{collections::BTreeMap, io::Read};

#[derive(Debug)]
pub struct Loca {
    pub offsets: Seq<u32>,
    pub format: LocaFormat,
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

        let format = match loc_format {
            0 => LocaFormat::Short,
            _ => LocaFormat::Long,
        };

        let offsets = match format {
            LocaFormat::Short => stream
                .read_seq::<u16>(length)?
                .into_iter()
                .map(|o| o as u32 * 2)
                .collect(),
            LocaFormat::Long => stream.read_seq(length)?,
        };

        Ok(Self { offsets, format })
    }
}

impl Encode for Loca {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self.format {
            LocaFormat::Short => self
                .offsets
                .iter()
                .map(|o| (o / 2) as u16)
                .try_for_each(|o| o.encode(encoder)),
            LocaFormat::Long => self.offsets.encode(encoder),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocaFormat {
    Short,
    Long,
}
