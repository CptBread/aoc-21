use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Reverse;
use sorted_bread_box::*;
use crate::utils::*;

type RevEntry<K, V> = SortedEntry<Reverse<K>, V>;

fn rev_entry<K, V>(k: K, v: V) -> RevEntry<K, V> {
	SortedEntry(Reverse(k), v)
}

#[allow(dead_code)]
pub fn solve() {
	let map = Array2D::load_file("data/day15.txt", |c| c.to_digit(10).unwrap());
	let mut best = Array2D::from_vec(map.width, vec![None; map.data.len()]);
	let end = map.idx_to_pos(map.data.len() - 1);
	let mut res = None;
	let mut to_check = BinaryHeap::new();
	to_check.push(rev_entry(0, (Pos::new(0, 0), None)));
	while let Some(p) = to_check.pop() {
		let at = p.1.0;
		let node = best.get_mut(at).unwrap();
		if node.is_some() {
			continue;
		}
		let cost = p.0.0;
		let from = p.1.1;
		*node = Some((cost, from));
		// println!("{} = {} from {:?}", at, cost, from);
		if at == end {
			res = Some(cost);
			break;
		}
		for p in map.neighbours(at).iter().filter_map(|p| *p) {
			let guess_cost = cost + map.get(p).unwrap();
			to_check.push(rev_entry(guess_cost, (p, Some(at))));
		}
	}
	let part1 = res;

	let mut bigger = Array2D::from_vec(map.width * 5, vec![0; map.data.len() * 5 * 5]);
	bigger.for_each_mut(|p, val| {
		let (wrap, div) = wrap_pos(p, map.width, map.height);
		let org = map.get(wrap).unwrap();
		*val = (org + div.x as u32 + div.y as u32 - 1) % 9 + 1;
	});
	// bigger.print(|v| char::from_digit(*v, 10).unwrap());
	let map = bigger;
	let mut best = Array2D::from_vec(map.width, vec![None; map.data.len()]);
	let end = map.idx_to_pos(map.data.len() - 1);
	let mut res = None;
	let mut to_check = BinaryHeap::new();
	to_check.push(rev_entry(0, (Pos::new(0, 0), None)));
	while let Some(p) = to_check.pop() {
		let at = p.1.0;
		let node = best.get_mut(at).unwrap();
		if node.is_some() {
			continue;
		}
		let cost = p.0.0;
		let from = p.1.1;
		*node = Some((cost, from));
		// println!("{} = {} from {:?}", at, cost, from);
		if at == end {
			res = Some(cost);
			break;
		}
		for p in map.neighbours(at).iter().filter_map(|p| *p) {
			let guess_cost = cost + map.get(p).unwrap();
			to_check.push(rev_entry(guess_cost, (p, Some(at))));
		}
	}

	// let mut backtrack = Some(end);
	// let mut path = HashSet::new();
	// while let Some(back) = backtrack {
	// 	path.insert(back);
	// 	print!("{} ", back);
	// 	backtrack = best.get(back).unwrap().unwrap().1;
	// }
	// println!("");

	// for y in 0..map.height {
	// 	for x in 0..map.width {
	// 		print!("{}", if path.contains(&Pos::new(x, y)) {'X'} else { char::from_digit(*map.get(Pos::new(x, y)).unwrap(), 10).unwrap() });
	// 	}
	// 	print!("\n");
	// }

	assert_eq!(part1, Some(508));
	assert_eq!(res, Some(2872));
	println!("{:?} {:?}", part1, res);
}

fn wrap_pos(p: Pos, w: usize, h: usize) -> (Pos, Pos) {
	(p % Pos::new(w, h), p / Pos::new(w, h))
}