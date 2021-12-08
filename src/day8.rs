use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::{HashSet, HashMap};

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day8.txt").unwrap();
	let mut it = BufReader::new(f).lines().map(Result::unwrap);
	let mut p1 = 0;
	let mut p2 = 0;
	while let Some(line) = it.next() {
		let (variants, code) = line.split_once(" | ").unwrap();
		p1 += code.split(' ').fold(0, |acc, s| {
			match s.len() {
				2 | 3 | 4 | 7 => acc + 1,
				_ => acc,
			}
		});
		let mut known = HashMap::new();
		let unkown: Vec<_> = variants.split(' ').map(|s| s.chars().collect::<HashSet<_>>()).filter(|s| {
			match s.len() {
				2 => { known.insert(1, s.clone()); false },
				3 => { known.insert(7, s.clone()); false },
				4 => { known.insert(4, s.clone()); false },
				7 => { known.insert(8, s.clone()); false },
				_ => true,
			}
		}).collect();
		let pat4 = known.get(&4).unwrap().clone();
		let pat1 = known.get(&1).unwrap().clone();
		for s in unkown.into_iter() {
			if s.is_superset(&pat4) {
				known.insert(9, s.clone());
			}
			else if s.len() == 6 && s.union(&pat1).count() == 7 {
				known.insert(6, s.clone());
			}
			else if s.len() == 5 && s.union(&pat4).count() == 7 {
				known.insert(2, s.clone());
			}
			else if s.len() == 5 && s.is_superset(&pat1) {
				known.insert(3, s.clone());
			}
			else if s.is_superset(&pat1) {
				known.insert(0, s.clone());
			}
			else if s.is_superset(&pat1) {
				known.insert(0, s.clone());
			}
			else {
				known.insert(5, s.clone()); 
			}
		}
		assert!(known.len() == 10);
		let num = code.split(' ').fold(0, |acc, s| {
			10 * acc + find_key_for_value(&known, &s.chars().collect()).unwrap()
		});
		p2 += num;
		// println!("{}", num);
	}

	println!("{} {}", p1, p2);
}

fn find_key_for_value<'a, V: Eq, K>(map: &'a HashMap<K, V>, value: &V) -> Option<&'a K> {
	map.iter().find_map(|(key, val)| if *val == *value { Some(key) } else { None })
}