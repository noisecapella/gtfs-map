extern crate csv;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::path::Path;
use error::Error;


pub struct Shape {
    pub shape_pt_lat : f64,
    pub shape_pt_lon : f64,
    pub shape_pt_sequence : u32,
    pub shape_dist_traveled : String
}

impl Shape {
    pub fn make_shapes(shapes_path : &Path) -> Result<BTreeMap<String, Vec<Shape>>, Error> {
        let mut reader = csv::Reader::from_file(shapes_path).unwrap();

        let mut map : BTreeMap<String, Vec<Shape>> = BTreeMap::new();

        for record in reader.decode() {
            let (shape_id, shape_pt_lat, shape_pt_lon, shape_pt_sequence, shape_dist_traveled) :
                (String, String, String, u32, String) = record.unwrap();

            let shape = Shape {
                shape_pt_lat : try!(shape_pt_lat.parse()),
                shape_pt_lon : try!(shape_pt_lon.parse()),
                shape_pt_sequence : shape_pt_sequence,
                shape_dist_traveled : shape_dist_traveled
            };

            let mut list = map.entry(shape_id).or_insert(vec![]);
            list.push(shape);
        }
        println!("Finished reading shapes");
        Ok(map)
    }
}
