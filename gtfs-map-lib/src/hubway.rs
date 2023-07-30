use async_rusqlite::Connection;
use crate::path;
use crate::constants::{HUBWAY_COLOR, HUBWAY_AGENCY_ID};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn generate_hubway(conn: &Connection, index: i32) -> Result<i32, Error> {

    conn.call(move |conn| {
        let route = "Hubway";
        let routetitle = "Bluebikes";
        let color = HUBWAY_COLOR;
        let oppositecolor = HUBWAY_COLOR;
        let listorder = index;
        let agencyid = HUBWAY_AGENCY_ID;
        let path = [];
        let pathblob = path::get_blob_from_path(&path);

        conn.execute(
            "INSERT INTO routes (route, routetitle, color, oppositecolor, listorder, agencyid, pathblob) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &route, &routetitle, &color, &oppositecolor, &listorder, &agencyid, &pathblob
            )
        )

    }).await?;
    Ok(index + 1)
}
