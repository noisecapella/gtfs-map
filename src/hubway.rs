use std::collections::HashSet;

use rusqlite::Connection;
use path;
use constants::{HUBWAY_COLOR, HUBWAY_AGENCY_ID};
use error::Error;

pub fn generate_hubway(conn: &Connection, index: i32) -> Result<i32, Error> {
    let info_url = "https://gbfs.thehubway.com/gbfs/en/station_information.json";
    let mut statement = conn.prepare("INSERT INTO routes (route, routetitle, color, oppositecolor, listorder, agencyid, pathblob) VALUES ($1, $2, $3, $4, $5, $6, $7)").unwrap();
    let route = "Hubway";
    let routetitle = "Hubway";
    let color = HUBWAY_COLOR;
    let oppositecolor = HUBWAY_COLOR;
    let listorder = index;
    let agencyid = HUBWAY_AGENCY_ID;
    let path = [];
    let pathblob = path::get_blob_from_path(&path);
    
    try!(statement.insert(&[&route, &routetitle, &color, &oppositecolor, &listorder, &agencyid, &pathblob]));
    Ok(index + 1)
}
