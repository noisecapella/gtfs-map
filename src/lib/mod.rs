use std::collections::HashSet;
use std::io::fs::File;
use std::io::BufferedReader;
use std::slice::Items;
use std::iter::Skip;
use std::io::Lines;
use std::io::IoResult;
use std::iter::Filter;
use std::rc::Rc;
use std::sync::Future;

use route::Route;
use shape::Shape;
use trip::Trip;
use stop::Stop;
use stop_times::StopTime;
use stop_times::StopTimes;
use std::collections::HashMap;

pub struct GtfsMap {
    routes : HashMap<String, Route>,
    shapes : HashMap<String, Shape>,
    trips : HashMap<String, Trip>,
    stops : HashMap<String, Stop>,
    stop_times : StopTimes
}

impl GtfsMap {
    pub fn new(gtfs_path : Path) -> GtfsMap { 
        let routes_path = gtfs_path.join("routes.txt");
        let shapes_path = gtfs_path.join("shapes.txt");
        let trips_path = gtfs_path.join("trips.txt");
        let stops_path = gtfs_path.join("stops.txt");
        let stop_times_path = gtfs_path.join("stop_times.txt");

        let routes = Future::spawn(move || Route::make_routes(&routes_path));
        let shapes = Future::spawn(move || Shape::make_shapes(&shapes_path));
        let trips = Future::spawn(move || Trip::make_trips(&trips_path));
        let stops = Future::spawn(move || Stop::make_stops(&stops_path));
        let stop_times = Future::spawn(move || StopTime::make_stop_times(&stop_times_path));

        GtfsMap {
            routes : routes.unwrap(),
            shapes : shapes.unwrap(),
            trips : trips.unwrap(),
            stops : stops.unwrap(),
            stop_times : stop_times.unwrap()
        }
    }
    
    pub fn find_routes_by_name<'a>(&'a self, name : &'a str) -> Vec<(&'a str, &'a Route)> {
        let mut ret : Vec<(&'a str, &'a Route)> = Vec::new();
        for (route_id, route) in self.routes.iter() {
            if route.route_short_name == name || route.route_long_name == name {
                ret.push((route_id.as_slice(), route));
            }
        }
        ret
    }

    pub fn find_shapes_by_route<'a>(&'a self, route_id : &'a str) -> Vec<(&'a str, &'a Shape)> {
        let mut shape_ids : HashSet<&str> = HashSet::new();

        for (trip_id, trip) in self.trips.iter() {
            if trip.route_id == route_id {
                shape_ids.insert(trip.shape_id.as_slice());
            }
        }

        let mut ret : Vec<(&'a str, &'a Shape)> = Vec::new();
        
        for (shape_id, shape) in self.shapes.iter() {
            let shape_id_slice = shape_id.as_slice();
            if shape_ids.contains(shape_id_slice) {
                ret.push((shape_id_slice, shape));
            }
        }
        ret
    }

    pub fn find_routes_by_route_type<'a>(&'a self, route_type : int) -> Vec<(&'a str, &'a Route)> {
        let mut ret : Vec<(&'a str, &'a Route)> = Vec::new();

        for (route_id, route) in self.routes.iter() {
            if route.route_type == route_type {
                ret.push((route_id.as_slice(), route));
            }
        }

        ret
    }

    pub fn find_stops_by_route<'a>(&'a self, route_id : &'a str) -> Vec<(&'a str, &'a Stop)> {
        let mut ret : Vec<(&'a str, &'a Stop)> = Vec::new();


        ret
    }
}