extern crate getopts;
extern crate rusqlite;
extern crate byteorder;
extern crate hyper;
extern crate xml;

use gtfs_map::GtfsMap;
use std::path::Path;
use getopts::Options;
use rusqlite::Connection;
use std::fs;
use std::process::Command;
use rusqlite::types::ToSql;
use std::collections::HashSet;

pub mod path;
pub mod gtfs_map;
pub mod hubway;
pub mod mbta;
pub mod db;
pub mod error;
pub mod nextbus;
pub mod route;
pub mod shape;
pub mod trip;
pub mod common;
pub mod simplify_path;
pub mod stop;
pub mod stop_times;

use std::env;
use error::Error;
use error::Error::GtfsMapError;

pub mod constants;


fn create_tables(connection: &Connection, generate_path: &Path) -> Result<(), Error> {
    println!("Creating tables...");
    let path = &generate_path.join("print_create_tables.py");
    let mut command = Command::new("python3");
    let process = command.arg(path);
    if !try!(process.status()).success() {
        return Err(GtfsMapError(format!("Unable to run {}", path.to_str().unwrap())));
    }
    let output = try!(process.output());
    for line in String::from_utf8_lossy(&output.stdout).split("\n") {
        let trim_line = line.trim();
        if !trim_line.is_empty() {
            try!(connection.execute(trim_line, &[]));
        }
    }
    Ok(())
}

fn generate(gtfs_map: GtfsMap, connection: Connection, generate_path: &Path) -> Result<(), Error> {
    try!(create_tables(&connection, generate_path));
    let mut index = 0;
    println!("Generating Hubway stops...");
    index = try!(hubway::generate_hubway(&connection, index));
    let mut stops_inserted: HashSet<String> = HashSet::new();
    println!("Generating commuter rail stops...");
    index = try!(mbta::generate_commuter_rail(&connection, index, &gtfs_map, &mut stops_inserted));
    println!("Generating heavy rail stops...");
    index = try!(mbta::generate_heavy_rail(&connection, index, &gtfs_map, &mut stops_inserted));
    println!("Generating nextbus stops...");
    index = try!(nextbus::generate(&connection, index, &gtfs_map, &mut stops_inserted));
    println!("routes inserted: {}", index);

    try!(connection.execute("COMMIT", &[]));
    Ok(())
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_args(args: Vec<String>) -> Result<(GtfsMap, Connection, String), Error> {
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

    let gtfs_path_str = try!(matches.opt_str("p").ok_or(GtfsMapError("Missing gtfs path".to_owned()))).to_string();
    let output_path_str = try!(matches.opt_str("o").ok_or(GtfsMapError("Missing output path".to_owned())));
    let output_path = Path::new(&output_path_str);

    let generate_path_str = try!(matches.opt_str("g").ok_or(GtfsMapError("Missing generate path".to_owned())));
    std::fs::remove_file(output_path);
    
    let gtfs_map = GtfsMap::new(gtfs_path_str);
    let connection = try!(Connection::open(&output_path));
    try!(connection.execute("BEGIN TRANSACTION", &[]));
    Ok((gtfs_map, connection, generate_path_str))
}

fn main()  {
    // TODO: make this useful
    let args : Vec<_> = env::args().collect();
    match parse_args(args) {
        Ok((gtfs_map, connection, generate_path_str)) => {
            generate(gtfs_map, connection, &Path::new(&generate_path_str)).unwrap()
        }
        Err(err) => {
            panic!(err);
        }
    }
}

