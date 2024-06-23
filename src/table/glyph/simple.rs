use crate::{
    error::Error,
    table::glyph::coord::Coord,
    utils::{
        bincode::{decode_from_reader, encode_to_vec},
        bitflag::BitFlag,
        reader::ReadSeq,
        types::Seq,
    },
};
use bincode::Encode;
use std::io::{Read, Seek};

const X_SHORT_VECTOR: u8 = 1;
const Y_SHORT_VECTOR: u8 = 2;
const REPEAT: u8 = 3;
const X_SAME_OR_POSITIVE: u8 = 4;
const Y_SAME_OR_POSITIVE: u8 = 5;

#[derive(Debug, Encode)]
pub struct SimpleGlyph {
    pub end_pts_of_contours: Seq<u16>,
    pub instruction_length: u16,
    pub instructions: Seq<u16>,
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
        let points = end_pts_of_contours.as_slice().last().unwrap() + 1;
        let (flags, flags_logical) = parse_outline_flags(points, stream)?;
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
    let mut queue = Vec::<u8>::new();
    let mut flags = Vec::<u8>::new();
    let mut flags_logical = Vec::<u8>::new();

    while i < points {
        let byte = decode_from_reader(stream)?;
        let last_flag = queue.pop().and_then(|l| l.has(REPEAT).then_some(l));

        if let Some(last) = last_flag {
            let repetition = vec![last; byte as usize];
            flags_logical.extend(repetition);
            i += byte as u16;
        } else {
            queue.push(byte);
            flags_logical.push(byte);
            i += 1;
        }

        flags.push(byte);
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
            let last = coordinates.last().unwrap();
            coordinates_logical.push(*last);
        }

        i += 1;
    }

    Ok((coordinates.into(), coordinates_logical))
}
