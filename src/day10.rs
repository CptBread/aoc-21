use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use crate::utils::*;

#[allow(dead_code)]
pub fn solve() {
	// let brakets: HashMap<char, (u8, bool)> = 
	let mut brackets = HashMap::new();
	let mut add_pair = |l, r, val| {
		brackets.insert(l, (r, true, val));
		brackets.insert(r, (r, false, val));
	};
	add_pair('(', ')', 3);
	add_pair('[', ']', 57);
	add_pair('{', '}', 1197);
	add_pair('<', '>', 25137);

	let f = File::open("data/day10.txt").unwrap();
	let mut it = BufReader::new(f).lines().map(Result::unwrap);
	let mut points = 0;
	let mut scores = Vec::new();
	while let Some(line) = it.next() {
		let mut stack = Vec::new();
		let mut last_open = None;
		let mut corrupt = false;
		for c in line.chars() {
			let (id, open, point) = brackets.get(&c).unwrap().clone();
			if open {
				stack.push(id);
				last_open = Some(id);
			}
			else if last_open == Some(id) {
				stack.pop();
				last_open = stack.last().copied();
			}
			else {
				points += point;
				corrupt = true;
				// println!("Error expected {:?} but found {:?}", last_open.unwrap(), id);
				break;
			}
		}

		if !corrupt {
			let mut score = 0u64;
			for c in stack.iter().rev().copied() {
				let v = match c {
					')' => 1,
					']' => 2,
					'}' => 3,
					'>' => 4,
					_ => panic!(),
				};
				score = score * 5 + v;
			}
			// println!("Completion score {}", score);
			scores.push(score);
		}
	}
	scores.sort();
	let score = scores[scores.len() / 2];
	println!("{:?} {:?}", points, score);
}
