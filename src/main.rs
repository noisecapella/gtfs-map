extern crate serialize;

use lib::GtfsMap;

use std::io::File;
use std::io::BufferedReader;

pub mod lib;
pub mod route;
pub mod shape;
pub mod trip;
pub mod common;


fn main()  {
    // TODO: make this useful
    
    let gtfs_path = Path::new("/home/schneg/Projects/bostonbusmap/tools/gtfs/mbta");
    let gtfs_map = GtfsMap::new(gtfs_path);
    let mut routes = gtfs_map.find_routes_by_name("Red Line");
    for &(route_id, route) in routes.iter() {
        println!("{}", route_id);
        // prints 931_, 933_
    }

    for &(shape_id, shape) in gtfs_map.find_shapes_by_route("931_").iter() {
        println!("{}", shape_id);
    }
}

