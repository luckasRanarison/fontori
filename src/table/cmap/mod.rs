mod format_12;
mod format_4;
mod format_6;

use crate::{
    error::Error,
    utils::{reader::TryFromStream, types::Seq},
};
use bincode::{enc::Encoder, error::EncodeError, Decode, Encode};
use std::{
    collections::HashSet,
    io::{Read, Seek},
};

#[derive(Debug, Encode)]
pub struct Cmap {
    pub index: CmapHeader,
    pub subtable_metadata: Seq<CmapSubtableMetadata>,
    pub subtables: Seq<CmapSubtable>,
}

impl TryFromStream for Cmap {
    fn try_from_stream<R>(stream: &mut R) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let index = CmapHeader::try_from_stream(stream)?;
        let subtable_metadata = (0..index.number_subtables)
            .map(|_| CmapSubtableMetadata::try_from_stream(stream))
            .collect::<Result<Seq<_>, _>>()?;
        let offsets = subtable_metadata
            .iter()
            .map(|t| t.offset)
            .collect::<HashSet<_>>();
        let subtables = offsets
            .into_iter()
            .map(|_| CmapSubtable::try_from_stream(stream))
            .collect::<Result<Seq<_>, _>>()?;

        Ok(Self {
            index,
            subtable_metadata,
            subtables,
        })
    }
}

#[derive(Debug, Encode, Decode)]
pub struct CmapHeader {
    pub version: u16,
    pub number_subtables: u16,
}

#[derive(Debug, Encode, Decode)]
pub struct CmapSubtableMetadata {
    pub platform_id: u16,
    pub platform_specific_id: u16,
    pub offset: u32,
}

#[derive(Debug)]
pub enum CmapSubtable {
    Format4(format_4::Format4),
    Format6(format_6::Format6),
    Format12(format_12::Format12),
}

impl TryFromStream for CmapSubtable {
    fn try_from_stream<R: Read>(stream: &mut R) -> Result<Self, Error> {
        todo!()
    }
}

impl Encode for CmapSubtable {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            CmapSubtable::Format4(table) => table.encode(encoder),
            CmapSubtable::Format6(table) => table.encode(encoder),
            CmapSubtable::Format12(table) => table.encode(encoder),
        }
    }
}
