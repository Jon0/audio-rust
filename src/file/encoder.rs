use std::f32;
use std::f32::consts::PI;
use std::vec::Vec;
use std::fs::File;
use std::io::Write;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use std::ptr::null_mut;
use std::ffi::CString;

use file::ffi::*;
use sound::generator::Generator;

pub struct Encoder {
	os: ogg_stream_state,
	og: ogg_page,
	op: ogg_packet,

	vi: vorbis_info,
	vc: vorbis_comment,
	vd: vorbis_dsp_state,
	vb: vorbis_block,

	header: ogg_packet,
	header_comm: ogg_packet,
	header_code: ogg_packet,

	output: Vec<u8>
}


impl Encoder {
	pub fn new() -> Encoder {
		Encoder {
			os: ogg_stream_state::default(),
			og: ogg_page::default(),
			op: ogg_packet::default(),
			vi: vorbis_info::default(),
			vc: vorbis_comment::default(),
			vd: vorbis_dsp_state::default(),
			vb: vorbis_block::default(),

			header: ogg_packet::default(),
			header_comm: ogg_packet::default(),
			header_code: ogg_packet::default(),

			output: Vec::new()
		}
	}

	pub fn init(&mut self) {
		let encoder = CString::new("Encoder").unwrap();
		let test = CString::new("Rust").unwrap();

		unsafe {
			vorbis_info_init(&mut self.vi);
			vorbis_encode_init_vbr(&mut self.vi, 2, 44100, 0.8);
			vorbis_encode_setup_init(&mut self.vi);

			vorbis_comment_init(&mut self.vc);
			vorbis_comment_add_tag(&mut self.vc, encoder.as_ptr(), test.as_ptr());
			vorbis_analysis_init(&mut self.vd, &mut self.vi);
			vorbis_block_init(&mut self.vd, &mut self.vb);

			ogg_stream_init(&mut self.os, 12345);

			vorbis_analysis_headerout(&mut self.vd, &mut self.vc, &mut self.header, &mut self.header_comm, &mut self.header_code);

			ogg_stream_packetin(&mut self.os, &mut self.header);
			ogg_stream_packetin(&mut self.os, &mut self.header_comm);
			ogg_stream_packetin(&mut self.os, &mut self.header_code);

			// output the header first so audio content will begin on a new page
			loop {
				let result = ogg_stream_flush(&mut self.os, &mut self.og);
				if result == 0 {
					break;
				}
				self.append_to_buffer();
			}
		}
	}

	pub fn write<G: Generator>(&mut self, min: usize, gen: &mut G) {
		unsafe {
			let mut gt = 0;
			while self.output.len() < min {
				let samples = 1024;
				let buffer = vorbis_analysis_buffer(&mut self.vd, samples);

				// copy data to buffer here
				// 32 bit float samples, copied to non-interleaved buffers
				let channels = from_raw_parts_mut(buffer, 2);
				let mut left = from_raw_parts_mut(channels[0], samples as usize);
				let mut right = from_raw_parts_mut(channels[1], samples as usize);
				
				// f32 is a mono SampleType
				gen.fill_async(gt, &mut left);
				gen.fill_async(gt, &mut right);
				gt += left.len();

				let write_result = vorbis_analysis_wrote(&mut self.vd, samples);
				self.output();
			}

			vorbis_analysis_wrote(&mut self.vd, 0);
			self.output();
		}
		println!("size: {}", self.output.len());
		println!("done!");
	}

	pub fn output(&mut self) {
		unsafe {
			loop {
				let out_result = vorbis_analysis_blockout(&mut self.vd, &mut self.vb);
				if out_result == 0 {
					break;
				}

				vorbis_analysis(&mut self.vb, &mut self.op);

				ogg_stream_packetin(&mut self.os, &mut self.op);

				loop {
					let fresult = ogg_stream_pageout(&mut self.os, &mut self.og);
					if fresult == 0 {
						break;
					}
					self.append_to_buffer();
				}
			}
		}
	}

	pub fn append_to_buffer(&mut self) {
		unsafe {
			self.output.extend_from_slice(from_raw_parts(self.og.header, self.og.header_len as usize));
			self.output.extend_from_slice(from_raw_parts(self.og.body, self.og.body_len as usize));
		}
	}


	pub fn write_to_file(&mut self) {
		let mut file = File::create("static/audio.ogg").unwrap();

		file.write(self.output.as_slice()).expect("Failed to write to file");
	}


	pub fn available(&self) -> &[u8] {
	return self.output.as_slice();		
	}

	pub fn close(&mut self) {
		unsafe {
			ogg_stream_clear(&mut self.os);
			vorbis_block_clear(&mut self.vb);
			vorbis_dsp_clear(&mut self.vd);
			vorbis_comment_clear(&mut self.vc);
			vorbis_info_clear(&mut self.vi);
		}
	}
}
