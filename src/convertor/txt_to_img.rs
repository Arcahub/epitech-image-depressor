use image::{ImageBuffer, Rgb};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io::BufRead, path::PathBuf, str::FromStr, u32};
use std::{io::BufReader, string::ParseError};

#[derive(Debug)]
struct Cluster {
    pub color: Rgb<u8>,
    pub pixels: Vec<PixelCoord>,
}

#[derive(Debug)]
struct PixelCoord {
    pub x: u32,
    pub y: u32,
}

impl FromStr for PixelCoord {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PIXEL_FMT: Regex =
                Regex::new(r"\(([0-9]+),([0-9]+)\) \(([0-9]+),([0-9]+),([0-9]+)\)").unwrap();
        }
        let cap = PIXEL_FMT.captures(s).unwrap();
        Ok(PixelCoord {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap(),
        })
    }
}

pub fn txt_to_img(input_path: PathBuf, output_path: PathBuf) {
    let input_file = File::open(input_path).expect("Can't open input file");
    let mut width: u32 = 1;
    let mut height: u32 = 1;

    let buff = BufReader::new(input_file);
    let mut set_cluster: bool = false;
    let mut clusters: Vec<Cluster> = Vec::new();
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

            let cluster: Cluster = Cluster {
                color: Rgb([
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                ]),
                pixels: Vec::new(),
            };
            clusters.push(cluster);
            continue;
        }
        let pixel: PixelCoord = line.parse().unwrap();
        if pixel.x > width {
            width = pixel.x;
        }
        if pixel.y > height {
            height = pixel.y;
        }
        clusters.last_mut().unwrap().pixels.push(pixel);
    }
    let mut output_buff = ImageBuffer::new(width + 1, height + 1);

    for cluster in clusters {
        for pixel in cluster.pixels {
            output_buff.put_pixel(pixel.x, pixel.y, cluster.color);
        }
    }

    output_buff
        .save(output_path)
        .expect("Can't open output file");
}
