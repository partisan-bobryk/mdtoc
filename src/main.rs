use clap::Parser;
use mdtoc::{
    args::Cli,
    table_of_contents::{generate_table_of_contents, process_file_lines, TableOfContentsHelper},
};

use std::io::{BufRead, BufReader};

fn main() {
    let args = Cli::parse();

    // Instantiate the generic helper
    let mut toc_helper = TableOfContentsHelper::new(&args.inbound_source);
    let lines_buffer = BufReader::new(&toc_helper.original_file).lines();

    // Process the buffer and extract the headings
    let headings = process_file_lines(lines_buffer);

    // Get formatted table of contents
    let toc_string = generate_table_of_contents(headings);

    // Start building the file with the table of contents
    toc_helper.build(toc_string)
}
