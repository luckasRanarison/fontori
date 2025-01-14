use crate::{
    error::Error,
    table::{glyph::Glyph, FontTable, GetFontTable, Tag},
    utils::{reader::TryFromStream, types::Seq},
};
use bincode::Encode;
use std::{
    collections::BTreeMap,
    io::{Read, Seek, SeekFrom},
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
        let loca = tables.loca()?;
        let table_offset = stream.stream_position()?;

        let offsets = loca.offsets.as_slice();
        let offsets = offsets.windows(2).filter(|w| w[0] != w[1]);
        let mut glyphs = Vec::new();

        for _ in offsets {
            let glyph = Glyph::try_from_stream(stream)?;
            let position = stream.stream_position()?;
            let offset = position - table_offset;
            let padding = (4 - (offset % 4)) % 4;
            let new_position = SeekFrom::Current(padding as i64);
            stream.seek(new_position)?;
            glyphs.push(glyph);
        }

        Ok(Self {
            glyphs: glyphs.into(),
        })
    }
}
