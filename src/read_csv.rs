
use std::fs;

/// The main function of this module; from the path to a csv file, returns a 2D
/// vector representing the table.
/// TODO: Error handling without expect...
pub fn read_csv_file(path: &String) -> Vec<Vec<String>> {
    let raw_csv = open_file_as_lines(path);
    return clean_csv(&raw_csv);
}

/// From the path of a file, returns a vector where each entry is a line of
/// the input file.
fn open_file_as_lines(path: &String) -> Vec<String> {
    let raw_csv = fs::read_to_string(path).expect("Unable to open input file.");
    return split_to_vec(&raw_csv, "\n");
}

/// This function is splits a string into a vector.
fn split_to_vec(full_str: &String, delimiter: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    for s in full_str.split(delimiter) {
        ret.push(s.to_string())
    }
    return ret;
}


/// Removes a copy of the string without trailing or leading white-space.
fn remove_leading_trailing_whitespace(s: &String) -> String {
    let mut ret: String = String::new();
    // Removing leading white-space
    let mut not_leading_whitespace = false;
    for c in s.chars() {
        if !c.is_whitespace() {
            not_leading_whitespace = true;
        }
        if not_leading_whitespace {
            ret.push(c)
        }
    }
    // Removing trailing white-space
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

/// This function runs the string split function on all elements from a vector.
fn split_strings_in_vector(v: &Vec<String>, delimiter: &str) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = Vec::new();
    for i in v {
        ret.push(split_to_vec(i, delimiter))
    }
    return ret;
}

/// Runs remove_leading_trailing_white-space on all the elements of a 2D vector
/// of strings. As the remove_leadinf_trailing_whitespace functions already
/// works on copies of the string, it makes sense that this function works on
/// a copy of the vector.
#[allow(non_snake_case)]
fn remove_leading_trailing_whitespace_2D(v: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = Vec::new();
    for line in v {
        let mut new_elem: Vec<String> = Vec::new();
        for s in line {
            new_elem.push(remove_leading_trailing_whitespace(s));
        }
        ret.push(new_elem);
    }
    return ret;
}

/// Returns a copy of the input vector but where all lines have the same
/// numbers of elements.
fn equilize_str_vec(v: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut max_len = 0;
    for line in v {
        if line.len() > max_len {
            max_len = line.len()
        }
    }
    let mut ret: Vec<Vec<String>> = Vec::new();
    for line in v {
        let mut new_line = line.clone();
        while new_line.len() < max_len {
            new_line.push(String::new());
        }
        ret.push(new_line);
    }
    return ret;
}

// TODO: remove trailing empty columns...

/// From the lines extracted from a csv file, returns a clean 2D vector of each
/// cases of the table.
fn clean_csv(csv_lines: &Vec<String>) -> Vec<Vec<String>> {
    let base_table = split_strings_in_vector(&csv_lines, ",");
    let pruned_table = remove_leading_trailing_whitespace_2D(&base_table);
    let balanced_table = equilize_str_vec(&pruned_table);
    return balanced_table;
}

