
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
    let mut ret: Vec<String> = Vec::new();
    for s in raw_csv.split("\n") {
        ret.push(s.to_string())
    }
    return ret;
}


