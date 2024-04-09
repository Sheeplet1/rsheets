mod commands;
mod dependencies;
pub mod spreadsheet;
pub mod utils;

use rsheet_lib::connect::{Manager, Reader, Writer};
use rsheet_lib::replies::Reply;
use spreadsheet::Spreadsheet;

use std::error::Error;
use std::sync::Arc;
use std::thread;

// use log::info;

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let spreadsheet = spreadsheet::new_shared_spreadsheet();

    loop {
        if let Ok((mut recv, mut send)) = manager.accept_new_connection() {
            log::info!("Accepted new connection");
            let spreadsheet = spreadsheet.clone();
            thread::spawn(move || {
                handle_connection(spreadsheet, &mut recv, &mut send);
            });
        } else {
            eprintln!("Failed to accept new connection");
            continue;
        }
    }
}

fn handle_connection<R, W>(spreadsheet: Arc<Spreadsheet>, reader: &mut R, writer: &mut W)
where
    R: Reader,
    W: Writer,
{
    loop {
        let msg = reader.read_message();
        log::info!("Received message");
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
                        Err(e) => {
                            writer.write_message(e).unwrap();
                        }
                    },
                    "set" => match commands::set::set(&spreadsheet, args) {
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
            Err(e) => {
                eprintln!("Failed to read message: {}", e);
                continue;
            }
        }
    }
}
