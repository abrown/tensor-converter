use std::fs;
use tensor_converter::{convert, Dimensions, Precision};

#[test]
fn test() {
    let converted = convert(
        "tests/test.jpg",
        Dimensions::new(481, 640, 3, Precision::U8),
    )
    .unwrap();

    let expected = fs::read("tests/test-1x3x640x481-u8.bgr").unwrap();
    assert_eq!(expected.len(), converted.len());
    for (i, (&e, c)) in expected.iter().zip(converted).enumerate() {
        if e != c {
            panic!(
                "First not-equal byte at index {}: expected {}, actual {}",
                i, e, c
            );
        }
    }
}
