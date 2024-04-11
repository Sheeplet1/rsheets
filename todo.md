Test test_06_05_one-level_dependency_changes (one-level_dependency_changes #05) - failed (Incorrect output)
Your program produced these 2 lines of output:
A2 = Error (hidden by mark mode)
A2 = 10

The correct 2 lines of output for this test were:
A2 = 8
A2 = 10

The difference between your output(-) and the correct output(+) is:

- A2 = Error (hidden by mark mode)

* A2 = 8
  A2 = 10

The input for this test was:
a: set A1 1
b: set A2 sleep_then(100, A1 + 7)
b: get A2
c: sleep 200
c: set A1 3
c: sleep 200
c: get A2
Note: last character in above input is not '\n'

You can reproduce this test by executing these commands:
6991 cargo build --target-dir target # crate.tar
echo -n 'a: set A1 1\nb: set A2 sleep_then(100, A1 + 7)\nb: get A2\nc: sleep 200\nc: set A1 3\nc: sleep 200\nc: get A2' | ./target/debug/rsheet

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
echo -n 'a: set A1 5\na: sleep 100\na: set A2 A1 + 1\na: set A3 A1 + A2\nb: set B1 A1 + 1\nb: set C1 B1 + A1\nc: set B2 A1\nc: set C3 2 \* B2\nd: sleep 250\nd: get A3\nd: get C3\nd: get C1\nd: set A1 9\nd: set D4 C3 + C1 + A3\ne: sleep 600\ne: get D4' | ./target/debug/rsheet
