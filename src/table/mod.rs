mod cmap;
mod head;
mod hhea;
mod hmtx;
mod maxp;

pub mod tags;

use crate::{
    error::Error,
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
    Head(head::Head),
    Hhea(hhea::Hhea),
    Maxp(maxp::Maxp),
    Hmtx(hmtx::Hmtx),
    Cmap(cmap::Cmap),
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
        current: &TableEntry,
        next: Option<&TableEntry>,
        tables: &BTreeMap<u32, Table>,
        stream: &mut T,
    ) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        stream.seek(SeekFrom::Start(current.offset.into()))?;

        match current.tag {
            tags::HEAD => Ok(Self::Head(head::Head::try_from_stream(stream)?)),
            tags::HHEA => Ok(Self::Hhea(hhea::Hhea::try_from_stream(stream)?)),
            tags::MAXP => Ok(Self::Maxp(maxp::Maxp::try_from_stream(stream)?)),
            tags::CMAP => Ok(Self::Cmap(cmap::Cmap::try_from_stream(stream)?)),
            tags::HMTX => Ok(Self::Hmtx(hmtx::Hmtx::try_from_params(tables, stream)?)),
            _ => {
                let length = next
                    .map(|next| next.offset - current.offset) // possible padding
                    .unwrap_or(current.length);
                let table = stream
                    .read_u8_seq(length as usize)
                    .map(|bytes| Self::Other(bytes.into()))?;
                Ok(table)
            }
        }
    }
}
