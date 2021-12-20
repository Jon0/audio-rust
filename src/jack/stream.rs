use format::format::Format;
use format::sample::SampleType;
use format::sample::Stream;
use format::sample::StereoSample;
use format::error::DriverError;
use sound::generator::Generator;

use std::sync::Arc;
use std::sync::Mutex;
use std::mem::transmute_copy;

pub struct JackAudioPlayer {

}


impl JackAudioPlayer {
	pub fn new() -> JackAudioPlayer {
		return JackAudioPlayer {};
	}

	pub fn run<G: 'static + Generator + std::marker::Send + std::marker::Sync>(&self, gen2: Arc<Mutex<G>>) {

		// 1. open a client
		let (client, _status) =
			jack::Client::new("rust_jack", jack::ClientOptions::NO_START_SERVER).unwrap();

		let mut format = jack::AudioOut::default();

		// 2. register port
		let mut out_port = client
			.register_port("audio_out", format)
			.unwrap();

		//let gen_arc = Arc::clone(&gen2);

		// 3. define process callback handler
		let frequency = 220.0;
		let sample_rate = client.sample_rate();
		let frame_t = 1.0 / sample_rate as f64;
		let mut time = 0.0;
		let mut samples = 0;

		let process = jack::ClosureProcessHandler::new(
		    move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
		        // Get output buffer
		        let out = out_port.as_mut_slice(ps);

				for v in out.iter_mut() {
					let x = frequency * time * 2.0 * std::f64::consts::PI;
					let y = x.sin();
					*v = 0.0 as f32;
					time += frame_t;
				}

		        let mut gen = gen2.lock().unwrap();
		        gen.fill_async(samples, out);
		        samples += out.len();

		        // Continue as normal
		        jack::Control::Continue
		    },
		);

		// 4. activate the client
		let active_client = client.activate_async((), process).unwrap();
		active_client
			.as_client()
			.connect_ports_by_name("rust_jack:audio_out", "system:playback_1")
			.unwrap();
		active_client
			.as_client()
			.connect_ports_by_name("rust_jack:audio_out", "system:playback_2")
			.unwrap();

		// processing starts here

		// 5. wait or do some processing while your handler is running in real time.
		loop {}

		// 6. Optional deactivate. Not required since active_client will deactivate on
		// drop, though explicit deactivate may help you identify errors in
		// deactivate.
		active_client.deactivate().unwrap();

	}
}
