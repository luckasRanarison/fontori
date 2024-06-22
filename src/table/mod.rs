mod cmap;
mod head;
mod hhea;
mod hmtx;
mod maxp;

pub mod tags;

pub use {cmap::Cmap, head::Head, hhea::Hhea, hmtx::Hmtx, maxp::Maxp};

use crate::{
    error::Error,
    table::tags::Tag,
    ttf::font_dir::TableEntry,
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
pub enum Table {
    Head(Head),
    Hhea(Hhea),
    Maxp(Maxp),
    Hmtx(Hmtx),
    Cmap(Cmap),
    Other(Seq<u8>),
}

impl Encode for Table {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Table::Head(head) => head.encode(encoder),
            Table::Hhea(hhea) => hhea.encode(encoder),
            Table::Maxp(maxp) => maxp.encode(encoder),
            Table::Hmtx(htmx) => htmx.encode(encoder),
            Table::Cmap(cmap) => cmap.encode(encoder),
            Table::Other(table) => table.encode(encoder),
        }
    }
}

impl Table {
    pub fn try_from_params<T>(
        entry: &TableEntry,
        tables: &BTreeMap<u32, Table>,
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
            // tags::CMAP => Ok(Self::Cmap(Cmap::try_from_stream(stream)?)),
            tags::HMTX => Ok(Self::Hmtx(Hmtx::try_from_params(tables, stream)?)),
            _ => Ok(stream.read_seq(entry.length as usize).map(Self::Other)?),
        }
    }

    pub fn hhea(&self) -> Result<&hhea::Hhea, Error> {
        match self {
            Table::Hhea(value) => Ok(value),
            _ => Err(Error::ExpectedTable("hhea")),
        }
    }
}

trait GetTable {
    fn head(&self) -> Result<&Head, Error>;
    fn hhea(&self) -> Result<&Hhea, Error>;
    fn maxp(&self) -> Result<&Maxp, Error>;
    fn hmtx(&self) -> Result<&Hmtx, Error>;
    fn cmap(&self) -> Result<&Cmap, Error>;
}

impl GetTable for BTreeMap<Tag, Table> {
    fn head(&self) -> Result<&Head, Error> {
        match &self[&tags::HEAD] {
            Table::Head(value) => Ok(value),
            _ => Err(Error::ExpectedTable("head")),
        }
    }

    fn hhea(&self) -> Result<&Hhea, Error> {
        match &self[&tags::HHEA] {
            Table::Hhea(value) => Ok(value),
            _ => Err(Error::ExpectedTable("hhea")),
        }
    }

    fn maxp(&self) -> Result<&Maxp, Error> {
        match &self[&tags::MAXP] {
            Table::Maxp(value) => Ok(value),
            _ => Err(Error::ExpectedTable("maxp")),
        }
    }

    fn hmtx(&self) -> Result<&Hmtx, Error> {
        match &self[&tags::HMTX] {
            Table::Hmtx(value) => Ok(value),
            _ => Err(Error::ExpectedTable("hmtx")),
        }
    }

    fn cmap(&self) -> Result<&Cmap, Error> {
        match &self[&tags::CMAP] {
            Table::Cmap(value) => Ok(value),
            _ => Err(Error::ExpectedTable("hmtx")),
        }
    }
}
