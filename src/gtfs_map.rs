use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::thread;
use std::collections::HashMap;
use std::path::Path;

use error::GtfsMapError;
use route::Route;
use shape::Shape;
use trip::Trip;
use stop::Stop;
use stop_times::StopTime;
use stop_times::StopTimes;

pub struct GtfsMap {
    routes : HashMap<String, Route>,
    shapes : HashMap<String, Shape>,
    trips : HashMap<String, Trip>,
    stops : HashMap<String, Stop>,
    stop_times : StopTimes
}

impl GtfsMap {
    pub fn new(gtfs_path : &Path) -> GtfsMap { 
        let routes_path = gtfs_path.join("routes.txt");
        let shapes_path = gtfs_path.join("shapes.txt");
        let trips_path = gtfs_path.join("trips.txt");
        let stops_path = gtfs_path.join("stops.txt");
        let stop_times_path = gtfs_path.join("stop_times.txt");

        let routes = thread::spawn(move || Route::make_routes(&routes_path));
        let shapes = thread::spawn(move || Shape::make_shapes(&shapes_path));
        let trips = thread::spawn(move || Trip::make_trips(&trips_path));
        let stops = thread::spawn(move || Stop::make_stops(&stops_path));
        let stop_times = thread::spawn(move || StopTime::make_stop_times(&stop_times_path));

        GtfsMap {
            routes : routes.join().unwrap(),
            shapes : shapes.join().unwrap(),
            trips : trips.join().unwrap(),
            stops : stops.join().unwrap(),
            stop_times : stop_times.join().unwrap(),
        }
    }

    pub fn find_routes_by_name<'a>(&'a self, name : &'a str) -> HashMap<&'a str, &'a Route>
    {
        self.routes.iter()
            .filter(|&(route_id, route)| route.route_short_name == name || route.route_long_name == name)
            .map(|(route_id, route)| (route_id.as_ref(), route))
            .collect()
    }

    pub fn find_route_by_id<'a>(&'a self, id : &'a str) -> Result<&Route, GtfsMapError>
    {
        self.routes.get(id).ok_or(GtfsMapError::String("No route found"))
    }

    pub fn find_shapes_by_route<'a>(&'a self, route_id : &'a str) -> HashMap<&'a str, &'a Shape> {
        self.trips.iter()
            .filter(|&(trip_id, trip)| trip.route_id == route_id)
            .map(|(trip_id, trip)| {
                let shape_id_slice = trip.shape_id.as_ref();
                (shape_id_slice, self.shapes.get(shape_id_slice).unwrap())
            }).collect()
    }

    pub fn find_routes_by_route_type<'a>(&'a self, route_type : u32) -> HashMap<&'a str, &'a Route> {
        self.routes.iter()
            .filter(|&(route_id, route)| route.route_type == route_type)
            .map(|(route_id, route)| (route_id.as_ref(), route))
            .collect()
    }

    pub fn find_stops_by_route<'a>(&'a self, route_id : &'a str) -> HashMap<&'a str, &'a Stop> {
        self.trips.iter()
            .filter(|&(trip_id, trip)| trip.route_id == route_id)
            .flat_map(|(trip_id, trip)| {
                let stop_times_indexes = self.stop_times.trip_lookup.get(trip_id).unwrap();

                stop_times_indexes.iter()
                    .map(|i| {
                        let stop_time = self.stop_times.stop_times.get(*i as usize).unwrap();
                        let slice = stop_time.stop_id.as_ref();
                        let stop = self.stops.get(slice).unwrap();
                        (slice, stop)
                    })
            }).collect()
    }

    pub fn find_trips_by_route<'a>(&'a self, route_id : &'a str) -> HashMap<&'a str, &'a Trip> {
        let mut ret : HashMap<&'a str, &'a Trip> = HashMap::new();
        // TODO
        ret
    }
}
