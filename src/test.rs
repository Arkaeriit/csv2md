
use crate::read_csv::read_csv_lines;
use crate::write_md::gen_md_table;
use crate::easy_io::split_to_vec;

#[test]
pub fn basic_table() {
    let basic_table = "1, 2\n3, 4";
    let ref_table = "|1|2|\n|-|-|\n|3|4|\n";

    let table_lines = split_to_vec(&basic_table, "\n");
    let vectorized_table = read_csv_lines(&table_lines, ",");
    let generated_table = gen_md_table(&vectorized_table);
    if generated_table != ref_table {
        println!("Expected:\n{}\nGot:{}\n", ref_table, generated_table);
        assert_eq!(ref_table, generated_table);
    }
}

