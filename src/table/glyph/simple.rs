use crate::{
    error::Error,
    utils::{bincode::decode_from_reader, bitflag::BitFlag, reader::ReadSeq, types::Seq},
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
        let (flags, flags_logical) = parse_outline_flags(points, stream)?;

        todo!();

        Ok(Self {
            end_pts_of_contours,
            instruction_length,
            instructions,
            flags,
        })
    }
}

fn parse_outline_flags<T>(points: u16, stream: &mut T) -> Result<(Seq<u8>, Vec<u8>), Error>
where
    T: Read,
{
    let mut count = 0;
    let mut queue = Vec::<u8>::new();
    let mut flags = Vec::<u8>::new();
    let mut flags_logical = Vec::<u8>::new();

    while count < points {
        let byte = decode_from_reader(stream)?;
        let last_flag = queue.pop().and_then(|l| l.has(3).then_some(l));

        if let Some(last) = last_flag {
            let repetition = vec![last; byte as usize];
            flags_logical.extend(repetition);
            count += byte as u16;
        } else {
            queue.push(byte);
            flags.push(byte);
            flags_logical.push(byte);
            count += 1;
        }
    }

    Ok((flags.into(), flags_logical))
}
