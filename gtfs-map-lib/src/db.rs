use async_rusqlite::Connection;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn insert_route(conn: &Connection, route_id: &str, route_title: &str, route_color: i32, route_opposite_color: i32, index: i32, agency_id: i32, pathblob: &Vec<u8>) -> Result<i32, Error> {
    let route_id_str = route_id.to_string();
    let route_title_str = route_title.to_string();
    let pathblob_vec = pathblob.clone();

    conn.call(move |conn| {
        conn.execute("INSERT INTO routes (route, routetitle, color, oppositecolor, listorder, agencyid, pathblob) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", (
            route_id_str, route_title_str, &route_color, &route_opposite_color, &index, &agency_id, pathblob_vec
        ))
    }).await?;

    Ok(1)
}

pub async fn insert_stop(conn: &Connection, stop_id: &str, stop_title: &str, lat: &str, lon: &str, parent: &str) -> Result<(), Error> {
    let stop_id_str = stop_id.to_string();
    let stop_title_str = stop_title.to_string();
    let parent_str = parent.to_string();
    let lon_str = lon.to_string();
    let lat_str = lat.to_string();
    conn.call(move |conn| {
        conn.execute("INSERT INTO stops (tag, title, lat, lon, parent) VALUES (?1, ?2, ?3, ?4, ?5)", (
            stop_id_str, stop_title_str, lat_str, lon_str, parent_str
        ))
    }).await?;

    Ok(())
}

pub async fn insert_stopmapping(conn: &Connection, stop_id: &str, route_id: &str) -> Result<(), Error> {
    let stop_id_str = stop_id.to_string();
    let route_id_str = route_id.to_string();
    conn.call(move |conn| {
        conn.execute("INSERT INTO stopmapping (route, tag) VALUES (?1, ?2)", (route_id_str, stop_id_str))
    }).await?;

    Ok(())
}

pub async fn insert_direction(conn: &Connection, tag: &str, title: &str, route_id: &str, name: &str, use_as_ui: bool) -> Result<(), Error> {
    let name_str = name.to_string();
    let route_id_str = route_id.to_string();
    let title_str = title.to_string();
    let tag_str = tag.to_string();
    conn.call(move |conn| {
        conn.execute("INSERT INTO directions (dirTag, dirTitleKey, dirRouteKey, dirNameKey, useAsUI) VALUES (?1, ?2, ?3, ?4, ?5)", (
            tag_str, title_str, route_id_str, name_str, &use_as_ui
        ))
    }).await?;

    Ok(())
}
