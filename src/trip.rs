use std::collections::HashSet;
use std::io::fs::File;
use std::io::BufferedReader;
use std::slice::Items;
use std::iter::Skip;
use std::io::Lines;
use std::io::IoResult;
use std::iter::Filter;
use std::rc::Rc;

use common::as_str;

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

pub struct TripIterator {
    reader : BufferedReader<IoResult<File>>
}

impl TripIterator {
    pub fn new(trips_path : &Path) -> TripIterator {
        let mut reader = BufferedReader::new(File::open(trips_path));
        
        reader.read_line();
        TripIterator {
            reader : reader
        }
    }
}

impl Iterator<Trip> for TripIterator {
    fn next(&mut self) -> Option<Trip> {
        let line = self.reader.read_line();

        match line {
            Ok(line_to_parse) => {
                let pieces : Vec<&str> = line_to_parse.as_slice().trim().split_str(",").collect();
                let trip = Trip {
                    route_id : as_str(pieces[0]),
                    service_id : as_str(pieces[1]),
                    trip_id : as_str(pieces[2]),
                    trip_headsign : as_str(pieces[3]),
                    trip_short_name : as_str(pieces[4]),
                    direction_id : from_str(pieces[5]).unwrap_or(0),
                    block_id : as_str(pieces[6]),
                    shape_id : as_str(pieces[7])
                };

                Some(trip)
            },
            Err(err) => None
        }
    }
}