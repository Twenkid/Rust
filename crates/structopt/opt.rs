//https://docs.rs/structopt/0.3.25/structopt/
//cargo new op
//cargo.toml
//[dependencies]
//
//structopt = { version = "0.3", features = [ "paw" ] }
//paw = "1.0"
//>cargo run "gzzz.txt" -o "krrr.txt" -v 0.567 --debug

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name //*** add last - if first: not expected
    #[structopt(short, long)]
    debug: bool,

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "v", long = "velocity", default_value = "42")]
    speed: f64,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}

//cargo new
//../src..
//cargo run -d true -v 0.563 -i guz.txt -o kur.txt -f pdf
//cargo run "gzzz.txt" -o "krrr.txt" -v 0.567 --debug
