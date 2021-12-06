use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day6.txt").unwrap();
	let mut line = String::new();
	BufReader::new(f).read_line(&mut line).unwrap();
	let mut fishes: HashMap<_, u64> = HashMap::new();
	for n in line.split(',') {
		*fishes.entry(n.parse().unwrap()).or_insert(0) += 1;
	}

	const DAYS:i32 = 80;
	for day in 0..DAYS {
		if let Some(num) = fishes.remove(&day) {
			*fishes.entry(day + 7).or_insert(0) += num;
			*fishes.entry(day + 9).or_insert(0) += num;
		}
	}
	let first = fishes.iter().fold(0, |acc, (_, num)| acc + num);
	for day in DAYS..256 {
		if let Some(num) = fishes.remove(&day) {
			*fishes.entry(day + 7).or_insert(0) += num;
			*fishes.entry(day + 9).or_insert(0) += num;
		}
	}

	let num = fishes.iter().fold(0, |acc, (_, num)| acc + num);
	println!("{} {}", first, num);
}
