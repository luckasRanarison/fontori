mod head;
mod hhea;
mod hmtx;
mod maxp;

use crate::{
    error::Error,
    ttf::font_dir::TableEntry,
    utils::{
        bincode::{decode, Seq},
        reader::ReadVec,
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
    Other(Seq<u8>),
}

impl Encode for Table {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Table::Head(head) => head.encode(encoder),
            Table::Hhea(hhea) => hhea.encode(encoder),
            Table::Maxp(maxp) => maxp.encode(encoder),
            Table::Hmtx(htmx) => htmx.encode(encoder),
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

        match &current.tag.to_be_bytes() {
            b"head" => Ok(Self::Head(decode(stream)?)),
            b"hhea" => Ok(Self::Hhea(decode(stream)?)),
            b"maxp" => Ok(Self::Maxp(decode(stream)?)),
            b"hmtx" => Ok(Self::Hmtx(hmtx::Hmtx::try_from_params(tables, stream)?)),
            _ => {
                let length = next
                    .map(|next| next.offset - current.offset) // possible padding
                    .unwrap_or(current.length);
                let table = stream
                    .read_u8_vec(length as usize)
                    .map(|bytes| Self::Other(bytes.into()))?;
                Ok(table)
            }
        }
    }
}
