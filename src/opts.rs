use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opts {
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    pub output: PathBuf,
}
