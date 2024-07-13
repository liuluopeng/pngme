use core::fmt;
use std::{convert::TryFrom, str::FromStr};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ChunkType {
    // Chunk Type Code (数据块类型码)	4字节	数据块类型码由ASCII字母(A-Z和a-z)组成
    chunk_type_code: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type_code
    }

    pub fn is_critical(&self) -> bool {
        /*

                   Ancillary bit: bit 5 of first byte
           0 (uppercase) = critical, 1 (lowercase) = ancillary.
        */

        //  1 1 1 1 1 1 1 1
        //        1 0 0 0 0
        // &      1

        // println!("{:?}", self.chunk_type_code[0]);

        self.chunk_type_code[0] & 32 == 0
    }

    pub fn is_public(&self) -> bool {
        // Private bit: bit 5 of second byte
        // 0 (uppercase) = public, 1 (lowercase) = private.

        println!("{:?}", self.chunk_type_code[1] & 32);

        self.chunk_type_code[1] & 32 == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        // Reserved bit: bit 5 of third byte
        // Must be 0 (uppercase) in files conforming to this version of PNG.

        self.chunk_type_code[2] & 32 == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        //         Safe-to-copy bit: bit 5 of fourth byte
        // 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.

        self.chunk_type_code[3] & 32 != 0
    }

    pub fn is_valid(&self) -> bool {
        for &b in self.chunk_type_code.iter() {
            if !b.is_ascii_lowercase() && !b.is_ascii_uppercase() {
                return false;
            }
        }

        if !self.is_reserved_bit_valid() {
            return false;
        }

        true
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    fn try_from(arr: [u8; 4]) -> Result<Self, Self::Error> {
        for &b in arr.iter() {
            if !b.is_ascii_lowercase() && !b.is_ascii_uppercase() {
                return Err("not abc or ABC".to_string());
            }
        }

        Ok(Self {
            chunk_type_code: arr,
        })
    }

    type Error = String;
}

impl FromStr for ChunkType {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("not 4 bytes".to_string());
        }

        let bytes = s.as_bytes();
        for &b in bytes.iter() {
            if !b.is_ascii_lowercase() && !b.is_ascii_uppercase() {
                return Err("not abc or ABC".to_string());
            }
        }

        Ok(ChunkType {
            chunk_type_code: [bytes[0], bytes[1], bytes[2], bytes[3]],
        })
    }
    type Err = String;
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for &b in self.chunk_type_code.iter() {
            s.push(b as char);
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
