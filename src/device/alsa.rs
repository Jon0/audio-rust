use libc::*;


pub enum snd_pcm_stream_t {}
pub enum snd_pcm_t {}

pub type snd_pcm_uframes_t = c_ulong;
pub type snd_pcm_sframes_t = c_long;


#[link(name = "asound")]
extern {
    fn snd_pcm_open(pcm: *mut *mut snd_pcm_t, name: *const c_char, stream: snd_pcm_stream_t, mode: c_int) -> c_int;
    fn snd_pcm_prepare(pcm: *mut snd_pcm_t) -> c_int;
    fn snd_pcm_writei(pcm: *mut snd_pcm_t, buffer: *const c_void, size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    fn snd_pcm_wait(pcm: *mut snd_pcm_t, timeout: c_int) -> c_int;
}
