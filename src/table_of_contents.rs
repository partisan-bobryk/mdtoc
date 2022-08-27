use std::{
    cmp,
    fs::{remove_file, rename, File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    vec,
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

    pub fn build(&mut self) {
        // Prepare a temp file where we can control in which order content gets inserted
        self.original_file.seek(SeekFrom::Start(0)).unwrap();

        let ref mut file_buffer = BufReader::new(&self.original_file);
        let mut file_contents: Vec<String> = vec![];

        let start_replace_token = "<!-- [mdtoc:start] -->";
        let end_replace_token = "<!-- [mdtoc:end] -->";

        // contains_start_tag will serve as place to insert toc
        let mut start_tag_index: i32 = -1;
        // contains_end_tag will indicate that operation should clear out existing content between the tags and replace with toc
        let mut end_tag_index: i32 = -1;
        let mut line_index: i32 = -1;

        /*
         * Analysis Loop
         *
         * We use a loop to confirm our assumptions and to filter out previous table of contents.
         */
        for line in file_buffer.lines() {
            line_index += 1;

            if let Ok(line) = line {
                if line.contains(start_replace_token) && start_tag_index == -1 {
                    start_tag_index = line_index;
                }

                if line.contains(end_replace_token) {
                    end_tag_index = line_index;
                }

                file_contents.push(line);
            }
        }

        /*
         * Transformation Loop
         *
         * We clean up the document by removing previous table of contents artifacts
         */
        let parsed_file: Vec<String> = file_contents
            .into_iter()
            .enumerate()
            .filter(|(i, _l)| {
                let idx: i32 = *i as i32;
                let is_in_toc_area: bool =
                    start_tag_index < idx && end_tag_index >= cmp::max(start_tag_index, idx);

                return !is_in_toc_area;
            })
            .map(|(_i, l)| l)
            .collect::<Vec<String>>();

        /*
         * Generate Terms of Contents from cleaned up file
         */
        // Process the buffer and extract the headings
        let headings = process_file_lines(parsed_file.to_owned());

        // Get formatted table of contents
        let toc_string = generate_table_of_contents(headings);
        let formatted_toc = format!(
            "{}\n{}\n{}",
            start_replace_token, toc_string, end_replace_token
        );

        /*
         * Write Loop
         */
        line_index = -1;
        if start_tag_index == -1 {
            self.temp_file.write(formatted_toc.as_bytes()).unwrap();
            self.temp_file.write("\n".as_bytes()).unwrap();
        }

        for line in parsed_file {
            line_index += 1;
            let mut modified_line = line;
            // Replace tag with table of contents)
            if line_index == start_tag_index {
                modified_line = formatted_toc.to_owned();
            }

            modified_line.push_str("\n");
            self.temp_file.write(modified_line.as_bytes()).unwrap();
        }

        // Close down the buffer as we are done writing to it
        self.temp_file.flush().unwrap();

        // Final stage to remove the original and rename the temp file to the original.
        remove_file(&self.original_file_name).unwrap();
        rename(&self.temp_file_name, &self.original_file_name).unwrap();
    }
}

pub fn process_file_lines(ref document_lines: Vec<String>) -> Vec<(String, usize)> {
    let mut headings: Vec<(String, usize)> = vec![];
    let heading_regex = Regex::new(r"(?P<hash>#{2,})\s(?P<heading>.*)").unwrap();

    for line in document_lines {
        if let Some((heading_title, heading_tier)) =
            extract_heading_from_line(&heading_regex, &line)
        {
            headings.push((heading_title, heading_tier));
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
