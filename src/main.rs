
mod read_csv;
//use crate::read_csv::open_file;
use read_csv::open_file;

fn main() {
    open_file("test.csv".to_string());
    println!("Bobignou!");
}
