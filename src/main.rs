
mod read_csv;
use crate::read_csv::read_csv_file;

mod write_md;
use crate::write_md::write_md_file;

fn main() {
    let csv_table = read_csv_file(&"test.csv".to_string());
    write_md_file(&"test.md".to_string(), &csv_table);
    println!("Bobignou!");
}
