use gtfs_map::GtfsMap;
use rusqlite::Connection;
use std::error::Error;

fn add_line(connection: &Connection, startorder: i32, route_ids: &[&str], as_route: &str, gtfs_map: &GtfsMap) -> Result<i32, Box<Error>> {
    let route_color = try!(gtfs_map.find_route_by_id(route_ids[0])).route_color;
    Ok(startorder + 1)
}

pub fn generate_heavy_rail(connection: &Connection, startorder: i32, gtfs_map: &GtfsMap) -> Result<i32, Box<Error>> {
    let mut index = startorder;
    index += add_line(connection, index, &["Red"], "Red", gtfs_map);
    Ok(index)
}
