use std::collections::HashSet;
use std::io::fs::File;
use std::io::BufferedReader;
use std::slice::Items;
use std::iter::Skip;
use std::io::Lines;
use std::io::IoResult;
use std::iter::Filter;
use std::rc::Rc;

use route::RouteIterator;
use shape::ShapeIterator;
use trip::TripIterator;

#[deriving(Decodable)]
pub struct CalendarRow {
    pub service_id: String,
    pub monday: u8,
    pub tuesday: u8,
    pub wednesday: u8,
    pub thursday: u8,
    pub friday: u8,
    pub saturday: u8,
    pub sunday: u8,
    pub start_date: String,
    pub end_date: String
}

pub struct GtfsMap {
    gtfs_path : Path
}

impl GtfsMap {
    pub fn new(gtfs_path : Path) -> GtfsMap {
        
        GtfsMap {
            gtfs_path: gtfs_path
        }
    }

    // TODO: make into iterators
    pub fn find_routes_by_name<'a>(&'a self, name : &'a str) -> RouteIterator {
        let routes_path = self.gtfs_path.join("routes.txt");
        RouteIterator::new(&routes_path, Some(name))
    }
    
    pub fn find_shapes_by_route<'a>(&'a self, route_id : &'a str) -> ShapeIterator {
        let trips_path = self.gtfs_path.join("trips.txt");
        let mut trip_shape_ids : HashSet<String> = HashSet::new();

        for trip in TripIterator::new(&trips_path) {
            if trip.route_id.as_slice() == route_id {
                trip_shape_ids.insert(trip.shape_id);
            }
        }

        // TODO: should we filter out duplicate shapes?
        let shapes_path = self.gtfs_path.join("shapes.txt");

        ShapeIterator::new(&shapes_path, Some(trip_shape_ids))
    }

}