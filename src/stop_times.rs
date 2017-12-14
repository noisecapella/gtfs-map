extern crate csv;

use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::iter::Map;
use std::rc::Rc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::str;

use common::read_header;

pub struct StopTimes {
    pub trip_lookup: BTreeMap<String, Vec<csv::Position>>,
    pub field_indexes: HashMap<String, usize>,
    pub stop_times_path: PathBuf,
}

impl StopTimes {
    pub fn make_stop_times(stop_times_path: &Path) -> Result<StopTimes, Error> {
        let f = try!(File::open(stop_times_path));
        let mut reader = csv::Reader::from_reader(BufReader::new(f));

        let mut trip_lookup: BTreeMap<String, Vec<csv::Position>> = BTreeMap::new();

        let field_indexes = read_header(&mut reader)?;

        let mut counter: u64 = 0;
        {
            let trip_id_index = *field_indexes.get("trip_id").unwrap();
            let mut iter = reader.into_records();
            let mut pos = csv::Position::new();
            loop {
                let next_pos = iter.reader().position().clone();
                match iter.next() {
                    Some(record) => {
                        let mut field_count = 0;
                        let trip_id = (record?)[trip_id_index].to_string();
                        let list = trip_lookup.entry(trip_id).or_insert(vec![]);
                        list.push(pos);

                        pos = next_pos;
                        
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
