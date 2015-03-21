extern crate csv;
use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::HashMap;


pub struct Shape {
    pub shape_pt_lat : String,
    pub shape_pt_lon : String,
    pub shape_pt_sequence : u32,
    pub shape_dist_traveled : String
}

impl Shape {
    pub fn make_shapes(shapes_path : &Path) -> HashMap<String, Shape> {
        let mut reader = csv::Reader::from_file(shapes_path).unwrap();

        let mut map : HashMap<String, Shape> = HashMap::new();

        for record in reader.decode() {
            let (shape_id, shape_pt_lat, shape_pt_lon, shape_pt_sequence, shape_dist_traveled) :
                (String, String, String, u32, String) = record.unwrap();

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
