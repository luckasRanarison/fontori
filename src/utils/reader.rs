use std::io::{self, Read};

pub trait ReadVec {
    fn read_u8_vec(&mut self, length: usize) -> io::Result<Vec<u8>>;
    fn read_u16_vec(&mut self, length: usize) -> io::Result<Vec<u16>>;
    fn read_i16_vec(&mut self, length: usize) -> io::Result<Vec<i16>>;
}

impl<T> ReadVec for T
where
    T: Read,
{
    fn read_u8_vec(&mut self, length: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn read_u16_vec(&mut self, length: usize) -> io::Result<Vec<u16>> {
        let mut buffer = vec![0; length * 2];
        self.read_exact(&mut buffer)?;
        let values = buffer
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        Ok(values)
    }

    fn read_i16_vec(&mut self, length: usize) -> io::Result<Vec<i16>> {
        let mut buffer = vec![0; length * 2];
        self.read_exact(&mut buffer)?;
        let values = buffer
            .chunks_exact(2)
            .map(|c| i16::from_be_bytes([c[0], c[1]]))
            .collect();
        Ok(values)
    }
}
