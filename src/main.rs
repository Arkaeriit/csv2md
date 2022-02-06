
#[cfg(test)]
mod test;

mod read_csv;
use crate::read_csv::read_csv_lines;

mod write_md;
use crate::write_md::gen_md_table;

mod easy_io;
use crate::easy_io::open_file_as_lines;
use crate::easy_io::write_to_path;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let raw_csv = open_file_as_lines(&args.input_file);
    let csv_table = read_csv_lines(&raw_csv, &parse_delimiter(&args.delimiter));
    if csv_table.len() >= 1 {
        let md = gen_md_table(&csv_table);
        write_to_path(&args.output_file, &md);
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

