use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use retain_mut::RetainMut;
use crate::utils::*;

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day4.txt").unwrap();
	let mut it = BufReader::new(f).lines().map(Result::unwrap).peekable();
	let nums: Vec<_> = it.next().unwrap().split(',').map(|s| {
		s.parse::<u8>().unwrap()
	}).collect();

	it.next();
	let mut boards = Vec::new();
	while it.peek().is_some() {
		let board = Array2D::load_split_lines_while(&mut it, ' ', |s| (s.parse::<u8>().unwrap(), false), |s| s.len() > 0);
		boards.push(board);
	}

	let mut best_score = None;
	let mut worst_score = 0;
	for num in nums.iter().copied() {
		if boards.len() == 1 {
			if mark_check(&mut boards[0], num) {
				worst_score = boards[0].data.iter().cloned().fold(0u32, |acc, (n, checked)| if checked {acc} else {acc + n as u32}) * num as u32;
				break;
			}
		}
		else {
			boards.retain_mut(|board| {
				if mark_check(board, num) {
					if best_score.is_none() {
						best_score = Some(board.data.iter().cloned().fold(0u32, |acc, (n, checked)| if checked {acc} else {acc + n as u32}) * num as u32);
					}
					false
				}
				else {
					true
				}
			});
		}
	}

	println!("{} {}", best_score.unwrap(), worst_score);
}

fn mark_check(board: &mut Array2D<(u8, bool)>, num: u8) -> bool {
	if mark(board, num) {
		check(board)
	}
	else {
		false
	}
}

fn check(board: &Array2D<(u8, bool)>) -> bool {
	for x in 0..board.width {
		let mut ok = true;
		for y in 0..board.height {
			if !board.get(Pos::new(x, y)).unwrap().1 {
				ok = false;
				break;
			}
		}
		if ok {
			return true;
		}
	}

	for y in 0..board.height {
		let mut ok = true;
		for x in 0..board.width {
			if !board.get(Pos::new(x, y)).unwrap().1 {
				ok = false;
				break;
			}
		}
		if ok {
			return true;
		}
	}
	false
}

// returns if we marked a number
fn mark(board: &mut Array2D<(u8, bool)>, num: u8) -> bool {
	for (val, checked) in board.data.iter_mut() {
		if *val == num {
			let old = !*checked;
			*checked = true;
			return old;
		}
	}
	false
}