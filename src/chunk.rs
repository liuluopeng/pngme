use core::fmt;

use crc;
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::{self, ChunkType};

#[derive(Debug)]
pub enum ChunkError {}

#[derive(Clone)]
pub(crate) struct Chunk {
    data_length: u32,                  // 这里不能用usize类型, 因为png规定最大是u32
    chunk_type: chunk_type::ChunkType, // 4个u8
    data: Vec<u8>,                     // 不定长
    crc: u32,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  detail: {}", self.data_as_string().unwrap())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(raw_chunk: &[u8]) -> Result<Self, Self::Error> {
        let (length, other) = raw_chunk.split_at(Chunk::DATA_LENGTH_BYTES);
        let length = u32::from_be_bytes(length.try_into().unwrap());

        let (chunk_type, other) = other.split_at(Chunk::CHUNK_TYPE_BYTES);
        let chunk_type: [u8; 4] = chunk_type.try_into().unwrap();
        // let chunk_type: [u8;4] = chunk_type.try_into().unwrap();
        let chunk_type = match ChunkType::try_from(chunk_type) {
            Ok(ct) => ct,
            Err(err) => return Err("chunk type error.".to_string()),
        };

        let (data, crc) = other.split_at(length.try_into().unwrap());
        let crc: u32 = u32::from_be_bytes(crc.try_into().unwrap());

        if length != data.len() as u32 {
            return Err("length is not same".to_string());
        }

        const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let bytes_for_crc: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc_should_be = CASTAGNOLI.checksum(&bytes_for_crc);

        if crc != crc_should_be {
            return Err("crc not ok".to_string());
        }

        Ok(Self {
            data_length: length,
            chunk_type,
            data: data.to_vec(),
            crc,
        })
    }
}

impl Chunk {
    pub const DATA_LENGTH_BYTES: usize = 4;
    pub const CHUNK_TYPE_BYTES: usize = 4;
    pub const CRC_BYTES: usize = 4;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;

        const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        let bytes_for_crc: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        let crc = CASTAGNOLI.checksum(&bytes_for_crc);

        Chunk {
            data_length: length,
            chunk_type: chunk_type,
            data: data,
            crc: crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.data_length
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn chunk_type(&self) -> ChunkType {
        self.chunk_type
    }

    pub fn data_as_string(&self) -> Result<String, ChunkError> {
        let mut res = String::new();

        for &b in self.data.iter() {
            res.push(b as char);
        }

        Ok(res)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res = vec![];

        let length_bytes = self.data_length.to_be_bytes();
        for &b in length_bytes.iter() {
            res.push(b);
        }

        for &b in self.chunk_type.bytes().iter() {
            res.push(b);
        }

        for &b in self.data.iter() {
            res.push(b);
        }

        let crc_bytes = self.crc.to_be_bytes();
        for &b in crc_bytes.iter() {
            res.push(b);
        }

        res
    }

    pub fn is_none(&self) -> bool {
        self.length() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
