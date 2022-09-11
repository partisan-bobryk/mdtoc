use crate::{
    table_of_contents::{generate_table_of_contents, process_file_lines},
    AppState,
};
use crossbeam::channel::Receiver;
use std::{
    fs::File,
    io::{Result, Write},
    sync::{Arc, Mutex},
};

pub fn write_loop(
    app_state: Arc<Mutex<AppState>>,
    write_source: &mut File,
    write_rx: Receiver<(String, bool)>,
) -> Result<()> {
    // Just collect the transformed values before we write it.
    // Use the collected data to generate a table of contents
    // ugh it still sucks.
    let mut filtered_lines: Vec<String> = vec![];
    loop {
        let buffer = write_rx.recv().unwrap();

        if buffer.0.is_empty() && buffer.1 {
            break;
        }

        filtered_lines.push(buffer.0);
    }

    let locked_app_state = app_state.lock().unwrap();

    // Generate table of contents
    // Process the buffer and extract the headings
    let headings = process_file_lines(filtered_lines.to_owned());

    // Get formatted table of contents

    let toc_string = generate_table_of_contents(headings);
    let formatted_toc = format!(
        "{}\n{}\n{}",
        locked_app_state.start_replace_token, toc_string, locked_app_state.end_replace_token
    );
    // Start pouring in the table of contents
    if locked_app_state.start_tag_index == -1 {
        write_source.write_all(formatted_toc.as_bytes()).unwrap();
        write_source.write_all(b"\n").unwrap();
    }

    // Write to file
    let mut line_index = -1;
    for line in filtered_lines {
        line_index += 1;
        let mut modified_line = line;
        // Replace tag with table of contents)
        if line_index == locked_app_state.start_tag_index {
            modified_line = formatted_toc.to_owned();
        }

        modified_line.push('\n');
        write_source.write_all(modified_line.as_bytes()).unwrap();
    }

    Ok(())
}
