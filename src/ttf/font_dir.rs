use crate::{
    error::Error,
    utils::{
        reader::{ReadSeq, TryFromStream},
        types::Seq,
    },
};
use bincode::{Decode, Encode};
use std::{
    collections::BTreeMap,
    io::{Read, Seek},
};

const ALIGNMENT: u32 = 4;

#[derive(Debug, Encode)]
pub struct FontDirectory {
    pub offset_subtable: OffsetSubtable,
    pub table_directory: Seq<TableEntry>,
}

impl TryFromStream for FontDirectory {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let offset_subtable = OffsetSubtable::try_from_stream(stream)?;
        let num_tables = offset_subtable.num_tables as usize;
        let table_directory = stream.read_seq(num_tables)?;

        Ok(Self {
            offset_subtable,
            table_directory,
        })
    }
}

impl FontDirectory {
    pub fn get_sorted_table_entries(&self) -> Vec<&TableEntry> {
        self.table_directory
            .iter()
            .map(|t| (t.offset, t))
            .collect::<BTreeMap<_, _>>()
            .into_values()
            .collect()
    }

    pub fn get_table_entries_map(&self) -> BTreeMap<u32, &TableEntry> {
        self.table_directory
            .iter()
            .map(|t| (t.tag, t))
            .collect::<_>()
    }
}

#[derive(Debug, Encode, Decode)]
pub struct OffsetSubtable {
    pub scaler_type: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

#[derive(Debug, Encode, Decode)]
pub struct TableEntry {
    pub tag: u32,
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
}

impl TableEntry {
    pub fn padding(&self) -> usize {
        let remainder = (self.offset + self.length) % ALIGNMENT;
        let padding = (ALIGNMENT - remainder) % ALIGNMENT;
        padding as usize
    }
}
