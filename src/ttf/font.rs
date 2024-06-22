use crate::{
    error::Error,
    table::{
        tags::{compare_tags, Tag},
        FontTable,
    },
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
    pub font_tables: BTreeMap<Tag, FontTable>,
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

        let mut font_tables = BTreeMap::new();
        let mut table_entries = font_directory.table_directory.iter().collect::<Vec<_>>();
        table_entries.sort_by(|a, b| compare_tags(a.tag, b.tag));

        for entry in table_entries {
            let table = FontTable::try_from_params(entry, &font_tables, stream)?;
            font_tables.insert(entry.tag, table);
        }

        Ok(Self {
            font_directory,
            font_tables,
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

            self.font_tables[&entry.tag].encode(encoder)?;
            padding.encode(encoder)?;
        }

        Ok(())
    }
}
