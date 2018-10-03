use sound::frame::*;

pub fn create_next_frame_v1(frames: &[Frame]) -> Frame {
    let i = frames.len() as u64;

    if ((frames.len() / 40) % 2 == 0) {
        return Frame::create(i, 4);
    }
    else {
        return Frame::create(i, 3);
    }

}
