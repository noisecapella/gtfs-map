extern crate csv;
extern crate serialize;

use std::collections::HashSet;

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

#[deriving(Decodable)]
pub struct Route {
    pub route_id : String,
    pub agency_id : String,
    pub route_short_name : String,
    pub route_long_name : String,
    pub route_desc : String,
    pub route_type : int,
    pub route_url : String,
    pub route_color : String,
    pub route_text_color : String
}

#[deriving(Decodable)]
pub struct Shape {
    pub shape_id : String,
    pub shape_pt_lat : String,
    pub shape_pt_lon : String,
    pub shape_pt_sequence : int,
    pub shape_dist_traveled : int
}

#[deriving(Decodable)]
pub struct Trip {
    pub route_id : String,
    pub service_id : String,
    pub trip_id : String,
    pub trip_headsign : String,
    pub trip_short_name : String,
    pub direction_id : int,
    pub block_id : String,
    pub shape_id : String
}

pub struct GtfsMap {
    gtfs_path : Path
}

impl GtfsMap {
    pub fn new(gtfs_path : String) -> GtfsMap {
        
        GtfsMap {
            gtfs_path: Path::new(gtfs_path)
        }
    }

    // TODO: make into iterators
    pub fn find_routes_by_name(&self, name : String) -> Vec<Route> {
        let routes_path = self.gtfs_path.join("routes.txt");
        let mut reader = csv::Reader::from_file(&routes_path);
        // TODO: this should be handled by reader

        let mut ret : Vec<Route> = Vec::new();

        for row in reader.decode() {
            let record : Route = row.unwrap();
            if record.route_short_name == name ||
                record.route_long_name == name {
                ret.push(record);
            }
        }
        ret
    }

    pub fn find_shapes_by_route(&self, route_id : String) -> Vec<Shape> {
        let trips_path = self.gtfs_path.join("trips.txt");
        let mut trip_reader = csv::Reader::from_file(&trips_path);
        // TODO: this should be handled by reader

        let mut trip_shape_ids : HashSet<String> = HashSet::new();

        for row in trip_reader.decode() {
            let trip : Trip = row.unwrap();
            if trip.route_id == route_id {
                trip_shape_ids.insert(trip.shape_id);
            }
        }

        let mut shapes : Vec<Shape> = Vec::new();
        
        // TODO: should we filter out duplicate shapes?
        let shapes_path = self.gtfs_path.join("shapes.txt");
        let mut shape_reader = csv::Reader::from_file(&shapes_path);
        for row in shape_reader.decode() {
            let shape : Shape = row.unwrap();
            if trip_shape_ids.contains(&shape.shape_id) {
                shapes.push(shape);
            }
                
        }
        shapes
    }

}