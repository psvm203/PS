use std::io::*;
use std::convert::TryInto;

fn main() {
	let stdin = read_to_string(stdin()).unwrap();
	
	let nums: Vec<usize> = stdin.split_ascii_whitespace().flat_map(|x| x.parse()).collect();
	let [n, a, b] = nums.try_into().unwrap();
	
	let count = n.min(a / 2 + b);
	
	println!("{count}");
}
