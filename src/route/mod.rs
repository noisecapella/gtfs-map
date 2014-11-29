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

pub struct Route {
    pub route_id : String,
    pub agency_id : String,
    pub route_short_name : String,
    pub route_long_name : String,
    pub route_desc : String,
    pub route_type : int,
    pub route_url : String,
    pub route_color : String,
    pub route_text_color : String
}

pub struct RouteIterator<'a>  {
    reader : BufferedReader<IoResult<File>>,
    name : Option<&'a str>
}

impl<'a> RouteIterator<'a> {
    pub fn new(routes_path : &Path, name : Option<&'a str>) -> RouteIterator<'a> {
        let mut reader = BufferedReader::new(File::open(routes_path));

        // skip first line
        reader.read_line();
        RouteIterator {
            reader : reader,
            name : name
        }
    }
}

impl<'a> Iterator<Route> for RouteIterator<'a> {
    fn next(&mut self) -> Option<Route> {
        loop {
            let line = self.reader.read_line();

            match line {
                Ok(line_to_parse) => {
                    let pieces : Vec<&str> = line_to_parse.as_slice().trim().split_str(",").collect();
                    
                    let route = Route {
                        route_id: as_str(pieces[0]),
                        agency_id: as_str(pieces[1]),
                        route_short_name: as_str(pieces[2]),
                        route_long_name: as_str(pieces[3]),
                        route_desc: as_str(pieces[4]),
                        route_type: from_str(pieces[5]).unwrap(),
                        route_url: as_str(pieces[6]),
                        route_color: as_str(pieces[7]),
                        route_text_color: as_str(pieces[8])
                    };

                    match self.name {
                        Some(name) => if route.route_long_name.as_slice() == name || route.route_short_name.as_slice() == name {
                            return Some(route)
                        },
                        None => return Some(route)
                    }
                },
                Err(err) => return None
            };
        }
    }
}

