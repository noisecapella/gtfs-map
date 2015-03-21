extern crate csv;
use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::HashMap;


pub struct Trip {
    pub route_id : String,
    pub service_id : String,
    pub trip_headsign : String,
    pub trip_short_name : String,
    pub direction_id : u32,
    pub block_id : String,
    pub shape_id : String
}

impl Trip {
    pub fn make_trips(trips_path : &Path) -> HashMap<String, Trip> {
        let mut reader = csv::Reader::from_file(trips_path).unwrap();

        let mut map : HashMap<String, Trip> = HashMap::new();

        for record in reader.decode() {
            let (route_id, service_id, trip_id, trip_headsign, trip_short_name, direction_id, block_id, shape_id) :
                (String, String, String, String, String, u32, String, String) = record.unwrap();

            let trip = Trip {
                route_id : route_id,
                service_id : service_id,
                trip_headsign: trip_headsign,
                trip_short_name : trip_short_name,
                direction_id : direction_id,
                block_id : block_id,
                shape_id : shape_id
            };
            map.insert(trip_id, trip);
        }
        println!("Finished reading trips");
        map
        
    }
}
