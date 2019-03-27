use libc::c_int;
use libc::c_uint;

use alsa::ffi::*;

pub type SndFormatId = c_int;

pub trait AlsaFormat {
	const FORMAT_ID: SndFormatId;
}

impl AlsaFormat for i16 {
	const FORMAT_ID: SndFormatId = SND_PCM_FORMAT_S16_LE;
}

impl AlsaFormat for i32 {
	const FORMAT_ID: SndFormatId = SND_PCM_FORMAT_S32_LE;
}
