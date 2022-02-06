
use crate::easy_io::split_to_vec;

/// The main function of this module; from a list of lines from a csv file,
/// returns a 2D vector representing the table.
pub fn read_csv_lines(raw_csv: &Vec<String>, delimiter: &str) -> Vec<Vec<String>> {
    return clean_csv(raw_csv, delimiter);
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

/// From a list of lines, removes all the empty trailing lines.
fn remove_trailing_empty_lines(s: &Vec<String>) -> Vec<String> {
    let mut ret: Vec<String> = s.clone();
    while ret.len() >= 1 && ret[ret.len()-1] == "" {
        ret.pop();
    }
    return ret;
}

/// Count the number of trailing empty strings in a vector of those.
fn count_empty_str(v: &Vec<String>) -> usize {
    for i in 0..v.len() {
        let used_index = v.len() - i - 1;
        if v[used_index] != "" {
            return i;
        }
    }
    return v.len();
}

/// Finds the number of empty columns in a 2D vector of strings.
fn count_empty_columns(v: &Vec<Vec<String>>) -> usize {
    if v.len() == 0 {
        return 0;
    }
    let mut min_empty_columns = v[0].len();
    for line in v {
        if count_empty_str(line) < min_empty_columns {
            min_empty_columns = count_empty_str(line);
        }
    }
    return min_empty_columns;
}

/// Removes empty trailing columns from a 2D vector of strings.
/// Unlike the other functions, this one is not pure.
fn remove_trailing_empty_columns(v: &mut Vec<Vec<String>>) {
    let min_empty_columns = count_empty_columns(v);
    for i in 0..v.len() {
        for _j in 0..min_empty_columns {
            v[i].pop();
        }
    }
}

/// From the lines extracted from a csv file, returns a clean 2D vector of each
/// cases of the table.
fn clean_csv(csv_lines: &Vec<String>, delimiter: &str) -> Vec<Vec<String>> {
    let clened_lines = remove_trailing_empty_lines(&csv_lines);
    let base_table = split_strings_in_vector(&clened_lines, delimiter);
    let pruned_table = remove_leading_trailing_whitespace_2D(&base_table);
    let mut balanced_table = equilize_str_vec(&pruned_table);
    remove_trailing_empty_columns(&mut balanced_table);
    if balanced_table.len() > 0 {
        if balanced_table[0].len() == 0 {
            return Vec::new();
        }
    }
    return balanced_table;
}

