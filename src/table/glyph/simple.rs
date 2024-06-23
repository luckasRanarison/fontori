use crate::{
    error::Error,
    table::glyph::coord::Coord,
    utils::{bincode::decode_from_reader, bitflag::BitFlag, reader::ReadSeq, types::Seq},
};
use bincode::Encode;
use std::{
    io::{Read, Seek},
    iter::repeat,
};

const X_SHORT_VECTOR: u8 = 1;
const Y_SHORT_VECTOR: u8 = 2;
const REPEAT: u8 = 3;
const X_SAME_OR_POSITIVE: u8 = 4;
const Y_SAME_OR_POSITIVE: u8 = 5;

#[derive(Debug, Encode)]
pub struct SimpleGlyph {
    pub end_pts_of_contours: Seq<u16>,
    pub instruction_length: u16,
    pub instructions: Seq<u8>,
    pub flags: Seq<u8>,
    pub x_coordinates: Seq<Coord>,
    pub y_coordinates: Seq<Coord>,
}

impl SimpleGlyph {
    pub fn try_from_params<T>(contours: i16, stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let end_pts_of_contours = stream.read_seq(contours as usize)?;
        let instruction_length = decode_from_reader(stream)?;
        let instructions = stream.read_seq(instruction_length as usize)?;
        let last_point = end_pts_of_contours.as_slice().last().cloned();
        let points_count = last_point.unwrap_or_default() + 1;
        let (flags, flags_logical) = parse_outline_flags(points_count, stream)?;
        let (x_coordinates, _) =
            parse_coordinates(&flags_logical, stream, X_SHORT_VECTOR, X_SAME_OR_POSITIVE)?;
        let (y_coordinates, _) =
            parse_coordinates(&flags_logical, stream, Y_SHORT_VECTOR, Y_SAME_OR_POSITIVE)?;

        Ok(Self {
            end_pts_of_contours,
            instruction_length,
            instructions,
            flags,
            x_coordinates,
            y_coordinates,
        })
    }
}

fn parse_outline_flags<T>(points: u16, stream: &mut T) -> Result<(Seq<u8>, Vec<u8>), Error>
where
    T: Read,
{
    let mut i = 0;
    let mut last_flag: Option<u8> = None;
    let mut flags = Vec::<u8>::new();
    let mut flags_logical = Vec::<u8>::new();

    while i < points {
        let value = decode_from_reader(stream)?;
        let repeated_flag = last_flag.take().filter(|l| l.has(REPEAT));

        if let Some(flag) = repeated_flag {
            let repeated = repeat(flag).take(value as usize);
            flags_logical.extend(repeated);
            i += value as u16;
        } else {
            flags_logical.push(value);
            last_flag = Some(value);
            i += 1;
        }

        flags.push(value);
    }

    Ok((flags.into(), flags_logical))
}

fn parse_coordinates<T>(
    flags: &[u8],
    stream: &mut T,
    size_flag: u8,
    sign_flag: u8,
) -> Result<(Seq<Coord>, Vec<Coord>), Error>
where
    T: Read,
{
    let mut i = 0;
    let mut coordinates = Vec::<Coord>::new();
    let mut coordinates_logical = Vec::<Coord>::new();

    while i < flags.len() {
        let flag = flags[i];
        let size_flag = flag.get(size_flag);
        let sign_flag = flag.get(sign_flag);

        let coord = match (size_flag, sign_flag) {
            (1, _) => Some(Coord::UInt8(decode_from_reader(stream)?)),
            (0, 0) => Some(Coord::Int16(decode_from_reader(stream)?)),
            _ => None,
        };

        if let Some(coord) = coord {
            coordinates.push(coord);
            coordinates_logical.push(coord);
        } else {
            let last = coordinates.last().cloned().unwrap_or(Coord::UInt16(0));
            coordinates_logical.push(last);
        }

        i += 1;
    }

    Ok((coordinates.into(), coordinates_logical))
}
