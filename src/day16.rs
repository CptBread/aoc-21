use std::fs::File;
use std::io::{
	BufReader,
	prelude::*,
};
use std::collections::{VecDeque, HashSet, HashMap};
use vek::vec::repr_c::{Vec2};
use intbits::Bits;

#[derive(Default, Clone)]
pub struct BitReader {
	cur: u64,
	len: u8,
	pub buff: VecDeque<u8>,
}

impl BitReader {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn read(&mut self, bits: u8) -> Option<u64> {
		assert!(bits <= 32);
		while self.len < bits {
			self.take_nib()?;
		}
		let end = self.len;
		let start = self.len - bits;
		let res = self.cur.bits(start..end);
		self.len = start;
		Some(res as u64)
	}

	fn clear_cur(&mut self) {
		self.cur = 0;
		self.len = 0;
	}

	fn take_nib(&mut self) -> Option<()> {
		self.len += 4;
		let v = self.buff.pop_front()?;
		assert!(v <= 0b1111);
		self.cur = (self.cur << 4) + v as u64;
		Some(())
	}
}

#[allow(dead_code)]
pub fn solve() {
	let mut f = File::open("data/day16.txt").unwrap();
	let mut data = String::new();
	f.read_to_string(&mut data).ok();

	let mut bits = BitReader::new();
	bits.buff = data.chars().map(|c| c.to_digit(16).unwrap() as u8).collect();

	let mut versum = 0;
	let (_, val) = read_packet(&mut bits, &mut |ver, typ| versum += ver);
	println!("{} {}", versum, val);
}

pub fn read_packet<F>(bits: &mut BitReader, pack_func: &mut F) -> (u64, u64) 
	where F: FnMut(u64, u64)
{
	let ver = bits.read(3).unwrap();
	let typ = bits.read(3).unwrap();
	pack_func(ver, typ);
	if typ == 4 {
		let (read, value) = read_val(bits, pack_func);
		// println!("VAL {}", value);
		return (read + 6, value);
	}
	else {
		let (read, value) = read_op(typ, bits, pack_func);
		return (read + 6, value);
	}
}

pub fn read_op<F>(op: u64, bits: &mut BitReader, pack_func: &mut F) -> (u64, u64) 
	where F: FnMut(u64, u64)
{
	// println!("OP {}", op);
	let l_mode = bits.read(1).unwrap();
	let mut read = 1;
	let mut vals = Vec::new();
	if l_mode == 1 {
		let len = bits.read(11).unwrap();
		read += 11;
		for _ in 0..len {
			let (rb, value) = read_packet(bits, pack_func);
			vals.push(value);
			read += rb;
		}
	}
	else {
		let mut bits_left = bits.read(15).unwrap();
		read += 15;
		while bits_left > 0 {
			let (rb, value) = read_packet(bits, pack_func);
			vals.push(value);
			read += rb;
			bits_left -= rb;
		}
	}
	(read, match op {
		0 => vals.iter().sum(),
		1 => vals.iter().product(),
		2 => vals.iter().min().copied().unwrap_or(0),
		3 => vals.iter().max().copied().unwrap_or(0),
		5 => (vals[0] > vals[1]) as u64, // greater than packets - their value is 1 if the value of the first sub-packet is greater than the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
		6 => (vals[0] < vals[1]) as u64, // less than packets - their value is 1 if the value of the first sub-packet is less than the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
		7 => (vals[0] == vals[1]) as u64, // equal to packets - their value is 1 if the value of the first sub-packet is equal to the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
		_ => panic!(),
	})
	// println!("OP END");
	// bits.clear_cur();
}

pub fn read_val<F>(bits: &mut BitReader, pack_func: &mut F) -> (u64, u64)
	where F: FnMut(u64, u64)
{
	let mut value = 0;
	let mut read = 0;
	loop {
		let cont = bits.read(1).unwrap();
		let part = bits.read(4).unwrap();
		read += 5;
		value = (value << 4) + part;
		if cont == 0 { break; }
	}
	(read, value)
}

#[cfg(test)]
mod tests {
	use crate::day16::*;
	#[test]
	fn test_wrapping() {
		let mut bits = BitReader::new();
		bits.buff.push_back(0b1101);
		bits.buff.push_back(0b1001);
		bits.buff.push_back(0b0111);
		bits.buff.push_back(0b0000);
		assert_eq!(bits.read(4), Some(0b1101));
		assert_eq!(bits.read(3), Some(0b100));
		assert_eq!(bits.read(3), Some(0b101));
		assert_eq!(bits.read(3), Some(0b110));

		let mut bits = BitReader::new();
		bits.buff.push_back(0b1101);
		bits.buff.push_back(0b1001);
		bits.buff.push_back(0b0111);
		bits.buff.push_back(0b0000);
		assert_eq!(bits.read(11), Some(0b11011001011));
	}
}