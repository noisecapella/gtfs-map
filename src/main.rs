extern crate csv;
extern crate serialize;

use lib::GtfsMap;

mod lib;

fn main()  {
    // TODO: make this useful
    
    let gtfs_path = "/home/schneg/Projects/bostonbusmap/tools/gtfs/mbta";

    let gtfs_map = GtfsMap::new(gtfs_path.to_string());
    for x in gtfs_map.find_routes_by_name("Red Line".to_string()).iter() {
        println!("{}", x.route_id);
        // prints 931_, 933_
    }
}

