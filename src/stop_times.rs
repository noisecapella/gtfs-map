extern crate csv;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;

use crate::common::read_header;

pub struct StopTimes {
    pub trip_lookup: BTreeMap<String, Vec<csv::Position>>,
    pub field_indexes: HashMap<String, usize>,
    pub stop_times_path: PathBuf,
}

impl StopTimes {
    pub fn make_stop_times(stop_times_path: &Path) -> Result<StopTimes, Error> {
        let f = (File::open(stop_times_path))?;
        let mut reader = csv::Reader::from_reader(BufReader::new(f));

        let mut trip_lookup: BTreeMap<String, Vec<csv::Position>> = BTreeMap::new();

        let field_indexes = read_header(&mut reader)?;

        {
            let trip_id_index = *field_indexes.get("trip_id").unwrap();
            let mut iter = reader.into_records();
            loop {
                let pos = iter.reader().position().clone();
                match iter.next() {
                    Some(record) => {
                        let trip_id = (record?)[trip_id_index].to_string();
                        let list = trip_lookup.entry(trip_id).or_insert(vec![]);
                        list.push(pos);
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        Ok(StopTimes {
            trip_lookup: trip_lookup,
            field_indexes: field_indexes,
            stop_times_path: PathBuf::from(stop_times_path)
        })
    }
}
