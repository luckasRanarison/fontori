use crate::utils::types::Seq;
use bincode::{de::Decoder, error::DecodeError, Decode, Encode};

#[derive(Debug, Encode)]
pub struct FontDirectory {
    pub offset_subtable: OffsetSubtable,
    pub table_directory: Seq<TableEntry>,
}

impl Decode for FontDirectory {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let offset_subtable = OffsetSubtable::decode(decoder)?;
        let table_directory = (0..offset_subtable.num_tables)
            .map(|_| TableEntry::decode(decoder))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            offset_subtable,
            table_directory,
        })
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
