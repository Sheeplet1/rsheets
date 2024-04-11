mod commands;
pub mod spreadsheet;
pub mod utils;

use rsheet_lib::connect::{Manager, Reader, Writer};
use rsheet_lib::replies::Reply;
use spreadsheet::Spreadsheet;

use std::sync::Arc;
use std::thread;

pub fn start_server<M>(mut manager: M)
where
    M: Manager,
{
    let spreadsheet = spreadsheet::new_shared_spreadsheet();
    let mut handlers = Vec::new();

    while let Ok((mut recv, mut send)) = manager.accept_new_connection() {
        let spreadsheet = spreadsheet.clone();
        let handler = thread::spawn(move || {
            handle_connection(spreadsheet, &mut recv, &mut send);
        });
        handlers.push(handler);
    }

    for handler in handlers {
        // TODO: NO unwrap
        handler.join().unwrap();
    }
}

fn handle_connection<R, W>(spreadsheet: Arc<Spreadsheet>, reader: &mut R, writer: &mut W)
where
    R: Reader,
    W: Writer,
{
    loop {
        let msg = reader.read_message();
        match msg {
            Ok(msg) => {
                let args: Vec<&str> = msg.split_whitespace().collect();
                if args.is_empty() {
                    continue;
                }

                let command = args[0];
                match command {
                    "get" => match commands::get::get(&spreadsheet, args) {
                        Ok((cell, cell_val)) => {
                            writer.write_message(Reply::Value(cell, cell_val)).unwrap();
                        }
                        Err((_cell, e)) => {
                            if let Reply::Error(ref s) = e {
                                if s.is_empty() {
                                    continue;
                                }
                                writer.write_message(e).unwrap();
                            }
                        }
                    },
                    "set" => match commands::set::set(&spreadsheet, args) {
                        Ok(_) => {}
                        Err(e) => {
                            if let Reply::Error(ref s) = e {
                                // TODO: Add reason for skipping empty error
                                // messages.
                                if s.is_empty() {
                                    continue;
                                }
                            }
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
                return;
            }
        }
    }
}
