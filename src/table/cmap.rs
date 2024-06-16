use crate::utils::reader::{ReadUsize, TryFromStream};
use std::io::{self, Read};

#[derive(Debug)]
pub struct Cmap {
    pub header: CmapHeader,
    pub encoding_subtables: Vec<EncodingSubtable>,
}

impl TryFromStream for Cmap {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        let header = CmapHeader::try_from_stream(stream)?;
        let encoding_subtables = (0..header.number_subtables)
            .map(|_| EncodingSubtable::try_from_stream(stream))
            .collect::<io::Result<_>>()?;

        Ok(Self {
            header,
            encoding_subtables,
        })
    }
}

#[derive(Debug)]
pub struct CmapHeader {
    pub version: u16,
    pub number_subtables: u16,
}

impl TryFromStream for CmapHeader {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        Ok(Self {
            version: stream.read_u16()?,
            number_subtables: stream.read_u16()?,
        })
    }
}

#[derive(Debug)]
pub struct EncodingSubtable {
    pub platform_id: u16,
    pub platform_specific_id: u16,
    pub offset: u32,
}

impl TryFromStream for EncodingSubtable {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self> {
        Ok(Self {
            platform_id: stream.read_u16()?,
            platform_specific_id: stream.read_u16()?,
            offset: stream.read_u32()?,
        })
    }
}
