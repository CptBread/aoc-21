use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::{HashMap};
// use vek::vec::repr_c::{Vec2};

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day14.txt").unwrap();
	let mut it = BufReader::new(f).lines().map(Result::unwrap);
	let chain: Vec<char> = it.next().unwrap().chars().collect();
	let mut recip = HashMap::new();
	// let mut pairs
	it.next();
	while let Some(line) = it.next() {
		let (pair, res) = line.split_once(" -> ").unwrap();
		let mut chars = pair.chars();
		let p0 = chars.next().unwrap();
		let p1 = chars.next().unwrap();
		let new = res.chars().next().unwrap();
		recip.insert((p0, p1), ((p0, new), (new, p1)));
	}

	let mut counts = HashMap::new();
	for idx in 0..(chain.len() - 1) {
		*counts.entry((chain[idx], chain[idx + 1])).or_insert(0) += 1;
	}

	for _ in 0..40 {
		// println!("{:?}", counts);
		let mut next = HashMap::new();
		for (p, count) in counts.iter() {
			let (p0, p1) = recip.get(p).unwrap();
			*next.entry(*p0).or_insert(0) += count;
			*next.entry(*p1).or_insert(0) += count;
		}
		counts = next;
	}
	// println!("{:?}", counts);

	let mut char_counts = HashMap::new();
	for (p, count) in counts.iter() {
		// *char_counts.entry(p.0).or_insert(0 as u32) += count;
		*char_counts.entry(p.1).or_insert(0 as u64) += count;
	}
	let mut min = ('A', !0);
	let mut max = ('A', 0);
	for p in char_counts.iter().map(|(c, v)| (*c, *v)) {
		min = if p.1 < min.1 {p} else {min};
		max = if p.1 > max.1 {p} else {max};
	}
	println!("{:?} {:?}", min, max);
	println!("{}", max.1 - min.1);
}
