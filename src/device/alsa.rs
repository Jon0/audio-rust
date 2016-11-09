use libc::*;


pub enum snd_pcm_t {}
pub enum snd_pcm_hw_params_t {}


pub type snd_pcm_stream_t = c_uint;
pub type snd_pcm_uframes_t = c_ulong;
pub type snd_pcm_sframes_t = c_long;


pub static SND_PCM_NONBLOCK: i32 = 0x1;
pub static SND_PCM_ASYNC: i32 = 0x2;
pub const SND_PCM_STREAM_PLAYBACK: c_uint = 0;


#[link(name = "asound")]
extern {
    pub fn snd_pcm_open(pcm: *mut *mut snd_pcm_t, name: *const c_char, stream: snd_pcm_stream_t, mode: c_int) -> c_int;
    pub fn snd_pcm_prepare(pcm: *mut snd_pcm_t) -> c_int;
    pub fn snd_pcm_writei(pcm: *mut snd_pcm_t, buffer: *const c_void, size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
    pub fn snd_pcm_wait(pcm: *mut snd_pcm_t, timeout: c_int) -> c_int;
    pub fn snd_pcm_hw_params_malloc(ptr: *mut *mut snd_pcm_hw_params_t) -> c_int;
    pub fn snd_strerror(err: c_int) -> *const c_char;
}
