use std::{
    fs::{remove_file, rename, File, OpenOptions},
    io::{copy, BufReader, Lines, Seek, SeekFrom, Write},
};

use regex::Regex;

pub struct TableOfContentsHelper {
    pub original_file: File,
    pub original_file_name: String,
    pub temp_file: File,
    pub temp_file_name: String,
}

impl TableOfContentsHelper {
    pub fn new(filename: &String) -> Self {
        let original_file: File = OpenOptions::new().read(true).open(filename).unwrap();
        let temp_file_name = format!("{}_temp.md", filename);
        let temp_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&temp_file_name)
            .unwrap();

        Self {
            original_file,
            temp_file,
            temp_file_name,
            original_file_name: filename.to_string(),
        }
    }

    pub fn build(&mut self, table_of_contents: String) {
        // Prepare a temp file where we can control in which order content gets inserted
        self.original_file.seek(SeekFrom::Start(0)).unwrap();
        let mut file_buffer = BufReader::new(&self.original_file);

        // Start writing table of contents to the top of the file
        self.temp_file
            .write_all(table_of_contents.as_bytes())
            .unwrap();

        // Use the file contents from the original document and append it after table of contents
        copy(&mut file_buffer, &mut self.temp_file).unwrap();

        // Final stage to remove the original and rename the temp file to the original.
        remove_file(&self.original_file_name).unwrap();
        rename(&self.temp_file_name, &self.original_file_name).unwrap();
    }
}

pub fn process_file_lines(lines_buffer: Lines<BufReader<&File>>) -> Vec<(String, usize)> {
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

    headings
}

pub fn extract_heading_from_line(regex_pattern: &Regex, line: &String) -> Option<(String, usize)> {
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

pub fn generate_table_of_contents(headings: Vec<(String, usize)>) -> String {
    let mut table_of_contents = String::from("## Table of Contents\n");

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
