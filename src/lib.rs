mod commands;
pub mod spreadsheet;
mod variables;

use rsheet_lib::connect::{Manager, Reader, Writer};
use rsheet_lib::replies::Reply;
use spreadsheet::Spreadsheet;

use std::error::Error;

use log::info;

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let (mut recv, mut send) = manager.accept_new_connection().unwrap();
    let mut spreadsheet = Spreadsheet::new();
    loop {
        info!("Just got message");
        let msg = recv.read_message()?;

        // parse the commands into their arguments
        let args: Vec<&str> = msg.split_whitespace().collect();
        if args.is_empty() {
            // TODO: Maybe send a message on how to use the program?
            continue;
        }

        let command = args[0];
        match command {
            "get" => match commands::get(&mut spreadsheet, args) {
                Ok((cell, cell_val)) => {
                    send.write_message(Reply::Value(cell, cell_val))?;
                }
                Err(e) => {
                    send.write_message(e)?;
                }
            },
            "set" => match commands::set(&mut spreadsheet, args) {
                Ok(_) => {}
                Err(e) => {
                    send.write_message(e)?;
                }
            },
            _ => {
                send.write_message(Reply::Error(format!("Invalid command: {}", command)))?;
            }
        }
    }
}
