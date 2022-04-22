use crate::{Error, Result};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum JceType {
    I8 = 0,
    I16 = 1,
    I32 = 2,
    I64 = 3,
    F32 = 4,
    F64 = 5,
    String1 = 6,
    String4 = 7,
    Map = 8,
    List = 9,
    StructBegin = 10,
    StructEnd = 11,
    Zero = 12,
    Bytes = 13,
}

impl TryFrom<u8> for JceType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(JceType::I8),
            1 => Ok(JceType::I16),
            2 => Ok(JceType::I32),
            3 => Ok(JceType::I64),
            4 => Ok(JceType::F32),
            5 => Ok(JceType::F64),
            6 => Ok(JceType::String1),
            7 => Ok(JceType::String4),
            8 => Ok(JceType::Map),
            9 => Ok(JceType::List),
            10 => Ok(JceType::StructBegin),
            11 => Ok(JceType::StructEnd),
            12 => Ok(JceType::Zero),
            13 => Ok(JceType::Bytes),
            _ => Err(Error::UnknownJceType),
        }
    }
}
