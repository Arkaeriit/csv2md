
use crate::read_csv::read_csv_lines;
use crate::write_md::gen_md_table;
use crate::easy_io::split_to_vec;

#[test]
fn basic_table() {
    let basic_table = "1, 2\n3, 4";
    let ref_table = "|1|2|\n|-|-|\n|3|4|\n";
    cmp_csv_md(basic_table, ref_table);
}

#[test]
fn testing_trailing_lines() {
    let basic_table = "1, 2\n3, 4\n\n\n";
    let ref_table = "|1|2|\n|-|-|\n|3|4|\n";
    cmp_csv_md(basic_table, ref_table);
}

#[test]
fn testing_trailing_columns() {
    let basic_table = "1, 2, , , , , \n3, 4, , , , ";
    let ref_table = "|1|2|\n|-|-|\n|3|4|\n";
    cmp_csv_md(basic_table, ref_table);
}

#[test]
fn testing_trailing_columns_and_lines() {
    let basic_table = "1, 2, , , , , \n3, 4, , , , \n5, 6\n\n";
    let ref_table = "|1|2|\n|-|-|\n|3|4|\n|5|6|\n";
    cmp_csv_md(basic_table, ref_table);
}

#[test]
fn testing_not_csv_input() {
    let not_csv = "Bobignou";
    let is_that_even_a_table = "|Bobignou|\n|--------|\n";
    cmp_csv_md(not_csv, is_that_even_a_table);
}

#[test]
fn testing_empty_input() {
    let no = "";
    cmp_csv_md(no, no);
}

#[test]
fn testing_empty_line() {
    let lines = "\n\n\n";
    let no = "";
    cmp_csv_md(lines, no);
}

#[test]
fn testing_empty_table() {
    let table = ", , , \n, ,  \n  ,  ,  ,\n";
    let no = "";
    cmp_csv_md(table, no);
}

#[test]
fn testing_empty_line_in_normal_table() {
    let csv = "a, b, , \n\n  c,  d,  ,\n";
    let md = "|a|b|\n|-|-|\n| | |\n|c|d|\n";
    cmp_csv_md(csv, md);
}

#[test]
fn testing_unicode() {
    let csv = "Ω, ω, , \n  é,  à,  ,\n";
    let md = "|Ω|ω|\n|-|-|\n|é|à|\n";
    cmp_csv_md(csv, md);
}

/// This functions tries to compare a CSV string and a md table.
fn cmp_csv_md(csv: &str, md: &str) {
    let table_lines = split_to_vec(&csv, "\n");
    let vectorized_table = read_csv_lines(&table_lines, ",");
    let generated_table = gen_md_table(&vectorized_table);
    if generated_table != md {
        println!("Expected:\n{}\nGot:{}\n", md, generated_table);
        assert_eq!(md, generated_table);
    }
}

