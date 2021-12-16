use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::{HashSet, HashMap};
use vek::vec::repr_c::{Vec2};

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day13.txt").unwrap();
	let mut it = BufReader::new(f).lines().map(Result::unwrap);
	let mut map = HashSet::new();
	let mut first = None;
	while let Some(line) = it.next() {
		if line == "" {
			break;
		}
		let (x, y) = line.split_once(',').unwrap();
		map.insert(Vec2::new(x, y).map(|s| s.parse::<i32>().unwrap()));
	}

	while let Some(line) = it.next() {
		// print_map(&map);
		let mut to_add = Vec::new();
		if let Some(Some(y)) = line.strip_prefix("fold along y=").map(|v| v.parse::<i32>().ok()) {
			// println!("FOLD Y {}", y);
			map.retain(|p| {
				if p.y > y {
					to_add.push(Vec2::new(p.x, y * 2 - p.y));
					false
				}
				else if p.y == y {
					false
				}
				else {
					true
				}
			});
			map.extend(to_add.iter());
		}
		else if let Some(Some(x)) = line.strip_prefix("fold along x=").map(|v| v.parse::<i32>().ok()) {
			map.retain(|p| {
				if p.x > x {
					to_add.push(Vec2::new(x * 2 - p.x, p.y));
					false
				}
				else if p.x == x {
					false
				}
				else {
					true
				}
			});
			map.extend(to_add.iter());
		}
		if first.is_none() {
			first = Some(map.len());
		}
	}
	print_map(&map);

	println!("{} {}", first.unwrap(), map.len());
}

fn print_map(map : &HashSet<Vec2<i32>>) {
	print!("\n");
	let max = map.iter().fold(Vec2::broadcast(0), |acc, p| Vec2::new(acc.x.max(p.x), acc.y.max(p.y)));
	for y in 0..=max.y {
		for x in 0..=max.x {
			print!("{}", if map.contains(&Vec2::new(x, y)) {'#'} else {'.'});
		}
		print!("\n");
	}
}