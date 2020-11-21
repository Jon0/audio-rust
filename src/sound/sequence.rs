use sound::array::*;
use sound::frame::*;
use sound::number::*;

pub fn create_next_frame_old(frame_number: usize, frames: &[Frame]) -> Frame {

	let mut frame = Frame::new();
	let i = frame_number as u64;
	let f = factors(i);

	let mut stack = Vec::new();
	let mut remain = i;
	while remain > 1 {
		let (sn, r, e) = fib(remain);	
		remain = r;
		let val = Factorised::create(sn);
		stack.push((val, r, e));
	}

	//println!("{:?} -> {:?}", i, stack);

	let mut n = 1;
	let mut d = 1;
	for fa in f {
		for (x, a, b) in &stack {
			n += (a % (fa + 1));
			d += (b % (fa + 1));
			//frame.push(a, b, 1.0);
		}
	}

	println!("{:?} -> {:?}, {:?}", i, n, d);

	let a_fct = factors(n % (i + 1));
	let b_fct = factors(d % 12);
	let amp = 1.0 / ((a_fct.len() * b_fct.len()) as f64);
	for x in &a_fct {
		for y in &b_fct {
			frame.push(*x, *y, amp);
		}
	}

	//return Frame::from_pair(1 + ((i + 1) - fb1), 1 + (fb2 - i));

	return Frame::from_pair(5 + (i / 64) % 33, 6 + (i / 66) % 4);

	//return frame;
}


static mut rv: f32 = 0.4;


pub fn create_next_frame_v1(frame_number: usize, frames: &[Frame]) -> Frame {

	//let mut frame = Frame::new();
	let i = frame_number as u64;
	//let f = factors(i);

	//return Frame::from_pair(i / (60 - ((i / 9) % 57)), (255 / ((i / 9) + 2)) % 42);
	unsafe {
		rv = rv * (1.0 - rv) * (3.99 - i as f32 * 0.0001);

		return Frame::from_pair((rv * 16.0) as u64, 12);
	}
}