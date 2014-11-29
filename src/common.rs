pub fn as_str(s : &str) -> String {
    if s.len() > 2 && s.starts_with("\"") && s.ends_with("\"") {
        s.trim_chars('"').to_string()
    }
    else {        
        s.to_string()
    }
}

