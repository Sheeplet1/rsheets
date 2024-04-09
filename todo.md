# Task 4

Dependency changes - `set B1 A1 * 2`

If A1 is changed to 3, then B1 must be set to 6.

We have two options:

1. We can re-evaluate the value for that cell whenever it is called with `get`.
2. Or if we create a dependency list for each cell, whenever that cell is changed,
   then we update each cell that is dependent on that cell.

So what do we need to do here?

1. Currently, our spreadsheet is of type `DashMap<String, CellValue>`. We need
   to figure out if we need to change the value of the `DashMap` to `String` to hold
   expressions such as `A1 * 2`, and then whenever we get that cell, we have to
   evaluate that expression recursively.

I am pretty sure that is the only change required, could be wrong. It just means
we need to make some parsing and evaluation helper function to assist with
these expressions.

===============================================================================

Looking at future Task 5 and Task 6, it seems they are pushing me to implement
the dependency list in a graphical or adjacency list approach.

Therefore, we'll need:

1. Need to add capabilities for CellValue::String to hold expressions.
2. Create a parsing helper to extract the cell name, and then the cell value
   recursively.

- Differentiate between CELL and VALUE. We can just check by seeing if that
  character has ASCII digits.
- If the CELL has no `_`, then it is a scalar type, just get that value.
- If the CELL has `_`, then get its type (Horizontal, Vertical, Matrix), then
  get its value.

3. A new module for managing and updating dependencies.

- If `set` creates a dependency, then need to add.
- If `set` removes a dependency, need to remove from the list.

===============================================================================

Currently, have `set` storing expressions in `CellValue::String` and then `get`
will recursively calculate that value at run-time/call which means Task 6
may require refactoring in the future.

If we are storing expressions as CellValue::String, then we also need to parse
basic commands such as `sum(A1_C1)` in the case of matrixes. Therefore, when
we do `get` we will also need to evaluate these expressions at run-time?

Decided to go against calculating values at run-time when `get` is called as
this doesn't solve the problem of circular dependencies.

So I will be calculating the value when `set` is called.

I might need to change the whole `set` function and mindmap the logic out.
