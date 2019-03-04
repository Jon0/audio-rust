use libc::c_int;
use libc::c_uint;

use alsa::ffi::*;


pub type SndFormatId = c_int;
pub type SndChannel = c_uint;

pub trait AlsaFormat {
    const FormatId: SndFormatId;
}

impl AlsaFormat for i16 {
	const FormatId: SndFormatId = SND_PCM_FORMAT_S16;
}
