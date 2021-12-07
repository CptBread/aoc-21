use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day7.txt").unwrap();
	let mut line = String::new();
	BufReader::new(f).read_line(&mut line).unwrap();
	// let mut crabs: HashMap<_, u64> = HashMap::new();
	let mut crabs: Vec<u32> = Vec::new();
	for n in line.split(',') {
		insert_sorted(&mut crabs, n.parse().unwrap());
		// *crabs.entry(n.parse().unwrap()).or_insert(0) += 1;
	}
	let med = crabs[crabs.len() / 2];
	// println!("{:?} {}", crabs, med);

	let mut dist = 0;
	let mut tri_dist = 0;
	for x in crabs.iter().copied() {
		let diff = diff_abs(x, med);
		dist += diff;
		tri_dist += tri_cost(diff);
	}

	let small = *crabs.first().unwrap();
	let big = *crabs.last().unwrap();
	let mut best_cost = tri_dist;
	let mut best = med;
	for x in small..med {
		let cost = crabs.iter().copied().fold(0, |acc, n| acc + tri_cost(diff_abs(x, n)));
		if cost < best_cost {
			best = x;
			best_cost = cost;
		}
	}

	for x in med..big {
		let cost = crabs.iter().copied().fold(0, |acc, n| acc + tri_cost(diff_abs(x, n)));
		if cost < best_cost {
			best = x;
			best_cost = cost;
		}
	}

	println!("{} {}", dist, med);
	println!("{} {}", best_cost, best);
}

fn diff_abs(v0: u32, v1: u32) -> u32 {
	if v0 > v1 {v0 - v1} else {v1 - v0}
}

fn tri_cost(v: u32) -> u32 {
	v * (v + 1) / 2
}

fn insert_sorted<T>(vec: &mut Vec<T>, v: T)
	where T: Ord
{
	match vec.binary_search(&v) {
		Ok(pos) => vec.insert(pos, v),
		Err(pos) => vec.insert(pos, v),
	}
}