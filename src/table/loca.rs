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
    pub format: LocFormat,
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
        let size = maxp.num_glyphs as usize + 1;

        let format = match loc_format {
            0 => LocFormat::Short,
            _ => LocFormat::Long,
        };

        let offsets = match format {
            LocFormat::Short => stream
                .read_seq::<u16>(size)?
                .into_iter()
                .map(|o| u32::from(o) * 2)
                .collect(),
            LocFormat::Long => stream.read_seq(size)?,
        };

        Ok(Self { offsets, format })
    }
}

impl Encode for Loca {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self.format {
            LocFormat::Short => self
                .offsets
                .iter()
                .map(|o| (o / 2) as u16)
                .collect::<Seq<_>>()
                .encode(encoder),
            LocFormat::Long => self.offsets.encode(encoder),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LocFormat {
    Short,
    Long,
}
