use core::{fmt, slice};
use log::info;
use opencv::core::{MatTraitManual, Scalar_};
use opencv::{
    self,
    core::{CV_32FC3, CV_8UC3},
};
use std::path::Path;

/// Convert an image a path to a resized sequence of bytes.
pub fn convert<P: AsRef<Path>>(
    path: P,
    dimensions: Dimensions,
) -> Result<Vec<u8>, ConversionError> {
    let path = path.as_ref();
    info!("Converting {} to {:?}", path.display(), dimensions);
    if !path.is_file() {
        return Err(ConversionError("The path is not a valid file.".to_string()));
    }

    let src = opencv::imgcodecs::imread(
        &path
            .to_str()
            .ok_or(ConversionError("Unable to stringify the path.".to_string()))?,
        opencv::imgcodecs::IMREAD_COLOR,
    )?;

    // Create a destination Mat of the right shape, filling it with 0s (see
    // https://docs.rs/opencv/0.46.3/opencv/core/struct.Mat.html#method.new_rows_cols_with_default).
    // TODO use Mat::zeros
    let mut dst = opencv::core::Mat::new_rows_cols_with_default(
        dimensions.height,
        dimensions.width,
        dimensions.as_type(),
        Scalar_::all(0.0),
    )?;

    // Resize the `src` Mat into the `dst` Mat using bilinear interpolation (see
    // https://docs.rs/opencv/0.46.3/opencv/imgproc/fn.resize.html).
    let dst_size = dst.size()?;
    opencv::imgproc::resize(
        &src,
        &mut dst,
        dst_size,
        0.0,
        0.0,
        opencv::imgproc::INTER_LINEAR,
    )?;

    let dst_slice = unsafe { slice::from_raw_parts(dst.data()? as *const u8, dimensions.len()) };
    Ok(dst_slice.to_vec())
}

#[derive(Debug)]
pub struct ConversionError(String);
impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<opencv::Error> for ConversionError {
    fn from(e: opencv::Error) -> Self {
        Self(e.message)
    }
}

#[derive(Debug)]
pub struct Dimensions {
    height: i32,
    width: i32,
    channels: i32,
    precision: Precision,
}

impl Dimensions {
    pub fn new(height: i32, width: i32, channels: i32, precision: Precision) -> Self {
        Self {
            height,
            width,
            channels,
            precision,
        }
    }
    pub fn len(&self) -> usize {
        self.height as usize * self.width as usize * self.channels as usize * self.precision.len()
    }
    fn as_type(&self) -> i32 {
        use Precision::*;
        match (self.precision, self.channels) {
            (FP32, 3) => CV_32FC3,
            (U8, 3) => CV_8UC3,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Precision {
    U8,
    FP32,
}
impl Precision {
    pub fn len(&self) -> usize {
        match self {
            Self::U8 => 1,
            Self::FP32 => 4,
        }
    }
}
