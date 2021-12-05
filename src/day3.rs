use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use intbits::Bits;

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day3.txt").unwrap();
	let mut it = BufReader::new(f).lines();
	let mut count = [0i32; u16::BITS as usize];
	let mut bits = 0;
	let mut nums = 0;
	let mut vec = Vec::new();
	while let Some(Ok(line)) = it.next() {
		let num = u16::from_str_radix(&line, 2).unwrap();
		vec.push(num);
		nums += 1;
		for idx in 0..u16::BITS as usize {
			if num.bit(idx) && idx > bits {
				bits = idx;
			}
			count[idx] += num.bit(idx) as i32;
		}
	}
	bits += 1;
	let half = nums / 2;
	let mut gamma = 0;
	let mut epsi = 0;
	
	for idx in (0..bits).rev() {
		let common = count[idx] > half;
		gamma.set_bit(idx, common);
		epsi.set_bit(idx, !common);
		
	}

	let mut ox_vec = vec.clone();
	for idx in (0..bits).rev() {
		let mut count = 0;
		for n in ox_vec.iter() {
			count += n.bit(idx) as i32;
		}
		let half = (ox_vec.len() as i32 + 1) / 2;
		let b = count >= half;
		ox_vec.retain(|n| n.bit(idx) == b);
		if ox_vec.len() <= 1 {
			println!("ox {} {} {}", idx, half, count);
			println!("{:?}", ox_vec.first());
			break;
		}
		// println!("{} {:?}", count, ox_vec);
	}
	
	let mut co_vec = vec;
	for idx in (0..bits).rev() {
		let mut count = 0;
		for n in co_vec.iter() {
			count += n.bit(idx) as i32;
		}
		let half = (co_vec.len() as i32 + 1) / 2;
		let b = count < half;
		co_vec.retain(|n| n.bit(idx) == b);
		if co_vec.len() <= 1 {
			println!("co {} {} {}", idx, half, count);
			println!("{:?}", co_vec.first());
			break;
		}
		// println!("{} {:?}", count, co_vec);
	}
	let ox = ox_vec[0];
	let co = co_vec[0];

	println!("{} {:b} {:b} {:?}", epsi * gamma, gamma, epsi, count);
	println!("{} {:b} {:b}", ox as u64 * co as u64, ox, co);
}
