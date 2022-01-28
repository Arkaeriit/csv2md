
pub fn write_md_file(_path: &String, _csv: &Vec<Vec<String>>) {
}

/// From a vector of string such as `["abc", "def ", "ghi"]`, makes a line of
/// a markdown table such as `|abc|def |ghi|`.
fn vec_to_md_line(v: &Vec<String>) -> String {
    let mut ret = "|".to_string();
    for elem in v {
        ret = ret + elem;
        ret.push('|');
    }
    return ret;
}

/// From a vector of string such as `["abc", "def ", "ghi"]`, makes a line of
/// a markdown table guard such as `|---|----|---|`.
fn vec_to_md_guard(v: &Vec<String>) -> String {
    let mut ret = "|".to_string();
    for elem in v {
        for _i in 1..elem.len() {
            ret.push('-');
        }
        ret.push('|');
    }
    return ret;
}

/// From a square 2D array of string, generates a similar vector but where the
/// lines and the columns have been inverted.
fn flip_vector(v: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = Vec::new();
    for i in 1..v[0].len() {
        let mut column: Vec<String> = Vec::new();
        for j in 1..v.len() {
            column.push(v[j][i].clone());
        }
        ret.push(column);
    }
    return ret;
}

/// From a vector of strings, returns a similar vector where all elements have
/// the same size. To make the elements the same size, trailing chars are
/// added to the elements that need it.
fn equalize_line(v: &Vec<String>, trail: char) -> Vec<String> {
    let mut max_len = 0;
    for elem in v {
        if elem.len() > max_len {
            max_len = elem.len();
        }
    }
    let mut ret: Vec<String> = Vec::new();
    for elem in v {
        let mut new_elem = elem.clone();
        while new_elem.len() < max_len {
            new_elem.push(trail);
        }
        ret.push(new_elem);
    }
    return ret;
}

