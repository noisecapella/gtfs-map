extern crate csv;

use std::fs::File;
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
    pub route_text_color : i32,
    pub route_sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct RouteCsv {
    route_id: String,
    agency_id: Option<String>,
    route_short_name: String,
    route_long_name: String,
    route_desc: String,
    route_type: i32,
    route_url: String,
    route_color: String,
    route_sort_order: Option<i32>,
}


impl Route {

    pub fn make_routes(routes_path : &Path) -> BTreeMap<String, Route> {
        let file = File::open(routes_path).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        let mut map : BTreeMap<String, Route> = BTreeMap::new();

        for record in reader.deserialize() {
            let row: RouteCsv = record.unwrap();

            let route_color = i32::from_str_radix(&row.route_color, 16).unwrap_or(0);
            let route_id = row.route_id.to_string();
            //if route_id == "CapeFlyer" {
            //    continue;
            //}

            let route = Route {
                agency_id : match row.agency_id {
                    Some(agency_id) => agency_id,
                    None => "1".to_string()
                },
                route_short_name : row.route_short_name,
                route_long_name : row.route_long_name,
                route_desc : row.route_desc,
                route_type : row.route_type,
                route_url : row.route_url,
                route_color : route_color,
                route_text_color : route_color,
                route_sort_order: row.route_sort_order,
            };
            map.insert(route_id.to_string(), route);
        }
        map
    }

    pub fn get_route_title<'a>(&'a self) -> &'a str {
        if self.route_short_name.len() != 0 {
            self.route_short_name.as_ref()
        } else if self.route_long_name.len() != 0 {
            self.route_long_name.as_ref()
        } else {
            panic!("Route without a title")
        }
    }
}
