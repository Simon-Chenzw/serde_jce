use hex_literal::hex;
use serde_jce::Value;

#[test]
fn zero() {
    let val = Value::Zero;
    let bytes = hex!("0c");
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn int_i8() {
    let val = Value::Int(0x12);
    let bytes = hex!("00 12");
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn int_i64() {
    let val = Value::Int(0x0123456789abcdef);
    let bytes = hex!("03 0123456789abcdef");
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn f32() {
    let val = Value::Float(f32::from_be_bytes(hex!("12345678")));
    let bytes = hex!("04 12345678");
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn f64() {
    let val = Value::Double(f64::from_be_bytes(hex!("0123456789abcdef")));
    let bytes = hex!("05 0123456789abcdef");
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn string() {
    let val = Value::String("1234".to_owned());
    let bytes = hex!("06 04 31323334");
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn bytes() {
    let val = Value::Bytes(hex!("12345678").to_vec());
    let bytes = hex!("0d 00 0004 12345678");
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn list() {
    let val = Value::List(vec![
        Value::Int(0x12),
        Value::Int(0x1234),
        Value::Bytes(hex!("12345678").to_vec()),
    ]);
    let bytes = hex!("09 0003 0012 011234 0d 00 0004 12345678");
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn map() {
    let val = Value::Map(
        [
            ("first".into(), "first_value".into()),
            ("second".into(), "second_value".into()),
        ]
        .into(),
    );
    let bytes = hex!(
        "08 0002"
        "06 05 6669727374"
        "16 0b 66697273745f76616c7565"
        "06 06 7365636f6e64"
        "16 0c 7365636f6e645f76616c7565"
    );
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}

#[test]
fn obj() {
    let val = Value::Object(
        [
            (1, "first".into()),
            (2, "second".into()),
            (3, "third".into()),
        ]
        .into(),
    );
    let bytes = hex!(
        "0a"
        "16 05 6669727374"
        "26 06 7365636f6e64"
        "36 05 7468697264"
        "0b"
    );
    assert_eq!(serde_jce::to_bytes(&val), Ok(bytes.to_vec()));
    assert_eq!(serde_jce::from_bytes(&bytes), Ok(val));
}
