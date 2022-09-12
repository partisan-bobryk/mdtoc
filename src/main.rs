use clap::Parser;
use crossbeam::channel::unbounded;
use mdtoc::{
    analyze::analyze_loop, args::Cli, table_of_contents::TableOfContentsHelper,
    transform::transform_loop, write::write_loop, AppState,
};
use std::{
    fs::{remove_file, rename},
    io::BufReader,
    io::{Seek, SeekFrom},
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let args = Cli::parse();

    let mut toc_helper = TableOfContentsHelper::new(&args.input_file);
    let thread_safe_apps_state = Arc::new(Mutex::new(AppState {
        start_replace_token: "<!-- [mdtoc:start] -->",
        end_replace_token: "<!-- [mdtoc:end] -->",
        start_tag_index: -1,
        end_tag_index: -1,
    }));

    // Wind back the file incase the cursor was placed other than the beginning.
    toc_helper.original_file.seek(SeekFrom::Start(0)).unwrap();
    let file_buffer = BufReader::new(toc_helper.original_file);

    // Instantiate channels of communication between the threads.
    let (transform_tx, transform_rx) = unbounded();
    let (write_tx, write_rx) = unbounded();

    // Prepare sharing of state between different threads
    let analyze_app_state = Arc::clone(&thread_safe_apps_state);
    let transform_app_state = Arc::clone(&thread_safe_apps_state);
    let write_app_state = Arc::clone(&thread_safe_apps_state);

    // Start threads
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
}
