use std::collections::{HashMap};
use crate::utils::*;

#[allow(dead_code)]
pub fn solve() {
	let map = Array2D::load_file("data/day9.txt", |c| c.to_digit(10).unwrap());
	let mut lows = 0;
	let mut marked = HashMap::new();
	let mut basins = Vec::new();
	for y in 0..map.height {
		'xloop: for x in 0..map.width {
			let at = Pos::new(x, y);
			let v = *map.get(at).unwrap();
			for p in map.neighbours(at).iter().filter_map(|p| *p) {
				if *map.get(p).unwrap() <= v {
					continue 'xloop;
				}
			}
			lows += v + 1;

			let mut to_check = vec![at];
			let mut basin = 0;
			while let Some(p) = to_check.pop() {
				basin += 1;
				for p in map.neighbours(p).iter().filter_map(|p| *p) {
					let v2 = *map.get(p).unwrap();
					if v2 != 9 && v2 > v {
						marked.entry(p).or_insert_with(|| {
							to_check.push(p);
							true
						});
					}
				}
			}
			basins.push(basin);
		}
	}
	basins.sort();
	let biggest: usize = basins.iter().rev().take(3).product();
	println!("{} {}", lows, biggest);
}