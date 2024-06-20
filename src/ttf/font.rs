use crate::{
    error::Result,
    table::Table,
    ttf::font_dir::FontDirectory,
    utils::{reader::TryFromStream, types::Seq},
};
use bincode::Encode;
use std::{
    collections::BTreeMap,
    io::{Read, Seek},
};

#[derive(Debug, Encode)]
pub struct Font {
    pub font_directory: FontDirectory,
    pub tables: Seq<Table>,
}

impl TryFromStream for Font {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self>
    where
        T: Read + Seek,
    {
        let font_directory = FontDirectory::try_from_stream(stream)?;
        let table_dir = font_directory.table_directory.as_slice();
        let mut tables_map = BTreeMap::<u32, Table>::new();
        let mut sorted_entries = table_dir.iter().collect::<Vec<_>>();
        sorted_entries.sort_by_key(|t| t.offset);

        for i in 0..sorted_entries.len() {
            let current = sorted_entries[i];
            let next = sorted_entries.get(i + 1).cloned();
            let table = Table::try_from_params(current, next, &tables_map, stream)?;
            tables_map.insert(current.tag, table);
        }

        let tables = sorted_entries
            .into_iter()
            .flat_map(|entry| tables_map.remove(&entry.tag))
            .collect();

        Ok(Self {
            font_directory,
            tables,
        })
    }
}
