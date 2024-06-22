mod cmap;
mod head;
mod hhea;
mod hmtx;
mod loca;
mod maxp;

pub mod tags;

pub use {cmap::Cmap, head::Head, hhea::Hhea, hmtx::Hmtx, loca::Loca, maxp::Maxp};

use crate::{
    error::Error,
    table::tags::Tag,
    ttf::font_dir::TableDirEntry,
    utils::{
        reader::{ReadSeq, TryFromStream},
        types::Seq,
    },
};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use std::{
    collections::BTreeMap,
    io::{Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub enum FontTable {
    Head(Head),
    Hhea(Hhea),
    Maxp(Maxp),
    Hmtx(Hmtx),
    Cmap(Cmap),
    Loca(Loca),
    Other(Seq<u8>),
}

impl Encode for FontTable {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            FontTable::Head(head) => head.encode(encoder),
            FontTable::Hhea(hhea) => hhea.encode(encoder),
            FontTable::Maxp(maxp) => maxp.encode(encoder),
            FontTable::Hmtx(htmx) => htmx.encode(encoder),
            FontTable::Cmap(cmap) => cmap.encode(encoder),
            FontTable::Loca(loca) => loca.encode(encoder),
            FontTable::Other(table) => table.encode(encoder),
        }
    }
}

impl FontTable {
    pub fn try_from_params<T>(
        entry: &TableDirEntry,
        tables: &BTreeMap<Tag, FontTable>,
        stream: &mut T,
    ) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let offset = entry.offset.into();
        let pos = SeekFrom::Start(offset);

        stream.seek(pos)?;

        match entry.tag {
            tags::HEAD => Ok(Self::Head(Head::try_from_stream(stream)?)),
            tags::HHEA => Ok(Self::Hhea(Hhea::try_from_stream(stream)?)),
            tags::MAXP => Ok(Self::Maxp(Maxp::try_from_stream(stream)?)),
            tags::CMAP => Ok(Self::Cmap(Cmap::try_from_stream(stream)?)),
            tags::LOCA => Ok(Self::Loca(Loca::try_from_params(tables, stream)?)),
            tags::HMTX => Ok(Self::Hmtx(Hmtx::try_from_params(tables, stream)?)),
            _ => Ok(stream.read_seq(entry.length as usize).map(Self::Other)?),
        }
    }
}

trait GetFontTable {
    fn head(&self) -> Result<&Head, Error>;
    fn hhea(&self) -> Result<&Hhea, Error>;
    fn maxp(&self) -> Result<&Maxp, Error>;
    fn hmtx(&self) -> Result<&Hmtx, Error>;
    fn cmap(&self) -> Result<&Cmap, Error>;
    fn loca(&self) -> Result<&Loca, Error>;
}

impl GetFontTable for BTreeMap<Tag, FontTable> {
    fn head(&self) -> Result<&Head, Error> {
        match &self[&tags::HEAD] {
            FontTable::Head(value) => Ok(value),
            _ => Err(Error::ExpectedTable("head")),
        }
    }

    fn hhea(&self) -> Result<&Hhea, Error> {
        match &self[&tags::HHEA] {
            FontTable::Hhea(value) => Ok(value),
            _ => Err(Error::ExpectedTable("hhea")),
        }
    }

    fn maxp(&self) -> Result<&Maxp, Error> {
        match &self[&tags::MAXP] {
            FontTable::Maxp(value) => Ok(value),
            _ => Err(Error::ExpectedTable("maxp")),
        }
    }

    fn hmtx(&self) -> Result<&Hmtx, Error> {
        match &self[&tags::HMTX] {
            FontTable::Hmtx(value) => Ok(value),
            _ => Err(Error::ExpectedTable("hmtx")),
        }
    }

    fn cmap(&self) -> Result<&Cmap, Error> {
        match &self[&tags::CMAP] {
            FontTable::Cmap(value) => Ok(value),
            _ => Err(Error::ExpectedTable("cmap")),
        }
    }

    fn loca(&self) -> Result<&Loca, Error> {
        match &self[&tags::LOCA] {
            FontTable::Loca(value) => Ok(value),
            _ => Err(Error::ExpectedTable("loca")),
        }
    }
}
