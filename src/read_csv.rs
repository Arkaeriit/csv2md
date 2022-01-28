
use std::fs;

pub fn open_file(path: String) {
    let _ = open_file_as_lines(path);
}

/// From the path of a file, returns a vector where heach entry is a line of
/// the input file.
fn open_file_as_lines(path: String) -> Vec<String> {
    println!("I wish I could open a file.");
    let raw_csv = fs::read_to_string(path).expect("Unable to open input file.");
    println!("{}", raw_csv);
    println!("{}", raw_csv.len());
    return split_to_vec(raw_csv, "\n".to_string());
}

/// This function is splits a string into a vector.
fn split_to_vec(full_str: String, delimiter: String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for s in full_str.split(&delimiter) {
        ret.push(s.to_string())
    }
    return ret;
}


/// Removes a copy of the string without trailing or leading whitespace.
fn remove_leading_trailing_whitespace(s: String) -> String {
    let mut ret: String = String::new();
    // Removing leading whitespace
    let mut not_leading_whitespace = false;
    for c in s.chars() {
        if !c.is_whitespace() {
            not_leading_whitespace = true;
        }
        if not_leading_whitespace {
            ret.push(c)
        }
    }
    // Removing trailing whitespace
    loop {
        let last_char = match ret.pop() {
            Some(c) => c,
            None => return ret,
        };
        if !last_char.is_whitespace() {
            ret.push(last_char);
            break;
        }
    }
    return ret;
}

