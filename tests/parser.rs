use hex_literal::hex;
use serde_jce::{JceParser, JceType};

macro_rules! parser_test {
    ($func:ident, $value:expr, $expected:expr) => {
        #[test]
        fn $func() {
            let mut parser = JceParser::from_bytes(&$value);
            assert_eq!(parser.$func(), Ok($expected));
            assert!(parser.done());
        }
    };
}

#[test]
fn pick_head() {
    {
        let parser = JceParser::from_bytes(&hex!("0c"));
        assert_eq!(parser.pick_type().unwrap(), serde_jce::JceType::Zero);
        assert_eq!(parser.pick_head().unwrap(), (0, serde_jce::JceType::Zero));
        assert!(!parser.done());
    }
    {
        let parser = JceParser::from_bytes(&hex!("fc ab"));
        assert_eq!(parser.pick_type().unwrap(), serde_jce::JceType::Zero);
        assert_eq!(
            parser.pick_head().unwrap(),
            (0xab, serde_jce::JceType::Zero)
        );
        assert!(!parser.done());
    }
}

parser_test!(i8, hex!("00 12"), 0x12_i8);
parser_test!(i16, hex!("01 1234"), 0x1234_i16);
parser_test!(i32, hex!("02 12345678"), 0x12345678_i32);
parser_test!(i64, hex!("03 0123456789abcdef"), 0x0123456789abcdef_i64);
parser_test!(
    f32,
    hex!("04 12345678"),
    f32::from_be_bytes(hex!("12345678"))
);
parser_test!(
    f64,
    hex!("05 0123456789abcdef"),
    f64::from_be_bytes(hex!("0123456789abcdef"))
);
parser_test!(str_small, hex!("06 04 31323334"), "1234");
parser_test!(
    str_big,
    hex!("07 0000012c"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"
    ),
    std::str::from_utf8(&[0x7f; 300]).unwrap()
);
parser_test!(str, hex!("06 04 31323334"), "1234");

#[test]
fn map() {
    let bytes = hex!(
        "08 0002"
        "06 05 6669727374"
        "16 0b 66697273745f76616c7565"
        "06 06 7365636f6e64"
        "16 0c 7365636f6e645f76616c7565"
    );
    let mut parser = JceParser::from_bytes(&bytes);
    assert_eq!(parser.map(), Ok(2));
    assert_eq!(parser.str(), Ok("first"));
    assert_eq!(parser.str(), Ok("first_value"));
    assert_eq!(parser.str(), Ok("second"));
    assert_eq!(parser.str(), Ok("second_value"));
    assert!(parser.done());
}

#[test]
fn list() {
    let bytes = hex!(
        "09 0002"
        "06 05 6669727374"
        "06 06 7365636f6e64"
    );
    let mut parser = JceParser::from_bytes(&bytes);
    assert_eq!(parser.list(), Ok(2));
    assert_eq!(parser.str(), Ok("first"));
    assert_eq!(parser.str(), Ok("second"));
    assert!(parser.done());
}

#[test]
fn jce_struct() {
    let bytes = hex!("0a 1012 213456 0b");
    let mut parser = JceParser::from_bytes(&bytes);
    assert_eq!(parser.struct_begin(), Ok(()));
    assert_eq!(parser.pick_tag(), Ok(1));
    assert_eq!(parser.i8(), Ok(0x12));
    assert_eq!(parser.pick_tag(), Ok(2));
    assert_eq!(parser.i16(), Ok(0x3456));
    assert_eq!(parser.struct_end(), Ok(()));
    assert!(parser.done());
}

parser_test!(zero, hex!("0c"), ());
parser_test!(
    bytes,
    hex!("0d 00 0008 0123456789abcdef"),
    hex!("0123456789abcdef").as_ref()
);

#[test]
fn long_tag() {
    let mut parser = JceParser::from_bytes(&hex!("f0 0f 12"));
    assert_eq!(parser.pick_tag(), Ok(0x0f));
    assert_eq!(parser.i8(), Ok(0x12));
    assert!(parser.done());
}

#[test]
fn downgraded() {
    let mut parser = JceParser::from_bytes(&hex!("00 12"));
    assert_eq!(parser.pick_type(), Ok(JceType::I8));
    assert_eq!(parser.i16(), Ok(0x12));
}

////////////////////////////////////////////////////////////////////////////////
// ignore test

#[test]
fn ignore_i8() {
    let mut parser = JceParser::from_bytes(&hex!("00 12 10 12"));
    assert_eq!(parser.ignore(), Ok(()));
    assert_eq!(parser.i8(), Ok(0x12));
}

#[test]
fn ignore_str() {
    let mut parser = JceParser::from_bytes(&hex!("06 04 31323334" "10 12"));
    assert_eq!(parser.ignore(), Ok(()));
    assert_eq!(parser.i8(), Ok(0x12));
}

#[test]
fn ignore_map() {
    let mut parser = JceParser::from_bytes(&hex!(
        "08 0002"
        "06 05 6669727374"
        "16 0b 66697273745f76616c7565"
        "06 06 7365636f6e64"
        "16 0c 7365636f6e645f76616c7565"
        "10 12"
    ));
    assert_eq!(parser.ignore(), Ok(()));
    assert_eq!(parser.i8(), Ok(0x12));
}

#[test]
fn ignore_list() {
    let mut parser = JceParser::from_bytes(&hex!(
        "09 0002"
        "06 05 6669727374"
        "06 06 7365636f6e64"
        "10 12"
    ));
    assert_eq!(parser.ignore(), Ok(()));
    assert_eq!(parser.i8(), Ok(0x12));
}

#[test]
fn ignore_struct() {
    let mut parser = JceParser::from_bytes(&hex!(
        "0a"
        "00 34"
        "10 34"
        "0b"
        "10 12"
    ));
    assert_eq!(parser.ignore(), Ok(()));
    assert_eq!(parser.i8(), Ok(0x12));
}

#[test]
fn ignore_zero() {
    let mut parser = JceParser::from_bytes(&hex!("0c 10 12"));
    assert_eq!(parser.ignore(), Ok(()));
    assert_eq!(parser.i8(), Ok(0x12));
}

#[test]
fn ignore_bytes() {
    let mut parser = JceParser::from_bytes(&hex!("0d 00 0008 0123456789abcdef" "10 12"));
    assert_eq!(parser.ignore(), Ok(()));
    assert_eq!(parser.i8(), Ok(0x12));
}

////////////////////////////////////////////////////////////////////////////////
// error test

#[test]
fn extra_input() {
    let mut parser = JceParser::from_bytes(&hex!("00 12 34"));
    assert_eq!(parser.i8(), Ok(0x12));
    assert_eq!(parser.done(), false);
}

#[test]
fn less_input() {
    let mut parser = JceParser::from_bytes(&hex!("03 12 34"));
    assert!(parser.i64().is_err());
}

#[test]
fn less_input_unfixed() {
    let mut parser = JceParser::from_bytes(&hex!("09 0002 06 05 6669727374"));
    assert_eq!(parser.list(), Ok(2));
    assert_eq!(parser.pick_type(), Ok(JceType::String1));
    assert_eq!(parser.str(), Ok("first"));
    assert!(parser.str().is_err());
}

#[test]
fn wrong_parse() {
    let mut parser = JceParser::from_bytes(&hex!("03 0123456789abcdef"));
    assert_eq!(parser.pick_type(), Ok(JceType::I64));
    assert!(parser.i16().is_err());
}
