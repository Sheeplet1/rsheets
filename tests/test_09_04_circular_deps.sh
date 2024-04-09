cargo build --target-dir target # crate.tar
echo -n -e 'A: set A1 A2\nB: sleep 100\nB: set A2 A1\nC: sleep 200\nC: get A1' | ./target/debug/rsheet --mark-mode
