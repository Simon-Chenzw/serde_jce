use hex_literal::hex;
use serde::Deserialize;

macro_rules! de_test {
    ($func:ident, $value:expr, $typ:ty, $expected:expr) => {
        #[test]
        fn $func() {
            let val: $typ = serde_jce::from_bytes(&$value).unwrap();
            assert_eq!(val, $expected);
        }
    };
}

////////////////////////////////////////////////////////////////////////////////
// simple test

de_test!(bool_true, hex!("00 01"), bool, true);
de_test!(bool_false, hex!("0c"), bool, false);
de_test!(i8, hex!("00 12"), i8, 0x12);
de_test!(i16, hex!("01 1234"), i16, 0x1234);
de_test!(i32, hex!("02 12345678"), i32, 0x12345678);
de_test!(i64, hex!("03 0123456789abcdef"), i64, 0x0123456789abcdef);
de_test!(u8_lower, hex!("00 12"), u8, 0x12);
de_test!(u8_upper, hex!("01 00 ff"), u8, u8::MAX);
de_test!(
    f32,
    hex!("04 12345678"),
    f32,
    f32::from_be_bytes(hex!("12345678"))
);
de_test!(
    f64,
    hex!("05 0123456789abcdef"),
    f64,
    f64::from_be_bytes(hex!("0123456789abcdef"))
);
de_test!(char, hex!("06 01 61"), char, 'a');
de_test!(str, hex!("06 04 31323334"), &str, "1234");
de_test!(
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
de_test!(
    bytes,
    hex!("0d 00 0008 0123456789abcdef"),
    &serde_bytes::Bytes,
    serde_bytes::Bytes::new(&hex!("0123456789abcdef"))
);
de_test!(none, hex!("0c"), Option<i8>, None);
de_test!(some, hex!("00 12"), Option<i8>, Some(0x12));
de_test!(unit, hex!("0c"), (), ());
de_test!(
    list,
    hex!("09 0004 0012 0034 0056 0078"),
    Vec<i8>,
    Vec::from([0x12, 0x34, 0x56, 0x78])
);
de_test!(tuple, hex!("09 0002 0012 0034"), (i8, i8), (0x12, 0x34));
de_test!(
    map,
    hex!("08 0002 0001 1002 0003 1004"),
    std::collections::BTreeMap<i8,i8>,
    std::collections::BTreeMap::from([(1, 2), (3, 4)])
);

////////////////////////////////////////////////////////////////////////////////
// Zero test

de_test!(i8_zero, hex!("0c"), i8, 0);
de_test!(i64_zero, hex!("0c"), i64, 0);
de_test!(f64_zero, hex!("05 0000000000000000"), f64, 0.0);
de_test!(char_zero, hex!("0c"), char, '\x00');
de_test!(str_zero_1, hex!("06 00"), &str, "");
de_test!(str_zero_2, hex!("06 02 0000"), &str, "\x00\x00");
de_test!(
    bytes_zero_1,
    hex!("0d 00 0c"),
    &serde_bytes::Bytes,
    serde_bytes::Bytes::new(&[])
);
de_test!(
    bytes_zero_2,
    hex!("0d 00 0002 0000"),
    &serde_bytes::Bytes,
    serde_bytes::Bytes::new(&[0x00, 0x00])
);
de_test!(list_zero, hex!("09 0c"), Vec<i8>, Vec::new());
de_test!(
    map_zero,
    hex!("08 0c"),
    std::collections::BTreeMap<i8, i8>,
    std::collections::BTreeMap::new()
);

////////////////////////////////////////////////////////////////////////////////
// failed test

#[test]
fn extra_bytes() {
    let res: serde_jce::Result<u64> = serde_jce::from_bytes(&hex!("00 12 34"));
    assert!(res.is_err())
}

#[test]
fn struct_tag_error() {
    #[derive(PartialEq, Debug, Deserialize)]
    struct Test {
        #[serde(rename = "0")]
        v0: i8,
        #[serde(rename = "0")]
        v1: i16,
    }
    let bytes = &hex!("0a 0001 1002 0b");
    let res: serde_jce::Result<Test> = serde_jce::from_bytes(bytes);
    assert!(res.is_err());
}

#[test]
fn data_tag_error() {
    #[derive(PartialEq, Debug, Deserialize)]
    struct Test {
        #[serde(rename = "0")]
        v0: i8,
        #[serde(rename = "1")]
        v1: i16,
    }
    let bytes = &hex!("0a 0001 0002 0b");
    let res: serde_jce::Result<Test> = serde_jce::from_bytes(bytes);
    assert!(res.is_err());
}

////////////////////////////////////////////////////////////////////////////////
// struct test

#[test]
fn struct_tag_skip() {
    #[derive(PartialEq, Debug, Deserialize)]
    struct Test {
        #[serde(rename = "1")]
        v0: i8,
        #[serde(rename = "3")]
        v1: i8,
    }
    let bytes = &hex!("0a 0001 1002 2003 3004 4005 0b");
    let res: serde_jce::Result<Test> = serde_jce::from_bytes(bytes);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), Test { v0: 2, v1: 4 });
}

#[test]
fn whole_struct() {
    #[derive(PartialEq, Debug, Deserialize)]
    struct TestSub {
        #[serde(rename = "0")]
        v0: i8,
        #[serde(rename = "1")]
        v1: i16,
    }

    #[derive(PartialEq, Debug, Deserialize)]
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

    let bytes = hex!(
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
    let val: Test = serde_jce::from_bytes(&bytes).unwrap();
    let expected = Test {
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
    assert_eq!(val, expected);
}
