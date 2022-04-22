use std::marker::PhantomData;

use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{forward_to_deserialize_any, serde_if_integer128, Deserialize};

use crate::{Error, JceParser, JceType, Result};

/// A structure that deserializes Jce into Rust values.
pub struct Deserializer<'de> {
    parser: JceParser<'de>,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(bytes: &'de [u8]) -> Self {
        Self {
            parser: JceParser::from_bytes(bytes),
        }
    }

    pub fn done(&self) -> bool {
        self.parser.done()
    }
}

/// Deserialize an instance of type `T` from bytes of Jce.
///
/// # Example
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Struct {
///     #[serde(rename = "0")]
///     v0: i8,
///     #[serde(rename = "1")]
///     v1: i16,
/// }
/// let bytes = [0x0a, 0x00, 0x12, 0x11, 0x34, 0x56, 0x0b];
/// assert_eq!(
///     serde_jce::from_bytes::<Struct>(&bytes).unwrap(),
///     Struct {
///         v0: 0x12,
///         v1: 0x3456,
///     }
/// );
/// ```
///
pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(bytes);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.done() {
        Ok(t)
    } else {
        Err(Error::TrailingBytes)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (_tag, tp) = self.parser.pick_head()?;
        match tp {
            JceType::I8 => self.deserialize_i8(visitor),
            JceType::I16 => self.deserialize_i16(visitor),
            JceType::I32 => self.deserialize_i32(visitor),
            JceType::I64 => self.deserialize_i64(visitor),
            JceType::F32 => self.deserialize_f32(visitor),
            JceType::F64 => self.deserialize_f64(visitor),
            JceType::String1 => visitor.visit_borrowed_str(self.parser.str_small()?),
            JceType::String4 => visitor.visit_borrowed_str(self.parser.str_big()?),
            JceType::Map => self.deserialize_map(visitor),
            JceType::List => self.deserialize_seq(visitor),
            JceType::StructBegin => {
                self.parser.struct_begin()?;
                visitor.visit_map(TagsAccess::new(self))
            }
            JceType::StructEnd => todo!(),
            JceType::Zero => {
                self.parser.zero()?;
                visitor.visit_none()
            }
            JceType::Bytes => self.deserialize_bytes(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parser.i8()? != 0)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parser.i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parser.i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parser.i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parser.i64()?)
    }

    serde_if_integer128! {
        fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            self.deserialize_i64(visitor)
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i16(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    serde_if_integer128! {
        fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            self.deserialize_i128(visitor)
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parser.f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parser.f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parser.str()?.chars().nth(0) {
            Some(ch) => visitor.visit_char(ch),
            None => visitor.visit_char('\x00'),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parser.str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.parser.bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parser.pick_head()?.1 {
            JceType::Zero => {
                self.parser.zero()?;
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.parser.zero()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.parser.list()?;
        visitor.visit_seq(Sequence::new(self, len))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.parser.map()?;
        visitor.visit_map(Sequence::new(self, len))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.parser.struct_begin()?;
        let acc = TagsAccess::new_with_fields(self, fields)?;
        visitor.visit_map(acc)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Sequence<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    cur: usize,
    size: usize,
}

impl<'a, 'de> Sequence<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, size: usize) -> Self {
        Self { de, cur: 0, size }
    }
}

impl<'de, 'a> MapAccess<'de> for Sequence<'a, 'de> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.size)
    }

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.cur == self.size {
            Ok(None)
        } else {
            Ok(Some(seed.deserialize(&mut *self.de)?))
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        self.cur += 1;
        seed.deserialize(&mut *self.de)
    }
}

impl<'de, 'a> SeqAccess<'de> for Sequence<'a, 'de> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.size)
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.cur == self.size {
            Ok(None)
        } else {
            self.cur += 1;
            Ok(Some(seed.deserialize(&mut *self.de)?))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

struct TagsAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    tags: std::collections::HashSet<u8>,
    fields: Option<std::collections::HashSet<u8>>,
}

struct StupidTagDeserializer<'de> {
    phantom: PhantomData<&'de u8>,
    tag: u8,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut StupidTagDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.tag)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct StupidStringDeserializer<'de> {
    phantom: PhantomData<&'de u8>,
    tag: u8,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut StupidStringDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.tag.to_string())
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, 'a> TagsAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self {
            de,
            tags: std::collections::HashSet::new(),
            fields: None,
        }
    }

    fn new_with_fields(
        de: &'a mut Deserializer<'de>,
        fields: &'static [&'static str],
    ) -> Result<Self> {
        let mut set = std::collections::HashSet::new();
        for &field in fields {
            match field.parse() {
                Ok(tag) => match set.insert(tag) {
                    true => Ok(()),
                    false => Err(Error::DuplicateFieldTagName),
                },
                Err(_) => Err(Error::ErrorFieldTag),
            }?;
        }
        Ok(Self {
            de,
            tags: std::collections::HashSet::new(),
            fields: Some(set),
        })
    }

    /// Check if it is a terminator & if duplicated
    fn get_tag(&mut self) -> Result<Option<u8>> {
        let (tag, tp) = self.de.parser.pick_head()?;
        if let JceType::StructEnd = tp {
            self.de.parser.struct_end()?;
            Ok(None)
        } else {
            if self.tags.insert(tag) {
                Ok(Some(tag))
            } else {
                Err(Error::DuplicateFieldTag)
            }
        }
    }
}

impl<'de, 'a> MapAccess<'de> for TagsAccess<'a, 'de> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        None
    }

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.fields.is_some() {
            // have fields name, return field name
            loop {
                if let Some(tag) = self.get_tag()? {
                    if self.fields.as_ref().unwrap().contains(&tag) {
                        let mut des = StupidStringDeserializer {
                            phantom: PhantomData,
                            tag,
                        };
                        break Ok(Some(seed.deserialize(&mut des)?));
                    } else {
                        self.de.parser.ignore()?
                    }
                } else {
                    break Ok(None);
                }
            }
        } else {
            // don't have fields name, return tag as field name
            if let Some(tag) = self.get_tag()? {
                let mut des = StupidTagDeserializer {
                    phantom: PhantomData,
                    tag: tag,
                };
                Ok(Some(seed.deserialize(&mut des)?))
            } else {
                Ok(None)
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}
