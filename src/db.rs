use rusqlite::Connection;
use error::Error;
use route::Route;
use stop::Stop;

pub fn insert_route(conn: &Connection, route_id: &str, route_title: &str, route_color: i32, route_opposite_color: i32, index: i32, agency_id: i32, pathblob: &str) -> Result<i32, Error> {
    let color_hex = format!("{:#x}", route_color);
    let opposite_color_hex = format!("{:#x}", route_opposite_color);

    let mut statement = try!(conn.prepare("INSERT INTO routes (route, routetitle, color, oppositecolor, listorder, agencyid, pathblob) VALUES ($1, $2, $3, $4, $5, $6, $7)"));
    try!(statement.insert(&[&route_id, &route_title, &color_hex, &opposite_color_hex, &index, &agency_id, &pathblob]));

    Ok(1)
}

pub fn insert_stop(conn: &Connection, stop_id: &str, stop_title: &str, lat: &str, lon: &str, parent: &str) -> Result<(), Error> {
    let mut statement = try!(conn.prepare("INSERT INTO stops (tag, title, lat, lon, parent) VALUES ($1, $2, $3, $4, $5)"));
    try!(statement.insert(&[&stop_id, &stop_title, &lat, &lon, &parent]));

    Ok(())
}

pub fn insert_stopmapping(conn: &Connection, stop_id: &str, route_id: &str) -> Result<(), Error> {
    let mut statement = try!(conn.prepare("INSERT INTO stopmapping (route, tag) VALUES ($1, $2)"));
    try!(statement.insert(&[&route_id, &stop_id]));

    Ok(())
}

pub fn insert_direction(conn: &Connection, tag: &str, title: &str, route_id: &str, name: &str, use_as_ui: bool) -> Result<(), Error> {
    let mut statement = try!(conn.prepare("INSERT INTO directions (dirTag, dirTitleKey, dirRouteKey, dirNameKey, useAsUI) VALUES ($1, $2, $3, $4, $5)"));
    try!(statement.insert(&[&tag, &title, &route_id, &name, &use_as_ui]));

    Ok(())
}
