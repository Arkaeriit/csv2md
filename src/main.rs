
mod read_csv;
use crate::read_csv::read_csv_file;

mod write_md;
use crate::write_md::write_md_file;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let csv_table = read_csv_file(&args.input_file, &parse_delimiter(&args.delimiter));
    if csv_table.len() >= 1 {
        write_md_file(&args.output_file, &csv_table);
    } else {
        println!("Error, input table is empty. Not writing to the output file.");
        std::process::exit(1);
    }
}

fn parse_delimiter(s: &str) -> String {
    if s == "tabs" {
        return "\t".to_string();
    }
    return s.to_string();
}

/// A tool to convert CSV files as markdown tables.
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// CSV file used as input.
    #[clap(short, long)]
    input_file: String,

    /// Markdown file where the table will be written.
    #[clap(short, long, default_value = "/dev/stdout")]
    output_file: String,

    /// Delimiter used between the cases of the CSV table.
    /// To use tabs as the delimiter, you can write "tabs".
    #[clap(short, long, default_value = ",")]
    delimiter: String,
}

