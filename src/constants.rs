/// Matches CONFIG_TYPE_* constants from libconfig.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SettingType {
    None = 0,
    Group = 1,
    Int = 2,
    Int64 = 3,
    Float = 4,
    String = 5,
    Bool = 6,
    Array = 7,
    List = 8,
}

impl SettingType {
    pub fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::Group),
            2 => Some(Self::Int),
            3 => Some(Self::Int64),
            4 => Some(Self::Float),
            5 => Some(Self::String),
            6 => Some(Self::Bool),
            7 => Some(Self::Array),
            8 => Some(Self::List),
            _ => None,
        }
    }
}

/// Matches CONFIG_FORMAT_* constants from libconfig.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SettingFormat {
    Default = 0,
    Hex = 1,
    #[cfg(libconfig_1_8)]
    Bin = 2,
    #[cfg(libconfig_1_8)]
    Oct = 3,
}

impl SettingFormat {
    pub fn from_raw(raw: u16) -> Option<Self> {
        match raw {
            0 => Some(Self::Default),
            1 => Some(Self::Hex),
            #[cfg(libconfig_1_8)]
            2 => Some(Self::Bin),
            #[cfg(libconfig_1_8)]
            3 => Some(Self::Oct),
            _ => None,
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ConfigOptions: i32 {
        const AUTOCONVERT                     = 0x01;
        const SEMICOLON_SEPARATORS            = 0x02;
        const COLON_ASSIGNMENT_FOR_GROUPS     = 0x04;
        const COLON_ASSIGNMENT_FOR_NON_GROUPS = 0x08;
        const OPEN_BRACE_ON_SEPARATE_LINE     = 0x10;
        const ALLOW_SCIENTIFIC_NOTATION       = 0x20;
        const FSYNC                           = 0x40;
        const ALLOW_OVERRIDES                 = 0x80;
    }
}

/// Matches CONFIG_ERR_* values from libconfig.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ErrorType {
    None = 0,
    FileIo = 1,
    Parse = 2,
}

impl ErrorType {
    pub fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::None),
            1 => Some(Self::FileIo),
            2 => Some(Self::Parse),
            _ => None,
        }
    }
}