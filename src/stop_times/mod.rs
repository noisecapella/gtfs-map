extern crate csv;

use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::HashMap;


pub struct StopTimes {
    pub stop_lookup : HashMap<String, Vec<u32>>,
    pub trip_lookup : HashMap<String, Vec<u32>>,
    pub stop_times : Vec<StopTime>
}

pub struct StopTime {
    pub trip_id : String,
    pub arrival_time : String,
    pub departure_time : String,
    pub stop_id : String,
    pub stop_sequence : u32,
    pub stop_headsign : String,
    pub pickup_type : u32,
    pub drop_off_type : u32
}

impl StopTime {

    pub fn make_stop_times<'a>(stop_times_path : &Path) -> StopTimes {

        let mut reader = csv::Reader::from_file(stop_times_path).unwrap();

        let mut stop_times : Vec<StopTime> = Vec::new();
        let mut stop_lookup : HashMap<String, Vec<u32>> = HashMap::new();
        let mut trip_lookup : HashMap<String, Vec<u32>> = HashMap::new();

        for record in reader.decode() {
            let (trip_id, arrival_time, departure_time, stop_id, stop_sequence, stop_headsign, pickup_type, drop_off_type) : 
                (String, String, String, String, u32, String, u32, u32) = record.unwrap();

            let new_index = stop_times.len() as u32;
            let trip_id_clone = trip_id.clone();

            if stop_lookup.contains_key(&stop_id) {
                let mut stop_indexes = stop_lookup.get_mut(&stop_id).unwrap();
                stop_indexes.push(new_index);
            }
            else {
                let stop_id_clone = stop_id.clone();
                let mut stop_indexes : Vec<u32> = Vec::new();
                stop_indexes.push(new_index);
                stop_lookup.insert(stop_id_clone, stop_indexes);
            }

            
            if trip_lookup.contains_key(&trip_id) {
                let mut trip_indexes = trip_lookup.get_mut(&trip_id).unwrap();
                trip_indexes.push(new_index);
            }
            else {
                let trip_id_clone = trip_id.clone();
                let mut trip_indexes : Vec<u32> = Vec::new();
                trip_indexes.push(new_index);
                trip_lookup.insert(trip_id_clone, trip_indexes);
            }

            let stop_time = StopTime {
                trip_id : trip_id,
                arrival_time : arrival_time,
                departure_time : departure_time,
                stop_id : stop_id,
                stop_sequence : stop_sequence,
                stop_headsign : stop_headsign,
                pickup_type : pickup_type,
                drop_off_type : drop_off_type
            };
            stop_times.push(stop_time);

        }
        println!("Finished reading stop_times");

        StopTimes {
            stop_lookup : stop_lookup,
            trip_lookup : trip_lookup,
            stop_times : stop_times
        }
    }
}
