use crate::AppState;
use crossbeam::channel::{Receiver, Sender};
use std::cmp;
use std::io::Result;
use std::sync::{Arc, Mutex};

pub fn transform_loop(
    app_state: Arc<Mutex<AppState>>,
    transform_rx: Receiver<(String, i32)>,
    write_tx: Sender<(String, bool)>,
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
            write_tx.send((line, false)).unwrap();
        }
    }

    // Done transforming data so let's notify the write thread that they
    // won't be receiving any more data.
    write_tx.send(("".to_owned(), true)).unwrap();
    Ok(())
}
