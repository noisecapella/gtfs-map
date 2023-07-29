extern crate csv;

use std::collections::HashMap;
use std;


pub fn read_header<T: std::io::Read>(reader: &mut csv::Reader<T>) -> Result<HashMap<String, usize>, csv::Error> {
    let mut field_indexes: HashMap<String, usize> = HashMap::new();
    let record = reader.headers()?;
    for (i, cell) in record.iter().enumerate() {
        field_indexes.insert(cell.to_string(), i);
    }
    Ok(field_indexes)
}
