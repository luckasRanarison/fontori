use crate::{
    error::Error,
    utils::{reader::TryFromStream, types::Seq},
};
use bincode::Encode;
use std::io::{Read, Seek};

#[derive(Debug, Encode)]
pub struct CompoundGlyph {
    pub components: Seq<ComponentGlyph>,
}

impl TryFromStream for CompoundGlyph {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        todo!()
    }
}

#[derive(Debug, Encode)]
pub struct ComponentGlyph {}
