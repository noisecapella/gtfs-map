use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::str;

use csv;

use crate::error;
use crate::route::Route;
use crate::shape::Shape;
use crate::trip::Trip;
use crate::stop::Stop;
use crate::stop_times::StopTimes;

pub struct GtfsMap {
    pub routes : BTreeMap<String, Route>,
    pub shapes : BTreeMap<String, Vec<Shape>>,
    pub trips : BTreeMap<String, Trip>,
    pub stops : BTreeMap<String, Stop>,
    pub stop_times : StopTimes,
}

type Error = Box<dyn std::error::Error>;

impl GtfsMap {
    pub fn new(gtfs_path : &Path) -> Result<Self, Error> {
        let routes_path = gtfs_path.join("routes.txt");
        let shapes_path = gtfs_path.join("shapes.txt");
        let trips_path = gtfs_path.join("trips.txt");
        let stops_path = gtfs_path.join("stops.txt");
        let stop_times_path = gtfs_path.join("stop_times.txt");

        let routes = Route::make_routes(&routes_path);
        let shapes = (Shape::make_shapes(&shapes_path))?;
        let trips = Trip::make_trips(&trips_path);
        let stops = Stop::make_stops(&stops_path);
        let stop_times = (StopTimes::make_stop_times(&stop_times_path))?;

        Ok(GtfsMap {
            routes : routes,
            shapes : shapes,
            trips : trips,
            stops : stops,
            stop_times : stop_times,
        })
    }

    pub fn find_routes(&self) -> BTreeMap<&str, &Route> {
        self.routes.iter()
            .map(|(route_id, route)| {
                println!("mapping route {}", route_id);
                (route_id.as_ref(), route)
            })
            .collect()
    }
    
    pub fn find_routes_by_name(&self, name : &str) -> BTreeMap<&str, &Route>
    {
        self.routes.iter()
            .filter(|&(_, route)| route.get_route_title() == name)
            .map(|(route_id, route)| (route_id.as_ref(), route))
            .collect()
    }

    pub fn find_route_by_id(&self, id : &str) -> Result<&Route, Error>
    {
        let err = Box::new(error::NoRouteError::new(&format!("No route found for {}", id)));
        self.routes.get(id).ok_or(err)
    }

    pub fn find_shapes_by_routes(&self, route_ids : &[&str]) -> Result<BTreeMap<&str, Vec<Shape>>, Error> {
        let mut shape_map: BTreeMap<&str, Vec<Shape>> = BTreeMap::new();

        for (_, trip) in self.trips.iter() {
            if !route_ids.contains(&trip.route_id.as_ref()) {
                continue;
            }
            let shape_id_slice = trip.shape_id.as_ref();
            match self.shapes.get(shape_id_slice) {
                Some(shapes_ref) => {
                    let mut shapes: Vec<Shape> = Vec::new();
                    for shape in shapes_ref.iter() {
                        shapes.push(Shape {
                            shape_pt_lat: shape.shape_pt_lat,
                            shape_pt_lon: shape.shape_pt_lon,
                            shape_pt_sequence: shape.shape_pt_sequence,
                        });
                    }
                    shapes.sort_by(|a, b| a.shape_pt_sequence.cmp(&b.shape_pt_sequence));
                    shape_map.insert(
                        shape_id_slice,
                        shapes
                    );
                }
                None => {
                    println!("Missing shape {}", shape_id_slice);
                }
            };
        }

        Ok(shape_map)
    }

    pub fn find_routes_by_route_type(&self, route_type : i32) -> BTreeMap<&str, &Route> {
        self.routes.iter()
            .filter(|&(_, route)| route.route_type == route_type)
            .map(|(route_id, route)| (route_id.as_ref(), route))
            .collect()
    }

    pub fn find_stops(&self) -> &BTreeMap<String, Stop> {
        &self.stops
    }

    pub fn find_stops_by_routes(&self, route_ids : &[&str]) -> Result<BTreeMap<String, &Stop>, Error> {
        let mut ret: BTreeMap<String, &Stop> = BTreeMap::new();
        let path = self.stop_times.stop_times_path.as_path();
        let f = (File::open(path))?;
        let mut reader = csv::Reader::from_reader(BufReader::new(f));

        for (trip_id, trip) in self.trips.iter() {
            if !route_ids.contains(&trip.route_id.as_ref()) {
                continue;
            }

            let stop_id_index = *self.stop_times.field_indexes.get("stop_id").unwrap();
            //println!("stop_id_index {}", stop_id_index);
            let stop_times_indexes = (self.stop_times.trip_lookup.get(trip_id).ok_or(error::NoTripError::new(&format!("No trip found in stop_times for {}", trip_id))))?;
            //let mut firstRow = csv::StringRecord::new();
            //reader.read_record(&mut firstRow);
            for pos in stop_times_indexes.iter() {
                (reader.seek(pos.clone()))?;

                let mut row = csv::StringRecord::new();
                reader.read_record(&mut row)?;
                let stop_id = row[stop_id_index].to_string();

                //println!("row {}\n", stop_id);
                if stop_id == "70838" {
                    println!("special route {} {}", trip_id, trip.route_id);
                }
                let stop = self.stops.get(&stop_id).unwrap();
                ret.insert(stop_id, stop);
            }
        }
        Ok(ret)
    }

    pub fn find_trips_by_routes(&self, route_ids : &[&str]) -> BTreeMap<&str, &Trip> {
        self.trips.iter()
            .filter(|&(_, trip)| route_ids.contains(&trip.route_id.as_ref()))
            .map(|(trip_id, trip)| (trip_id.as_ref(), trip))
            .collect()
    }
}
