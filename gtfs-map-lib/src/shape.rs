extern crate csv;
use std::fs::File;
use std::collections::BTreeMap;
use std::path::Path;

pub struct Shape {
    pub shape_pt_lat : f64,
    pub shape_pt_lon : f64,
    pub shape_pt_sequence : u32,
}

#[derive(Debug, Deserialize)]
pub struct ShapeCsv {
    shape_id: String,
    shape_pt_lat : f64,
    shape_pt_lon : f64,
    shape_pt_sequence : u32,
}

impl Shape {
    pub fn make_shapes(shapes_path : &Path) -> Result<BTreeMap<String, Vec<Shape>>, std::io::Error> {
        let file = File::open(shapes_path)?;
        let mut reader = csv::Reader::from_reader(file);

        let mut map : BTreeMap<String, Vec<Shape>> = BTreeMap::new();

        for record in reader.deserialize() {
            let row: ShapeCsv = record.unwrap();

            let list = map.entry(row.shape_id).or_insert(vec![]);
            list.push(Shape {
                shape_pt_lat: row.shape_pt_lat,
                shape_pt_lon: row.shape_pt_lon,
                shape_pt_sequence: row.shape_pt_sequence,
            });
        }
        Ok(map)
    }
}
