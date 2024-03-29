use std::str;
use async_rusqlite::Connection;
#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate async_rusqlite;
extern crate byteorder;
extern crate reqwest;
extern crate xml;

use gtfs_map::GtfsMap;
use std::path::Path;
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
pub mod constants;

type Error = Box<dyn std::error::Error + Send + Sync>;

async fn create_tables(connection: &Connection) -> Result<(), Error> {
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
            connection.call(move |connection| {
                connection.execute(trim_line, ())
            }).await?;
        }
    }
    Ok(())
}

pub async fn generate(gtfs_map: &GtfsMap, connection: Connection, nextbus_agency: &str) -> Result<(), Error> {
    (create_tables(&connection)).await?;
    let mut index = 0;
    let mut stops_inserted: HashSet<String> = HashSet::new();
    if nextbus_agency == "mbta" {
        index = (mbta::generate_commuter_rail(&connection, index, &gtfs_map, &mut stops_inserted)).await?;
        index = (mbta::generate_heavy_rail(&connection, index, &gtfs_map, &mut stops_inserted)).await?;
        index = (mbta::generate_bus(&connection, index, &gtfs_map, &mut stops_inserted)).await?;
    }
    if nextbus_agency != "mbta" {
        let future = nextbus::generate(&connection, index, &gtfs_map, &mut stops_inserted, nextbus_agency);
        index = future.await?;
    }
    if nextbus_agency == "mbta" {
        index = (hubway::generate_hubway(&connection, index)).await?;
    }

    connection.call(move |connection| {
        connection.execute("COMMIT", ())
    }).await?;
    Ok(())
}


pub async fn initialize_db(output_path: &Path) -> Result<Connection, Error> {
    let _ = std::fs::remove_file(output_path);

    let connection = (Connection::open(&output_path)).await?;

    connection.call(move |connection| {
        connection.execute("BEGIN TRANSACTION", ())
    }).await?;

    Ok(connection)
}
