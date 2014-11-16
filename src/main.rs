extern crate csv;
extern crate serialize;

use lib::GtfsMap;

mod lib;

fn main()  {
    // TODO: make this useful
    
    let gtfs_path = "/home/schneg/Projects/bostonbusmap/tools/gtfs/mbta";

    let gtfs_map = GtfsMap::new(gtfs_path.to_string());
    for route in gtfs_map.find_routes_by_name("Red Line".to_string()).iter() {
        println!("{}", route.route_id);
        // prints 931_, 933_
    }

    for shape in gtfs_map.find_shapes_by_route("931_".to_string()).iter() {
        println!("{}", shape.shape_id);
    }
}

