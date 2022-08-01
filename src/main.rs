use clap::Parser;
use regex::Regex;
use std::{
    fs::{remove_file, rename, OpenOptions},
    io::{copy, BufRead, BufReader, Write},
    vec,
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
    let args = Cli::parse();

    // TODO: Move this to its own function
    let file = OpenOptions::new()
        .read(true)
        .open(&args.inbound_source)
        .unwrap();

    let temp_file_name = format!("{}_temp.md", args.inbound_source);
    let mut temp_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&temp_file_name)
        .unwrap();

    let lines_buffer = BufReader::new(&file).lines();

    let heading_regex = Regex::new(r"(?P<hash>#{2,})\s(?P<heading>.*)").unwrap();
    let mut headings: Vec<(String, usize)> = vec![];
    for line in lines_buffer {
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

    let file_contents = OpenOptions::new()
        .read(true)
        .open(&args.inbound_source)
        .unwrap();

    let mut file_buffer = BufReader::new(file_contents);

    let toc_string = generate_table_of_contents(headings);
    temp_file.write(toc_string.as_bytes()).unwrap();
    copy(&mut file_buffer, &mut temp_file).unwrap();
    temp_file.flush().unwrap();
    remove_file(&args.inbound_source).unwrap();
    rename(temp_file_name, args.inbound_source).unwrap();
}

fn generate_table_of_contents(headings: Vec<(String, usize)>) -> String {
    let mut table_of_contents = String::from("\n## Table of Contents\n");

    for header in headings {
        let mut tabs = String::new();
        for _ in 0..header.1 {
            tabs.push_str("  ");
        }
        let formatted_line = format!("{} - {}\n", tabs, generate_md_link(header.0));
        table_of_contents.push_str(&formatted_line);
    }

    table_of_contents.push_str("\n");
    table_of_contents
}

fn generate_md_link(link_text: String) -> String {
    let link = link_text.replace(" ", "-").to_lowercase();
    format!("[{}](#{})", link_text, link)
}
