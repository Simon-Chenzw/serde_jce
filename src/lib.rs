//! Jce format support for `serde-rs`
//!
//! This crate is base on `serde`.
//! If you have never used`serde` yet, it is recommended to read the documentation of `serde_json` first
//!
//! # Public member
//!
//! * `to_bytes` - Serialize object to Jce format
//! * `to_bytes_with_tag` - Serialize object to Jce format with specific tag
//! * `from_bytes` - Deserialize Jce format to object
//! * `Jcebuilder` - Utils for build Jce format
//! * `JceParser` - Utils for parse Jce format
//! * `Value` - An recursive enum that might be able to represent all legal Jce data
//!
//! # Strongly typed data structures
//!
//!
//! ```
//! use serde::{Deserialize, Serialize}; // with serde_derive
//!
//! #[derive(PartialEq, Debug, Serialize, Deserialize)]
//! struct Struct {
//!     #[serde(rename = "0")]
//!     v0: i8,
//!     #[serde(rename = "1")]
//!     v1: i16,
//! }
//!
//! let val = Struct {
//!     v0: 0x12,
//!     v1: 0x3456,
//! };
//! let bytes = vec![0x0a, 0x00, 0x12, 0x11, 0x34, 0x56, 0x0b];
//!
//! assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.clone()));
//! assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
//!
//! ```
//!
//! ## with bytes
//!
//! If you want serialize/deserialize a `&[u8]`/`Vec<u8>` fields as `bytes` in Jce format.
//!
//! Please mark field with `#[serde(with = "serde_bytes")]`, which provided by `serde_bytes`.
//!
//! # serde_jce::Value
//!
//! ```
//! use std::collections::BTreeMap as Map;
//!
//! enum Value {
//!     Zero,
//!     Int(i64),
//!     Float(f32),
//!     Double(f64),
//!     String(String),
//!     Bytes(Vec<u8>),
//!     List(Vec<Value>),
//!     Map(Map<Value, Value>), // std::collections::BTreeMap
//!     Object(Map<u8, Value>), // std::collections::BTreeMap
//! }
//! ```
//!

mod de;
mod error;
mod ser;
mod types;
mod value;

pub use de::{from_bytes, Deserializer, JceParser};
pub use error::{Error, Result};
pub use ser::{to_bytes, to_bytes_with_tag, Jcebuilder, Serializer};
pub use types::JceType;
pub use value::Value;
