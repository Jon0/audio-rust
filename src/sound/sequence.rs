use sound::array::*;
use sound::frame::*;

pub fn create_next_frame_v1(frame_number: usize, frames: &[Frame]) -> Frame {

    let mut frame = Frame::new();
    let i = frame_number as u64;
    let f = factors(i);



    let mut fb1 = 1;
    let mut fb2 = 1;
    while fb2 <= i {
        fb2 = fb1 + fb2;
        fb1 = fb2 - fb1;
    }

    let mut t = 1;
    for a in i + 1..i + 12 {
        let d = i / a;
        let m = i % a;
    }


    frame.push(1, 5, 1.0);

    //return Frame::create(1 + ((i + 1) - fb1), 1 + (fb2 - i));

    //return Frame::create(5 + (i / 8) % 33, 6 + (t / 9) % 4);

    return frame;
}
