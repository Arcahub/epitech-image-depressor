use crate::convertor::img_to_txt;
use crate::convertor::txt_to_img;
use crate::opts::Opts;

use image::io::Reader as ImageReader;
use structopt::StructOpt;

pub fn run() {
    let opts: Opts = Opts::from_args();

    match ImageReader::open(opts.input.clone())
        .expect(&format!(
            "Can't open input file {}",
            opts.input.to_str().unwrap()
        ))
        .decode()
    {
        Ok(img) => img_to_txt::img_to_txt(img, opts.output),
        Err(_) => txt_to_img::txt_to_img(opts.input, opts.output),
    }
}
