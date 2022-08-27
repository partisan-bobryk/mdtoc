use clap::Parser;
use mdtoc::{args::Cli, table_of_contents::TableOfContentsHelper};

fn main() {
    let args = Cli::parse();

    // Instantiate the generic helper
    let mut toc_helper = TableOfContentsHelper::new(&args.input_file);
    // Start building the file with the table of contents
    toc_helper.build()
}
