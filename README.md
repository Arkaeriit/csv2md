# csv2md

A too to convert CSV tables such as this:

```
header 1, header 2, header 3
data 1, data 2, data 3
1, 2, 3
```

into markdown tables such as this:

```
|header 1|header 2|header 3|
|--------|--------|--------|
|data 1  |data 2  |data 3  |
|1       |2       |3       |
```

The table can then be rendered like this:

|header 1|header 2|header 3|
|--------|--------|--------|
|data 1  |data 2  |data 3  |
|1       |2       |3       |

## Usage

```
csv2md 
A tool to convert CSV files as markdown tables

USAGE:
    csv2md [OPTIONS] --input-file <INPUT_FILE>

OPTIONS:
    -d, --delimiter <DELIMITER>        Delimiter used between the cases of the CSV table. To use
                                       tabs as the delimiter, you can write "tabs" [default: ,]
    -h, --help                         Print help information
    -i, --input-file <INPUT_FILE>      CSV file used as input
    -o, --output-file <OUTPUT_FILE>    Markdown file where the table will be written [default:
                                       /dev/stdout]
```

## Disclaimer

There is a lot of similar tools online which do the same thing as this one. Some of them even have the same name, `csv2md`. This one does not have any relation with any of the other tools and I don't pretend that it is any better. I just wanted to do a quick project to try and lean Rust.

