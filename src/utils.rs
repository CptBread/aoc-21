use std::io::{BufReader, prelude::*};
use std::fs::File;
use std::convert::TryInto;
use vek::vec::repr_c::{Vec2};

#[macro_export]
macro_rules! try_block {
	{ $($token:tt)* } => {{
		|| -> Option<()> {
			$($token)*
			Some(())
		}()
	}}
}

pub type Pos = Vec2<usize>;

// Maybe remove this struct and just have free loading functions...
#[derive(Clone, Debug, Default)]
pub struct Array2D<T> {
	pub data: Vec<T>,
	pub width: usize,
	pub height: usize,
}

#[allow(dead_code)]
impl<T> Array2D<T> {
	pub fn load_read<R, F>(read: R, f: F) -> Self
		where F: FnMut(char) -> T, R: Read,
	{
		Self::load_reader(&mut BufReader::new(read), f)
	}

	pub fn load_file<F>(path: &str, f: F) -> Self
		where F: FnMut(char) -> T
	{
		let file = File::open(path).unwrap();
		Self::load_reader(&mut BufReader::new(&file), f)
	}

	pub fn load_lines<F, I>(it: &mut I, mut f: F) -> Self
		where
			F: FnMut(char) -> T,
			I: Iterator<Item=String>,
	{
		let mut height = 0;
		let mut width = 0;
		let mut data = Vec::new();
		while let Some(l) = it.next() {
			let w = l.len();
			if width == 0 {
				width = w;
			}
			else if width != w {
				panic!("Inconsistent width! Assumed {} got {}", width, w);
			}
			height += 1;
			data.extend(l.chars().map(&mut f));
		}
		Array2D{
			data,
			width,
			height,
		}
	}

	pub fn from_vec(w: usize, data: Vec<T>) -> Self {
		let h = data.len() / w;
		assert_eq!(h * w, data.len());
		Self{
			data,
			width: w,
			height: h,
		}
	}

	// Will consume the first invalid item
	pub fn load_lines_while<F, F2, I>(it: &mut I, mut f: F, mut check: F2) -> Self
		where
			F: FnMut(char) -> T,
			F2: FnMut(&str) -> bool,
			I: Iterator<Item=String>,
	{
		let mut height = 0;
		let mut width = 0;
		let mut data = Vec::new();
		while let Some(l) = it.next() {
			if !check(&l) {
				break;
			}
			let w = l.len();
			if width == 0 {
				width = w;
			}
			else if width != w {
				panic!("Inconsistent width! Assumed {} got {}", width, w);
			}
			height += 1;
			data.extend(l.chars().map(&mut f));
		}
		Array2D{
			data,
			width,
			height,
		}
	}

	// Will consume the first invalid item
	pub fn load_split_lines_while<F, F2, I>(it: &mut I, split_char: char, mut f: F, mut check: F2) -> Self
	where
		F: FnMut(&str) -> T,
		F2: FnMut(&str) -> bool,
		I: Iterator<Item=String>,
	{
		let mut height = 0;
		let mut width = 0;
		let mut data = Vec::new();
		while let Some(l) = it.next() {
			if !check(&l) {
				break;
			}
			let mut it2 = l.split(split_char).filter(|s| s.len() > 0);
			let mut added = 0;
			while let Some(s) = it2.next() {
				data.push(f(s));
				added += 1;
			}

			let w = added;
			if width == 0 {
				width = w;
			}
			else if width != w {
				panic!("Inconsistent width! Assumed {} got {}", width, w);
			}
			height += 1;
		}
		Array2D{
			data,
			width,
			height,
		}
	}

	pub fn load_reader<F>(read: &mut dyn BufRead, f: F) -> Self
		where F: FnMut(char) -> T
	{
		let mut it = read.lines().map(Result::unwrap);
		Self::load_lines(&mut it, f)
	}

	pub fn to_tuple(self) -> (usize, usize, Vec<T>) {
		(self.width, self.height, self.data)
	}

	pub fn pos_to_idx_no_bounds(&self, pos: Pos) -> usize {
		pos.y * self.width + pos.x
	}

	pub fn pos_to_idx<P: TryInto<Pos>>(&self, pos: P) -> Option<usize> {
		let pos: Pos = pos.try_into().ok()?;
		if pos.x >= self.width {
			None
		} else if pos.y >= self.height {
			None
		} else {
			Some(self.pos_to_idx_no_bounds(pos))
		}
	}

	pub fn idx_to_pos(&self, idx: usize) -> Pos {
		Pos::new(idx % self.width, idx / self.width)
	}

	pub fn wrap_pos_x(&self, pos: Pos) -> Pos {
		Vec2::new(pos.x % self.width, pos.y)
	}

	pub fn get<P: TryInto<usize>>(&self, pos: Vec2<P>) -> Option<&T> {
		self.data.get(self.pos_to_idx(Pos::new(pos.x.try_into().ok()?, pos.y.try_into().ok()?))?)
	}

	pub fn get_mut<P: TryInto<usize>>(&mut self, pos: Vec2<P>) -> Option<&mut T> {
		let idx = self.pos_to_idx(Pos::new(pos.x.try_into().ok()?, pos.y.try_into().ok()?))?;
		self.data.get_mut(idx)
	}

	pub fn print<F: FnMut(&T) -> char>(&self, mut f : F) {
		for chunk in self.data.chunks(self.width) {
			for t in chunk.iter() {
				print!("{}", f(t));
			}
			print!("\n");
		}
	}

	pub fn rows_iter(&self) -> std::slice::ChunksExact<T> {
		self.data.chunks_exact(self.width)
	}

	pub fn rows_iter_mut(&mut self) -> std::slice::ChunksExactMut<T> {
		self.data.chunks_exact_mut(self.width)
	}
}