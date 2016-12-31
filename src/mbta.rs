use gtfs_map::GtfsMap;
use rusqlite::Connection;
use error::Error;

fn add_line(connection: &Connection, startorder: i32, route_ids: &[&str], as_route: &str, gtfs_map: &GtfsMap) -> Result<i32, Error> {
    let route = try!(gtfs_map.find_route_by_id(route_ids[0]));
    let ref route_color = route.route_color;
    Ok(startorder + 1)
}

pub fn generate_heavy_rail(connection: &Connection, startorder: i32, gtfs_map: &GtfsMap) -> Result<i32, Error> {
    let mut index = startorder;
    index += try!(add_line(connection, index, &["Red"], "Red", gtfs_map));
    Ok(index)
}
