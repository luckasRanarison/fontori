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
        let maxp = tables.maxp()?;
        let table_offset = stream.stream_position()?;
        let mut glyphs = Vec::new();

        for _ in 0..maxp.num_glyphs {
            let glyph = Glyph::try_from_stream(stream)?;
            let position = stream.stream_position()?;
            let offset = position - table_offset;
            let padding = (4 - (offset % 4)) % 4;
            stream.seek(SeekFrom::Current(padding as i64))?;
            glyphs.push(glyph);
        }

        Ok(Self {
            glyphs: glyphs.into(),
        })
    }
}
