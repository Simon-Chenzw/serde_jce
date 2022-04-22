use hex_literal::hex;
use serde::Serialize;

macro_rules! ser_test {
    ($func:ident, $expected:expr, $typ:ty, $value:expr) => {
        #[test]
        fn $func() {
            let val: $typ = $value;
            assert_eq!(serde_jce::to_bytes(&val), Ok($expected.to_vec()));
        }
    };
}

////////////////////////////////////////////////////////////////////////////////
// simple test

ser_test!(bool_true, hex!("00 01"), bool, true);
ser_test!(bool_false, hex!("0c"), bool, false);
ser_test!(i8, hex!("00 12"), i8, 0x12);
ser_test!(i16, hex!("01 1234"), i16, 0x1234);
ser_test!(i32, hex!("02 12345678"), i32, 0x12345678);
ser_test!(i64, hex!("03 0123456789abcdef"), i64, 0x0123456789abcdef);
ser_test!(u8_lower, hex!("00 12"), u8, 0x12);
ser_test!(u8_upper, hex!("01 00 ff"), u8, u8::MAX);
ser_test!(
    f32,
    hex!("04 12345678"),
    f32,
    f32::from_be_bytes(hex!("12345678"))
);
ser_test!(
    f64,
    hex!("05 0123456789abcdef"),
    f64,
    f64::from_be_bytes(hex!("0123456789abcdef"))
);
ser_test!(char, hex!("06 01 61"), char, 'a');
ser_test!(str, hex!("06 04 31323334"), &str, "1234");
ser_test!(
    str_long,
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
    &str,
    std::str::from_utf8(&[0x7f; 300]).unwrap()
);
ser_test!(
    bytes,
    hex!("0d 00 0008 0123456789abcdef"),
    &serde_bytes::Bytes,
    serde_bytes::Bytes::new(&hex!("0123456789abcdef"))
);
ser_test!(none, hex!("0c"), Option<i8>, None);
ser_test!(some, hex!("00 12"), Option<i8>, Some(0x12));
ser_test!(unit, hex!("0c"), (), ());
ser_test!(
    list,
    hex!("09 0004 0012 0034 0056 0078"),
    Vec<i8>,
    Vec::from([0x12, 0x34, 0x56, 0x78])
);
ser_test!(tuple, hex!("09 0002 0012 0034"), (i8, i8), (0x12, 0x34));
ser_test!(
    map,
    hex!("08 0002 0001 1002 0003 1004"),
    std::collections::BTreeMap<i8,i8>,
    std::collections::BTreeMap::from([(1, 2), (3, 4)])
);

#[test]
fn with_tag() {
    let val = 0x12_u8;
    assert_eq!(
        serde_jce::to_bytes_with_tag(0xab, &val).unwrap(),
        hex!("f0 ab 12")
    );
}

////////////////////////////////////////////////////////////////////////////////
// Zero test

ser_test!(i8_zero, hex!("0c"), i8, 0);
ser_test!(i64_zero, hex!("0c"), i64, 0);
ser_test!(f64_zero, hex!("05 0000000000000000"), f64, 0.0);
ser_test!(str_zero_1, hex!("06 00"), &str, "");
ser_test!(str_zero_2, hex!("06 02 0000"), &str, "\x00\x00");
ser_test!(
    bytes_zero_1,
    hex!("0d 00 0c"),
    &serde_bytes::Bytes,
    serde_bytes::Bytes::new(&[])
);
ser_test!(
    bytes_zero_2,
    hex!("0d 00 0002 0000"),
    &serde_bytes::Bytes,
    serde_bytes::Bytes::new(&[0x00, 0x00])
);
ser_test!(list_zero, hex!("09 0c"), Vec<i8>, Vec::new());
ser_test!(
    map_zero,
    hex!("08 0c"),
    std::collections::BTreeMap<i8, i8>,
    std::collections::BTreeMap::new()
);

////////////////////////////////////////////////////////////////////////////////
// failed test

#[test]
fn u64_error() {
    let val = u64::MAX;
    assert!(serde_jce::to_bytes(&val).is_err());
}

#[test]
fn struct_tag_error() {
    #[derive(Serialize)]
    struct Test {
        v0: i8,
        v1: i16,
    }
    let val = Test { v0: 0, v1: 0 };
    assert!(serde_jce::to_bytes(&val).is_err());
}

#[test]
fn struct_tag_duplicate() {
    #[derive(Serialize)]
    struct Test {
        #[serde(rename = "0")]
        v0: i8,
        #[serde(rename = "0")]
        v1: i16,
    }
    let val = Test { v0: 0, v1: 0 };
    assert!(serde_jce::to_bytes(&val).is_err());
}

////////////////////////////////////////////////////////////////////////////////
// struct test

#[test]
fn jce_struct() {
    #[derive(Serialize)]
    struct TestSub {
        #[serde(rename = "0")]
        v0: i8,
        #[serde(rename = "1")]
        v1: i16,
    }

    #[derive(Serialize)]
    struct Test {
        #[serde(rename = "0")]
        v0: i8,
        #[serde(rename = "1")]
        v1: i16,
        #[serde(rename = "2")]
        v2: i32,
        #[serde(rename = "3")]
        v3: i64,
        #[serde(rename = "4")]
        v4: f32,
        #[serde(rename = "5")]
        v5: f64,
        #[serde(rename = "6")]
        v6: String,
        #[serde(rename = "7")]
        v7: String,
        #[serde(rename = "8")]
        v8: std::collections::BTreeMap<i8, i8>,
        #[serde(rename = "9")]
        v9: Vec<i8>,
        #[serde(rename = "10")]
        v10: TestSub,
        #[serde(rename = "12")]
        v12: Option<u8>,
        #[serde(rename = "13")]
        #[serde(with = "serde_bytes")]
        v13: Vec<u8>,
        #[serde(rename = "200")]
        v200: i8,
    }

    let test = Test {
        v0: 0x01,
        v1: 0x0123,
        v2: 0x01234567,
        v3: 0x0123456789abcdef,
        v4: f32::from_be_bytes([0x12, 0x34, 0x56, 0x78]),
        v5: f64::from_be_bytes([0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]),
        v6: String::from("\x12\x34\x56\x78"),
        v7: "\x7f".repeat(300),
        v8: std::collections::BTreeMap::from([(1, 2), (3, 4)]),
        v9: vec![1, 2, 3, 4],
        v10: TestSub {
            v0: 0x12,
            v1: 0x1234,
        },
        v12: None,
        v13: vec![0x11, 0x22, 0x33, 0x44],
        v200: 0x01,
    };

    let expected = hex!(
        "0a"
        "00 01"
        "11 0123"
        "22 01234567"
        "33 0123456789abcdef"
        "44 12345678"
        "55 8877665544332211"
        "66 04 12345678"
        "77 0000012c"
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
        "88 0002 0001 1002 0003 1004"
        "99 0004 0001 0002 0003 0004"
        "aa 0012 111234 0b"
        "cc"
        "dd00 0004 11223344"
        "f0c8 01"
        "0b"
    );

    assert_eq!(serde_jce::to_bytes(&test).unwrap(), expected);
}
