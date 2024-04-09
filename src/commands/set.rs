use std::{collections::HashMap, sync::Arc};

use rsheet_lib::{
    cells::{column_name_to_number, column_number_to_name},
    command_runner::{CellArgument, CommandRunner},
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

    let mut var_map: HashMap<String, CellArgument> = HashMap::new();

    for var in runner.find_variables() {
        let var_type: VariableType = categorize_variable(&var);

        match var_type {
            VariableType::Scalar => {
                let cell_val = CellArgument::Value(spreadsheet.get(var.to_string()));
                var_map.insert(var, cell_val);
            }
            VariableType::VerticalVector(start_col, start_row, end_row) => {
                let start_row: u32 = start_row.parse().unwrap();
                let end_row: u32 = end_row.parse().unwrap();

                // Fill out vertical cell vector.
                let mut cell_vec = Vec::new();

                for row in start_row..=end_row {
                    let cell = format!("{}{}", start_col, row).to_string();
                    let cell_val = spreadsheet.get(cell);

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
                    let cell_val = spreadsheet.get(cell);

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
                        let cell_val = spreadsheet.get(cell);

                        row_vec.push(cell_val);
                    }

                    cell_matrix.push(row_vec);
                }

                var_map.insert(var, CellArgument::Matrix(cell_matrix));
            }
        }
    }

    let cell_val = runner.run(&var_map);
    spreadsheet.set(cell.to_string(), cell_val);
    Ok(())
}