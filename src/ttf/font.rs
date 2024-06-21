use crate::{
    error::Error,
    table::{tags, Table},
    ttf::font_dir::FontDirectory,
    utils::reader::TryFromStream,
};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use std::{
    collections::BTreeMap,
    io::{Read, Seek},
};

#[derive(Debug)]
pub struct Font {
    pub font_directory: FontDirectory,
    pub tables: BTreeMap<u32, Table>,
}

impl TryFromStream for Font {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let font_directory = FontDirectory::try_from_stream(stream)?;
        let mut tables = BTreeMap::new();
        let table_entry_map = font_directory.get_table_entries_map();
        let sorted_tags = font_directory.get_sorted_tags();

        let head = table_entry_map[&tags::HEAD];
        let head = Table::try_from_params(head, None, &tables, stream)?;
        tables.insert(tags::HEAD, head);

        let maxp = table_entry_map[&tags::MAXP];
        let head = Table::try_from_params(maxp, None, &tables, stream)?;
        tables.insert(tags::MAXP, head);

        for i in 0..sorted_tags.len() {
            let curr_tag = sorted_tags[i];

            if tables.contains_key(&curr_tag) {
                continue;
            }

            let next_tag = sorted_tags.get(i + 1);
            let curr_entry = table_entry_map[&curr_tag];
            let next_entry = next_tag.map(|n| table_entry_map[&n]);
            let table = Table::try_from_params(curr_entry, next_entry, &tables, stream)?;
            tables.insert(curr_entry.tag, table);
        }

        Ok(Self {
            font_directory,
            tables,
        })
    }
}

impl Encode for Font {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.font_directory.encode(encoder)?;
        self.font_directory
            .get_sorted_tags()
            .into_iter()
            .try_for_each(|tag| self.tables[&tag].encode(encoder))
    }
}
