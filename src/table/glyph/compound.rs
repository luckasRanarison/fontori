use crate::{
    error::Error,
    sfnt::types::F2Dot14,
    table::glyph::Coord,
    utils::{
        bincode::decode_from_reader,
        bitflag::BitFlag,
        reader::{ReadSeq, TryFromStream},
        types::{Opt, Seq},
    },
};
use bincode::Encode;
use std::io::{Read, Seek};

const ARGS_1_AND_2_ARE_WORDS: u16 = 0;
const ARGS_1_AND_2_ARE_XY_VALUES: u16 = 1;
const WE_HAVE_SCALE: u16 = 3;
const MORE_COMPONENTS: u16 = 5;
const WE_HAVE_X_AND_Y_SCALE: u16 = 6;
const WE_HAVE_A_TWO_BY_TWO: u16 = 7;
const WE_HAVE_INSTRUCTIONS: u16 = 8;

#[derive(Debug, Encode)]
pub struct CompoundGlyph {
    pub components: Seq<ComponentGlyph>,
    pub instruction_length: Opt<u16>,
    pub instructions: Opt<Seq<u8>>,
}

impl TryFromStream for CompoundGlyph {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read + Seek,
    {
        let mut components = Vec::new();
        let mut instruction_length = None;
        let mut instructions = None;

        loop {
            let component = ComponentGlyph::try_from_stream(stream)?;
            let has_more = component.flags.has(MORE_COMPONENTS);
            components.push(component);

            if !has_more {
                break;
            }
        }

        let has_instructions = components
            .last()
            .is_some_and(|l| l.flags.has(WE_HAVE_INSTRUCTIONS));

        if has_instructions {
            let length = decode_from_reader(stream)?;
            instruction_length = Some(length);
            instructions = Some(stream.read_seq(length as usize)?);
        }

        Ok(Self {
            components: components.into(),
            instruction_length: instruction_length.into(),
            instructions: instructions.into(),
        })
    }
}

#[derive(Debug, Encode)]
pub struct ComponentGlyph {
    pub flags: u16,
    pub glyph_index: u16,
    pub argument1: Coord,
    pub argument2: Coord,
    pub scale: Opt<F2Dot14>,
    pub x_scale: Opt<F2Dot14>,
    pub scale_01: Opt<F2Dot14>,
    pub scale_10: Opt<F2Dot14>,
    pub y_scale: Opt<F2Dot14>,
}

impl TryFromStream for ComponentGlyph {
    fn try_from_stream<T>(stream: &mut T) -> Result<Self, Error>
    where
        T: Read,
    {
        let flags: u16 = decode_from_reader(stream)?;
        let glyph_index = decode_from_reader(stream)?;
        let argument1 = read_argument(flags, stream)?;
        let argument2 = read_argument(flags, stream)?;
        let mut scale = None;
        let mut x_scale = None;
        let mut scale_01 = None;
        let mut scale_10 = None;
        let mut y_scale = None;

        if flags.has(WE_HAVE_SCALE) {
            scale = Some(decode_from_reader(stream)?);
        } else if flags.has(WE_HAVE_X_AND_Y_SCALE) {
            x_scale = Some(decode_from_reader(stream)?);
            y_scale = Some(decode_from_reader(stream)?);
        } else if flags.has(WE_HAVE_A_TWO_BY_TWO) {
            x_scale = Some(decode_from_reader(stream)?);
            scale_01 = Some(decode_from_reader(stream)?);
            scale_10 = Some(decode_from_reader(stream)?);
            y_scale = Some(decode_from_reader(stream)?);
        }

        Ok(Self {
            flags,
            glyph_index,
            argument1,
            argument2,
            scale: scale.into(),
            x_scale: x_scale.into(),
            scale_01: scale_01.into(),
            scale_10: scale_10.into(),
            y_scale: y_scale.into(),
        })
    }
}

fn read_argument<T>(flags: u16, stream: &mut T) -> Result<Coord, Error>
where
    T: Read,
{
    let size_flag = flags.get(ARGS_1_AND_2_ARE_WORDS);
    let sign_flag = flags.get(ARGS_1_AND_2_ARE_XY_VALUES);

    match (size_flag, sign_flag) {
        (0, 0) => Ok(Coord::UInt8(decode_from_reader(stream)?)),
        (0, 1) => Ok(Coord::Int8(decode_from_reader(stream)?)),
        (1, 0) => Ok(Coord::UInt16(decode_from_reader(stream)?)),
        (_, _) => Ok(Coord::Int16(decode_from_reader(stream)?)),
    }
}
