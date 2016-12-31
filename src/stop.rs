extern crate csv;
use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;

pub struct Stop {
    pub stop_code : String,
    pub stop_name : String,
    pub stop_desc : String,
    pub stop_lat : String,
    pub stop_lon : String,
    pub zone_id : String,
    pub stop_url : String,
    pub location_type : u32,
    pub parent_station : String
}

impl Stop {

    pub fn make_stops(routes_path : &Path) -> HashMap<String, Stop> {
        let mut reader = csv::Reader::from_file(routes_path).unwrap();

        let mut map : HashMap<String, Stop> = HashMap::new();

        for record in reader.decode() {
            let (stop_id, stop_code, stop_name, stop_desc, stop_lat, stop_lon, zone_id, stop_url, location_type, parent_station) :
                (String, String, String, String, String, String, String, String, u32, String) = record.unwrap();

            let stop = Stop {
                stop_code : stop_code,
                stop_name : stop_name,
                stop_desc : stop_desc,
                stop_lat : stop_lat,
                stop_lon : stop_lon,
                zone_id : zone_id,
                stop_url : stop_url,
                location_type : location_type,
                parent_station : parent_station
            };
            map.insert(stop_id, stop);
        }
        println!("Finished reading stops");
        map
    }


}
