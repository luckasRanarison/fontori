use crate::{
    error::Result,
    table::Table,
    ttf::font_dir::FontDirectory,
    utils::bincode::{decode, Seq},
};
use bincode::Encode;
use std::io::{Read, Seek};

#[derive(Debug, Encode)]
pub struct Font {
    pub font_directory: FontDirectory,
    pub tables: Seq<Table>,
}

impl Font {
    pub fn try_from_stream<T>(stream: &mut T) -> Result<Self>
    where
        T: Read + Seek,
    {
        let font_directory: FontDirectory = decode(stream)?;
        let table_dir = font_directory.table_directory.as_slice();
        let mut sorted_tables = table_dir.iter().collect::<Vec<_>>();
        sorted_tables.sort_by_key(|t| t.offset);

        let tables = (0..sorted_tables.len())
            .map(|i| (sorted_tables[i], sorted_tables.get(i + 1)))
            .map(|(c, n)| Table::try_from_entries(c, n, stream))
            .collect::<Result<_>>()?;

        Ok(Self {
            font_directory,
            tables,
        })
    }
}
