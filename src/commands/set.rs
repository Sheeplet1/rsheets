use std::{collections::HashMap, sync::Arc};

use rsheet_lib::{
    cells::{column_name_to_number, column_number_to_name},
    command_runner::{CellArgument, CellValue, CommandRunner},
    replies::Reply,
};

use crate::{spreadsheet::Spreadsheet, utils::is_valid_cell};

use super::variables::{categorize_variable, VariableType};

/// Sets the value of a cell in the spreadsheet.
pub fn set(spreadsheet: &Arc<Spreadsheet>, args: Vec<&str>) -> Result<(), Reply> {
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

    let vars = runner.find_variables();
    let var_map = fill_variable_map(spreadsheet, &vars);

    let cell_val = runner.run(&var_map);
    match vars.is_empty() {
        true => spreadsheet.set_cell(cell, cell_val, None),
        false => spreadsheet.set_cell(cell, cell_val, Some(expr)),
    }
    // TODO: Update dependencies here and check for circular dependencies.

    Ok(())
}

fn fill_variable_map(
    spreadsheet: &Arc<Spreadsheet>,
    variables: &Vec<String>,
) -> HashMap<String, CellArgument> {
    // We need to get the values of the variables in the expression and
    // store them into the variables hashmap for the CommandRunner.
    let mut var_map: HashMap<String, CellArgument> = HashMap::new();
    for var in variables {
        let var_type: VariableType = categorize_variable(var);
        let var = var.to_string();

        match var_type {
            VariableType::Scalar => {
                let (cell_val, _) = spreadsheet.get_cell(&var);
                var_map.insert(var, CellArgument::Value(cell_val));
            }
            VariableType::VerticalVector(start_col, start_row, end_row) => {
                let start_row: u32 = start_row.parse().unwrap();
                let end_row: u32 = end_row.parse().unwrap();

                // Fill out vertical cell vector.
                let mut cell_vec = Vec::new();

                for row in start_row..=end_row {
                    let cell = format!("{}{}", start_col, row).to_string();
                    let (cell_val, _) = spreadsheet.get_cell(&cell);

                    cell_vec.push(cell_val);
                }

                var_map.insert(var, CellArgument::Vector(cell_vec));
            }
            VariableType::HorizontalVector(start_row, start_col, end_col) => {
                // Convert columns name to numbers for iteration
                let start_col = column_name_to_number(start_col);
                let end_col = column_name_to_number(end_col);

                // Fill out horizontal cell vector.
                let mut cell_vec = Vec::new();

                for col in start_col..=end_col {
                    let col = column_number_to_name(col);

                    let cell = format!("{}{}", col, start_row).to_string();
                    let (cell_val, _) = spreadsheet.get_cell(&cell);

                    cell_vec.push(cell_val)
                }

                var_map.insert(var, CellArgument::Vector(cell_vec));
            }
            VariableType::Matrix((start_col, start_row), (end_col, end_row)) => {
                let start_row: u32 = start_row.parse().unwrap();
                let end_row: u32 = end_row.parse().unwrap();

                let start_col = column_name_to_number(start_col);
                let end_col = column_name_to_number(end_col);

                // Fill out the matrix.
                let mut cell_matrix = Vec::new();

                for row in start_row..=end_row {
                    let mut row_vec = Vec::new();
                    for col in start_col..=end_col {
                        let col = column_number_to_name(col);

                        let cell = format!("{}{}", col, row).to_string();
                        let (cell_val, _) = spreadsheet.get_cell(&cell);

                        row_vec.push(cell_val);
                    }

                    cell_matrix.push(row_vec);
                }

                var_map.insert(var, CellArgument::Matrix(cell_matrix));
            }
        }
    }
    var_map
}
