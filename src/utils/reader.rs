use std::io::{self, Read};

pub trait TryFromStream: Sized {
    fn try_from_stream<T: Read>(stream: &mut T) -> io::Result<Self>;
}

pub trait ReadUsize {
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_u32(&mut self) -> io::Result<u32>;
}

impl<T: Read> ReadUsize for T {
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }
}

pub trait ReadIsize {
    fn read_i16(&mut self) -> io::Result<i16>;
    fn read_i64(&mut self) -> io::Result<i64>;
}

impl<T: Read> ReadIsize for T {
    fn read_i16(&mut self) -> io::Result<i16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(i64::from_be_bytes(buffer))
    }
}
