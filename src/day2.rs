use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day2.txt").unwrap();
	let mut it = BufReader::new(f).lines();
	let mut x = 0;
	let mut y = 0;
	while let Some(Ok(line)) = it.next() {
		let (token, num) = &line.split_once(' ').unwrap();
		let num = num.parse::<i32>();
		match *token {
			"forward" => x += num.unwrap(),
			"up" => y -= num.unwrap(),
			"down" => y += num.unwrap(),
			_ => {}
		}
	}
	println!("{} {} {}", x * y, x, y);

	let f = File::open("data/day2.txt").unwrap();
	let mut it = BufReader::new(f).lines();
	let mut x = 0;
	let mut y = 0;
	let mut aim = 0;
	while let Some(Ok(line)) = it.next() {
		let (token, num) = &line.split_once(' ').unwrap();
		let num = num.parse::<i32>();
		match *token {
			"forward" => {
				let num = num.unwrap();
				x += num;
				y += num * aim;
			},
			"up" => aim -= num.unwrap(),
			"down" => aim += num.unwrap(),
			_ => {}
		}
	}
	println!("{} {} {}", x * y, x, y);
}