extern crate getopts;
extern crate rusqlite;

use gtfs_map::GtfsMap;
use std::path::Path;
use getopts::Options;
use rusqlite::Connection;
use std::fs;
use std::process::Command;
use std::error::Error;
use rusqlite::types::ToSql;

pub mod path;
pub mod gtfs_map;
pub mod hubway;
pub mod mbta;
pub mod error;
pub mod route;
pub mod shape;
pub mod trip;
pub mod common;
pub mod stop;
pub mod stop_times;

use std::env;
use error::GtfsMapError;

pub mod constants;


fn create_tables(connection: &Connection, generate_path: &Path) -> Result<(), GtfsMapError> {
    println!("Creating tables...");
    let output = Command::new("python3").arg(generate_path.join("print_create_tables.py")).output().expect("could not execute print_create_tables.py");
    for line in String::from_utf8_lossy(&output.stdout).split("\n") {
        let trim_line = line.trim();
        if !trim_line.is_empty() {
            try!(connection.execute(trim_line, &[]));
        }
    }
}

fn generate(gtfs_map: GtfsMap, connection: Connection, generate_path: &Path) -> Result<(), GtfsMapError> {
    try!(create_tables(&connection, generate_path));
    let mut index = 0;
    println!("Generating Hubway stops...");
    index = try!(hubway::generate_hubway(&connection, index));
    index = try!(mbta::generate_heavy_rail(&connection, index, &gtfs_map));
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_args(args: Vec<String>) -> Result<(GtfsMap, Connection, String), GtfsMapError> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print help menu");
    opts.optopt("p", "path", "Path to GTFS", "GTFS_PATH");
    opts.optopt("o", "output_database", "Path to output sqlite database", "DB_PATH");
    opts.optopt("g", "generate", "Path to bostonbusmap/tools/generate", "GENERATE_PATH");

    let matches = try!(opts.parse(&args[1..]));
    if matches.opt_present("h") {
        let program = args[0].clone();
    
        print_usage(&program, opts);
        panic!("");
    }

    let gtfs_path_str = try!(matches.opt_str("p").ok_or("Missing gtfs path".to_owned()));
    let gtfs_path = Path::new(&gtfs_path_str);
    let output_path_str = try!(matches.opt_str("o").ok_or("Missing output path".to_owned()));
    let output_path = Path::new(&output_path_str);

    let generate_path_str = try!(matches.opt_str("g").ok_or("Missing generate path".to_owned()));
    std::fs::remove_file(output_path);
    
    let gtfs_map = GtfsMap::new(gtfs_path);
    let connection = try!(Connection::open(&output_path));
    Ok((gtfs_map, connection, generate_path_str))
}

fn main()  {
    // TODO: make this useful
    let args : Vec<_> = env::args().collect();
    match parse_args(args) {
        Ok((gtfs_map, connection, generate_path_str)) => {
            generate(gtfs_map, connection, &Path::new(&generate_path_str))
        }
        Err(err) => {
            panic!(err.description().to_owned());
        }
    }
}

