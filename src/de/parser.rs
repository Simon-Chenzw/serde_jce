use crate::{Error, JceType, Result};

/// Manually construct jce format
///
/// # Example
///
/// ```
/// use serde_jce::{JceParser, JceType};
///
/// let mut parser = JceParser::from_bytes(&[0x00, 0x12, 0x11, 0x12, 0x34]);
/// assert_eq!(parser.pick_head(), Ok((0, JceType::I8)));
/// assert_eq!(parser.i8(), Ok(0x12));
/// assert_eq!(parser.pick_head(), Ok((1, JceType::I16)));
/// assert_eq!(parser.i16(), Ok(0x1234));
/// assert_eq!(parser.done(), true);
/// ```
///
/// # Error
///
/// When `JceParser` returns `Error`, remaining bytes will change due to incorrect parsing
///
/// This means, the parsing operation is not atomic
///
pub struct JceParser<'de> {
    bytes: &'de [u8],
}

////////////////////////////////////////////////////////////////////////////////

impl<'de> JceParser<'de> {
    pub fn from_bytes(bytes: &'de [u8]) -> Self {
        Self { bytes }
    }

    pub fn done(&self) -> bool {
        self.bytes.is_empty()
    }
}

////////////////////////////////////////////////////////////////////////////////
// pick parsing

impl<'de> JceParser<'de> {
    pub fn pick_tag(&self) -> Result<u8> {
        Ok(self.pick_head()?.0)
    }

    pub fn pick_type(&self) -> Result<JceType> {
        match self.bytes.get(0) {
            Some(head) => TryFrom::try_from(head & 0x0f),
            None => Err(Error::NotEnoughtBytes),
        }
    }

    pub fn pick_head(&self) -> Result<(u8, JceType)> {
        match self.bytes.get(0) {
            Some(head) => {
                let tag = head >> 4;
                let tp = TryFrom::try_from(head & 0x0f)?;
                if tag != 0x0f {
                    Ok((tag, tp))
                } else {
                    match self.bytes.get(1) {
                        Some(tag) => Ok((*tag, tp)),
                        None => Err(Error::NotEnoughtBytes),
                    }
                }
            }
            None => Err(Error::NotEnoughtBytes),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// get parsing

impl<'de> JceParser<'de> {
    fn get_head(&mut self) -> Result<(u8, JceType)> {
        let (tag, tp) = self.pick_head()?;
        if tag < 15 {
            self.bytes = &self.bytes[1..];
        } else {
            self.bytes = &self.bytes[2..];
        }
        Ok((tag, tp))
    }

    fn get_bytes<T>(&mut self, len: T) -> Result<&'de [u8]>
    where
        T: TryInto<usize>,
    {
        let len: usize = match len.try_into() {
            Ok(l) => Ok(l),
            Err(_) => Err(Error::NotEnoughtBytes),
        }?;
        if len <= self.bytes.len() {
            let (left, right) = self.bytes.split_at(len);
            self.bytes = right;
            Ok(left)
        } else {
            Err(Error::NotEnoughtBytes)
        }
    }

    fn get_bytes_fixed<const N: usize>(&mut self) -> Result<[u8; N]> {
        if N <= self.bytes.len() {
            let (left, right) = self.bytes.split_at(N);
            self.bytes = right;
            Ok(left.try_into().unwrap())
        } else {
            Err(Error::NotEnoughtBytes)
        }
    }

    pub fn i8(&mut self) -> Result<i8> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(0),
            JceType::I8 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i8::from_be_bytes(buf))
            }
            _ => Err(Error::WrongType),
        }
    }

    pub fn i16(&mut self) -> Result<i16> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(0),
            JceType::I8 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i8::from_be_bytes(buf) as i16)
            }
            JceType::I16 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i16::from_be_bytes(buf))
            }
            _ => Err(Error::WrongType),
        }
    }

    pub fn i32(&mut self) -> Result<i32> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(0),
            JceType::I8 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i8::from_be_bytes(buf) as i32)
            }
            JceType::I16 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i16::from_be_bytes(buf) as i32)
            }
            JceType::I32 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i32::from_be_bytes(buf))
            }
            _ => Err(Error::WrongType),
        }
    }

    pub fn i64(&mut self) -> Result<i64> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(0),
            JceType::I8 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i8::from_be_bytes(buf) as i64)
            }
            JceType::I16 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i16::from_be_bytes(buf) as i64)
            }
            JceType::I32 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i32::from_be_bytes(buf) as i64)
            }
            JceType::I64 => {
                let buf = self.get_bytes_fixed()?;
                Ok(i64::from_be_bytes(buf))
            }
            _ => Err(Error::WrongType),
        }
    }

    pub fn f32(&mut self) -> Result<f32> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(0.0),
            JceType::F32 => {
                let buf = self.get_bytes_fixed()?;
                Ok(f32::from_be_bytes(buf))
            }
            _ => Err(Error::WrongType),
        }
    }

    pub fn f64(&mut self) -> Result<f64> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(0.0),
            JceType::F32 => {
                let buf = self.get_bytes_fixed()?;
                Ok(f32::from_be_bytes(buf) as f64)
            }
            JceType::F64 => {
                let buf = self.get_bytes_fixed()?;
                Ok(f64::from_be_bytes(buf))
            }
            _ => Err(Error::WrongType),
        }
    }

    pub fn str_small(&mut self) -> Result<&'de str> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(""),
            JceType::String1 => {
                let len = u8::from_be_bytes(self.get_bytes_fixed()?);
                let buf = self.get_bytes(len)?;
                match std::str::from_utf8(buf) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(Error::StringIsNotUtf8),
                }
            }
            _ => Err(Error::WrongType),
        }
    }

    pub fn str_big(&mut self) -> Result<&'de str> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(""),
            JceType::String4 => {
                let len = u32::from_be_bytes(self.get_bytes_fixed()?);
                let buf = self.get_bytes(len)?;
                match std::str::from_utf8(buf) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(Error::StringIsNotUtf8),
                }
            }
            _ => Err(Error::WrongType),
        }
    }

    pub fn str(&mut self) -> Result<&'de str> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(""),
            JceType::String1 => {
                let len = u8::from_be_bytes(self.get_bytes_fixed()?);
                let buf = self.get_bytes(len)?;
                match std::str::from_utf8(buf) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(Error::StringIsNotUtf8),
                }
            }
            JceType::String4 => {
                let len = u32::from_be_bytes(self.get_bytes_fixed()?);
                let buf = self.get_bytes(len)?;
                match std::str::from_utf8(buf) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(Error::StringIsNotUtf8),
                }
            }
            _ => Err(Error::WrongType),
        }
    }

    /// swallow headers & return the length of map
    pub fn map<'a>(&'a mut self) -> Result<usize> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(0),
            JceType::Map => match self.i32()?.try_into() {
                Ok(val) => Ok(val),
                Err(_) => Err(Error::WrongLength),
            },
            _ => Err(Error::WrongType),
        }
    }

    /// swallow headers & return the length of list
    pub fn list<'a>(&'a mut self) -> Result<usize> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(0),
            JceType::List => match self.i32()?.try_into() {
                Ok(val) => Ok(val),
                Err(_) => Err(Error::WrongLength),
            },
            _ => Err(Error::WrongType),
        }
    }

    /// Basically do nothing but swallow headers
    pub fn struct_begin<'a>(&mut self) -> Result<()> {
        match self.get_head()?.1 {
            JceType::StructBegin => Ok(()),
            _ => Err(Error::WrongType),
        }
    }

    /// Basically do nothing but swallow headers
    pub fn struct_end<'a>(&mut self) -> Result<()> {
        match self.get_head()?.1 {
            JceType::StructEnd => Ok(()),
            _ => Err(Error::WrongType),
        }
    }

    pub fn zero(&mut self) -> Result<()> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(()),
            _ => Err(Error::WrongType),
        }
    }

    pub fn bytes(&mut self) -> Result<&'de [u8]> {
        match self.get_head()?.1 {
            JceType::Zero => Ok(&[]),
            JceType::Bytes => match self.get_head()?.1 {
                JceType::I8 => {
                    let len: usize = match self.i32()?.try_into() {
                        Ok(val) => Ok(val),
                        Err(_) => Err(Error::WrongLength),
                    }?;
                    self.get_bytes(len)
                }
                _ => Err(Error::WrongType),
            },
            _ => Err(Error::WrongType),
        }
    }

    pub fn ignore(&mut self) -> Result<()> {
        match self.pick_type()? {
            JceType::I8 => {
                self.i8()?;
            }
            JceType::I16 => {
                self.i16()?;
            }
            JceType::I32 => {
                self.i32()?;
            }
            JceType::I64 => {
                self.i64()?;
            }
            JceType::F32 => {
                self.f32()?;
            }
            JceType::F64 => {
                self.f64()?;
            }
            JceType::String1 => {
                self.str_small()?;
            }
            JceType::String4 => {
                self.str_big()?;
            }
            JceType::Map => {
                let len = self.map()?;
                for _ in 0..len {
                    self.ignore()?;
                    self.ignore()?;
                }
            }
            JceType::List => {
                let len = self.list()?;
                for _ in 0..len {
                    self.ignore()?;
                }
            }
            JceType::StructBegin => {
                self.struct_begin()?;
                loop {
                    match self.pick_type()? {
                        JceType::StructEnd => {
                            self.struct_end()?;
                            break;
                        }
                        _ => {
                            self.ignore()?;
                        }
                    }
                }
            }
            JceType::StructEnd => {
                self.struct_end()?;
            }
            JceType::Zero => {
                self.zero()?;
            }
            JceType::Bytes => {
                self.bytes()?;
            }
        }
        Ok(())
    }
}
