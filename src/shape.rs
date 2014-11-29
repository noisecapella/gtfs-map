extern crate serialize;
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

pub struct Shape {
    pub shape_id : String,
    pub shape_pt_lat : String,
    pub shape_pt_lon : String,
    pub shape_pt_sequence : int,
    pub shape_dist_traveled : String
}

pub struct ShapeIterator {
    reader : BufferedReader<IoResult<File>>,
    trip_shape_ids : Option<HashSet<String>>
}

impl ShapeIterator {
    pub fn new(shapes_path : &Path, trip_shape_ids : Option<HashSet<String>>) -> ShapeIterator {
        let mut reader = BufferedReader::new(File::open(shapes_path));

        reader.read_line();
        ShapeIterator {
            reader : reader,
            trip_shape_ids : trip_shape_ids
        }
    }
}

impl Iterator<Shape> for ShapeIterator {
    fn next(&mut self) -> Option<Shape> {
        loop {
            let line = self.reader.read_line();

            match line {
                Ok(line_to_parse) => {
                    let pieces : Vec<&str> = line_to_parse.as_slice().trim().split_str(",").collect();
                    let shape = Shape {
                        shape_id: as_str(pieces[0]),
                        shape_pt_lat : as_str(pieces[1]),
                        shape_pt_lon: as_str(pieces[2]),
                        shape_pt_sequence: from_str(pieces[3]).unwrap(),
                        shape_dist_traveled: as_str(pieces[4])
                    };

                    match self.trip_shape_ids {
                        Some(ref trip_shape_ids) => {
                            if (*trip_shape_ids).contains(&shape.shape_id) {
                                return Some(shape)
                            }
                        },
                        None => return Some(shape)
                    }
                },
                Err(err) => return None
            }
        }
    }
}

