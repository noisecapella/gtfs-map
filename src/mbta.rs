use gtfs_map::GtfsMap;
use rusqlite::Connection;
use error::Error;
use path::{Point, get_blob_from_path};
use simplify_path::simplify_path;
use constants::{SUBWAY_AGENCY_ID, COMMUTER_RAIL_AGENCY_ID, BUS_AGENCY_ID};
use std::collections::HashSet;
use db;

pub fn add_line(conn: &Connection, route_sort_order: i32, route_ids: &[&str], as_route: &str, route_title: &str, agency_id: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>, color_override: Option<i32>) -> Result<i32, Error> {
    println!("Adding route {}...", as_route);
    let route = try!(gtfs_map.find_route_by_id(route_ids[0]));

    let shapes = try!(gtfs_map.find_shapes_by_routes(route_ids));
    let paths: Vec<Vec<Point>> = shapes.iter().map(
        |(_, shapes)| {
            let path: Vec<Point> = shapes.iter().map(|shape| Point::from(shape)).collect();
            simplify_path(&path)
        }
    ).collect();
    let pathblob = get_blob_from_path(&paths);

    let color = color_override.unwrap_or(route.route_color);
    let opposite_color = color;
    let routes_added = try!(db::insert_route(conn, as_route, route_title, color, opposite_color, route_sort_order, agency_id, &pathblob));

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
    for (route_id, route) in gtfs_map.routes.iter() {
        let routes;
        let as_route;
        let route_title;
        if route_id.starts_with("Green") {
            if green_handled {
                continue;
            }
            as_route = "Green";
            routes = vec!["Green-B", "Green-C", "Green-D", "Green-E"];
            green_handled = true;
            route_title = "Green Line";
        } else {
            as_route = route_id;
            routes = vec![route_id];
            if as_route.starts_with("Logan") {
                route_title = as_route;
            } else {
                route_title = route.get_route_title();
            }
        }
        add_line(connection, route.route_sort_order.unwrap_or(index), &routes, as_route, route_title, get_source_id(as_route), gtfs_map, stops_inserted, None)?;
        index += 1;
    }
    
    Ok(index)
}

fn get_source_id(route: &str) -> i32 {
    if route.starts_with("CR-") {
        return COMMUTER_RAIL_AGENCY_ID
    }
    
    match route {
        "Red" => SUBWAY_AGENCY_ID,
        "Blue" => SUBWAY_AGENCY_ID,
        "Orange" => SUBWAY_AGENCY_ID,
        "Green" => SUBWAY_AGENCY_ID,
        "741" => SUBWAY_AGENCY_ID, // SL1
        "742" => SUBWAY_AGENCY_ID, // SL2
        "751" => SUBWAY_AGENCY_ID, // SL4
        "749" => SUBWAY_AGENCY_ID, // SL5
        "746" => SUBWAY_AGENCY_ID, // Silver Line Waterfront
        _ => BUS_AGENCY_ID
    }
}
 
