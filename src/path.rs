use std;
use shape::Shape;
use byteorder::{BigEndian, WriteBytesExt};

#[derive(Copy, Debug)]
pub struct Point {
    pub lat: f32,
    pub lon: f32,
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

pub fn get_blob_string_from_path(path: &[Vec<Point>]) -> String {
    let mut bytearray: Vec<u8> = Vec::new();

    fn add_int(bytes: &mut Vec<u8>, x: i32) {
        bytes.write_i32::<BigEndian>(x);
    };

    fn add_float(bytes: &mut Vec<u8>, f: f32) {
        bytes.write_f32::<BigEndian>(f);
    }

    fn add_path(bytes: &mut Vec<u8>, path: &[Point]) {
        add_int(bytes, (path.len() as i32) * 2);
        for point in path {
            add_float(bytes, point.lat);
            add_float(bytes, point.lon);
        }
    };

    fn add_paths(bytes: &mut Vec<u8>, paths: &[Vec<Point>]) {
        add_int(bytes, paths.len() as i32);
        for path in paths {
            add_path(bytes, path);
        }
    };
    
    add_paths(&mut bytearray, path);
    let mut hex: String = "X'".to_string();
    for byte in bytearray {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex.push_str("'");
    return hex;
}
