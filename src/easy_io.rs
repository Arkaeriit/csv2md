
use std::io::Write;
use std::fs;

/// From the path of a file, returns a vector where each entry is a line of
/// the input file.
/// TODO: Error handling without expect...
pub fn open_file_as_lines(path: &String) -> Vec<String> {
    let raw_csv = fs::read_to_string(path).expect("Unable to open input file.");
    return split_to_vec(&raw_csv, "\n");
}

/// This function is splits a string into a vector.
pub fn split_to_vec(full_str: &str, delimiter: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for s in full_str.split(delimiter) {
        ret.push(s.to_string())
    }
    return ret;
}

/// Writes a string to a file at the given path path.
pub fn write_to_path(path: &str, content: &str) {
    let mut f = fs::File::create(path).expect("Oops...");
    write!(f, "{}", content).expect("Is this really possible that the program fails here?");
}

