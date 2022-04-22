use std::cmp::Ordering;
use std::collections::BTreeMap as Map;
use std::fmt;

use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub enum Value {
    Zero,
    Int(i64),
    Float(f32),
    Double(f64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(Map<Value, Value>),
    Object(Map<u8, Value>),
}

////////////////////////////////////////////////////////////////////////////////
// impl shortcut from

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Double(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_owned())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl shortcut getter

macro_rules! value_getter {
    ($func:ident, $enum:path, $arg:ty, $ret:ty) => {
        pub fn $func(self: $arg) -> Option<$ret> {
            match self {
                $enum(v) => Some(v),
                _ => None,
            }
        }
    };
}

impl Value {
    value_getter!(int, Value::Int, Self, i64);
    value_getter!(int_ref, Value::Int, &Self, &i64);
    value_getter!(int_mut, Value::Int, &mut Self, &mut i64);

    value_getter!(float, Value::Float, Self, f32);
    value_getter!(float_ref, Value::Float, &Self, &f32);
    value_getter!(float_mut, Value::Float, &mut Self, &mut f32);

    value_getter!(double, Value::Double, Self, f64);
    value_getter!(double_ref, Value::Double, &Self, &f64);
    value_getter!(double_mut, Value::Double, &mut Self, &mut f64);

    value_getter!(string, Value::String, Self, String);
    value_getter!(string_ref, Value::String, &Self, &String);
    value_getter!(string_mut, Value::String, &mut Self, &mut String);

    value_getter!(bytes, Value::Bytes, Self, Vec<u8>);
    value_getter!(bytes_ref, Value::Bytes, &Self, &Vec<u8>);
    value_getter!(bytes_mut, Value::Bytes, &mut Self, &mut Vec<u8>);

    value_getter!(list, Value::List, Self, Vec<Value>);
    value_getter!(list_ref, Value::List, &Self, &Vec<Value>);
    value_getter!(list_mut, Value::List, &mut Self, &mut Vec<Value>);

    value_getter!(map, Value::Map, Self, Map<Value,Value>);
    value_getter!(map_ref, Value::Map, &Self, &Map<Value,Value>);
    value_getter!(map_mut, Value::Map, &mut Self, &mut Map<Value,Value>);

    value_getter!(obj, Value::Object, Self, Map<u8,Value>);
    value_getter!(obj_ref, Value::Object, &Self, &Map<u8,Value>);
    value_getter!(obj_mut, Value::Object, &mut Self, &mut Map<u8,Value>);
}

////////////////////////////////////////////////////////////////////////////////
// impl fmt

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Zero => f.write_str("Zero"),
            Value::Int(v) => v.fmt(f),
            Value::Float(v) => f.write_fmt(format_args!("{}f32", v)),
            Value::Double(v) => f.write_fmt(format_args!("{}f64", v)),
            Value::String(v) => v.fmt(f),
            Value::Bytes(v) => f.write_fmt(format_args!("Bytes({})", &base64::encode(v))),
            Value::List(v) => v.fmt(f),
            Value::Map(v) => v.fmt(f),
            Value::Object(v) => f.debug_tuple("Object").field(v).finish(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl Ord for Map

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Zero => match other {
                Value::Zero => true,
                _ => false,
            },
            Value::Int(lhs) => match other {
                Value::Int(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Float(lhs) => match other {
                Value::Float(rhs) => lhs.to_bits() == rhs.to_bits(),
                _ => false,
            },
            Value::Double(lhs) => match other {
                Value::Double(rhs) => lhs.to_bits() == rhs.to_bits(),
                _ => false,
            },
            Value::String(lhs) => match other {
                Value::String(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Bytes(lhs) => match other {
                Value::Bytes(rhs) => lhs == rhs,
                _ => false,
            },
            Value::List(lhs) => match other {
                Value::List(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Map(lhs) => match other {
                Value::Map(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Object(lhs) => match other {
                Value::Object(rhs) => lhs == rhs,
                _ => false,
            },
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Value::Zero => match other {
                Value::Zero => Ordering::Equal,
                _ => Ordering::Less,
            },
            Value::Int(lhs) => match other {
                Value::Zero => Ordering::Greater,
                Value::Int(rhs) => Ord::cmp(lhs, rhs),
                _ => Ordering::Less,
            },
            Value::Float(lhs) => match other {
                Value::Zero => Ordering::Greater,
                Value::Int(_) => Ordering::Greater,
                Value::Float(rhs) => Ord::cmp(&lhs.to_bits(), &rhs.to_bits()),
                _ => Ordering::Less,
            },
            Value::Double(lhs) => match other {
                Value::Zero => Ordering::Greater,
                Value::Int(_) => Ordering::Greater,
                Value::Float(_) => Ordering::Greater,
                Value::Double(rhs) => Ord::cmp(&lhs.to_bits(), &rhs.to_bits()),
                _ => Ordering::Less,
            },
            Value::String(lhs) => match other {
                Value::Zero => Ordering::Greater,
                Value::Int(_) => Ordering::Greater,
                Value::Float(_) => Ordering::Greater,
                Value::Double(_) => Ordering::Greater,
                Value::String(rhs) => Ord::cmp(lhs, rhs),
                _ => Ordering::Less,
            },
            Value::Bytes(lhs) => match other {
                Value::Bytes(rhs) => Ord::cmp(lhs, rhs),
                Value::List(_) => Ordering::Less,
                Value::Map(_) => Ordering::Less,
                Value::Object(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            Value::List(lhs) => match other {
                Value::List(rhs) => Ord::cmp(lhs, rhs),
                Value::Map(_) => Ordering::Less,
                Value::Object(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            Value::Map(lhs) => match other {
                Value::Map(rhs) => Ord::cmp(lhs, rhs),
                Value::Object(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            Value::Object(lhs) => match other {
                Value::Object(rhs) => Ord::cmp(lhs, rhs),
                _ => Ordering::Greater,
            },
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl Serialize

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Zero => serializer.serialize_none(),
            Value::Int(v) => serializer.serialize_i64(*v),
            Value::Float(v) => serializer.serialize_f32(*v),
            Value::Double(v) => serializer.serialize_f64(*v),
            Value::String(v) => serializer.serialize_str(v),
            Value::Bytes(v) => serializer.serialize_bytes(v),
            Value::List(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for e in v {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Value::Map(v) => {
                let mut seq = serializer.serialize_map(Some(v.len()))?;
                for (k, v) in v {
                    seq.serialize_entry(k, v)?;
                }
                seq.end()
            }
            Value::Object(v) => {
                // stupid dirty trick, thanks to serde
                const STR_TABLE: [&'static str; 256] = [
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27",
                    "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40",
                    "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53",
                    "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66",
                    "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
                    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92",
                    "93", "94", "95", "96", "97", "98", "99", "100", "101", "102", "103", "104",
                    "105", "106", "107", "108", "109", "110", "111", "112", "113", "114", "115",
                    "116", "117", "118", "119", "120", "121", "122", "123", "124", "125", "126",
                    "127", "128", "129", "130", "131", "132", "133", "134", "135", "136", "137",
                    "138", "139", "140", "141", "142", "143", "144", "145", "146", "147", "148",
                    "149", "150", "151", "152", "153", "154", "155", "156", "157", "158", "159",
                    "160", "161", "162", "163", "164", "165", "166", "167", "168", "169", "170",
                    "171", "172", "173", "174", "175", "176", "177", "178", "179", "180", "181",
                    "182", "183", "184", "185", "186", "187", "188", "189", "190", "191", "192",
                    "193", "194", "195", "196", "197", "198", "199", "200", "201", "202", "203",
                    "204", "205", "206", "207", "208", "209", "210", "211", "212", "213", "214",
                    "215", "216", "217", "218", "219", "220", "221", "222", "223", "224", "225",
                    "226", "227", "228", "229", "230", "231", "232", "233", "234", "235", "236",
                    "237", "238", "239", "240", "241", "242", "243", "244", "245", "246", "247",
                    "248", "249", "250", "251", "252", "253", "254", "255",
                ];
                let mut seq = serializer.serialize_struct("Value", v.len())?;
                for (k, v) in v {
                    seq.serialize_field(STR_TABLE[*k as usize], v)?;
                }
                seq.end()
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// impl Deserialize

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a jce encoded object")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Zero)
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Int(value as i64))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Int(value as i64))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Int(value as i64))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Int(value as i64))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Float(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Double(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Value::Bytes(v.to_owned()))
    }

    fn visit_seq<A>(self, mut acc: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec: Vec<Value> = Vec::new();
        while let Some(value) = acc.next_element()? {
            vec.push(value);
        }
        Ok(Value::List(vec))
    }

    fn visit_map<A>(self, mut acc: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if acc.size_hint().is_none() {
            // Object
            let mut map: Map<u8, Value> = Map::new();
            while let Some((key, value)) = acc.next_entry()? {
                map.insert(key, value);
            }
            Ok(Value::Object(map))
        } else {
            // Map
            let mut map: Map<Value, Value> = Map::new();
            while let Some((key, value)) = acc.next_entry()? {
                map.insert(key, value);
            }
            Ok(Value::Map(map))
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}
