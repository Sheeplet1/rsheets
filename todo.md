Test test_02_05_simple_calculations (simple_calculations #05) - failed (Incorrect output)
Your program produced this line of output:
Error (hidden by mark mode)

The correct 1 lines of output for this test were:
A1 = Error (hidden by mark mode)

The difference between your output(-) and the correct output(+) is:

- Error (hidden by mark mode)

* A1 = Error (hidden by mark mode)
  ? +++++

The input for this test was:
set A1 asdf-not-a-command
get A1
Note: last character in above input is not '\n'

You can reproduce this test by executing these commands:
6991 cargo build --target-dir target # crate.tar
echo -n -e 'set A1 asdf-not-a-command\nget A1' | ./target/debug/rsheet --mark-mode
Test test_02_06_simple_calculations (simple_calculations #06) - failed (Incorrect output)
Your program produced these 2 lines of output:
Error (hidden by mark mode)
A7 = "test"

The correct 2 lines of output for this test were:
A6 = Error (hidden by mark mode)
A7 = "test"

The difference between your output(-) and the correct output(+) is:

- Error (hidden by mark mode)

* A6 = Error (hidden by mark mode)
  ? +++++

...

The input for this test was:
set A7 "test"
set A6 silly-error
get A6
get A7
Note: last character in above input is not '\n'

You can reproduce this test by executing these commands:
6991 cargo build --target-dir target # crate.tar
echo -n -e 'set A7 "test"\nset A6 silly-error\nget A6\nget A7' | ./target/debug/rsheet --mark-mode

Test test_03_06_simple_references (simple_references #06) - failed (Incorrect output)
Your program produced these 2 lines of output:
Error (hidden by mark mode)
A2 = "'this' can only be used in functions (line 1, position 7)"

The correct 2 lines of output for this test were:
A1 = Error (hidden by mark mode)
Error (hidden by mark mode)

The difference between your output(-) and the correct output(+) is:

- A1 = Error (hidden by mark mode)
  Error (hidden by mark mode)

* A2 = "'this' can only be used in functions (line 1, position 7)"

The input for this test was:
set A1 error-this-isn't real code
set A2 A1
get A1
get A2
Note: last character in above input is not '\n'

You can reproduce this test by executing these commands:
6991 cargo build --target-dir target # crate.tar
echo -n -e 'set A1 error-this-isn'"'"'t real code\nset A2 A1\nget A1\nget A2' | ./target/debug/rsheet --mark-mode

Test test_08_05_multi-layered_deps (multi-layered_deps #05) - failed (Incorrect output)
Your program produced these 4 lines of output:
A3 = 11
C3 = 10
C1 = 11
D4 = 56

The correct 4 lines of output for this test were:
A3 = 11
C3 = 10
C1 = 11
D4 = 32

The difference between your output(-) and the correct output(+) is:
...
C1 = 11

- D4 = 56

* D4 = 32

The input for this test was:
a: set A1 5
a: sleep 100
a: set A2 A1 + 1
a: set A3 A1 + A2
b: set B1 A1 + 1
b: set C1 B1 + A1
c: set B2 A1
c: set C3 2 \* B2
d: sleep 250
d: get A3
d: get C3
d: get C1
d: set A1 9
d: set D4 C3 + C1 + A3
e: sleep 600
e: get D4
Note: last character in above input is not '\n'

You can reproduce this test by executing these commands:
6991 cargo build --target-dir target # crate.tar
echo -n -e 'a: set A1 5\na: sleep 100\na: set A2 A1 + 1\na: set A3 A1 + A2\nb: set B1 A1 + 1\nb: set C1 B1 + A1\nc: set B2 A1\nc: set C3 2 \* B2\nd: sleep 250\nd: get A3\nd: get C3\nd: get C1\nd: set A1 9\nd: set D4 C3 + C1 + A3\ne: sleep 600\ne: get D4' | ./target/debug/rsheet --mark-mode
Test test_09_01_circular_deps (circular_deps #01) - failed (Incorrect output)
Your program produced this line of output:
Error (hidden by mark mode)

The correct 2 lines of output for this test were:
Error (hidden by mark mode)
Error (hidden by mark mode)

The difference between your output(-) and the correct output(+) is:
Error (hidden by mark mode)

- Error (hidden by mark mode)

The input for this test was:
set A1 A2
set A2 A1
get A1
get A2
Note: last character in above input is not '\n'

You can reproduce this test by executing these commands:
6991 cargo build --target-dir target # crate.tar
echo -n -e 'set A1 A2\nset A2 A1\nget A1\nget A2' | ./target/debug/rsheet --mark-mode

Test test_09_04_circular_deps (circular_deps #04) - failed (errors)
Your program's output was correct but errors occurred:
thread '<unnamed>' panicked at src/commands/set.rs:155:60:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at /tmp/tmp1np7h3nr/autotest/src/lib.rs:29:24:
called `Result::unwrap()` on an `Err` value: Any { .. }
Apart from the above errors, your program's output was correct.

The input for this test was:
A: set A1 A2
B: set A2 A1
C: sleep 200
C: get A1
Note: last character in above input is not '\n'

You can reproduce this test by executing these commands:
6991 cargo build --target-dir target # crate.tar
echo -n -e 'A: set A1 A2\nB: set A2 A1\nC: sleep 200\nC: get A1' | ./target/debug/rsheet --mark-mode
