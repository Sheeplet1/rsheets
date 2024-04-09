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

fn get_row_col(cell: &str) -> (&str, &str) {
    let (col, row) = cell.split_at(
        cell.find(|c: char| c.is_ascii_digit())
            .expect("Invalid cells should not make it to this stage."),
    );
    (col, row)
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
