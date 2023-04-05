use rusqlite::Connection;
use rusqlite::types::ToSql;
use crate::path;
use crate::constants::{HUBWAY_COLOR, HUBWAY_AGENCY_ID};
use crate::error::Error;

pub fn generate_hubway(conn: &Connection, index: i32) -> Result<i32, Error> {
    let mut statement = conn.prepare("INSERT INTO routes (route, routetitle, color, oppositecolor, listorder, agencyid, pathblob) VALUES ($1, $2, $3, $4, $5, $6, $7)").unwrap();
    let route = "Hubway";
    let routetitle = "Bluebikes";
    let color = HUBWAY_COLOR;
    let oppositecolor = HUBWAY_COLOR;
    let listorder = index;
    let agencyid = HUBWAY_AGENCY_ID;
    let path = [];
    let pathblob = path::get_blob_from_path(&path);

    let fields: &[&dyn ToSql] = &[&route, &routetitle, &color, &oppositecolor, &listorder, &agencyid, &pathblob];
    
    (statement.execute(fields))?;
    Ok(index + 1)
}
