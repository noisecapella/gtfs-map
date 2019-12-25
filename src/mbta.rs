use gtfs_map::GtfsMap;
use rusqlite::Connection;
use error::Error;
use path::{Point, get_blob_from_path};
use simplify_path::simplify_path;
use constants::{BUS_AGENCY_ID, COMMUTER_RAIL_AGENCY_ID, SUBWAY_AGENCY_ID};
use std::collections::{BTreeMap, HashSet};
use db;

pub fn add_line(conn: &Connection, route_sort_order: i32, route_ids: &[&str], as_route: &str, route_title: &str, agency_id: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>, color_override: Option<i32>) -> Result<i32, Error> {
    println!("Adding route {}...", as_route);
    let route = try!(gtfs_map.find_route_by_id(route_ids[0]));

    let shapes = match gtfs_map.find_shapes_by_routes(route_ids) {
        Ok(x) => x,
        Error => BTreeMap::new()
    };
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

static commuter_rail_routes: &'static [&'static str] = &[
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
];

static subway_routes: &'static [&'static str] = &[
    "Red",
    "Orange",
    "Blue",
    "Green-B",
    "Green-C",
    "Green-D",
    "Green-E",
    "Mattapan",
    "712",
    "713"
];

pub fn generate_bus(connection: &Connection, startorder: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>) -> Result<i32, Error> {
    let mut index = startorder;

    //subway_routes.contains(&"x");
    let mut titleSet = HashSet::new();
    for (route_id, route) in gtfs_map.find_routes() {
        if commuter_rail_routes.contains(&route_id) {
            println!("skipping commuter rail {}", route_id);
            continue;
        }

        if subway_routes.contains(&route_id) {
            println!("skipping subway {}", route_id);
            continue;
        }

        if route.route_desc == "Ferry" {
            continue;
        }

        let mut name = if route.route_short_name.len() == 0 {
            route.route_long_name.to_string()
        } else {
            route.route_short_name.to_string()
        };

        let mut counter = 0;
        let oldName = name.to_string();
        while titleSet.contains(&name) {
            counter += 1;
            name = format!("{} ({})", oldName.to_string(), counter);
        }
        titleSet.insert(name.to_string());

        println!("adding {}", route_id);
        index += try!(add_line(connection, index, &[route_id], route_id, &name, BUS_AGENCY_ID, gtfs_map, stops_inserted, None));
    }
    
    Ok(index)
}

pub fn generate_heavy_rail(connection: &Connection, startorder: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>) -> Result<i32, Error> {
    println!("Generating heavy rail stops...");
    let mut index = startorder;

    let mut green_handled = false;
    let mut red_line_shuttle_count = 0;
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
            route_title = "Green Line".to_string();
        } else {
            as_route = route_id;
            routes = vec![route_id];
            if as_route.starts_with("Logan") {
                route_title = as_route.to_string();
            } else if route.get_route_title().to_string().starts_with("Red Line Shuttle") {
                route_title = format!("Red Line Shuttle ({})", red_line_shuttle_count + 1).to_string();
                red_line_shuttle_count += 1;
            } else {
                route_title = route.get_route_title().to_string();
            }
        }
        add_line(connection, route.route_sort_order.unwrap_or(index), &routes, as_route, &route_title, get_source_id(as_route), gtfs_map, stops_inserted, None)?;
        index += 1;
    }
    
    Ok(index)
}

pub fn generate_commuter_rail(connection: &Connection, startorder: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>) -> Result<i32, Error> {
    let mut index = startorder;
    const purple: i32 = 0x940088;

    for route_id in commuter_rail_routes.iter() {
        let route = try!(gtfs_map.find_route_by_id(route_id));
        let route_title = if route.route_short_name.len() != 0 {
            &route.route_short_name
        } else {
            &route.route_long_name
        };

        index += try!(add_line(connection, index, &[route_id], route_id, &route_title, COMMUTER_RAIL_AGENCY_ID, gtfs_map, stops_inserted, Some(purple)));
    }
}
 
