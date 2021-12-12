use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::{HashSet, HashMap};
use crate::utils::*;

#[allow(dead_code)]
pub fn solve() {
	let mut map = Array2D::load_file("data/day11.txt", |c| c.to_digit(10).unwrap());
	let mut flashes = 0;
	let mut synced = None;
	for s in 0..100 {
		let mut flash = Vec::new();
		let mut flashed = HashSet::new();
		for idx in 0..map.data.len() {
			let v = map.data.get_mut(idx).unwrap();
			*v += 1;
			if *v > 9 {
				let pos = map.idx_to_pos(idx);
				flash.push(pos);
				flashed.insert(pos);
			}
		}
		while let Some(at) = flash.pop() {
			flashes += 1;
			for pos in map.neighbours_diag(at).iter().filter_map(|v| *v) {
				let v = map.get_mut(pos).unwrap();
				*v += 1;
				if *v > 9 && flashed.insert(pos) {
					flash.push(pos);
				}
			}
		}
		if synced.is_none() && flashed.len() == map.data.len() {
			synced = Some(s);
		}
		for pos in flashed.iter() {
			*map.get_mut(*pos).unwrap() = 0;
		}
	}
	
	'inf: for s in 100.. {
		let mut flash = Vec::new();
		let mut flashed = HashSet::new();
		for idx in 0..map.data.len() {
			let v = map.data.get_mut(idx).unwrap();
			*v += 1;
			if *v > 9 {
				let pos = map.idx_to_pos(idx);
				flash.push(pos);
				flashed.insert(pos);
			}
		}
		while let Some(at) = flash.pop() {
			flashes += 1;
			for pos in map.neighbours_diag(at).iter().filter_map(|v| *v) {
				let v = map.get_mut(pos).unwrap();
				*v += 1;
				if *v > 9 && flashed.insert(pos) {
					flash.push(pos);
				}
			}
		}
		if synced.is_none() && flashed.len() == map.data.len() {
			synced = Some(s);
			break 'inf;
		}
		for pos in flashed.iter() {
			*map.get_mut(*pos).unwrap() = 0;
		}
	}
	// map.print(|v| char::from_digit(*v, 10).unwrap());
	
	println!("{} {}", flashes, synced.unwrap_or(0) + 1);
}