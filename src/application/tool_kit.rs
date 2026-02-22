use crate::domain::entities::{Dimensions, SegmentationResult};
use anyhow::Result;
use image::ImageReader;
use ndarray::{Array4, Axis};

pub(super) fn preprocess(image_bytes: Vec<u8>) -> Result<Array4<f32>> {
    let img = ImageReader::new(std::io::Cursor::new(image_bytes))
        .with_guessed_format()?
        .decode()?
        .to_rgb8();

    let (width, height) = img.dimensions();

    let mut array = Array4::<f32>::zeros((1, 3, height as usize, width as usize));

    for (x, y, pixel) in img.enumerate_pixels() {
        let [r, g, b] = pixel.0;

        array[[0, 0, y as usize, x as usize]] = r as f32 / 255.0;
        array[[0, 1, y as usize, x as usize]] = g as f32 / 255.0;
        array[[0, 2, y as usize, x as usize]] = b as f32 / 255.0;
    }

    Ok(array)
}

pub(super) fn postprocess(output: Array4<f32>) -> Result<SegmentationResult> {
    let output = output.index_axis(Axis(0), 0);
    let output = output.index_axis(Axis(0), 0);

    let height = output.shape()[0];
    let width = output.shape()[1];

    let mask: Vec<u8> = output
        .iter()
        .map(|&x| if x > 0.5 { 255 } else { 0 })
        .collect();

    Ok(SegmentationResult {
        mask,
        dimensions: Dimensions { width, height },
    })
}
