use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use anyhow::Result;
use serde::{de, Serialize, Serializer};

use super::base64::{byte_to_char, char_to_byte};
use super::category::IdCategory;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum IdError {
    IdStringUnknownLength(usize),
    IdZeroedStringIsReserved,
    UnknownIdCategory(char),
    UnknownLiteralChar(char),
    CharByteValueIsOutOfRange(u8),
}

/// ID is unique game entity ID. This type is
/// alias to u32, but with features:
/// - first byte is [IdCategory], you can use [Self::category] to extract it
/// - next u24 bits is id related to category (up to 16_777_216)
/// - each ID can be written as string ['xABCD'], where:
///   - x - [IdCategory] (u=units, d=doodads, ...)
///   - A - every other char from str(1..=4) is base64 char [(0-9 A-Z a-z + -)]
/// - so id range is 64 ^ 4
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Id {
    /// layout:
    /// u8  - category (enum 0..255)
    /// u24 - 6 bits ([A-Z,a-z,0-9,+-]) * 4 char
    raw: u32,
}

impl Id {
    pub fn new(id: &str) -> Result<Id, IdError> {
        if id.len() != 5 {
            return Err(IdError::IdStringUnknownLength(id.len()));
        }

        let mut chars: [char; 5] = ['0'; 5];
        id.chars().enumerate().for_each(|(i, c)| {
            chars[i] = c;
        });

        let id = Self::from_chars(chars)?;
        if id.relative_id() == 0 {
            return Err(IdError::IdZeroedStringIsReserved);
        }

        Ok(id)
    }

    /// Layout
    /// - category enum, allowed only: [u=Units, d=Doodads]
    /// - next 4 char, can use any value in [0-9, A-Z, a-z, +-] (each char has value 0..63)
    pub fn from_chars(chars: [char; 5]) -> Result<Id, IdError> {
        let raw: u32 = 0b0;

        let category = IdCategory::from_char(chars[0])?.to_u8() as u32;
        let byte1 = char_to_byte(chars[4])? as u32;
        let byte2 = char_to_byte(chars[3])? as u32;
        let byte3 = char_to_byte(chars[2])? as u32;
        let byte4 = char_to_byte(chars[1])? as u32;

        let raw = raw + (byte1 << 0);
        let raw = raw + (byte2 << 6);
        let raw = raw + (byte3 << 12);
        let raw = raw + (byte4 << 18);
        let raw = raw + (category << 24);

        Ok(Id { raw })
    }

    pub fn to_chars(&self) -> Result<[char; 5], IdError> {
        let byte1 = ((self.raw & 0b00000000_111111_000000_000000_000000) >> 18) as u8;
        let byte2 = ((self.raw & 0b00000000_000000_111111_000000_000000) >> 12) as u8;
        let byte3 = ((self.raw & 0b00000000_000000_000000_111111_000000) >> 06) as u8;
        let byte4 = ((self.raw & 0b00000000_000000_000000_000000_111111) >> 00) as u8;

        let char1 = byte_to_char(byte1)?;
        let char2 = byte_to_char(byte2)?;
        let char3 = byte_to_char(byte3)?;
        let char4 = byte_to_char(byte4)?;

        let category = self.category();
        Ok([category.to_char()?, char1, char2, char3, char4])
    }

    #[inline(always)]
    pub fn category(&self) -> IdCategory {
        IdCategory::from_u8((self.raw >> 24) as u8)
    }

    /// actually is NOT [u32]
    /// this is id belongs to some category and have size of [u24]
    /// up to 16_777_216 unique id per category
    #[inline(always)]
    pub fn relative_id(&self) -> u32 {
        //         _category_ _ID________________________
        self.raw & 0b00000000_111111_111111_111111_111111
    }

    #[inline(always)]
    pub fn to_u32(&self) -> u32 {
        self.raw
    }

    #[inline(always)]
    /// creating ID from u32.
    /// Not any u32 is valid ID
    /// so expected that you pass here only numbers given by [Self::to_u32()]
    pub fn unchecked_from_u32(raw: u32) -> Self {
        Self { raw }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.to_chars() {
            Ok(c) => write!(f, "{}{}{}{}{}", c[0], c[1], c[2], c[3], c[4]),
            Err(e) => write!(f, "InvalidId({:?})", e),
        }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> de::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Id;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(
                    f,
                    "some id string (ex: 'dABCD'). First chat is type([d,u,..]), next 4 is ID (0-9A-Za-z+-)"
                )
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                match Id::new(value) {
                    Ok(id) => Ok(id),
                    Err(e) => Err(E::custom(format!("invalid id: {:?}", e))),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_string() {
        // cant create any 0000 ids for any category
        assert_eq!(
            Id::new("u0000").unwrap_err(),
            IdError::IdZeroedStringIsReserved
        );
        assert_eq!(
            Id::new("d0000").unwrap_err(),
            IdError::IdZeroedStringIsReserved
        );

        // check category
        assert_eq!(Id::new("u0001").unwrap().category(), IdCategory::Units);
        assert_eq!(Id::new("d0001").unwrap().category(), IdCategory::Doodads);

        // check relative ids
        assert_eq!(Id::new("d0001").unwrap().relative_id(), 1);
        assert_eq!(Id::new("d0002").unwrap().relative_id(), 2);
        assert_eq!(Id::new("u0001").unwrap().relative_id(), 1);
        assert_eq!(Id::new("u0002").unwrap().relative_id(), 2);
        assert_eq!(Id::new("u----").unwrap().relative_id(), 16_777_215);

        // first IDs in each category
        // - units
        // assert_eq!(Id::new("u0000").unwrap().raw, 0); // 0 is not allowed
        assert_eq!(Id::new("u0001").unwrap().raw, 1);
        assert_eq!(Id::new("u0002").unwrap().raw, 2);
        assert_eq!(Id::new("u0003").unwrap().raw, 3);
        assert_eq!(Id::new("u----").unwrap().raw, 16_777_215);

        // - doodads
        // assert_eq!(Id::new("d0000").unwrap().raw, 16_777_215 + 1); // 0 is (+1), but 0 is not allowed
        assert_eq!(Id::new("d0001").unwrap().raw, 16_777_215 + 2);
        assert_eq!(Id::new("d0002").unwrap().raw, 16_777_215 + 3);
        assert_eq!(Id::new("d0003").unwrap().raw, 16_777_215 + 4);

        // create id from u32 and back
        let id_from_str = "dABCD";
        let id = Id::new(id_from_str).unwrap();
        let id_u32 = id.to_u32();
        let id_from_u32 = Id::unchecked_from_u32(id_u32);
        let id_to_str = id_from_u32.to_string();
        assert_eq!(id_from_str, id_to_str);
    }
}
