use std::sync::Arc;

use rsheet_lib::{command_runner::CellValue, replies::Reply};

use crate::{spreadsheet::Spreadsheet, utils::is_valid_cell};

/// Gets the value of a cell in the spreadsheet.
pub fn get(spreadsheet: &Arc<Spreadsheet>, args: Vec<&str>) -> Result<(String, CellValue), Reply> {
    // Check that number of arguments is correct
    if args.len() < 2 {
        return Err(Reply::Error(
            "Invalid number of arguments for get".to_string(),
        ));
    }

    // Check that cell is valid
    let cell = args[1];
    if !is_valid_cell(cell) {
        return Err(Reply::Error("Invalid cell provided.".to_string()));
    }

    let (cell_val, _) = spreadsheet.get_cell(cell);

    Ok((cell.to_string(), cell_val))
}
