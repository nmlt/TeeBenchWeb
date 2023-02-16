//! Fake teebench script that outputs teebench output if the right arguments are given.
//! Used for testing.
//!
//! Set `TEEBENCH_PLATFORM_APP_NAME` environment variable to ./native or ./sgx
use structopt::StructOpt;

//use common::data_types::{Algorithm, Dataset};

#[derive(Debug, StructOpt, PartialEq, Eq)]
#[allow(dead_code)]
#[structopt(name = "teebench", about = "fake placeholder for testing that outputs teebench output. Because I don't have SGX on my dev machine.")]
struct TeebenchArgs {
    #[structopt(skip = std::env::args().next().unwrap())]
    app_name: String,
    ///`-a` - join algorithm name. Currently working: `CHT`, `PHT`, `PSM`, `RHO`, `RHT`, `RSM`. Default: `RHO`
    #[structopt(short = "a", long, default_value = "RHO")]
    algorithm: String,
    ///`-c` - seal chunk size in kBs. if set to 0 then seal everything at once. Default: `0`
    #[structopt(short = "c", long, default_value = "0")]
    seal_chunk_size: u32,
    ///`-d` - name of pre-defined dataset. Currently working: `cache-fit`, `cache-exceed`, `L`. Default: `none`
    #[structopt(short = "d", long, default_value = "cache-fit")]
    dataset: String,
    ///`-l` - join selectivity. Should be a number between 0 and 100. Default: `100`
    #[structopt(short = "l", long, default_value = "100")]
    selectivity: u8,
    ///`-n` - number of threads used to execute the join algorithm. Default: `2`
    #[structopt(short = "n", long, default_value = "2")]
    threads: u8,
    ///`-r` - number of tuples of R relation. Default: `2097152`
    #[structopt(short = "r", long, default_value = "2097152")]
    r_tuples: u32,
    ///`-s` - number of tuples of S relation. Default: `2097152`
    #[structopt(short = "s", long, default_value = "2097152")]
    s_tuples: u32,
    ///`-t | --r-path` - filepath to build R relation. Default: `none`
    #[structopt(short = "t", long)]
    r_path: Option<String>,
    ///`-u | --s-path` - filepath to build S relation. Default `none`
    #[structopt(short = "u", long)]
    s_path: Option<String>,
    ///`-x` - size of R in MBs. Default: `none`
    #[structopt(short = "x", long)]
    r_size: Option<u32>,
    ///`-y` - size of S in MBs. Default: `none`
    #[structopt(short = "y", long)]
    s_size: Option<u32>,
    ///`-z` - data skew. Default: `0`
    #[structopt(short = "z", long, default_value = "0")]
    data_skew: u32,
}

fn main() {
    let opt = TeebenchArgs::from_args();
    println!("{:#?}", opt);
}
