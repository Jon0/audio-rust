extern crate libc;
extern crate rand;
extern crate num_integer;
extern crate num_rational;
extern crate audio_rust;

use std::io;
use std::env;
use audio_rust::player::player::*;
use audio_rust::alsa::mixer::*;
use audio_rust::alsa::stream::*;
use audio_rust::format::sample::*;
use audio_rust::sound::array::*;
use audio_rust::sound::generator::*;
use audio_rust::sound::sampler::*;

fn use_device(mut dev: Device) {
	let mut generator = FrameGenerator::new(1024 * 16);
	let player = AudioPlayer::new();

	dev.blocking(true);
	dev.prepare();

	// testing new stream type
	let mut stream = AlsaStream::<StereoSample<i16>>::open(dev).expect("Failed to open device");

	player.run(&mut stream, &mut generator);

	// wait for completion
	let mut buf = String::new();
	io::stdin().read_line(&mut buf).expect("failed to read line");
}


fn main() {
	let args: Vec<String> = env::args().collect();

	match Device::open(&args[1]) {
		Ok(mut dev) => {
			use_device(dev)
		},
		Err(err) => println!("Open error: {}", err.as_string()),
	}
}
