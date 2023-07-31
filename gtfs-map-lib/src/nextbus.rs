use std::{thread, time};

use crate::db;
use crate::error;
use async_rusqlite::Connection;
use crate::gtfs_map::GtfsMap;
use crate::path::{Point, get_blob_from_path};
use crate::simplify_path::simplify_path;
use std::collections::{HashSet, HashMap};
use reqwest;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

use error::OtherError;

use crate::constants::{BUS_AGENCY_ID};

type Error = Box<dyn std::error::Error + Send + Sync>;

fn make_url(command: &str, route_name: Option<&str>, nextbus_agency: &str) -> String {
    format!(
        "https://retro.umoiq.com/service/publicXMLFeed?a={agency}&command={command}{other}",
        agency=nextbus_agency,
        command=command,
        other=(match route_name {
            Some(route) => format!("&r={}&verbose", route),
            None => "".to_string()
        }),
    )
}

fn get_attribute<'a>(attributes: &'a [OwnedAttribute], key: &str) -> Result<&'a str, error::XmlAttributeError> {
    for attribute in attributes {
        if attribute.name.local_name == key {
            return Ok(&attribute.value);
        }
    }

    Err(error::XmlAttributeError::new(&format!("Missing attribute {}", key)))
}

async fn get_routes(nextbus_agency: &str) -> Result<Vec<(String, String)>, Error> {
    let route_list_url = make_url("routeList", None, nextbus_agency);

    let route_list_response = reqwest::get(&route_list_url).await?.text().await?;
    let route_list_data = route_list_response.as_bytes();

    let parser = EventReader::new(route_list_data);
    let mut routes = vec![];
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) =>
                if name.local_name == "route" {
                    let route_name = (get_attribute(&attributes, "tag"))?;
                    let route_title = (get_attribute(&attributes, "title"))?;
                    routes.push((route_name.to_string(), route_title.to_string()));
                },
            Ok(_) => {},
            Err(_err) => { return Err(Box::new(_err)); }
            
        }
    };

    if routes.len() == 0 {
        Err(Box::new(OtherError::new("unable to fetch any routes")))
    } else {
        Ok(routes)
    }
}

fn make_route_config_url(route_name: &str, nextbus_agency: &str) -> String {
    make_url("routeConfig", Some(route_name), nextbus_agency)
}

async fn add_route(conn: &Connection, gtfs_map: &GtfsMap, route_name: &str, stops_inserted: &mut HashSet<String>, parents: &HashMap<&str, &str>, start_order: i32, nextbus_agency: &str) -> Result<i32, Error> {
    let mut maybe_route_config_data = reqwest::get(&make_route_config_url(route_name, nextbus_agency)).await;
    if let Err(_) = maybe_route_config_data {
        // try one more time
        maybe_route_config_data = reqwest::get(&make_route_config_url(route_name, nextbus_agency)).await;
    }
    let route_config_data = maybe_route_config_data?;

    let route_config_text = route_config_data.text().await?;
    let parser = EventReader::new(route_config_text.as_bytes());
    let mut in_direction = false;
    let mut current_route: Option<(String, String, i32, i32)> = None;
    let mut current_path_points: Vec<Point> = vec![];
    let mut current_paths: Vec<Vec<Point>> = vec![];
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_ref() {
                    "route" => {
                        let route_id = (get_attribute(&attributes, "tag"))?.to_string();
                        let route_title = (get_attribute(&attributes, "title"))?.to_string();
                        let color_string = (get_attribute(&attributes, "color"))?;
                        let color = (i32::from_str_radix(color_string, 16))?;
                        let opposite_color_string = (get_attribute(&attributes, "oppositeColor"))?;
                        let opposite_color = (i32::from_str_radix(opposite_color_string, 16))?;
                        current_route = Some((route_id, route_title, color, opposite_color));
                    },
                    "stop" => {
                        let tag = (get_attribute(&attributes, "tag"))?;
                        if !in_direction {
                            if !stops_inserted.contains(tag) {
                                stops_inserted.insert(tag.to_string());
                                let title = (get_attribute(&attributes, "title"))?;
                                let lat = (get_attribute(&attributes, "lat"))?;
                                let lon = (get_attribute(&attributes, "lon"))?;
                                
                                let maybe_stop_id = tag.split("_").next();
                                let mut parent_id = "";
                                if let Some(stop_id) = maybe_stop_id {
                                    let parent = parents.get(stop_id);
                                    if let Some(id) = parent {
                                        parent_id = id;
                                    } else {
                                        println!("{agency}: WARNING: tag {tag} not in GTFS", agency=gtfs_map.agency, tag=tag);
                                    }
                                }
                                
                                (db::insert_stop(conn, tag, title, lat, lon, parent_id)).await?;
                            }
                            
                            (db::insert_stopmapping(conn, tag, &current_route.as_ref().unwrap().0)).await?;
                        }
                    },
                    "direction" => {
                        let dir_tag = (get_attribute(&attributes, "tag"))?;
                        in_direction = true;
                        if let Some(&(ref route_id, _, _, _)) = current_route.as_ref() {
                            let dir_title = (get_attribute(&attributes, "title"))?;
                            let dir_name = (get_attribute(&attributes, "name"))?;
                            let use_for_ui_string = (get_attribute(&attributes, "useForUI"))?;
                            let use_for_ui = use_for_ui_string == "true";
                            (db::insert_direction(conn, dir_tag, dir_title, route_id, dir_name, use_for_ui)).await?;
                        }
                    },
                    "point" => {
                        let lat_string = (get_attribute(&attributes, "lat"))?;
                        let lon_string = (get_attribute(&attributes, "lon"))?;
                        let lat: f64 = (lat_string.parse())?;
                        let lon: f64 = (lon_string.parse())?;
                        current_path_points.push(Point { lat: lat, lon: lon });
                    },
                    _ => {}
                }
            },
            Ok(XmlEvent::EndElement { name, .. }) => {
                match name.local_name.as_ref() {
                    "direction" => {
                        in_direction = false;
                    },
                    "route" => {
                        if let Some(&(ref route_id, ref route_title, color, opposite_color)) = current_route.as_ref() {
                            let pathblob = get_blob_from_path(&current_paths);
                            (db::insert_route(conn, route_id, route_title, color, opposite_color, start_order, BUS_AGENCY_ID, &pathblob)).await?;
                            current_paths.clear();
                        }
                        current_route = None;
                    },
                    "path" => {
                        current_paths.push(simplify_path(&current_path_points));
                        current_path_points.clear();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    Ok(1)
}

fn make_parents_map<'a>(gtfs_map: &'a GtfsMap) -> HashMap<&'a str, &'a str> {
    gtfs_map.find_stops().iter().map(
        |(stop_id, stop)| (stop_id.as_ref(), stop.parent_station.as_ref())
    ).collect()
}

pub async fn generate(conn: &Connection, start_order: i32, gtfs_map: &GtfsMap, stops_inserted: &mut HashSet<String>, nextbus_agency: &str) -> Result<i32, Error> {
    let mut index = start_order;

    let parents = make_parents_map(gtfs_map);
    
    println!("{}: Downloading NextBus route data (this will take 10 or 20 minutes)...", gtfs_map.agency);
    let routes = get_routes(nextbus_agency).await?;

    for (route_name, route_title) in routes {
        println!("{}: {}...", gtfs_map.agency, route_title);

        index += (add_route(conn, &gtfs_map, &route_name, stops_inserted, &parents, index, nextbus_agency)).await?;

        // NextBus rate limiting
        thread::sleep(time::Duration::from_secs(3));
    }
    
    Ok(index)
}
