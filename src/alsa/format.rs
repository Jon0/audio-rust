use libc::c_int;
use libc::c_uint;

use alsa::ffi::*;


pub type SndFormatId = c_int;
pub type SndChannel = c_uint;

trait SndFormat {
    fn fmt() -> SndFormatId;
}


impl SndFormat for i16 {
    fn fmt() -> SndFormatId { SND_PCM_FORMAT_S16 }
}
