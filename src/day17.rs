use std::ops::RangeInclusive;
use vek::vec::repr_c::{Vec2};

struct State {
	pos: Vec2<i32>,
	vel: Vec2<i32>,
}

pub fn solve() {
	// let input = "target area: x=20..30, y=-10..-5";
	let input = "target area: x=206..250, y=-105..-57";
	let rest = input.strip_prefix("target area: x=").unwrap();
	let (sx, sy) = rest.split_once(", y=").unwrap();
	let rx = parse_range(sx).unwrap();
	let ry = parse_range(sy).unwrap();

	let steps = ry.start().abs() * 2;
	let vy = ry.start().abs() - 1;
	let high = triang_num(vy);
	// let mut vx = 0;
	for v in 0.. {
		if rx.contains(&x_at_steps(v, steps)) {
			// vx = v;
			break;
		}
	}
	// println!("v: {:?}, steps: {}, high: {}", (vx, vy), steps, high);

	let mut count = 0;
	let max_vy = vy;
	let min_vy = *ry.start();
	for vx in 1..=*rx.end() {
		for vy in min_vy..=max_vy {
			if will_reach_target(vx, vy, &rx, &ry) {
				count += 1;
			}
		}
	}

	println!("{} {}", high, count);
	// println!("{:?} {:?}", rx, ry);
}

fn x_at_steps(vx: i32, steps: i32) -> i32 {
	if vx > steps {
		triang_num(steps) + 2 * (vx - steps)
	}
	else {
		triang_num(vx)
	}
}

fn triang_num(num: i32) -> i32 {
	((num + 1) * num) / 2
}

fn will_reach_target(vx: i32, vy: i32, rx: &RangeInclusive<i32>, ry: &RangeInclusive<i32>) -> bool {
	let mut state = State{
		pos: Vec2::zero(),
		vel: Vec2::new(vx, vy),
	};
	loop {
		match step(&mut state, rx, ry) {
			(true, _) => return true,
			(_, true) => return false,
			_ => {},
		};
	} 
}

fn step(state: &mut State, rx: &RangeInclusive<i32>, ry: &RangeInclusive<i32>) -> (bool, bool) {
	state.pos += state.vel;
	state.vel.y -= 1;
	state.vel.x -= state.vel.x.signum();
	let target = rx.contains(&state.pos.x) && ry.contains(&state.pos.y);
	let fail = state.pos.y < *ry.start() || state.pos.x > *rx.end();
	(target, fail)
}

fn parse_range(s: &str) -> Option<RangeInclusive<i32>> {
	let (s0, s1) = s.split_once("..")?;
	Some(s0.parse().ok()?..=(s1.parse().ok()?))
}