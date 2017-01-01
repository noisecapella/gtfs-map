use error::Error;
use rusqlite::Connection;
use gtfs_map::GtfsMap;
use std::collections::HashSet;

pub fn generate<'a>(conn: &Connection, start_order: i32, gtfs_map: &'a GtfsMap, stops_inserted: &mut HashSet<&'a str>) -> Result<i32, Error> {
    Ok(0)
}
