use std::sync::Arc;

use rsheet_lib::{command_runner::CellValue, replies::Reply};

use crate::{spreadsheet::Spreadsheet, utils::is_valid_cell};

/// Gets the value of a cell in the spreadsheet.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use rsheet_lib::command_runner::CellValue;
/// use rsheet_server::spreadsheet::Spreadsheet;
/// use rsheet_server::commands::get::get;
///
/// let spreadsheet = spreadsheet::new_shared_spreadsheet();
///
/// let (cell, cell_val) = get(&spreadsheet, vec!["get", "A1"]).unwrap();
/// assert_eq!(cell, "A1");
/// assert_eq!(cell_val, CellValue::None);
/// ```
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

    let cell = args[1];

    // Check that cell is valid
    if !is_valid_cell(cell) {
        return Err((cell.to_string(), Reply::Error("Invalid cell".to_string())));
    }

    let cell_val = spreadsheet.get_cell_val(cell);
    let cell_expr = spreadsheet.get_cell_expr(cell);

    // NOTE: This is a hacky way to handle dependent cells. I would have liked to
    // have a more elegant solution.
    if let Some(s) = cell_expr {
        if s == "Dependent" {
            return Err((
                cell.to_string(),
                Reply::Error(format!("A dependent cell contained an error: {}", cell_val)),
            ));
        } else if s == "Circular Dependency" {
            if let CellValue::Error(s) = cell_val {
                return Err((cell.to_string(), Reply::Error(s)));
            }
        }
    }

    Ok((cell.to_string(), cell_val))
}
