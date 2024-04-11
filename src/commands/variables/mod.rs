use std::collections::HashMap;

use rsheet_lib::{
    cells::{column_name_to_number, column_number_to_name},
    command_runner::{CellArgument, CellValue},
};

use crate::spreadsheet::Spreadsheet;

type StartCol<'a> = &'a str;
type EndCol<'a> = &'a str;
type StartRow<'a> = &'a str;
type EndRow<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub enum VariableType<'a> {
    /// Basic variables will have only one cell.
    /// Example: A1
    Scalar,

    /// Horizontal vectors will have the same numbers.
    /// Example: A1_C1
    HorizontalVector(StartRow<'a>, StartCol<'a>, EndCol<'a>),

    /// Vertical vectors will have the same letters.
    /// Example: A1_A3
    VerticalVector(StartCol<'a>, StartRow<'a>, EndRow<'a>),

    /// Matrixes will have different numbers and letters.
    /// Example: A1_C3
    Matrix((StartCol<'a>, StartRow<'a>), (EndCol<'a>, EndRow<'a>)),
}

pub fn categorize_variable(variable: &str) -> VariableType {
    let cells: Vec<&str> = variable.split('_').collect();

    if cells.len() == 1 {
        return VariableType::Scalar;
    }

    let (start_col, start_row) = get_row_col(cells[0]);
    let (end_col, end_row) = get_row_col(cells[1]);

    if start_col == end_col {
        return VariableType::VerticalVector(start_col, start_row, end_row);
    }

    if start_row == end_row {
        return VariableType::HorizontalVector(start_row, start_col, end_col);
    }

    VariableType::Matrix((start_col, start_row), (end_col, end_row))
}

pub fn variable_map_for_runner(
    spreadsheet: &Spreadsheet,
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
                let cell_vec =
                    create_cell_vec(start_row, end_row, start_col, start_col, spreadsheet);
                var_map.insert(var, CellArgument::Vector(cell_vec));
            }
            VariableType::HorizontalVector(start_row, start_col, end_col) => {
                let cell_vec =
                    create_cell_vec(start_row, start_row, start_col, end_col, spreadsheet);
                var_map.insert(var, CellArgument::Vector(cell_vec));
            }
            VariableType::Matrix((start_col, start_row), (end_col, end_row)) => {
                let cell_matrix =
                    create_cell_matrix(start_row, end_row, start_col, end_col, spreadsheet);
                var_map.insert(var, CellArgument::Matrix(cell_matrix));
            }
        }
    }
    var_map
}

fn get_row_col(cell: &str) -> (&str, &str) {
    let (col, row) = cell.split_at(
        cell.find(|c: char| c.is_ascii_digit())
            .expect("Invalid cells should not make it to this stage."),
    );
    (col, row)
}

fn create_cell_vec(
    start_row: &str,
    end_row: &str,
    start_col: &str,
    end_col: &str,
    spreadsheet: &Spreadsheet,
) -> Vec<CellValue> {
    let start_row: u32 = start_row.parse().unwrap();
    let end_row: u32 = end_row.parse().unwrap();
    let start_col = column_name_to_number(start_col);
    let end_col = column_name_to_number(end_col);
    let mut cell_vec = Vec::new();

    for row in start_row..=end_row {
        for col in start_col..=end_col {
            let col = column_number_to_name(col);
            let cell = format!("{}{}", col, row).to_string();
            let cell_val = spreadsheet.get_cell_val(&cell);

            cell_vec.push(cell_val);
        }
    }

    cell_vec
}

fn create_cell_matrix(
    start_row: &str,
    end_row: &str,
    start_col: &str,
    end_col: &str,
    spreadsheet: &Spreadsheet,
) -> Vec<Vec<CellValue>> {
    let start_row: u32 = start_row.parse().unwrap();
    let end_row: u32 = end_row.parse().unwrap();
    let start_col = column_name_to_number(start_col);
    let end_col = column_name_to_number(end_col);
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

    cell_matrix
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_categorize_variable() {
        let scalar = "A1";
        let horizontal_vector = "A1_C1";
        let vertical_vector = "A1_A3";
        let matrix = "A1_C3";

        assert_eq!(categorize_variable(scalar), VariableType::Scalar);
        assert_eq!(
            categorize_variable(horizontal_vector),
            VariableType::HorizontalVector("1", "A", "C")
        );
        assert_eq!(
            categorize_variable(vertical_vector),
            VariableType::VerticalVector("A", "1", "3")
        );
        assert_eq!(
            categorize_variable(matrix),
            VariableType::Matrix(("A", "1"), ("C", "3"))
        );
    }
}
