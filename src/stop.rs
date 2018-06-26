extern crate csv;
use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

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

    pub fn make_stops(stops_path : &Path) -> BTreeMap<String, Stop> {
        let mut reader = csv::Reader::from_file(stops_path).unwrap();

        let mut map : BTreeMap<String, Stop> = BTreeMap::new();

        for record in reader.decode() {
            let unwrapped = record.unwrap();
            println!("printing...");
            println!("stop {:?}", unwrapped);
            let (stop_id, stop_code, stop_name, stop_desc, stop_lat, stop_lon, stop_url, location_type, parent_station, tpis_name) :
                (String, String, String, String, String, String, String, String, String, String) = unwrapped;

            let stop = Stop {
                stop_code : stop_code,
                stop_name : stop_name,
                stop_desc : stop_desc,
                stop_lat : stop_lat,
                stop_lon : stop_lon,
                zone_id : "".to_string(),
                stop_url : stop_url,
                location_type : 1,
                parent_station : parent_station
            };
            map.insert(stop_id, stop);
        }
        println!("Finished reading stops");
        map
    }


}
