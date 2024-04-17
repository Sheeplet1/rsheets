mod commands;
pub mod spreadsheet;
pub mod utils;

use rayon::ThreadPoolBuilder;
use rsheet_lib::connect::{Manager, Reader, Writer};
use rsheet_lib::replies::Reply;
use spreadsheet::Spreadsheet;

use std::collections::VecDeque;
use std::sync::Arc;
use std::{thread, time};
// use std::thread;

pub fn start_server<M>(mut manager: M)
where
    M: Manager + Send + 'static,
{
    let spreadsheet = spreadsheet::new_shared_spreadsheet();
    // let update_queue = VecDeque::new();
    //
    // let worker = thread::spawn({
    //     let spreadsheet = spreadsheet.clone();
    //     let update_queue = update_queue.clone();
    //     move || {
    //         while let Some(task) = update_queue.pop_front() {
    //             // TODO: Update dependencies based on the command popped from
    //             // the queue
    //         }
    //     }
    // });

    // BUG: When letting Rayon manage the threads, the program context switches
    // and causes autotest failures. Increasing the number of threads does not
    // fix the core issue.
    let pool = match ThreadPoolBuilder::new().num_threads(8).build() {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error creating thread pool: {}", e);
            return;
        }
    };

    // // Using `scope` to ensure that all threads complete their work before
    // // the program exists.
    pool.scope(|s| {
        while let Ok((mut recv, mut send)) = manager.accept_new_connection() {
            let spreadsheet = spreadsheet.clone();
            s.spawn(move |_| {
                handle_connection(&spreadsheet, &mut recv, &mut send);
            })
        }
    })
}

fn handle_connection<R, W>(spreadsheet: &Arc<Spreadsheet>, reader: &mut R, writer: &mut W)
where
    R: Reader,
    W: Writer,
{
    loop {
        let msg = reader.read_message();

        // Append timestamp to the front of the message
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        match msg {
            Ok(msg) => {
                let args: Vec<&str> = msg.split_whitespace().collect();
                if args.is_empty() {
                    continue;
                }

                // A valid message will always have the command as the first
                // argument.
                let command = args[0];
                match command {
                    "get" => match commands::get::get(spreadsheet, args) {
                        Ok((cell, cell_val)) => {
                            writer.write_message(Reply::Value(cell, cell_val)).unwrap();
                        }
                        Err((_cell, e)) => {
                            writer.write_message(e).unwrap();
                        }
                    },
                    "set" => match commands::set::set(spreadsheet, args.clone(), timestamp) {
                        Ok(_) => {}
                        Err(e) => {
                            writer.write_message(e).unwrap();
                        }
                    },
                    _ => {
                        writer
                            .write_message(Reply::Error(format!("Invalid command: {}", command)))
                            .unwrap();
                    }
                }
            }
            Err(_) => {
                // If we get an error reading the message, we assume the client
                // has disconnected.
                return;
            }
        }
    }
}
