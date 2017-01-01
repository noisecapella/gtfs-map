use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::thread;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use error::Error;
use route::Route;
use shape::Shape;
use trip::Trip;
use stop::Stop;
use stop_times::StopTime;
use stop_times::StopTimes;

pub struct GtfsMap {
    routes : BTreeMap<String, Route>,
    shapes : BTreeMap<String, Vec<Shape>>,
    trips : BTreeMap<String, Trip>,
    stops : BTreeMap<String, Stop>,
    stop_times : StopTimes,
}

impl GtfsMap {
    pub fn new(gtfs_path_str : String) -> GtfsMap {
        let gtfs_path = Path::new(&gtfs_path_str);
        let routes_path = gtfs_path.join("routes.txt");
        let shapes_path = gtfs_path.join("shapes.txt");
        let trips_path = gtfs_path.join("trips.txt");
        let stops_path = gtfs_path.join("stops.txt");
        let stop_times_path = gtfs_path.join("stop_times.txt");

        let routes = Route::make_routes(&routes_path);
        let shapes = Shape::make_shapes(&shapes_path);
        let trips = Trip::make_trips(&trips_path);
        let stops = Stop::make_stops(&stops_path);
        let stop_times = StopTime::make_stop_times(&stop_times_path);

        GtfsMap {
            routes : routes,
            shapes : shapes,
            trips : trips,
            stops : stops,
            stop_times : stop_times,
        }
    }

    pub fn find_routes_by_name(&self, name : &str) -> BTreeMap<&str, &Route>
    {
        self.routes.iter()
            .filter(|&(route_id, route)| route.route_short_name == name || route.route_long_name == name)
            .map(|(route_id, route)| (route_id.as_ref(), route))
            .collect()
    }

    pub fn find_route_by_id(&self, id : &str) -> Result<&Route, Error>
    {
        self.routes.get(id).ok_or(Error::GtfsMapError("No route found".to_owned()))
    }

    pub fn find_shapes_by_routes(&self, route_ids : &[&str]) -> BTreeMap<&str, &Vec<Shape>> {
        self.trips.iter()
            .filter(|&(trip_id, trip)| route_ids.contains(&trip.route_id.as_ref()))
            .map(|(trip_id, trip)| {
                let shape_id_slice = trip.shape_id.as_ref();
                (shape_id_slice, self.shapes.get(shape_id_slice).unwrap())
            }).collect()
    }

    pub fn find_routes_by_route_type(&self, route_type : i32) -> BTreeMap<&str, &Route> {
        self.routes.iter()
            .filter(|&(route_id, route)| route.route_type == route_type)
            .map(|(route_id, route)| (route_id.as_ref(), route))
            .collect()
    }

    pub fn find_stops(&self) -> &BTreeMap<String, Stop> {
        &self.stops
    }

    pub fn find_stops_by_routes(&self, route_ids : &[&str]) -> BTreeMap<&str, &Stop> {
        self.trips.iter()
            .filter(|&(trip_id, trip)| route_ids.contains(&trip.route_id.as_ref()))
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

    pub fn find_trips_by_routes(&self, route_ids : &[&str]) -> BTreeMap<&str, &Trip> {
        self.trips.iter()
            .filter(|&(trip_id, trip)| route_ids.contains(&trip.route_id.as_ref()))
            .map(|(trip_id, trip)| (trip_id.as_ref(), trip))
            .collect()
    }
}
