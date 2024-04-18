use std::sync::Arc;

use rsheet_lib::{
    command_runner::{CellValue, CommandRunner},
    replies::Reply,
};

use crate::{
    commands::{
        dependencies::{add_dependencies, remove_all_dependencies, update_dependency},
        variables::variable_map_for_runner,
    },
    spreadsheet::Spreadsheet,
    utils::is_valid_cell,
};

use super::variables::{categorize_variable, VariableType};

/// Sets the value of a cell in the spreadsheet.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use rsheet_lib::command_runner::CellValue;
/// use rsheet_server::spreadsheet::Spreadsheet;
/// use rsheet_server::commands::set::set;
/// use rsheet_server::commands::get::get;
///
/// let spreadsheet = spreadsheet::new_shared_spreadsheet();
/// let result = set(&spreadsheet, vec!["set", "A1", "5"], 0);
/// assert!(result.is_ok());
///
/// let (cell, cell_val) = get(&spreadsheet, vec!["get", "A1"]).unwrap();
/// assert_eq!(cell, "A1");
/// assert_eq!(cell_val, CellValue::Int(5));
/// ```
pub fn set(spreadsheet: &Arc<Spreadsheet>, args: Vec<&str>, timestamp: u64) -> Result<(), Reply> {
    // Set should have at least 3 arguments: <set> <cell> <expression>
    if args.len() < 3 {
        return Err(Reply::Error(
            "Invalid number of arguments supplied for set".to_string(),
        ));
    }

    // Check that the cell is valid.
    let cell = args[1];
    if !is_valid_cell(cell) {
        return Err(Reply::Error("Invalid cell provided.".to_string()));
    }

    // Merge the rest of the arguments into a single expression for CommandRunner
    let expr = args[2..].join(" ");
    let runner = CommandRunner::new(&expr);

    // When we set the cell again, we remove all dependencies associated with
    // the old expression.
    remove_all_dependencies(spreadsheet, cell, &expr);

    let vars = runner.find_variables();

    for var in &vars {
        // If the variable's value is an error, we set the cell's value to be
        // an error as well, but we set the expression to "Dependent" to
        // signal that the cell is dependent on an error cell.
        let var_val = spreadsheet.get_cell_val(var);
        if let CellValue::Error(_) = var_val {
            spreadsheet.set_cell(cell, var_val, Some("Dependent".to_string()), timestamp);
            return Ok(());
        }

        // Otherwise, we add the cell as a dependent to the variables in it's
        // expression.
        let var_type = categorize_variable(var);
        match var_type {
            VariableType::Scalar => spreadsheet.add_dependency(var, cell),
            VariableType::VerticalVector(start_col, start_row, end_row) => {
                add_dependencies(spreadsheet, cell, start_col, start_row, start_col, end_row)
            }
            VariableType::HorizontalVector(start_row, start_col, end_col) => {
                add_dependencies(spreadsheet, cell, start_col, start_row, end_col, start_row)
            }
            VariableType::Matrix((start_col, start_row), (end_col, end_row)) => {
                add_dependencies(spreadsheet, cell, start_col, start_row, end_col, end_row)
            }
        }
    }

    let var_map = variable_map_for_runner(spreadsheet, &vars);
    let cell_val = runner.run(&var_map);

    match vars.is_empty() {
        true => spreadsheet.set_cell(cell, cell_val, None, timestamp),
        false => spreadsheet.set_cell(cell, cell_val, Some(expr), timestamp),
    }

    update_dependency(spreadsheet, cell, &mut Vec::new(), timestamp)?;
    Ok(())
}
