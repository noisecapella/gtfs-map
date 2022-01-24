use std;
use shape::Shape;
use byteorder::{BigEndian, WriteBytesExt};

#[derive(Copy, Debug)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

impl Clone for Point {
    fn clone(&self) -> Point { *self }
}

impl<'a> std::convert::From<&'a Shape> for Point {
    fn from(shape: &'a Shape) -> Point {
        Point {
            lat: shape.shape_pt_lat,
            lon: shape.shape_pt_lon,
        }
    }
}

pub fn get_blob_from_path(path: &[Vec<Point>]) -> Vec<u8> {
    let mut bytearray: Vec<u8> = Vec::new();

    fn add_int(bytes: &mut Vec<u8>, x: i32) {
        bytes.write_i32::<BigEndian>(x).unwrap();
    }

    fn add_float(bytes: &mut Vec<u8>, f: f64) {
        bytes.write_f32::<BigEndian>(f as f32).unwrap();
    }

    fn add_path(bytes: &mut Vec<u8>, path: &[Point]) {
        add_int(bytes, (path.len() as i32) * 2);
        for point in path {
            add_float(bytes, point.lat);
            add_float(bytes, point.lon);
        }
    }

    fn add_paths(bytes: &mut Vec<u8>, paths: &[Vec<Point>]) {
        add_int(bytes, paths.len() as i32);
        for path in paths {
            add_path(bytes, path);
        }
    }
    
    add_paths(&mut bytearray, path);
    bytearray
}
