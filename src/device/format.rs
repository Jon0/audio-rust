use libc::c_int;

use device::alsa::*;


pub type SndFormatId = c_int;


trait SndFormat {
    fn fmt() -> SndFormatId;
}


impl SndFormat for i16 {
    fn fmt() -> SndFormatId { SND_PCM_FORMAT_S16 }
}
