use image::DynamicImage;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn img_to_txt(img: DynamicImage, output_path: PathBuf) {
    let mut output_file: File = File::create(output_path).expect("Error can't open output file");
    let pixels = img.to_rgb8();
    let mut res: String = String::from("");
    pixels.enumerate_pixels().for_each(|(x, y, pixel)| {
        res.push_str(&format!(
            "({},{}) ({},{},{})\n",
            &x, &y, pixel[0], pixel[1], pixel[2]
        ));
    });
    output_file
        .write_all(res.as_bytes())
        .expect("Failed to write result to output file");
}
