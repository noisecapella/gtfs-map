use getopts::Options;
use std::path::{Path, PathBuf};
use std::env;
use tokio::runtime::Runtime;

use gtfs_map_lib::gtfs_map::GtfsMap;
use gtfs_map_lib::error;
use gtfs_map_lib::{ generate, initialize_db };



fn parse_args(args: Vec<String>) -> Result<(PathBuf, PathBuf, String), Box<dyn std::error::Error>> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print help menu");
    opts.optopt("p", "path", "Path to GTFS", "GTFS_PATH");
    opts.optopt("o", "output_database", "Path to output sqlite database", "DB_PATH");
    opts.optopt("a", "nextbus_agency", "The agency to use when querying nextbus data", "NEXTBUS_AGENCY");

    let matches = (opts.parse(&args[1..]))?;
    if matches.opt_present("h") {
        let program = args[0].clone();

        print_usage(&program, opts);
        panic!("");
    }

    let gtfs_path_str = (matches.opt_str("p").ok_or(error::ArgumentError::new("Missing gtfs path")))?;
    let gtfs_path = PathBuf::from(&gtfs_path_str);
    let output_path_str = (matches.opt_str("o").ok_or(error::ArgumentError::new("Missing output path")))?;
    let output_path = PathBuf::from(&output_path_str);
    let nextbus_agency = matches.opt_str("a").ok_or(error::ArgumentError::new("Missing nextbus_agency"))?;

    Ok((gtfs_path, output_path, nextbus_agency))
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[tokio::main]
async fn main()  {
    // TODO: make this useful
    let args : Vec<_> = env::args().collect();

    let rt = Runtime::new().unwrap();

    match parse_args(args) {
        Ok((gtfs_path, output_path, nextbus_agency)) => {
            let gtfs_map = GtfsMap::new(&nextbus_agency, &gtfs_path).unwrap();
            let connection = initialize_db(&output_path).await.unwrap();
            rt.block_on(generate(&gtfs_map, connection, &nextbus_agency)).unwrap();
        }
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}

