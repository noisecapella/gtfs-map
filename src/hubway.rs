use std::collections::HashSet;

use rusqlite::Connection;
use rusqlite::types::ToSql;
use path;
use constants::{HUBWAY_COLOR, HUBWAY_AGENCY_ID};
use error::Error;

pub fn generate_hubway(conn: &Connection, index: i32) -> Result<i32, Error> {
    let info_url = "https://gbfs.thehubway.com/gbfs/en/station_information.json";
    let mut statement = conn.prepare("INSERT INTO routes (route, routetitle, color, oppositecolor, listorder, agencyid, pathblob) VALUES ($1, $2, $3, $4, $5, $6, $7)").unwrap();
    let route = "Hubway";
    let routetitle = "Bluebikes";
    let color = HUBWAY_COLOR;
    let oppositecolor = HUBWAY_COLOR;
    let listorder = index;
    let agencyid = HUBWAY_AGENCY_ID;
    let path = [];
    let pathblob = path::get_blob_from_path(&path);

    let fields: &[&ToSql] = &[&route, &routetitle, &color, &oppositecolor, &listorder, &agencyid, &pathblob];
    
    try!(statement.execute(fields));
    Ok(index + 1)
}
