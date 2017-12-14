extern crate csv;

use std::collections::HashMap;
use std;
use std::str;
use error::Error;


pub fn read_header<T: std::io::Read>(reader: &mut csv::Reader<T>) -> Result<HashMap<String, usize>, Error> {
    let mut field_indexes: HashMap<String, usize> = HashMap::new();
    let mut header_field_count = 0;
    let mut record = csv::StringRecord::new();
    reader.read_record(&mut record)?;
    for (i, cell) in record.iter().enumerate() {
        field_indexes.insert(cell.to_string(), i);
    }
    Ok(field_indexes)
}
