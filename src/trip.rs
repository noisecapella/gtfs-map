extern crate csv;
use std::collections::HashSet;
use std::fs::File;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::path::Path;

pub struct Trip {
    pub route_id : String,
    pub service_id : String,
    pub trip_headsign : String,
    pub direction_id : u32,
    pub block_id : String,
    pub shape_id : String
}

#[derive(Debug, Deserialize)]
struct TripCsv {
    trip_id: String,
    route_id: String,
    service_id: String,
    trip_headsign: String,
    direction_id: u32,
    block_id: String,
    shape_id: String,
}

impl Trip {
    pub fn make_trips(trips_path : &Path) -> BTreeMap<String, Trip> {
        let file = File::open(trips_path).unwrap();
        let mut reader = csv::Reader::from_reader(file);

        let mut map : BTreeMap<String, Trip> = BTreeMap::new();

        for record in reader.deserialize() {
            let row: TripCsv = record.unwrap();

            let trip = Trip {
                route_id : row.route_id,
                service_id : row.service_id,
                trip_headsign: row.trip_headsign,
                direction_id : row.direction_id,
                block_id : row.block_id,
                shape_id : row.shape_id
            };
            map.insert(row.trip_id, trip);
        }
        println!("Finished reading trips");
        map
        
    }
}
