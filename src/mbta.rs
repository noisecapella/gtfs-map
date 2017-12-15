use gtfs_map::GtfsMap;
use rusqlite::Connection;
use error::Error;
use path::{Point, get_blob_from_path};
use simplify_path::simplify_path;
use constants::{COMMUTER_RAIL_AGENCY_ID, SUBWAY_AGENCY_ID};
use std::collections::HashSet;
use db;

pub fn add_line(conn: &Connection, startorder: i32, route_ids: &[&str], as_route: &str, route_title: &str, agency_id: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>, color_override: Option<i32>) -> Result<i32, Error> {
    println!("Adding route {}...", as_route);
    let route = try!(gtfs_map.find_route_by_id(route_ids[0]));

    let shapes = try!(gtfs_map.find_shapes_by_routes(route_ids));
    let paths: Vec<Vec<Point>> = shapes.iter().map(
        |(_, shapes)| {
            let path: Vec<Point> = shapes.iter().map(|shape| Point::from(shape)).collect();
            let ret = simplify_path(&path);
            ret
        }
    ).collect();
    let pathblob = get_blob_from_path(&paths);

    let color = color_override.unwrap_or(route.route_color);
    let opposite_color = color;
    let routes_added = try!(db::insert_route(conn, as_route, route_title, color, opposite_color, startorder, agency_id, &pathblob));

    println!("Adding stops...");
    let stop_rows = try!(gtfs_map.find_stops_by_routes(route_ids));

    for (stop_id, stop) in stop_rows {
        if !stops_inserted.contains(&stop_id) {
            try!(db::insert_stop(conn, &stop_id, &stop.stop_name, &stop.stop_lat, &stop.stop_lon, &stop.parent_station));
            stops_inserted.insert(stop_id.to_string());
        }
        try!(db::insert_stopmapping(conn, &stop_id, as_route));
    }

    println!("Adding directions...");
    for (trip_id, trip) in gtfs_map.find_trips_by_routes(route_ids) {
        //println!("tag {}", trip_id);
        try!(db::insert_direction(conn, trip_id, &trip.trip_headsign, as_route, "", true));
    }
    Ok(routes_added)
}

pub fn generate_heavy_rail(connection: &Connection, startorder: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>) -> Result<i32, Error> {
    println!("Generating heavy rail stops...");
    let mut index = startorder;

    let mut green_handled = false;
    for route in gtfs_map.routes.keys() {
        let routes;
        if route.starts_with("Green") {
            if green_handled {
                continue;
            }
            routes = vec!["Green-B", "Green-C", "Green-D", "Green-E"];
            green_handled = true;
        } else {
            routes = vec![route];
        }
        index += try!(add_line(connection, index, &routes, route, route, SUBWAY_AGENCY_ID, gtfs_map, stops_inserted, None));
    }
    
    Ok(index)
}

pub fn generate_commuter_rail(connection: &Connection, startorder: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>) -> Result<i32, Error> {
    println!("Generating commuter rail stops...");
    let routes_in_order = [
        "CR-Greenbush",
        "CR-Kingston",
        "CR-Middleborough",
        "CR-Fairmount",
        "CR-Providence",
        "CR-Franklin",
        "CR-Needham",
        "CR-Worcester",
        "CR-Fitchburg",
        "CR-Lowell",
        "CR-Haverhill",
        "CR-Newburyport",
        "CapeFlyer",
    ];
    let mut index = startorder;
    const PURPLE: i32 = 0x940088;

    for route_id in routes_in_order.iter() {
        let route = try!(gtfs_map.find_route_by_id(route_id));
        let route_title = if route.route_short_name.len() != 0 {
            &route.route_short_name
        } else {
            &route.route_long_name
        };

        index += try!(add_line(connection, index, &[route_id], route_id, &route_title, COMMUTER_RAIL_AGENCY_ID, gtfs_map, stops_inserted, Some(PURPLE)));
    }

    Ok(index)
}
