use format::format::Format;

pub trait SampleType {
	type Sample;
	const Channels: usize;
}

pub struct StereoSample<T> {
	left: T,
	right: T
}

impl<T> SampleType for StereoSample<T> {
	type Sample = T;
	const Channels: usize = 2;
}