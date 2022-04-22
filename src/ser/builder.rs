use crate::types::JceType;

/// Manually construct Jce format.
///
/// # Example
///
/// ```
/// use serde_jce::Jcebuilder;
///
/// let mut builder = Jcebuilder::new();
/// builder.i8(0, 0x12).i16(1, 0x1234);
/// assert_eq!(builder.done(), vec![0x00, 0x12, 0x11, 0x12, 0x34]);
/// ```
///
/// # Constant
///
/// * `Jcebuilder::STRING_MAX_LENGTH` - The maximum length of the string, the rest will be trimmed
///
/// * `Jcebuilder::BYTES_MAX_LENGTH` - The maximum length of the bytes, the rest will be trimmed
///
pub struct Jcebuilder {
    bytes: Vec<u8>,
}

impl Jcebuilder {
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    pub fn done(self) -> Vec<u8> {
        self.bytes
    }
}

impl Jcebuilder {
    fn push_head(&mut self, tag: u8, tp: JceType) -> &mut Self {
        if tag < 15 {
            self.push_byte((tag << 4) + (tp as u8));
        } else {
            self.push_byte(0xf0 + (tp as u8));
            self.push_byte(tag);
        }
        self
    }

    fn push_byte(&mut self, byte: u8) -> &mut Self {
        self.bytes.push(byte);
        self
    }

    fn push_bytes<T>(&mut self, bytes: T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.bytes.extend_from_slice(bytes.as_ref());
        self
    }
}

impl Jcebuilder {
    pub fn i8(&mut self, tag: u8, v: i8) -> &mut Self {
        if v == 0 {
            self.zero(tag)
        } else {
            self.push_head(tag, JceType::I8).push_bytes(v.to_be_bytes())
        }
    }

    pub fn i16(&mut self, tag: u8, v: i16) -> &mut Self {
        if std::i8::MIN as i16 <= v && v <= std::i8::MAX as i16 {
            self.i8(tag, v as i8)
        } else {
            self.push_head(tag, JceType::I16)
                .push_bytes(v.to_be_bytes())
        }
    }

    pub fn i32(&mut self, tag: u8, v: i32) -> &mut Self {
        if std::i16::MIN as i32 <= v && v <= std::i16::MAX as i32 {
            self.i16(tag, v as i16)
        } else {
            self.push_head(tag, JceType::I32)
                .push_bytes(v.to_be_bytes())
        }
    }

    pub fn i64(&mut self, tag: u8, v: i64) -> &mut Self {
        if std::i32::MIN as i64 <= v && v <= std::i32::MAX as i64 {
            self.i32(tag, v as i32)
        } else {
            self.push_head(tag, JceType::I64)
                .push_bytes(v.to_be_bytes())
        }
    }

    pub fn f32(&mut self, tag: u8, v: f32) -> &mut Self {
        self.push_head(tag, JceType::F32)
            .push_bytes(v.to_be_bytes())
    }

    pub fn f64(&mut self, tag: u8, v: f64) -> &mut Self {
        self.push_head(tag, JceType::F64)
            .push_bytes(v.to_be_bytes())
    }

    pub const STRING_MAX_LENGTH: usize = u32::MAX as usize;

    /// Insert a str of length less than `u32::MAX`
    ///
    /// # Arguments
    ///
    /// * `tag` - object tag
    /// * `v` - A string with length less than `u32::MAX`
    ///
    pub fn str<T>(&mut self, tag: u8, v: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let v = v.as_ref().as_bytes();
        if v.len() <= 255 {
            self.push_head(tag, JceType::String1)
                .push_byte(v.len() as u8)
                .push_bytes(v)
        } else {
            let n = std::cmp::min(v.len(), Self::STRING_MAX_LENGTH);
            self.push_head(tag, JceType::String4)
                .push_bytes((n as u32).to_be_bytes())
                .push_bytes(&v[..n])
        }
    }

    /// Insert a map header
    ///
    /// # Note
    /// * `map key` - `tag = 0`
    /// * `map value` - `tag = 1`
    ///
    pub fn map_begin(&mut self, tag: u8, len: i32) -> &mut Self {
        self.push_head(tag, JceType::Map).i32(0, len)
    }

    /// Insert a map header
    ///
    /// # Note
    /// * `list key` - `tag = 0`
    ///
    pub fn list_begin(&mut self, tag: u8, len: i32) -> &mut Self {
        self.push_head(tag, JceType::List).i32(0, len)
    }

    pub fn struct_begin(&mut self, tag: u8) -> &mut Self {
        self.push_head(tag, JceType::StructBegin)
    }

    pub fn struct_end(&mut self) -> &mut Self {
        self.push_head(0, JceType::StructEnd)
    }

    pub fn zero(&mut self, tag: u8) -> &mut Self {
        self.push_head(tag, JceType::Zero)
    }

    pub const BYTES_MAX_LENGTH: usize = i32::MAX as usize;

    /// Insert a bytes of length less than `i32::MAX`
    ///
    /// # Arguments
    ///
    /// * `tag` - object tag
    /// * `v` - A bytes with length less than `i32::MAX`
    ///
    pub fn bytes<T>(&mut self, tag: u8, v: T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        let v = v.as_ref();
        let n = std::cmp::min(v.len(), Self::BYTES_MAX_LENGTH);
        self.push_head(tag, JceType::Bytes)
            .push_head(0, JceType::I8)
            .i32(0, n as i32)
            .push_bytes(&v[..n])
    }
}
