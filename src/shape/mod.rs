extern crate csv;
use std::collections::HashSet;
use std::io::fs::File;
use std::io::BufferedReader;
use std::slice::Items;
use std::iter::Skip;
use std::io::Lines;
use std::io::IoResult;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::HashMap;

use common::as_str;

pub struct Shape {
    pub shape_pt_lat : String,
    pub shape_pt_lon : String,
    pub shape_pt_sequence : int,
    pub shape_dist_traveled : String
}

impl Shape {
    pub fn make_shapes(shapes_path : &Path) -> HashMap<String, Shape> {
        let mut reader = csv::Reader::from_file(shapes_path);

        let mut map : HashMap<String, Shape> = HashMap::new();

        for record in reader.decode() {
            let (shape_id, shape_pt_lat, shape_pt_lon, shape_pt_sequence, shape_dist_traveled) :
                (String, String, String, int, String) = record.unwrap();

            let shape = Shape {
                shape_pt_lat : shape_pt_lat,
                shape_pt_lon : shape_pt_lon,
                shape_pt_sequence : shape_pt_sequence,
                shape_dist_traveled : shape_dist_traveled
            };
            map.insert(shape_id, shape);
        }
        println!("Finished reading shapes");
        map
    }
}