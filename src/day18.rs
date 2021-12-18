use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cell::Cell;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Data {
	Num(u32),
	Node(usize),
}

impl Data {
	fn unwrap_num(&self) -> u32 {
		match self {
			Data::Num(n) => *n,
			_ => panic!(),
		}
	}

	fn unwrap_node(&self) -> usize {
		match self {
			Data::Node(n) => *n,
			_ => panic!(),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
	Left,
	Right,
}

#[derive(Clone, Debug)]
struct Node {
	parent: Option<usize>,
	data: Cell<(Data, Data)>,
	alive: Cell<bool>,
}

#[derive(Default, Clone, Debug)]
struct SnailNum {
	root: Option<usize>,
	nodes: Vec<Node>,
}


impl Node {
	fn new(parent: Option<usize>, data: (Data, Data)) -> Self {
		Node {
			parent,
			data: Cell::new(data),
			alive: Cell::new(true),
		}
	}

	fn get_leaf(&self) -> Option<(u32, u32)> {
		if let (Data::Num(l), Data::Num(r)) = self.data.get() {
			Some((l, r))
		}
		else {
			None
		}
	}

	fn is_leaf(&self) -> bool {
		if let (Data::Num(_), Data::Num(_)) = self.data.get() {
			true
		}
		else {
			false
		}
	}

	fn update_side<F: FnOnce(Data) -> Data>(&self, side: Side, f: F) {
		let mut d = self.data.get();
		match side {
			Side::Left => d.0 = f(d.0),
			Side::Right => d.1 = f(d.1),
		};
		self.data.set(d);
	}

	fn left(&self) -> Data {
		self.data.get().0
	}

	fn right(&self) -> Data {
		self.data.get().1
	}

	fn left_mut(&mut self) -> &mut Data {
		&mut self.data.get_mut().0
	}

	fn right_mut(&mut self) -> &mut Data {
		&mut self.data.get_mut().1
	}

	fn left_node(&self) -> Option<usize> {
		match self.data.get().0 {
			Data::Node(n) => Some(n),
			_ => None,
		}
	}

	fn right_node(&self) -> Option<usize> {
		match self.data.get().1 {
			Data::Node(n) => Some(n),
			_ => None,
		}
	}
}

enum WalkCommand {
	Walk, // Continue
	Back, // Do not go further down
	End, // Fully end the walk
}

impl SnailNum {
	fn parse(s: &str) -> Option<Self> {
		// let mut root
		let mut nodes = Vec::new();
		let mut stack = Vec::new();
		for c in s.chars() {
			match c {
				']' => {
					let r = stack.pop().unwrap();
					let l = stack.pop().unwrap();
					let id = nodes.len();
					nodes.push(Node::new(None, (l, r)));
					if let Data::Node(n) = l {
						nodes[n].parent = Some(id);
					}
					if let Data::Node(n) = r {
						nodes[n].parent = Some(id);
					}
					stack.push(Data::Node(id))
				}
				'0'..='9' => {
					let v = Data::Num(c.to_digit(10).unwrap());
					stack.push(v);
				},
				',' | '[' => {},
				_ => panic!(""),
			}
		}
		Some(SnailNum{
			nodes,
			root: stack.pop().and_then(|v| if let Data::Node(n) = v {Some(n)} else {None}),
		})
	}

	fn left_of(&self, id: usize) -> Option<(usize, Side)> {
		let mut last = id;
		let mut next = self.nodes[id].parent;
		while let Some(cur) = next {
			let node = &self.nodes[cur];
			match node.left() {
				Data::Node(n) => {
					if n == last {
						next = node.parent;
						last = cur;
					}
					else {
						return Some((self.right_leaf(n), Side::Right));
					}
				}
				Data::Num(_) => return Some((cur, Side::Left)),
			}
		}
		None
	}

	fn right_of(&self, id: usize) -> Option<(usize, Side)> {
		let mut last = id;
		let mut next = self.nodes[id].parent;
		while let Some(cur) = next {
			let node = &self.nodes[cur];
			match node.right() {
				Data::Node(n) => {
					if n == last {
						next = node.parent;
						last = cur;
					}
					else {
						return Some((self.left_leaf(n), Side::Left));
					}
				}
				Data::Num(_) => return Some((cur, Side::Right)),
			}
		}
		None
	}

	fn left_leaf(&self, mut id: usize) -> usize {
		while let Some(n) = self.nodes[id].left_node() {
			id = n;
		}
		id
	}

	fn right_leaf(&self, mut id: usize) -> usize {
		while let Some(n) = self.nodes[id].right_node() {
			id = n;
		}
		id
	}

	fn replace(&mut self, id: usize, data: Data) {
		// Mark dead
		self.walk_tree_from(id, 0, &mut |_, node, _| {
			node.alive.set(false);
			WalkCommand::Walk
		});
		let parent = self.nodes[id].parent.unwrap();
		let node = &mut self.nodes[parent];
		if Data::Node(id) == node.left() {
			*node.left_mut() = data;
		}
		else if Data::Node(id) == node.right() {
			*node.right_mut() = data;
		}
	}

	fn reduce_step(&mut self) -> bool {
		if !self.explode_step() {
			self.split_step()
		}
		else {
			true
		}
	}

	fn explode_step(&mut self) -> bool {
		let mut found = None;
		self.walk_tree(|id, node, depth| {
			// println!("{} {} {:?}", id, depth, node);
			if depth >= 4 && node.is_leaf() {
				found = Some(id);
				return WalkCommand::End;
			}
			WalkCommand::Walk
		});
		if let Some(found) = found {
			let (l, r) = self.nodes[found].get_leaf().unwrap();
			if let Some((id, side)) = self.left_of(found) {
				self.nodes[id].update_side(side, |d| {
					Data::Num(d.unwrap_num() + l)
				});
			}
			if let Some((id, side)) = self.right_of(found) {
				self.nodes[id].update_side(side, |d| {
					Data::Num(d.unwrap_num() + r)
				});
			}
			self.replace(found, Data::Num(0));
			true
		}
		else {
			false
		}
	}

	fn split_step(&mut self) -> bool {
		let mut found = None;
		self.walk_leafs(|id, num, _| {
			if num >= 10 {
				found = Some((id, num));
				false
			}
			else {
				true
			}
		});
		if let Some(((id, side), val)) = found {
			let r = Data::Num((val + 1) / 2);
			let l = Data::Num(val / 2);
			let new_id = self.nodes.len();
			self.nodes.push(Node::new(Some(id), (l, r)));
			self.nodes[id].update_side(side, |_| Data::Node(new_id));
			true
		}
		else {
			false
		}
	}

	// id, node ref, depth
	fn walk_tree<F>(&self, mut func: F) where F: FnMut(usize, &Node, usize) -> WalkCommand {
		self.walk_tree_from(self.root.unwrap(), 0, &mut func);
	}

	fn walk_tree_from<F>(&self, id: usize, depth: usize, func: &mut F) -> bool
		where F: FnMut(usize, &Node, usize) -> WalkCommand 
	{
		let node = &self.nodes[id];
		match func(id, &node, depth) {
			WalkCommand::Walk => {},
			WalkCommand::Back => return true,
			WalkCommand::End => return false,
		}
		if let Some(l) = node.left_node() {
			if !self.walk_tree_from(l, depth + 1, func) {
				return false;
			}
		}
		if let Some(r) = node.right_node() {
			if !self.walk_tree_from(r, depth + 1, func) {
				return false;
			}
		}
		true
	}

	fn walk_leafs<F>(&self, mut func: F) where F: FnMut((usize, Side), u32, usize) -> bool {
		self.walk_leafs_from(self.root.unwrap(), 0, &mut func);
	}

	fn walk_leafs_from<F>(&self, id: usize, depth: usize, func: &mut F) -> bool
		where F: FnMut((usize, Side), u32, usize) -> bool 
	{
		let node = &self.nodes[id];
		if match node.left() {
			Data::Node(l) => self.walk_leafs_from(l, depth + 1, func),
			Data::Num(n) => func((id, Side::Left), n, depth),
		} {
			match node.right() {
				Data::Node(l) => self.walk_leafs_from(l, depth + 1, func),
				Data::Num(n) => func((id, Side::Right), n, depth),
			} 
		}
		else {
			false
		}
	}

	fn to_string(&self) -> String {
		let mut s = "[".to_string();
		let id = self.root.unwrap();

		let node = &self.nodes[id];
		self.build_string(node.data.get().0, &mut s);
		s.push(',');
		self.build_string(node.data.get().1, &mut s);
		s.push(']');
		s
	}

	fn build_string(&self, data: Data, s: &mut String) {
		match data {
			Data::Node(id) => {
				s.push('[');
				let node = &self.nodes[id];
				self.build_string(node.data.get().0, s);
				s.push(',');
				self.build_string(node.data.get().1, s);
				s.push(']');
			},
			Data::Num(num) => {
				s.push_str(&num.to_string());
			}
		}
	}

	fn magnitude(&self) -> u32 {
		self.magnitude_rec(Data::Node(self.root.unwrap()))
	}

	fn magnitude_rec(&self, data: Data) -> u32 {
		match data {
			Data::Node(id) => {
				let node = &self.nodes[id];
				3 * self.magnitude_rec(node.left()) + 2 * self.magnitude_rec(node.right())
			},
			Data::Num(num) => {
				num
			}
		}
	}

	fn append(&mut self, other: &Self) {
		*self = self.add(other);
	}

	fn add(&self, other: &Self) -> Self {
		// lazy way
		let mut s = self.to_string();
		s.insert(0, '[');
		s.push(',');
		s.push_str(&other.to_string());
		s.push(']');
		let mut res = Self::parse(&s).unwrap();
		res.reduce();
		res
	}

	fn reduce(&mut self) {
		// println!("{}", self.to_string());
		while self.reduce_step() {
			// println!("{}", self.to_string());
		}
	}
}

pub fn solve() {
	let f = File::open("data/day18.txt").unwrap();
	let mut it = BufReader::new(f).lines().map(Result::unwrap);
	let mut curr = SnailNum::parse(&it.next().unwrap()).unwrap();
	let mut nums = vec![curr.clone()];
	// println!("{}", curr.to_string());
	while let Some(l) = it.next() {
		let num = SnailNum::parse(&l).unwrap();
		curr.append(&num);
		nums.push(num);
	}
	// println!("{}", curr.to_string());
	println!("{}", curr.magnitude());
	let mut max = 0;
	for (l, r) in nums.iter().tuple_combinations() {
		let n0 = l.add(r).magnitude();
		let n1 = r.add(l).magnitude();
		max = max.max(n0).max(n1);
	}
	println!("{}", max);

	// test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
	// test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
	// test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
	// test("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
	// test("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

	// let mut num = SnailNum::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
	// num.reduce_step();
	// assert_eq!(&num.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
	// num.reduce_step();
	// assert_eq!(&num.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
	// num.reduce_step();
	// assert_eq!(&num.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
	// num.reduce_step();
	// assert_eq!(&num.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
	// num.reduce_step();
	// assert_eq!(&num.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

// fn test(s: &str, exp: &str) {
// 	let mut num = SnailNum::parse(s).unwrap();
// 	num.reduce_step();
// 	assert_eq!(&num.to_string(), exp);
// }