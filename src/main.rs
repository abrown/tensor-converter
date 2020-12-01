use std::{env, fs, path::PathBuf};
use tensor_converter::{convert, Dimensions, Precision};

fn main() {
    let input_image_path = env::args_os()
        .nth(1)
        .expect("Missing input image path; usage: tensor-converter [input.jpg] [output.raw]")
        .into_string()
        .unwrap();
    assert!(PathBuf::from(&input_image_path).is_file());
    let output_tensor_path = env::args_os()
        .nth(2)
        .expect("Missing output tensor path; usage: tensor-converter [input.jpg] [output.raw]");

    let tensor_data = convert(
        input_image_path,
        Dimensions::new(300, 300, 3, Precision::FP32),
    )
    .unwrap();

    fs::write(output_tensor_path, tensor_data).expect("Failed to write tensor")
}
