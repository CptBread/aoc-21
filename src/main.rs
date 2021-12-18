mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
mod utils;

use std::env;

fn main() {
	let solvers: Vec<fn()> = vec![
		day1::solve,
		day2::solve,
		day3::solve,
		day4::solve,
		day5::solve,
		day6::solve,
		day7::solve,
		day8::solve,
		day9::solve,
		day10::solve,
		day11::solve,
		day12::solve,
		day13::solve,
		day14::solve,
		day15::solve,
		day16::solve,
		day17::solve,
		day18::solve,
		// day19::solve,
		// day20::solve,
		// day21::solve,
		// day22::solve,
		// day23::solve,
		// day24::solve,
		// day25::solve,
	];
	let day = solvers.len();
	let cmd = env::args().skip(1).next();
	let (all, day) = cmd.map_or((false, day), |s| (s == "all", s.parse::<usize>().unwrap_or(day)));
	if all {
		for (idx, f) in solvers.iter().enumerate() {
			println!("Day {}:", idx + 1);
			f();
		}
	}
	else {
		println!("Day {}:", day);
		solvers.get(day - 1).expect("Invalid day!")();
	}
}
