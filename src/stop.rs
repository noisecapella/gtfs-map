extern crate csv;
use std::collections::HashSet;
use std::iter::Skip;
use std::fs::File;
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
    pub stop_url : String,
    pub parent_station : String
}

#[derive(Debug, Deserialize)]
struct StopCsv {
    stop_id: String,
    stop_code : String,
    stop_name : String,
    stop_desc : String,
    stop_lat : String,
    stop_lon : String,
    stop_url : String,
    parent_station : String
}

impl Stop {

    pub fn make_stops(stops_path : &Path) -> BTreeMap<String, Stop> {
        let file = File::open(stops_path).unwrap();
        let mut reader = csv::Reader::from_reader(file);

        let mut map : BTreeMap<String, Stop> = BTreeMap::new();

        for record in reader.deserialize() {
            let row: StopCsv = record.unwrap();

            let stop = Stop {
                stop_code : row.stop_code,
                stop_name : row.stop_name,
                stop_desc : row.stop_desc,
                stop_lat : row.stop_lat,
                stop_lon : row.stop_lon,
                stop_url : row.stop_url,
                parent_station : row.parent_station
            };
            map.insert(row.stop_id, stop);
        }
        println!("Finished reading stops");
        map
    }


}
