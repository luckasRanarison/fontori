use crate::{
    error::Error,
    table::{tags, Table},
    ttf::font_dir::FontDirectory,
    utils::{reader::TryFromStream, types::Seq},
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

        if !font_directory.contains_required_tags() {
            return Err(Error::MissingRequiredTable);
        }

        let table_entry_map = font_directory.get_table_entries_map();
        let mut tables = BTreeMap::new();

        let head = table_entry_map[&tags::HEAD];
        let head = Table::try_from_params(head, &tables, stream)?;
        tables.insert(tags::HEAD, head);

        let maxp = table_entry_map[&tags::MAXP];
        let maxp = Table::try_from_params(maxp, &tables, stream)?;
        tables.insert(tags::MAXP, maxp);

        let hhea = table_entry_map[&tags::HHEA];
        let hhea = Table::try_from_params(hhea, &tables, stream)?;
        tables.insert(tags::HHEA, hhea);

        let remaining_entries = table_entry_map
            .into_values()
            .filter(|e| !tables.contains_key(&e.tag))
            .collect::<Vec<_>>();

        for entry in remaining_entries {
            tables.insert(entry.tag, Table::try_from_params(entry, &tables, stream)?);
        }

        Ok(Self {
            font_directory,
            tables,
        })
    }
}

impl Encode for Font {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let sorted_entries = self.font_directory.get_sorted_table_entries();

        self.font_directory.encode(encoder)?;

        for entry in sorted_entries {
            let padding_size = entry.padding();
            let padding = vec![0u8; padding_size];
            let padding = Seq::from(padding);

            self.tables[&entry.tag].encode(encoder)?;
            padding.encode(encoder)?;
        }

        Ok(())
    }
}
