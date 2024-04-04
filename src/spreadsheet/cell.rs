use rsheet_lib::command_runner::CellValue;

pub struct Cell {
    pub value: CellValue,
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        Cell { value }
    }
}
