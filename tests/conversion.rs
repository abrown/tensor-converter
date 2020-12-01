use std::fs;
use tensor_converter::{convert, Dimensions, Precision};
use pretty_env_logger;

#[test]
#[ignore]
fn match_fixture_result() {
    let converted = convert(
        "tests/test.jpg",
        Dimensions::new(481, 640, 3, Precision::U8),
    )
    .unwrap();

    let expected = fs::read("tests/test-1x3x640x481-u8.bgr").unwrap();
    assert_same_bytes(&expected, &converted);
}

#[test]
fn same_result_twice_u8() {
    let input = "tests/test.jpg";
    let dimensions = Dimensions::new(227, 227, 3, Precision::U8);

    let first = convert(input, dimensions.clone()).unwrap();
    let second = convert(input, dimensions).unwrap();
    assert_same_bytes(&first, &second);
}

#[test]
fn same_result_twice_fp32() {
    pretty_env_logger::init();
    let input = "tests/test.jpg";
    let dimensions = Dimensions::new(227, 227, 3, Precision::FP32);

    let first = convert(input, dimensions.clone()).unwrap();
    let second = convert(input, dimensions).unwrap();
    assert_same_bytes(&first, &second);
}

fn assert_same_bytes(a: &[u8], b: &[u8]) {
    assert_eq!(a.len(), b.len());
    for (i, (&a, &b)) in a.iter().zip(b).enumerate() {
        if a != b {
            panic!(
                "First not-equal byte at index {}: {} != {}",
                i, a, b
            );
        }
    }
}