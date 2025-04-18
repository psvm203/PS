problem=$1
rustc -o ./target/BOJ/$problem ./BOJ/$problem.rs
./target/BOJ/$problem < stdin.txt
