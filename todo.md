echo -n 'a: set A1 5\na: sleep 100\na: set A2 A1 + 1\na: set A3 A1 + A2\nb: set B1 A1 + 1\nb: set C1 B1 + A1\nc: set B2 A1\nc: set C3 2 * B2\nd: sleep 250\nd: get A3\nd: get C3\nd: get C1\nd: set A1 9\nd: set D4 C3 + C1 + A3\ne: sleep 600\ne: get D4' | ./target/debug/rsheet

echo -n -e 'a: set A1 1\nb: set A2 sleep_then(100, A1 + 7)\nb: get A2\nc: sleep 200\nc: set A1 3\nc: sleep 200\nc: get A2' | ./target/debug/rsheet --mark-mode
