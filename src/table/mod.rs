mod head;
mod hhea;

use crate::{
    error::Error,
    ttf::font_dir::TableEntry,
    utils::{
        bincode::{decode, Seq},
        reader::ReadVec,
    },
};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum Table {
    Head(head::Head),
    Hhea(hhea::Hhea),
    Other(Seq<u8>),
}

impl Encode for Table {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Table::Head(head) => head.encode(encoder),
            Table::Hhea(hhea) => hhea.encode(encoder),
            Table::Other(table) => table.encode(encoder),
        }
    }
}

impl Table {
    pub fn try_from_entries<T>(
        current: &TableEntry,
        next: Option<&&TableEntry>,
        stream: &mut T,
    ) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        stream.seek(SeekFrom::Start(current.offset.into()))?;

        match &current.tag.to_be_bytes() {
            b"head" => Ok(Self::Head(decode(stream)?)),
            b"hhea" => Ok(Self::Hhea(decode(stream)?)),
            _ => {
                let length = next
                    .map(|next| next.offset - current.offset)
                    .unwrap_or(current.length);
                let table = stream
                    .read_u8_vec(length as usize)
                    .map(|bytes| Self::Other(Seq::new(bytes)))?;
                Ok(table)
            }
        }
    }
}
