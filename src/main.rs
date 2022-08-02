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

    let mut headings: Vec<(String, usize)> = vec![];
    let heading_regex = Regex::new(r"(?P<hash>#{2,})\s(?P<heading>.*)").unwrap();
    for line in lines_buffer {
        match line {
            Err(err) => eprintln!("{}", err),
            Ok(line) => {
                if let Some((heading_title, heading_tier)) =
                    extract_heading_from_line(&heading_regex, &line)
                {
                    headings.push((heading_title, heading_tier));
                }
            }
        }
    }

    // Get formatted table of contents
    let toc_string = generate_table_of_contents(headings);

    // Prepare a temp file where we can control in which order content gets inserted
    let file_contents = OpenOptions::new()
        .read(true)
        .open(&args.inbound_source)
        .unwrap();
    let mut file_buffer = BufReader::new(file_contents);

    // Start writing table of contents to the top of the file
    temp_file.write_all(toc_string.as_bytes()).unwrap();

    // Use the file contents from the original document and append it after table of contents
    copy(&mut file_buffer, &mut temp_file).unwrap();

    // Final stage to remove the original and rename the temp file to the original.
    remove_file(&args.inbound_source).unwrap();
    rename(temp_file_name, args.inbound_source).unwrap();
}

fn extract_heading_from_line(regex_pattern: &Regex, line: &String) -> Option<(String, usize)> {
    if let Some(cap) = regex_pattern.captures(&line) {
        if cap.name("heading").is_none() || cap.name("hash").is_none() {
            return None;
        }

        let heading = cap.name("heading").unwrap();
        let hash = cap.name("hash").unwrap();
        return Some((
            heading.as_str().to_string(),
            hash.as_str().chars().count() - 1,
        ));
    }

    None
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
