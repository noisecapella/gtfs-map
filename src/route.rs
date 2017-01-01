extern crate csv;
use std::collections::HashSet;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::BTreeMap;

use std::path::Path;
pub struct Route {
    pub agency_id : String,
    pub route_short_name : String,
    pub route_long_name : String,
    pub route_desc : String,
    pub route_type : i32,
    pub route_url : String,
    pub route_color : i32,
    pub route_text_color : i32
}

impl Route {

    pub fn make_routes(routes_path : &Path) -> BTreeMap<String, Route> {
        let mut reader = csv::Reader::from_file(routes_path).unwrap();

        let mut map : BTreeMap<String, Route> = BTreeMap::new();

        for record in reader.decode() {
            let (route_id, agency_id, route_short_name, route_long_name, route_desc, route_type, route_url, route_color, route_text_color) : 
                (String, String, String, String, String, i32, String, String, String) = record.unwrap();

            let route = Route {
                agency_id : agency_id,
                route_short_name : route_short_name,
                route_long_name : route_long_name,
                route_desc : route_desc,
                route_type : route_type,
                route_url : route_url,
                route_color : i32::from_str_radix(&route_color, 16).unwrap_or(0),
                route_text_color : i32::from_str_radix(&route_color, 16).unwrap_or(0)
            };
            map.insert(route_id, route);
        }
        println!("Finished reading routes");
        map
    }


}
