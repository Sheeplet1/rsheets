# Tests

These tests currently pass inconsistently. I believe this to be due to
concurrency issues which is a pain to debug.

## Test 1

echo -n 'a: set A1 5\na: sleep 100\na: set A2 A1 + 1\na: set A3 A1 + A2\nb: set B1 A1 + 1\nb: set C1 B1 + A1\nc: set B2 A1\nc: set C3 2 \* B2\nd: sleep 250\nd: get A3\nd: get C3\nd: get C1\nd: set A1 9\nd: set D4 C3 + C1 + A3\ne: sleep 600\ne: get D4' | ./target/debug/rsheet

a: set A1 5 -> A1 = 5
a: sleep 100
a: set A2 A1 + 1 -> A2 = 6
a: set A3 A1 + A2 -> A3 = 11
b: set B1 A1 + 1 -> B1 = 6
b: set C1 B1 + A1 -> C1 = 11
c: set B2 A1 -> B2 = 5
c: set C3 2 \* B2 -> C3 = 10
d: sleep 250
d: get A3  
d: get C3
d: get C1
d: set A1 9 -> A1 = 9 | A2 = 10 | A3 = 19 | B1 = 10 | C1 = 19 | B2 = 9 | C3 = 18
d: set D4 C3 + C1 + A3
e: sleep 600
e: get D4

My issue is occurring at `d: set A1 9` which is not updating concurrently?

### Answer

A3 = 11
C3 = 10
C1 = 11
D4 = 56

### Errors

A3 = 11
C3 = Error: "Function not found: \* (i64, ()) (line 1, position 1)"
Error: A dependent cell contained an error: Error: "Function not found: + ((), i64) (line 1,
position 1)"
Error: A dependent cell contained an error: Error: "Function not found: + ((), i64) (line 1,
position 1)"

A3 = 11
C3 = 10
C1 = 11
D4 = 32

A3 = 11
C3 = 10
C1 = 11
D4 = 48

## Test 2

echo -n -e 'a: set A1 1\nb: set A2 sleep_then(100, A1 + 7)\nb: get A2\nc: sleep 200\nc: set A1 3\nc: sleep 200\nc: get A2' | ./target/debug/rsheet --mark-mode
