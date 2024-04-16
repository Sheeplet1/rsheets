use std::sync::Arc;

use rsheet_lib::{
    cells::{column_name_to_number, column_number_to_name},
    command_runner::{CellValue, CommandRunner},
    replies::Reply,
};

use crate::{commands::variables::variable_map_for_runner, spreadsheet::Spreadsheet};

/// Add the current cell as a dependency to all cells in the given range.
pub fn add_as_dependent(
    spreadsheet: &Arc<Spreadsheet>,
    cell: &str,
    start_row: &str,
    start_col: &str,
    end_row: &str,
    end_col: &str,
) {
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

/// Removes all dependencies associated with the old expression for the cell.
pub fn remove_all_dependencies(spreadsheet: &Arc<Spreadsheet>, cell: &str, expr: &String) {
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

/// Updates the dependencies of the cell.
pub fn update_dependency(
    spreadsheet: &Arc<Spreadsheet>,
    parent: &str,
    path: &mut Vec<String>,
    timestamp: usize,
) -> Result<(), Reply> {
    if path.contains(&parent.to_string()) {
        // If the parent is in the path, then we have found a circular
        // dependency
        handle_circular_dependency(spreadsheet, parent);
        return Ok(());
    }

    path.push(parent.to_string());

    let dependencies = spreadsheet.get_dependencies(parent).unwrap_or_default();

    // Update that children and then update their children recursively.
    for dep in dependencies {
        let expr = match spreadsheet.get_cell_expr(&dep) {
            Some(expr) => expr,
            None => {
                // If there is no expression, then skip the cell. Realistically,
                // this shouldn't happen since if there is a dependency, there
                // should be an expression. However, this is a safety check.
                continue;
            }
        };
        let runner = CommandRunner::new(&expr);
        let vars = runner.find_variables();
        let var_map = variable_map_for_runner(spreadsheet, &vars);

        let cell_val = runner.run(&var_map);
        match vars.is_empty() {
            true => spreadsheet.set_cell(&dep, cell_val, None, timestamp),
            false => spreadsheet.set_cell(&dep, cell_val, Some(expr), timestamp),
        }

        // Each dependency could have its own set of dependencies, so we need
        // to update those as well.
        let mut new_path = path.clone();
        update_dependency(spreadsheet, &dep, &mut new_path, timestamp)?;
    }

    path.pop();

    Ok(())
}

fn handle_circular_dependency(spreadsheet: &Arc<Spreadsheet>, parent: &str) {
    let parent_timestamp = spreadsheet.get_cell_timestamp(parent);
    spreadsheet.set_cell(
        parent,
        CellValue::Error(format!("Cell {} is self-referential", parent)),
        Some("Circular Dependency".to_string()),
        parent_timestamp,
    );

    // Each dependency could have its own dependencies, so we need to also
    // update them to contain the error message. `unwrap_or_default` is
    // used here to handle the case where the parent cell has no
    // dependencies.
    let dependencies = spreadsheet.get_dependencies(parent).unwrap_or_default();
    for dep in dependencies {
        let dep_timestamp = spreadsheet.get_cell_timestamp(&dep);
        spreadsheet.set_cell(
            &dep,
            CellValue::Error(format!("Cell {} is involved in a circular dependency", dep)),
            Some("Circular Dependency".to_string()),
            dep_timestamp,
        );
    }
}
