extern crate csv;

use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::str;

pub struct StopTimes {
    pub trip_lookup: BTreeMap<String, Vec<u64>>,
    pub stop_times_path: PathBuf,
}

impl StopTimes {
    pub fn make_stop_times(stop_times_path: &Path) -> Result<StopTimes, Error> {
        let f = try!(File::open(stop_times_path));
        let mut reader = csv::Reader::from_reader(BufReader::new(f));

        let mut counter: u64 = 0;
        let mut trip_lookup: BTreeMap<String, Vec<u64>> = BTreeMap::new();
        while !reader.done() {
            let mut field_count = 0;
            loop {
                let pointer = reader.byte_offset();
                match reader.next_bytes() {
                    csv::NextField::EndOfCsv => break,
                    csv::NextField::EndOfRecord => {
                        counter += 1;
                        if counter % 100000 == 0 {
                            println!("Read {} rows {}", counter, pointer);
                        }
                        break;
                    },
                    csv::NextField::Error(err) => panic!(err),
                    csv::NextField::Data(field) => {
                        if field_count == 0 {
                            let s = str::from_utf8(field).unwrap();
                            let list = trip_lookup.entry(s.to_string()).or_insert(vec![]);
                            list.push(pointer);
                        }
                        field_count += 1;
                    }
                }
            }
        }
        Ok(StopTimes {
            trip_lookup: trip_lookup,
            stop_times_path: PathBuf::from(stop_times_path)
        })
    }
}
