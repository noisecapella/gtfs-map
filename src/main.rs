#![deny(warnings)]

extern crate csv;
extern crate getopts;
extern crate rusqlite;
extern crate byteorder;
extern crate reqwest;
extern crate xml;
#[macro_use]
extern crate serde_derive;

use gtfs_map::GtfsMap;
use std::path::Path;
use getopts::Options;
use rusqlite::Connection;
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
use std::str;
use error::Error;
use error::Error::GtfsMapError;

pub mod constants;


fn create_tables(connection: &Connection) -> Result<(), Error> {
    println!("Creating tables...");
    let create_sql = "CREATE TABLE IF NOT EXISTS bounds (route TEXT, weekdays INTEGER, start INTEGER, stop INTEGER)
CREATE TABLE IF NOT EXISTS directions (dirTag TEXT PRIMARY KEY, dirNameKey TEXT, dirTitleKey TEXT, dirRouteKey TEXT, useAsUI INTEGER)
CREATE TABLE IF NOT EXISTS directionsStops (dirTag TEXT, tag TEXT)
CREATE TABLE IF NOT EXISTS favorites (tag TEXT PRIMARY KEY)
CREATE TABLE IF NOT EXISTS locations (lat FLOAT, lon FLOAT, name TEXT PRIMARY KEY)
CREATE TABLE IF NOT EXISTS routes (route TEXT PRIMARY KEY, color INTEGER, oppositecolor INTEGER, pathblob BLOB, listorder INTEGER, agencyid INTEGER, routetitle TEXT)
CREATE TABLE IF NOT EXISTS stopmapping (route TEXT, tag TEXT, PRIMARY KEY (route, tag))
CREATE INDEX IF NOT EXISTS idxstopmappingroute ON stopmapping (route)
CREATE INDEX IF NOT EXISTS idxstopmappingtag ON stopmapping (tag)
CREATE TABLE IF NOT EXISTS stops (tag TEXT PRIMARY KEY, lat FLOAT, lon FLOAT, title TEXT, parent TEXT)
";
    for line in create_sql.split("\n") {
        let trim_line = line.trim();
        if !trim_line.is_empty() {
            let v: [String; 0] = [];
            (connection.execute(trim_line, &v))?;
        }
    }
    Ok(())
}

fn generate(gtfs_map: GtfsMap, connection: Connection, nextbus_agency: &str) -> Result<(), Error> {
    (create_tables(&connection))?;
    let mut index = 0;
    let mut stops_inserted: HashSet<String> = HashSet::new();
    if nextbus_agency == "mbta" {
        println!("Generating commuter rail stops...");
        index = (mbta::generate_commuter_rail(&connection, index, &gtfs_map, &mut stops_inserted))?;
        println!("Generating heavy rail stops...");
        index = (mbta::generate_heavy_rail(&connection, index, &gtfs_map, &mut stops_inserted))?;
        println!("Generating bus stops...");
        index = (mbta::generate_bus(&connection, index, &gtfs_map, &mut stops_inserted))?;
    }
    println!("Generating nextbus stops...");
    if nextbus_agency != "mbta" {
        index = (nextbus::generate(&connection, index, &gtfs_map, &mut stops_inserted, nextbus_agency))?;
    }
    if nextbus_agency == "mbta" {
        println!("Generating Hubway stops...");
        index = (hubway::generate_hubway(&connection, index))?;
    }
    println!("routes inserted: {}", index);

    let empty: [String; 0] = [];
    (connection.execute("COMMIT", &empty))?;
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
    opts.optopt("a", "nextbus_agency", "The agency to use when querying nextbus data", "NEXTBUS_AGENCY");

    let matches = (opts.parse(&args[1..]))?;
    if matches.opt_present("h") {
        let program = args[0].clone();
    
        print_usage(&program, opts);
        panic!("");
    }

    let gtfs_path_str = (matches.opt_str("p").ok_or(GtfsMapError("Missing gtfs path".to_owned())))?.to_string();
    let output_path_str = (matches.opt_str("o").ok_or(GtfsMapError("Missing output path".to_owned())))?;
    let output_path = Path::new(&output_path_str);
    let nextbus_agency = matches.opt_str("a").ok_or(GtfsMapError("Missing nextbus_agency".to_string()))?;

    let _ = std::fs::remove_file(output_path);

    let gtfs_map = (GtfsMap::new(gtfs_path_str))?;
    let connection = (Connection::open(&output_path))?;
    let empty: [String; 0] = [];
    (connection.execute("BEGIN TRANSACTION", &empty))?;
    Ok((gtfs_map, connection, nextbus_agency))
}

fn main()  {
    // TODO: make this useful
    let args : Vec<_> = env::args().collect();
    match parse_args(args) {
        Ok((gtfs_map, connection, nextbus_agency)) => {
            generate(gtfs_map, connection, &nextbus_agency).unwrap()
        }
        Err(err) => {
            panic!(err);
        }
    }
}

