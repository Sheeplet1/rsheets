pub mod spreadsheet;

use rsheet_lib::command_runner::{CellArgument, CommandRunner};
use rsheet_lib::connect::{Manager, Reader, Writer};
use rsheet_lib::replies::Reply;

use std::collections::HashMap;
use std::error::Error;

use log::info;

use crate::spreadsheet::is_valid_cell;

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let (mut recv, mut send) = manager.accept_new_connection().unwrap();
    let mut spreadsheet = spreadsheet::new();
    loop {
        info!("Just got message");
        let msg = recv.read_message()?;

        // parse the commands into their arguments
        let args: Vec<&str> = msg.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        let command = args[0];
        match command {
            "get" => {
                // Check that number of arguments is correct
                if args.len() < 2 {
                    send.write_message(Reply::Error(
                        "Invalid number of arguments for get".to_string(),
                    ))?;
                    continue;
                }

                // check that cell is valid
                let cell = args[1];
                if !is_valid_cell(cell) {
                    // TODO: Extract common errors into a separate module
                    send.write_message(Reply::Error("Invalid cell provided".to_string()))?;
                }

                let cell_val = spreadsheet.get(cell.to_string());

                send.write_message(Reply::Value(cell.to_string(), cell_val))?;
            }
            "set" => {
                // check that the number of arguments is correct
                if args.len() < 3 {
                    send.write_message(Reply::Error(
                        "Invalid number of arguments for set".to_string(),
                    ))?;
                    continue;
                }

                // check that the cell is valid
                let cell = args[1];
                if !is_valid_cell(cell) {
                    send.write_message(Reply::Error("Invalid cell provided".to_string()))?;
                }

                // TODO: Eventually, we need to handle arithmetics with cells
                let variables: HashMap<String, CellArgument> = HashMap::new();
                let expr = args[2..].join(" ");

                let cell_val = CommandRunner::new(&expr).run(&variables);
                spreadsheet.set(cell.to_string(), cell_val);
            }
            _ => {
                send.write_message(Reply::Error("Invalid command".to_string()))?;
            }
        }
    }
}
