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

    // Conditional check beforehand to confirm that strings contain quotation
    // marks.

    // Merge the rest of the arguments into a single expression for CommandRunner
    let expr = args[2..].join(" ");
    let runner = CommandRunner::new(&expr);

    let vars = runner.find_variables();
    let var_map = fill_variable_map(spreadsheet, &vars);

    // TODO: This is a bit of a hacky way to handle dependencies. We should
    // realistically compare variables and only remove the ones that are no
    // longer considered.

    // When we set the cell again, we destroy all parent-child links and then
    // reconstruct them. This is done by getting the old expression and removing
    // all links associated with the old variables.
    remove_all_dependencies(spreadsheet, cell, &expr);

    // If the cell is dependent on another cell that has an error, we set the
    // expression of the cell to be "Dependent" to signal this. Returns early.
    for var in &vars {
        // TODO: Combine this into the below code block
        let var_val = spreadsheet.get_cell_val(var);
        if let CellValue::Error(_) = var_val {
            spreadsheet.set_cell(cell, var_val, Some("Dependent".to_string()));
            return Ok(());
        }
    }

    // Add the current cell as a child to the cells in its expression.
    vars.clone().into_iter().for_each(|var| {
        // Check if variable's value is an error - if so, we return Reply::Error.

        let var_type = categorize_variable(&var);

        // TODO: Refactor to be cleaner
        match var_type {
            VariableType::Scalar => spreadsheet.add_dependency(&var, cell),
            VariableType::VerticalVector(start_col, start_row, end_row) => {
                let start_row: u32 = start_row.parse().unwrap();
                let end_row: u32 = end_row.parse().unwrap();

                for row in start_row..=end_row {
                    let parent = format!("{}{}", start_col, row).to_string();
                    spreadsheet.add_dependency(&parent, cell);
                }
            }
            VariableType::HorizontalVector(start_row, start_col, end_col) => {
                let start_col = column_name_to_number(start_col);
                let end_col = column_name_to_number(end_col);

                for col in start_col..=end_col {
                    let col = column_number_to_name(col);
                    let parent = format!("{}{}", col, start_row).to_string();
                    spreadsheet.add_dependency(&parent, cell);
                }
            }
            VariableType::Matrix((start_col, start_row), (end_col, end_row)) => {
                let start_row: u32 = start_row.parse().unwrap();
                let end_row: u32 = end_row.parse().unwrap();

                let start_col = column_name_to_number(start_col);
                let end_col = column_name_to_number(end_col);

                for row in start_row..=end_row {
                    for col in start_col..=end_col {
                        let col = column_number_to_name(col);
                        let parent = format!("{}{}", col, row).to_string();
                        spreadsheet.add_dependency(&parent, cell);
                    }
                }
            }
        }
    });

    let cell_val = runner.run(&var_map);
    // println!("cell_val: {:?}", cell_val);

    match vars.is_empty() {
        true => spreadsheet.set_cell(cell, cell_val, None),
        false => spreadsheet.set_cell(cell, cell_val, Some(expr)),
    }

    update_children(spreadsheet, cell, &mut Vec::new())?;
    Ok(())
}

/// Removes all dependencies associated with the old expression for the cell.
fn remove_all_dependencies(spreadsheet: &Arc<Spreadsheet>, cell: &str, expr: &String) {
    let old_expr = spreadsheet.get_cell_expr(cell);

    if let Some(old_expr) = old_expr {
        if old_expr != *expr {
            let old_vars = CommandRunner::new(&old_expr).find_variables();
            old_vars.into_iter().for_each(|var| {
                // This isn't actually comparing the old variables and the
                // new variables, its just removing all variables from the old
                // expression.
                spreadsheet.remove_dependency(&var, cell);
            });
        }
    }
}

pub fn update_children(
    spreadsheet: &Arc<Spreadsheet>,
    parent: &str,
    path: &mut Vec<String>,
) -> Result<(), Reply> {
    // Checking for circular dependencies here.

    const CIRCULAR_DEP: &str = "Circular Dependency";

    if path.contains(&parent.to_string()) {
        // TODO: Add a variable to replace the string "Circular Dependency"
        // for clarity.
        spreadsheet.set_cell(
            parent,
            CellValue::Error(format!("Cell {} is self-referential", parent)),
            Some(CIRCULAR_DEP.to_string()),
        );

        // update the cells that has this parent as a dependency to be error too
        let children = spreadsheet.get_children(parent).unwrap_or_default();
        for child in children {
            spreadsheet.set_cell(
                &child,
                CellValue::Error(format!("Cell {} is self-referential", child)),
                Some(CIRCULAR_DEP.to_string()),
            );
        }

        return Ok(());
    }

    path.push(parent.to_string());

    let children = spreadsheet.get_children(parent).unwrap_or_default();

    // Update that children and then update their children recursively.
    for child in children {
        let child_expr = spreadsheet.get_cell_expr(&child).unwrap();
        let runner = CommandRunner::new(&child_expr);
        let vars = runner.find_variables();
        let var_map = fill_variable_map(spreadsheet, &vars);

        let cell_val = runner.run(&var_map);
        match vars.is_empty() {
            true => spreadsheet.set_cell(&child, cell_val, None),
            false => spreadsheet.set_cell(&child, cell_val, Some(child_expr)),
        }

        // A child could have their own children as well, so we need to update
        // them too.
        let mut new_path = path.clone();
        update_children(spreadsheet, &child, &mut new_path)?;
    }

    path.pop();

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
                let cell_val = spreadsheet.get_cell_val(&var);
                var_map.insert(var, CellArgument::Value(cell_val));
            }
            VariableType::VerticalVector(start_col, start_row, end_row) => {
                let start_row: u32 = start_row.parse().unwrap();
                let end_row: u32 = end_row.parse().unwrap();

                // Fill out vertical cell vector.
                let mut cell_vec = Vec::new();

                for row in start_row..=end_row {
                    let cell = format!("{}{}", start_col, row).to_string();
                    let cell_val = spreadsheet.get_cell_val(&cell);

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
                    let cell_val = spreadsheet.get_cell_val(&cell);

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
                        let cell_val = spreadsheet.get_cell_val(&cell);

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
