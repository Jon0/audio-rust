use sound::array::*;
use sound::frame::*;
use sound::number::*;

pub fn create_next_frame_v1(frame_number: usize, frames: &[Frame]) -> Frame {

    let mut frame = Frame::new();
    let i = frame_number as u64;
    let f = factors(i);

    let mut stack = Vec::new();
    let mut remain = i;
    while remain > 0 {
        let (sn, r) = fib(remain);
        remain = r;
        let val = Factorised::create(sn);
        stack.push(val);
    }

    println!("{:?} -> {:?}", i, stack);

    let mut n = 1;
    let mut d = 1;
    for x in stack {
        frame.push(x.val(), first, 1.0);
    }

    //return Frame::create(1 + ((i + 1) - fb1), 1 + (fb2 - i));

    //return Frame::create(5 + (i / 8) % 33, 6 + (t / 9) % 4);

    return frame;
}
