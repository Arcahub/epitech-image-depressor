use image::{Rgb, RgbImage};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::{fs::File, num::ParseIntError, path::PathBuf, str::FromStr, u32};

struct PixelDescr {
    pub x: u32,
    pub y: u32,
    pub color: Rgb<u8>,
}

impl FromStr for PixelDescr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref FMT: Regex =
                Regex::new(r"\(([0-9]+),([0-9]+)\) \(([0-9]+),([0-9]+),([0-9]+)\)").unwrap();
        }
        let cap = FMT.captures(s).unwrap();
        Ok(PixelDescr {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap(),
            color: Rgb([0, 0, 0]),
        })
    }
}

pub fn txt_to_img(input_path: PathBuf, output_path: PathBuf) {
    let input_file = File::open(input_path).expect("Can't open input file");
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut pixels: Vec<PixelDescr> = Vec::new();

    let buff = BufReader::new(input_file);
    let mut set_cluster: bool = false;
    let mut cluster_color: Rgb<u8> = Rgb([0, 0, 0]);
    let cluster_r: Regex = Regex::new(r"\((\d{1,3}),(\d{1, 3}),(\d{1,3})\)").unwrap();

    for line_raw in buff.lines() {
        let line = line_raw.unwrap();
        if line == "--" {
            set_cluster = true;
            continue;
        } else if line == "-" {
            set_cluster = false;
            continue;
        } else if set_cluster {
            let cap = cluster_r.captures(&line).expect("Invalid line format");

            cluster_color = Rgb([
                cap.get(1).unwrap().as_str().parse().unwrap(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
                cap.get(3).unwrap().as_str().parse().unwrap(),
            ]);
            continue;
        }
        let mut pixel: PixelDescr = line.parse().unwrap();
        if pixel.x > width {
            width = pixel.x;
        }
        if pixel.y > height {
            height = pixel.y
        }
        pixel.color = cluster_color;
        pixels.push(pixel);
    }
    println!("{}, {}", width, height);
    let mut output_buff = RgbImage::new(width + 1, height + 1);

    for pixel in pixels {
        output_buff.put_pixel(pixel.x, pixel.y, pixel.color);
    }
    output_buff
        .save(output_path)
        .expect("Can't open output file");
}
