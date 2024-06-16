use crate::utils::reader::{ReadUsize, TryFromStream};
use std::io::{self, Read};

#[derive(Debug)]
pub struct FontDirectory {
    pub offset_subtable: OffsetSubtable,
    pub table_directory: Vec<TableEntry>,
}

impl TryFromStream for FontDirectory {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        let offset_subtable = OffsetSubtable::try_from_stream(stream)?;
        let table_directory = (0..offset_subtable.num_tables)
            .map(|_| TableEntry::try_from_stream(stream))
            .collect::<io::Result<_>>()?;

        Ok(Self {
            offset_subtable,
            table_directory,
        })
    }
}

#[derive(Debug)]
pub struct OffsetSubtable {
    pub scaler_type: u32,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
}

impl TryFromStream for OffsetSubtable {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        Ok(Self {
            scaler_type: stream.read_u32()?,
            num_tables: stream.read_u16()?,
            search_range: stream.read_u16()?,
            entry_selector: stream.read_u16()?,
            range_shift: stream.read_u16()?,
        })
    }
}

#[derive(Debug)]
pub struct TableEntry {
    pub tag: u32,
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
}

impl TryFromStream for TableEntry {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        Ok(Self {
            tag: stream.read_u32()?,
            check_sum: stream.read_u32()?,
            offset: stream.read_u32()?,
            length: stream.read_u32()?,
        })
    }
}
