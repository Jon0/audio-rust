use num_integer::Integer;

use format::format::Format;

pub trait SampleType {
	type Sample;
	const Channels: usize;

	fn zero() -> Self;
	fn mono(sample: &Self::Sample) -> Self;
	fn mono_i16(sample: i16) -> Self;
	fn add(&mut self, other: &Self);
	fn to_string(&self) -> String;
}


#[derive(Clone, Debug)]
pub struct StereoSample<T> {
	left: T,
	right: T
}


impl<T: From<i16> + Copy + Clone + Integer + ToString> SampleType for StereoSample<T> {
	type Sample = T;
	const Channels: usize = 2;

	fn zero() -> Self {
		StereoSample { left: T::zero(), right: T::zero() }
	}

	fn mono(sample: &Self::Sample) -> Self {
		StereoSample { left: *sample, right: *sample }
	}

	fn mono_i16(sample: i16) -> Self {
		StereoSample { left: T::from(sample), right: T::from(sample) }
	}

	fn add(&mut self, other: &Self) {
		self.left = self.left.add(other.left);
		self.right = self.right.add(other.right);
	}

	fn to_string(&self) -> String {
		return self.left.to_string();
	}
}


pub trait Stream<S: SampleType> {
	fn push(&mut self, frames: &[S]);
}