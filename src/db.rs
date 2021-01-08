use rusqlite::Connection;
use rusqlite::types::ToSql;
use error::Error;

pub fn insert_route(conn: &Connection, route_id: &str, route_title: &str, route_color: i32, route_opposite_color: i32, index: i32, agency_id: i32, pathblob: &Vec<u8>) -> Result<i32, Error> {
    let mut statement = (conn.prepare("INSERT INTO routes (route, routetitle, color, oppositecolor, listorder, agencyid, pathblob) VALUES ($1, $2, $3, $4, $5, $6, $7)"))?;
    let fields : &[&dyn ToSql] = &[&route_id, &route_title, &route_color, &route_opposite_color, &index, &agency_id, pathblob];
    (statement.execute(fields))?;

    Ok(1)
}

pub fn insert_stop(conn: &Connection, stop_id: &str, stop_title: &str, lat: &str, lon: &str, parent: &str) -> Result<(), Error> {
    let mut statement = (conn.prepare("INSERT INTO stops (tag, title, lat, lon, parent) VALUES ($1, $2, $3, $4, $5)"))?;
    let fields: &[&dyn ToSql] = &[&stop_id, &stop_title, &lat, &lon, &parent];
    (statement.execute(fields))?;

    Ok(())
}

pub fn insert_stopmapping(conn: &Connection, stop_id: &str, route_id: &str) -> Result<(), Error> {
    let mut statement = (conn.prepare("INSERT INTO stopmapping (route, tag) VALUES ($1, $2)"))?;
    let fields : &[&dyn ToSql] = &[&route_id, &stop_id];
    (statement.execute(fields))?;

    Ok(())
}

pub fn insert_direction(conn: &Connection, tag: &str, title: &str, route_id: &str, name: &str, use_as_ui: bool) -> Result<(), Error> {
    let mut statement = (conn.prepare("INSERT INTO directions (dirTag, dirTitleKey, dirRouteKey, dirNameKey, useAsUI) VALUES ($1, $2, $3, $4, $5)"))?;
    let fields : &[&dyn ToSql] = &[&tag, &title, &route_id, &name, &use_as_ui];
    (statement.execute(fields))?;

    Ok(())
}
