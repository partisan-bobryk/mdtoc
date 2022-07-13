use clap::Parser;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, Write},
    os::unix::prelude::FileExt,
    path::Path,
    process, vec,
};

#[derive(Parser)]
#[clap(name = "mdtoc")]
#[clap(author = "Maksym Y. <maks@revent.studio>")]
#[clap(version = "0.1.0")]
#[clap(about = "Generate table of contents in a markdown file")]
struct Cli {
    #[clap(long, value_parser)]
    inbound_source: String,
}

fn main() {
    let ref debug: bool = true;
    let args = Cli::parse();

    let lines = read_lines(&args.inbound_source).unwrap_or_else(|err| {
        eprintln!("Issue reading {}", args.inbound_source);
        if debug == &true {
            eprintln!("{}", err);
        }
        process::exit(1);
    });

    let heading_regex = Regex::new(r"(?P<hash>#{2,})\s(?P<heading>.*)").unwrap();
    let mut headings: Vec<(String, usize)> = vec![];
    for line in lines {
        match line {
            Err(err) => eprintln!("{}", err),
            Ok(line) => match heading_regex.captures(&line) {
                Some(cap) => {
                    if cap.name("heading").is_none() || cap.name("hash").is_none() {
                        continue;
                    }

                    let heading = cap.name("heading").unwrap();
                    let hash = cap.name("hash").unwrap();
                    headings.push((
                        heading.as_str().to_string(),
                        hash.as_str().chars().count() - 1,
                    ));
                }
                None => (),
            },
        }
    }

    dbg!(&headings);

    let toc_string = generate_table_of_contents(headings);

    let mut output_file = File::create("./sample_edited.md").unwrap();
    output_file.write_all(toc_string.as_bytes()).unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn generate_table_of_contents(headings: Vec<(String, usize)>) -> String {
    let mut table_of_contents = String::from("## Table of Contents\n");

    for header in headings {
        dbg!(&header);
        let mut tabs = String::new();
        for _ in 0..header.1 {
            tabs.push_str("  ");
        }
        let formatted_line = format!("{} - {}\n", tabs, generate_md_link(header.0));
        table_of_contents.push_str(&formatted_line);
    }

    table_of_contents
}

fn generate_md_link(link_text: String) -> String {
    let link = link_text.replace(" ", "-").to_lowercase();
    format!("[{}](#{})", link_text, link)
}
