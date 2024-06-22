use crate::{
    error::Error,
    table::{glyph::Glyph, FontTable, GetFontTable, Tag},
    utils::{reader::TryFromStream, types::Seq},
};
use bincode::Encode;
use std::{
    collections::BTreeMap,
    io::{Read, Seek},
};

#[derive(Debug, Encode)]
pub struct Glyf {
    pub glyphs: Seq<Glyph>,
}

impl Glyf {
    pub fn try_from_params<T>(
        tables: &BTreeMap<Tag, FontTable>,
        stream: &mut T,
    ) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let maxp = tables.maxp()?;

        let glyphs = (0..maxp.num_glyphs)
            .map(|_| Glyph::try_from_stream(stream))
            .collect::<Result<_, _>>()?;

        Ok(Self { glyphs })
    }
}
