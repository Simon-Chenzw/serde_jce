use serde::{ser, serde_if_integer128, Serialize};

use crate::{Error, Jcebuilder, Result};

/// A structure for serializing Rust values into Jce.
pub struct Serializer {
    pub tag: u8,
    builder: Jcebuilder,
}

/// Serialize the given data to Jce format.
///
/// # Example
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Struct {
///     #[serde(rename = "0")]
///     v0: i8,
///     #[serde(rename = "1")]
///     v1: i16,
/// }
/// let val = Struct {
///     v0: 0x12,
///     v1: 0x3456,
/// };
/// assert_eq!(
///     serde_jce::to_bytes(&val).unwrap(),
///     [0x0a, 0x00, 0x12, 0x11, 0x34, 0x56, 0x0b]
/// );
/// ```
///
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.done())
}

/// Serialize the given data to Jce format with specific tag.
///
/// # Example
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct TestSub {
///     #[serde(rename = "0")]
///     v0: i8,
///     #[serde(rename = "1")]
///     v1: i16,
/// }
/// let val = TestSub {
///     v0: 0x12,
///     v1: 0x3456,
/// };
/// assert_eq!(
///     serde_jce::to_bytes_with_tag(0xab, &val).unwrap(),
///     [0xfa, 0xab, 0x00, 0x12, 0x11, 0x34, 0x56, 0x0b]
/// );
/// ```
///
pub fn to_bytes_with_tag<T>(tag: u8, value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    serializer.tag = tag;
    value.serialize(&mut serializer)?;
    Ok(serializer.done())
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            tag: 0,
            builder: Jcebuilder::new(),
        }
    }

    pub fn done(self) -> Vec<u8> {
        self.builder.done()
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = StructSerializer<'a>;
    type SerializeStructVariant = StructSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_i8(v as i8)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.builder.i8(self.tag, v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.builder.i16(self.tag, v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.builder.i32(self.tag, v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.builder.i64(self.tag, v);
        Ok(())
    }

    serde_if_integer128! {
        fn serialize_i128(self, v: i128) -> Result<()> {
            if v <= std::i64::MAX as i128 {
                self.serialize_i64(v as i64)
            } else {
                Err(Error::IntTooBig)
            }
        }
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_i16(v as i16)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        if v <= std::i64::MAX as u64 {
            self.serialize_i64(v as i64)
        } else {
            Err(Error::IntTooBig)
        }
    }

    serde_if_integer128! {
        fn serialize_u128(self, v: u128) -> Result<()> {
            if v <= std::i64::MAX as u128 {
                self.serialize_i64(v as i64)
            } else {
                Err(Error::IntTooBig)
            }
        }
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.builder.f32(self.tag, v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.builder.f64(self.tag, v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(v.to_string().as_ref())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        if v.len() <= Jcebuilder::STRING_MAX_LENGTH {
            self.builder.str(self.tag, v);
            Ok(())
        } else {
            Err(Error::StringTooLong)
        }
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        if v.len() <= Jcebuilder::BYTES_MAX_LENGTH {
            self.builder.bytes(self.tag, v);
            Ok(())
        } else {
            Err(Error::BytesTooLong)
        }
    }

    fn serialize_none(self) -> Result<()> {
        self.builder.zero(self.tag);
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        variant.serialize(self)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(len) => match len.try_into() {
                Ok(len) => {
                    self.builder.list_begin(self.tag, len);
                    Ok(self)
                }
                Err(_) => Err(Error::SeqTooLong),
            },
            None => Err(Error::NeedLength),
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_tuple_struct(name, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        match len {
            Some(len) => match len.try_into() {
                Ok(len) => {
                    self.builder.map_begin(self.tag, len);
                    Ok(self)
                }
                Err(_) => Err(Error::MapTooLong),
            },
            None => Err(Error::NeedLength),
        }
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.builder.struct_begin(self.tag);
        Ok(Self::SerializeStruct::new(self))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_struct(name, len)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let cur_tag = self.tag;
        self.tag = 0;

        value.serialize(&mut **self)?;

        self.tag = cur_tag;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let cur_tag = self.tag;
        self.tag = 0;

        key.serialize(&mut **self)?;

        self.tag = cur_tag;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let cur_tag = self.tag;
        self.tag = 1;

        value.serialize(&mut **self)?;

        self.tag = cur_tag;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct StructSerializer<'a> {
    ser: &'a mut Serializer,
    tags: std::collections::HashSet<u8>,
}

impl<'a> StructSerializer<'a> {
    pub fn new(ser: &'a mut Serializer) -> Self {
        Self {
            ser: ser,
            tags: std::collections::HashSet::new(),
        }
    }
}

impl<'a> ser::SerializeStruct for StructSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match key.parse() {
            Ok(tag) => {
                if self.tags.insert(tag) {
                    let cur_tag = self.ser.tag;
                    self.ser.tag = tag;
                    value.serialize(&mut *self.ser)?;
                    self.ser.tag = cur_tag;
                    Ok(())
                } else {
                    Err(Error::DuplicateFieldTag)
                }
            }
            Err(_) => Err(Error::ErrorFieldTag),
        }
    }

    fn end(self) -> Result<()> {
        self.ser.builder.struct_end();
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for StructSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeStruct::end(self)
    }
}
