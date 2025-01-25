use super::id::IdError;

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum IdCategory {
    #[default]
    Unknown,
    Units,
    Doodads,
    DoodadsCategory,
}

impl IdCategory {
    pub fn to_u8(&self) -> u8 {
        match self {
            IdCategory::Units => 0,
            IdCategory::Doodads => 1,
            IdCategory::DoodadsCategory => 2,

            IdCategory::Unknown => 255,
            _ => 255,
        }
    }

    pub fn from_u8(value: u8) -> IdCategory {
        match value {
            0 => IdCategory::Units,
            1 => IdCategory::Doodads,
            2 => IdCategory::DoodadsCategory,

            255 => IdCategory::Unknown,
            _ => IdCategory::Unknown,
        }
    }

    pub fn from_char(c: char) -> anyhow::Result<IdCategory, IdError> {
        match c {
            'u' => Ok(IdCategory::Units),
            'd' => Ok(IdCategory::Doodads),
            'c' => Ok(IdCategory::DoodadsCategory),
            _ => Err(IdError::UnknownIdCategory(c)),
        }
    }

    pub fn to_char(&self) -> Result<char, IdError> {
        match self {
            IdCategory::Units => Ok('u'),
            IdCategory::Doodads => Ok('d'),
            IdCategory::DoodadsCategory => Ok('c'),
            IdCategory::Unknown => Err(IdError::UnknownIdCategory('_')),
        }
    }
}
