use hex_literal::hex;
use serde_jce::Jcebuilder;

macro_rules! builder_test {
    ($func:ident, $value:expr, $expected:expr) => {
        #[test]
        fn $func() {
            let mut builder = Jcebuilder::new();
            builder.$func(0, $value);
            assert_eq!(builder.done(), $expected);
        }
    };
}

#[test]
fn const_check() {
    assert_eq!(
        u32::try_from(Jcebuilder::STRING_MAX_LENGTH).unwrap(),
        u32::MAX
    );
    assert_eq!(
        i32::try_from(Jcebuilder::BYTES_MAX_LENGTH).unwrap(),
        i32::MAX
    );
}

#[test]
fn big_tag() {
    let mut builder = Jcebuilder::new();
    builder.i8(0xab, 0x12);
    assert_eq!(builder.done(), hex!("f0 ab 12"));
}

builder_test!(i8, 0x12, hex!("00 12"));
builder_test!(i16, 0x1234, hex!("01 1234"));
builder_test!(i32, 0x12345678, hex!("02 12345678"));
builder_test!(i64, 0x0123456789abcdef, hex!("03 0123456789abcdef"));
builder_test!(
    f32,
    f32::from_be_bytes(hex!("12345678")),
    hex!("04 12345678")
);
builder_test!(
    f64,
    f64::from_be_bytes(hex!("0123456789abcdef")),
    hex!("05 0123456789abcdef")
);
builder_test!(str, "1234", hex!("06 04 31323334"));

#[test]
fn str_long() {
    let mut builder = Jcebuilder::new();
    let str = "\x7f".repeat(300);
    builder.str(0, str);
    let expected: Vec<u8> = hex!("07 0000012c")
        .into_iter()
        .chain([0x7f; 300].into_iter())
        .collect();
    assert_eq!(builder.done(), expected);
}

#[test]
fn map() {
    let mut builder = Jcebuilder::new();
    builder
        .map_begin(0, 2)
        .str(0, "first")
        .str(1, "first_value")
        .str(0, "second")
        .str(1, "second_value");
    let expected = hex!(
        "08 0002"
        "06 05 6669727374"
        "16 0b 66697273745f76616c7565"
        "06 06 7365636f6e64"
        "16 0c 7365636f6e645f76616c7565"
    );
    assert_eq!(builder.done(), expected);
}

#[test]
fn list() {
    let mut builder = Jcebuilder::new();
    builder.list_begin(0, 2).str(0, "first").str(0, "second");
    let expected = hex!(
        "09 0002"
        "06 05 6669727374"
        "06 06 7365636f6e64"
    );
    assert_eq!(builder.done(), expected);
}

#[test]
fn jce_struct() {
    let mut builder = Jcebuilder::new();
    builder
        .struct_begin(0)
        .i8(1, 0x12)
        .i16(2, 0x3456)
        .struct_end();
    let expected = hex!(
        "0a"
        "10 12"
        "21 3456"
        "0b"
    );
    assert_eq!(builder.done(), expected);
}

#[test]
fn zero() {
    let mut builder = Jcebuilder::new();
    builder.zero(0);
    assert_eq!(builder.done(), hex!("0c"));
}

builder_test!(
    bytes,
    hex!("0123456789abcdef"),
    hex!("0d 00 0008 0123456789abcdef")
);
