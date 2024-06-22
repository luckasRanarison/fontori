use crate::{
    error::Error,
    utils::{bincode::decode_from_reader, reader::ReadSeq, types::Seq},
};
use bincode::Encode;
use std::io::Read;

#[derive(Debug, Encode)]
pub struct SimpleGlyph {
    pub end_pts_of_contours: Seq<u16>,
    pub instruction_length: u16,
    pub instructions: Seq<u16>,
    pub flags: Seq<u8>,
}

impl SimpleGlyph {
    pub fn try_from_params<T>(contours: i16, stream: &mut T) -> Result<Self, Error>
    where
        T: Read,
    {
        let end_pts_of_contours = stream.read_seq(contours as usize)?;
        let instruction_length = decode_from_reader(stream)?;
        let instructions = stream.read_seq(instruction_length as usize)?;
        let points = end_pts_of_contours.as_slice().last().unwrap() + 1;
        let flags = parse_outline_flags(points, stream)?;

        Ok(Self {
            end_pts_of_contours,
            instruction_length,
            instructions,
            flags,
        })
    }
}

fn parse_outline_flags<T>(points: u16, stream: &mut T) -> Result<Seq<u8>, Error>
where
    T: Read,
{
    todo!()
}
