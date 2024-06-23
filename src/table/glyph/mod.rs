mod compound;
mod coord;
mod simple;

pub use compound::{ComponentGlyph, CompoundGlyph};
pub use simple::SimpleGlyph;

use crate::{error::Error, utils::reader::TryFromStream};
use bincode::{enc::Encoder, error::EncodeError, Decode, Encode};
use std::io::{Read, Seek};

#[derive(Debug, Encode)]
pub struct Glyph {
    pub header: GlyphHeader,
    pub data: GlyphData,
}

impl TryFromStream for Glyph {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let header = GlyphHeader::try_from_stream(stream)?;
        let contours = header.number_of_contours;

        let data = match contours {
            0.. => SimpleGlyph::try_from_params(contours, stream).map(GlyphData::Simple),
            _ => CompoundGlyph::try_from_stream(stream).map(GlyphData::Compound),
        }?;

        Ok(Self { header, data })
    }
}

#[derive(Debug, Encode, Decode)]
pub struct GlyphHeader {
    pub number_of_contours: i16,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
}

#[derive(Debug)]
pub enum GlyphData {
    Simple(SimpleGlyph),
    Compound(CompoundGlyph),
}

impl Encode for GlyphData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            GlyphData::Simple(value) => value.encode(encoder),
            GlyphData::Compound(value) => value.encode(encoder),
        }
    }
}
