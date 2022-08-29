use std::{
    cmp,
    fs::{remove_file, rename, File},
    io::BufReader,
    io::{BufRead, BufWriter, Result, Write},
    sync::{Arc, Mutex},
    thread,
};

use clap::Parser;
use crossbeam::channel::{unbounded, Receiver, Sender};
use mdtoc::{args::Cli, table_of_contents::TableOfContentsHelper};

fn main() {
    let args = Cli::parse();

    let mut toc_helper = TableOfContentsHelper::new(&args.input_file);
    let thread_safe_apps_state = Arc::new(Mutex::new(AppState {
        start_replace_token: "<!-- [mdtoc:start] -->",
        end_replace_token: "<!-- [mdtoc:end] -->",
        start_tag_index: -1,
        end_tag_index: -1,
    }));

    let (transform_tx, transform_rx) = unbounded();
    let (write_tx, write_rx) = unbounded();

    let file_buffer = BufReader::new(toc_helper.original_file);

    let analyze_app_state = Arc::clone(&thread_safe_apps_state);
    let transform_app_state = Arc::clone(&thread_safe_apps_state);
    let write_app_state = Arc::clone(&thread_safe_apps_state);

    let analyze_handle =
        thread::spawn(move || analyze_loop(analyze_app_state, file_buffer, transform_tx));
    let transform_handle =
        thread::spawn(move || transform_loop(transform_app_state, transform_rx, write_tx));
    let write_handle =
        thread::spawn(move || write_loop(write_app_state, &mut toc_helper.temp_file, write_rx));

    // Crash if any threads have crashed
    // Otherwise wait for the threads to finish
    let _analyze_result = analyze_handle.join().unwrap();
    let _transform_result = transform_handle.join().unwrap();
    let _write_handle = write_handle.join().unwrap();

    // Final stage to remove the original and rename the temp file to the original.
    remove_file(&toc_helper.original_file_name).unwrap();
    rename(toc_helper.temp_file_name, toc_helper.original_file_name).unwrap();

    // TODO: Retire this code
    // Instantiate the generic helper
    // Start building the file with the table of contents
    // toc_helper.build()
}

struct AppState<'a> {
    start_replace_token: &'a str,
    end_replace_token: &'a str,
    start_tag_index: i32,
    end_tag_index: i32,
}

fn analyze_loop(
    app_state: Arc<Mutex<AppState>>,
    file_buffer: BufReader<File>,
    transform_tx: Sender<(String, i32)>,
) -> Result<()> {
    let analyzed_lines: Vec<String> = vec![];
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

fn transform_loop(
    app_state: Arc<Mutex<AppState>>,
    transform_rx: Receiver<(String, i32)>,
    write_tx: Sender<(String, i32)>,
) -> Result<()> {
    loop {
        let buffer = transform_rx.recv().unwrap();

        if buffer.0.is_empty() && buffer.1.eq(&-1i32) {
            break;
        }

        let line = buffer.0;
        let idx = buffer.1;
        let locked_app_state = app_state.lock().unwrap();

        let is_in_toc_area: bool = locked_app_state.start_tag_index < idx
            && locked_app_state.end_tag_index >= cmp::max(locked_app_state.start_tag_index, idx);

        if !is_in_toc_area {
            write_tx.send((line, idx)).unwrap();
        }
    }

    // Done transforming data so let's notify the write thread that they
    // won't be receiving any more data.
    write_tx.send(("".to_owned(), -1i32)).unwrap();
    Ok(())
}

fn write_loop(
    app_state: Arc<Mutex<AppState>>,
    write_source: &mut File,
    write_rx: Receiver<(String, i32)>,
) -> Result<()> {
    loop {
        let buffer = write_rx.recv().unwrap();

        if buffer.0.is_empty() && buffer.1.eq(&-1i32) {
            break;
        }

        // TODO: Figure out how to generate table of contents on the fly

        let line = buffer.0;
        let idx = buffer.1;
        let locked_app_state = app_state.lock().unwrap();

        let mut modified_line = line;
        // Replace tag with table of contents)
        if idx == locked_app_state.start_tag_index {
            // TODO: inject Table of contents
            // modified_line = formatted_toc.to_owned();
        }

        modified_line.push_str("\n");
        write_source.write(modified_line.as_bytes()).unwrap();
    }

    write_source.flush().unwrap();
    Ok(())
}
