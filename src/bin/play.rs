extern crate libc;
extern crate rand;
extern crate num_integer;
extern crate num_rational;
extern crate audio_rust;

use std::env;

use audio_rust::alsa::device::AlsaDevice;
use audio_rust::alsa::stream::AlsaStream;
use audio_rust::player::player::*;
use audio_rust::format::sample::*;
use audio_rust::sound::generator::*;


fn use_device(dev: AlsaDevice) {
	let mut generator = FrameGenerator::new(1024); // * 16
	let player = AudioPlayer::new();

	dev.blocking(true).expect("Failed to set blocking");
	dev.prepare().expect("Failed to prepare device");

	// testing new stream type
	let mut stream = AlsaStream::<StereoSample<i16>>::open(dev).expect("Failed to open device");

	player.run(&mut stream, &mut generator);
}


/// hw:1,0
fn main() {
	let args: Vec<String> = env::args().collect();

	match AlsaDevice::open(&args[1]) {
		Ok(dev) => {
			use_device(dev)
		},
		Err(err) => println!("Open error: {}", err.as_string()),
	}
}
