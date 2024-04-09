use std::sync::Arc;

use rsheet_lib::{command_runner::CellValue, replies::Reply};

use crate::{spreadsheet::Spreadsheet, utils::is_valid_cell};

/// Gets the value of a cell in the spreadsheet.
pub fn get(
    spreadsheet: &Arc<Spreadsheet>,
    args: Vec<&str>,
) -> Result<(String, CellValue), (String, Reply)> {
    // Check that number of arguments is correct
    if args.len() < 2 {
        return Err((
            "".to_string(),
            Reply::Error("Invalid number of arguments for get".to_string()),
        ));
    }

    // Check that cell is valid
    let cell = args[1];
    if !is_valid_cell(cell) {
        return Err((
            cell.to_string(),
            Reply::Error("Invalid cell provided.".to_string()),
        ));
    }

    let cell_val = spreadsheet.get_cell_val(cell);

    // This can only occur when its in a circular dependency.
    if let CellValue::Error(s) = cell_val {
        // Based on the autotests, if there is a circular dependency, we should
        // not print any message other than the error. So, so we return an
        // Ok variant with the error message.
        if s == *"Circular Dependency".to_string() {
            Ok((cell.to_string(), CellValue::Error(s)))
        } else {
            // Otherwise, we should print the error message.
            Err((cell.to_string(), Reply::Error(s)))
        }
    } else {
        Ok((cell.to_string(), cell_val))
    }
}
