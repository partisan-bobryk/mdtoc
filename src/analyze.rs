use crate::AppState;
use crossbeam::channel::Sender;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    sync::{Arc, Mutex},
};

pub fn analyze_loop(
    app_state: Arc<Mutex<AppState>>,
    file_buffer: BufReader<File>,
    transform_tx: Sender<(String, i32)>,
) -> Result<()> {
    let mut analyzed_lines: Vec<String> = vec![];
    let mut line_index: i32 = -1;

    for line in file_buffer.lines() {
        line_index += 1;

        if let Ok(line) = line {
            let mut locked_app_state = app_state.lock().unwrap();

            if line.contains(locked_app_state.start_replace_token)
                && locked_app_state.start_tag_index == -1
            {
                locked_app_state.start_tag_index = line_index;
            }

            if line.contains(locked_app_state.end_replace_token) {
                locked_app_state.end_tag_index = line_index;
            }

            analyzed_lines.push(line);
        }
    }

    // Before we can start filtering out the previous table of contents, we need to go through all the lines
    // otherwise we can be wrong about assumptions.
    line_index = -1;
    for line in analyzed_lines {
        line_index += 1;

        // Send line string and the index to the transform function.
        transform_tx.send((line, line_index.to_owned())).unwrap();
    }

    // Done reading the buffer so let's notify the transform thread that they shouldn't expect
    // any more data.
    transform_tx.send(("".to_owned(), -1)).unwrap();

    Ok(())
}
