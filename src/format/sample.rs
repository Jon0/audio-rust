use format::format::Format;

pub trait Sample {
	fn channels() -> usize;
}

pub struct StereoSample<T> {
	left: T,
	right: T
}

impl<T> Sample for StereoSample<T> {
	fn channels() -> usize {
		return 2;
	}
}