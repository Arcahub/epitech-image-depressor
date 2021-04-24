use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opts {
    #[structopt(name = "input_path", short, long, parse(from_os_str))]
    pub input: PathBuf,

    #[structopt(name = "output_path", short, long, parse(from_os_str))]
    pub output: PathBuf,
}
