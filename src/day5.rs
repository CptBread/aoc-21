use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::HashMap;
use retain_mut::RetainMut;
use vek::vec::repr_c::{Vec2};

type Pos = Vec2<i32>;

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day5.txt").unwrap();
	let mut it = BufReader::new(f).lines().map(Result::unwrap);
	let mut map = HashMap::new();
	let mut map_diags = HashMap::new();
	let mut overlaps = 0;
	let mut overlaps_diags = 0;
	// let mut max = Vec2::<i32>::zero();
	while let Some(line) = it.next() {
		let (x0, rest) = line.split_once(',').unwrap();
		let (y0, rest) = rest.split_once(" -> ").unwrap();
		let (x1, y1) = rest.split_once(",").unwrap();
		let start = Pos::new(x0.parse().unwrap(), y0.parse().unwrap());
		let end = Pos::new(x1.parse().unwrap(), y1.parse().unwrap());
		// max = Vec2::partial_max(max, end);
		// max = Vec2::partial_max(max, start);
		let step = (end - start).map(|v| v.signum());
		let diag = (step.x != 0) == (step.y != 0);
		let mut at = start;
		loop {
			if !diag {
				let v = map.entry(at).or_insert(0);
				*v += 1;
				if *v == 2 {
					overlaps += 1;
				}
			}
			let v = map_diags.entry(at).or_insert(0);
			*v += 1;
			if *v == 2 {
				overlaps_diags += 1;
			}
			
			if at == end {
				break;
			}
			at += step;
		}
	}
	// for x in 0..=max.x {
	// 	for y in 0..=max.y {
	// 		let at = Vec2::new(x, y);
	// 		print!("{}", map.get(&at).map_or('.', |v| char::from_digit(*v, 10).unwrap()));
	// 	}
	// 	println!("");
	// }
	println!("{} {}", overlaps, overlaps_diags);
}
