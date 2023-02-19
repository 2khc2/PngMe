#![allow(unused_variables)]

use std::{
    fmt::{self, Display},
    str::FromStr,
};
// use thiserror::Error;
// use anyhow::Result;
use color_eyre::{eyre::eyre, Result};

#[derive(Debug)]
pub struct ChunkType {
    bytes: [u8; 4],
    valid: bool,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_critical(&self) -> bool {
        if Self::get_bit_n(self.bytes[0], 5) == 0 {
            true
        } else {
            false
        }
    }

    pub fn is_public(&self) -> bool {
        println!("{:?}", self.bytes[1] as char);
        if Self::get_bit_n(self.bytes[1], 5) == 0 {
            true
        } else {
            false
        }
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        if Self::get_bit_n(self.bytes[2], 5) == 0 {
            true
        } else {
            false
        }
    }

    pub fn is_safe_to_copy(&self) -> bool {
        if Self::get_bit_n(self.bytes[3], 5) == 1 {
            true
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String {
        // std::str::from_utf8_unchecked(&self.bytes)
        let x = std::str::from_utf8(&self.bytes).unwrap();
        String::from(x)
    }

    fn byte_in_range(byte: u8) -> bool {
        if !((byte >= 65 && byte <= 90) || (97 <= byte && byte <= 122)) {
            false
        } else {
            true
        }
    }

    fn get_bit_n(byte: u8, n: u8) -> u8 {
        (byte & 32) >> n
    }

    // fn valid_or_char(bytes: u8) -> Result<[u8; 4], ()> {

    //     for (i, b) in bytes.iter().enumerate() {
    //         if !Self::byte_in_range(*b) {
    //             return Err(());
    //         } else if i == 2 && Self::get_bit_n(b, 5) == 1 {
    //             valid = false;
    //         }
    //     }
    // }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.bytes)
    }
}

impl FromStr for ChunkType {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() > 4 {
            Err(eyre!("too long"))
        } else {
            let mut valid = true;
            let mut x: [u8; 4] = [0; 4];
            for (i, c) in s.chars().enumerate() {
                let c_as_byte = c as u8;
                if !Self::byte_in_range(c_as_byte) {
                    return Err(eyre!("Invalid char"));
                } else if i == 2 && Self::get_bit_n(c_as_byte, 5) == 1 {
                    valid = false;
                }
                x[i] = c_as_byte;
            }
            Ok(Self { bytes: x, valid })
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = color_eyre::Report;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let mut valid = true;
        for (i, b) in bytes.iter().enumerate() {
            if !Self::byte_in_range(*b) {
                // return Err(color_eyre::Report("asd"));
                return Err(eyre!("123"));
            } else if i == 2 && Self::get_bit_n(*b, 5) == 1 {
                valid = false;
            }
        }

        Ok(Self { bytes, valid })
    }

    // type Error = ();
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes && self.valid == other.valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    // color_eyre::install()?;

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
// Ok(())
// }
