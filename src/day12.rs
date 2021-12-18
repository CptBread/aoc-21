use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::{HashMap};

struct Node(bool, Vec<usize>, String);

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day12.txt").unwrap();
	let mut it = BufReader::new(f).lines().map(Result::unwrap);
	let mut ids = HashMap::new();
	let mut nodes = HashMap::new();
	let (start, _) = parse_name("start", &mut ids);
	let (end, _) = parse_name("end", &mut ids);
	while let Some(line) = it.next() {
		let (n0, n1) = line.split_once('-').unwrap();
		let (id0, l0) = parse_name(n0, &mut ids);
		let (id1, l1) = parse_name(n1, &mut ids);

		nodes.entry(id0).or_insert(Node(l0, Vec::new(), n0.to_string())).1.push(id1);
		nodes.entry(id1).or_insert(Node(l1, Vec::new(), n1.to_string())).1.push(id0);
	}
	let mut count = 0;
	walk(start, end, &mut Vec::new(), &nodes, &mut |_path| {
		count += 1;
		// print_path(_path, &nodes);
	});
	let mut count2 = 0;
	walk2(start, false, start, end, &mut Vec::new(), &nodes, &mut |_path| {
		count2 += 1;
		// print_path(_path, &nodes);
	});

	// println!("{:?} {:?}", ids, nodes);
	println!("{} {}", count, count2);
}

fn print_path(path: &Vec<usize>, nodes:  &HashMap<usize, Node>) {
	for n in path.iter() {
		print!("{}->", nodes.get(n).unwrap().2);
	}
	println!("end");
}

fn walk<F>(id: usize, end: usize, path: &mut Vec<usize>, nodes: &HashMap<usize, Node>, func: &mut F) 
	where F: FnMut(&Vec<usize>)
{
	if id == end {
		func(path);
		return;
	}
	let node = nodes.get(&id).unwrap();
	if !node.0 && path.contains(&id) {
		return;
	}
	path.push(id);
	for n in node.1.iter() {
		walk(*n, end, path, nodes, func);
	}
	path.pop();
}

fn walk2<F>(id: usize, mut used_twice: bool, start: usize, end: usize, path: &mut Vec<usize>, nodes: &HashMap<usize, Node>, func: &mut F) 
	where F: FnMut(&Vec<usize>)
{
	if id == end {
		func(path);
		return;
	}
	if id == start && path.len() > 0 {
		return;
	}
	let node = nodes.get(&id).unwrap();
	if !node.0 && path.contains(&id) {
		if used_twice {
			return;
		}
		used_twice = true;
	}

	path.push(id);
	for n in node.1.iter() {
		walk2(*n, used_twice, start, end, path, nodes, func);
	}
	path.pop();
}

fn parse_name(s: &str, ids: &mut HashMap<String, (usize, bool)>) -> (usize, bool) {
	let len = ids.len();
	*ids.entry(s.to_string()).or_insert_with(||{
		(len, s.chars().next().unwrap().is_ascii_uppercase())
	})
}