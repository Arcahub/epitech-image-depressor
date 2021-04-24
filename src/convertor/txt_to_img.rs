use image::{ImageBuffer, Rgb};
use lazy_static::lazy_static;
extern crate nom;

use nom::{
    alt, character::complete::digit0, combinator::eof, do_parse, eof, many_till, map_res,
    multi::many_till, named, opt, peek, tag, take_until,
};
use regex::Regex;
use std::string::ParseError;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr, u32};

#[derive(Debug)]
struct Cluster {
    pub color: Rgb<u8>,
    pub pixels: Vec<PixelCoord>,
}

named!(uint8 <&str, u8>,
    map_res!(digit0, FromStr::from_str)
);

named!(uint32 <&str, u32>,
    map_res!(digit0, FromStr::from_str)
);

named!(colorf<&str, Rgb<u8>>, do_parse!(
    tag!("(") >>
    r: uint8 >>
    tag!(",") >>
    g: uint8 >>
    tag!(",") >>
    b: uint8 >>
    tag!(")\n") >>
    (Rgb([r, g, b]))
));

named!(pixelf<&str, PixelCoord>, do_parse!(
    tag!("(") >>
    x: uint32 >>
    tag!(",") >>
    y: uint32 >>
    tag!(")") >>
    take_until!("\n") >>
    opt!(tag!("\n")) >>
    (PixelCoord {x: x, y: y})
));

named!(pixelsf < &str, Vec < PixelCoord >>, do_parse!(
    pixels: many_till!(
        pixelf,
        alt!(
            eof!() |
            peek!(tag!("--\n"))
        )) >>
    (pixels.0)
));

named!(
    clusterf<&str, Cluster>,
    do_parse!(
        tag!("--\n") >>
        color: colorf >>
        tag!("-\n") >>
        pixels: pixelsf >>
        (Cluster { color: color, pixels: pixels })
    )
);

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
    let mut input_file = File::open(input_path).expect("Can't open input file");
    let mut content: String = String::from("");

    input_file.read_to_string(&mut content).unwrap();

    let (_, (clusters, _)) = many_till(clusterf, eof)(&content).unwrap();
    let mut width: u32 = 1;
    let mut height: u32 = 1;

    for cluster in &clusters {
        for pixel in &cluster.pixels {
            if pixel.x > width {
                width = pixel.x;
            }
            if pixel.y > height {
                height = pixel.y;
            }
        }
    }
    let mut output_buff = ImageBuffer::new(width + 1, height + 1);

    for cluster in &clusters {
        for pixel in &cluster.pixels {
            output_buff.put_pixel(pixel.x, pixel.y, cluster.color);
        }
    }

    output_buff
        .save(output_path)
        .expect("Can't open output file");
}
