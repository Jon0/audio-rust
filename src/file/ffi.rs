#![allow(non_snake_case)]

use std::ptr::null_mut;
use std::os::raw::{c_char, c_uchar, c_float, c_int, c_long, c_void};

pub type ogg_int64_t = i64;

#[repr(C)]
pub struct ogg_packet {
    pub packet: *mut c_uchar,
    pub bytes: c_long,
    pub b_o_s: c_long,
    pub e_o_s: c_long,

    pub granulepos: ogg_int64_t,

    pub packetno: ogg_int64_t,
}

impl Default for ogg_packet {
	fn default () -> ogg_packet {
		ogg_packet {
			packet: null_mut(),
			bytes: 0,
			b_o_s: 0,
			e_o_s: 0,
			granulepos: 0,
			packetno: 0
		}
	}
}

#[repr(C)]
pub struct oggpack_buffer {
    endbyte: c_long,
    endbit: c_int,

    buffer: *mut c_uchar,
    ptr: *mut c_uchar,
    storage: c_long,
}

impl Default for oggpack_buffer {
	fn default () -> oggpack_buffer {
		oggpack_buffer {
			endbyte: 0,
			endbit: 0,
			buffer: null_mut(),
			ptr: null_mut(),
			storage: 0
		}
	}
}

#[repr(C)]
pub struct ogg_page {
    pub header: *mut c_uchar,
    pub header_len: c_long,
    pub body: *mut c_uchar,
    pub body_len: c_long
}

impl Default for ogg_page {
	fn default () -> ogg_page {
		ogg_page {
			header: null_mut(),
		    header_len: 0,
		    body: null_mut(),
		    body_len: 0
		}
	}
}

#[repr(C)]
pub struct ogg_stream_state {
    body_data: *mut c_uchar,
    body_storage: c_long,
    body_fill: c_long,
    body_returned: c_long,

    lacing_vals: *mut c_int,
    granule_vals: *mut ogg_int64_t,

    lacing_storage: c_long,
    lacing_fill: c_long,
    lacing_packet: c_long,
    lacing_returned: c_long,

    header: [c_uchar; 282],
    header_fill: c_int,

    e_o_s: c_int,
    b_o_s: c_int,
    serialno: c_long,
    pageno: c_long,
    packetno: ogg_int64_t,
    granulepos: ogg_int64_t
}

impl Default for ogg_stream_state {
	fn default () -> ogg_stream_state {
		ogg_stream_state {
			body_data: null_mut(),
			body_storage: 0,
			body_fill: 0,
			body_returned: 0,

			lacing_vals: null_mut(),
			granule_vals: null_mut(),

			lacing_storage: 0,
			lacing_fill: 0,
			lacing_packet: 0,
			lacing_returned: 0,

			header: [0; 282],
			header_fill: 0,

			e_o_s: 0,
			b_o_s: 0,
			serialno: 0,
			pageno: 0,
			packetno: 0,
			granulepos: 0
		}
	}
}

#[repr(C)]
pub struct vorbis_info {
	version: c_int,
	channels: c_int,
	rate: c_long,
	bitrate_upper: c_long,
	bitrate_nominal: c_long,
	bitrate_lower: c_long,
	bitrate_window: c_long,
	codec_setup: *mut c_void
}

impl Default for vorbis_info {
	fn default () -> vorbis_info {
		vorbis_info {
			version: 0,
			channels: 0,
			rate: 0,
			bitrate_upper: 0,
			bitrate_nominal: 0,
			bitrate_lower: 0,
			bitrate_window: 0,
			codec_setup: null_mut()
		}
	}
}

#[repr(C)]
pub struct vorbis_dsp_state {
    analysisp: c_int,
    vi: *mut vorbis_info,

    pcm: *mut *mut c_float,
    pcmret: *mut *mut c_float,
    pcm_storage: c_int,
    pcm_current: c_int,
    pcm_returned: c_int,

    preextrapolate: c_int,
    eofflag: c_int,

    lW: c_long,
    W: c_long,
    nW: c_long,
    centerW: c_long,

    granulepos: ogg_int64_t,
    sequence: ogg_int64_t,

    glue_bits: ogg_int64_t,
    time_bits: ogg_int64_t,
    floor_bits: ogg_int64_t,
    res_bits: ogg_int64_t,

    backend_state: *mut c_void,
}

impl Default for vorbis_dsp_state {
	fn default () -> vorbis_dsp_state {
		vorbis_dsp_state {
			analysisp: 0,
			vi: null_mut(),

			pcm: null_mut(),
			pcmret: null_mut(),
			pcm_storage: 0,
			pcm_current: 0,
			pcm_returned: 0,

			preextrapolate: 0,
			eofflag: 0,

			lW: 0,
			W: 0,
			nW: 0,
			centerW: 0,

			granulepos: 0,
			sequence: 0,

			glue_bits: 0,
			time_bits: 0,
			floor_bits: 0,
			res_bits: 0,

			backend_state: null_mut(),
		}
	}
}

#[repr(C)]
pub struct vorbis_comment {
	user_comments: *mut *mut c_char,
	comment_lengths: *mut c_int,
	comments: c_int,
	vendor: *mut c_char,
}

impl Default for vorbis_comment {
	fn default () -> vorbis_comment {
		vorbis_comment {
			user_comments: null_mut(),
			comment_lengths: null_mut(),
			comments: 0,
			vendor: null_mut()
		}
	}
}

#[repr(C)]
pub struct vorbis_block {
    pcm: *mut *mut c_float,
    opb: oggpack_buffer,

    lW: c_long,
    W: c_long,
    nW: c_long,
    pcmend: c_int,
    mode: c_int,

    eofflag: c_int,
    granulepos: ogg_int64_t,
    sequence: ogg_int64_t,
    vd: *mut vorbis_dsp_state,

    localstore: *mut c_void,
    localtop: c_long,
    localalloc: c_long,
    totaluse: c_long,
    reap: *mut alloc_chain,

    glue_bits: c_long,
    time_bits: c_long,
    floor_bits: c_long,
    res_bits: c_long,

    internal: *mut c_void
}

impl Default for vorbis_block {
	fn default () -> vorbis_block {
		vorbis_block {
			pcm: null_mut(),
		    opb: oggpack_buffer::default(),

		    lW: 0,
		    W: 0,
		    nW: 0,
		    pcmend: 0,
		    mode: 0,

		    eofflag: 0,
		    granulepos: 0,
		    sequence: 0,
		    vd: null_mut(),

		    localstore: null_mut(),
		    localtop: 0,
		    localalloc: 0,
		    totaluse: 0,
		    reap: null_mut(),

		    glue_bits: 0,
		    time_bits: 0,
		    floor_bits: 0,
		    res_bits: 0,

		    internal: null_mut()
		}
	}
}

#[repr(C)]
pub struct alloc_chain {
    ptr: *mut c_char,
    next: *mut alloc_chain
}

impl Default for alloc_chain {
	fn default () -> alloc_chain {
		alloc_chain {
			ptr: null_mut(),
		    next: null_mut(),
		}
	}
}

pub const OV_FALSE: c_int = -1;
pub const OV_EOF: c_int = -2;
pub const OV_HOLE: c_int = -3;

pub const OV_EREAD: c_int = -128;
pub const OV_EFAULT: c_int = -129;
pub const OV_EIMPL: c_int = -130;
pub const OV_EINVAL: c_int = -131;
pub const OV_ENOTVORBIS: c_int = -132;
pub const OV_EBADHEADER: c_int = -133;
pub const OV_EVERSION: c_int = -134;
pub const OV_ENOTAUDIO: c_int = -135;
pub const OV_EBADPACKET: c_int = -136;
pub const OV_EBADLINK: c_int = -137;
pub const OV_ENOSEEK: c_int = -138;


#[link(name = "ogg")]
extern {
	pub fn ogg_stream_init(os: *mut ogg_stream_state, serialno: c_int) -> c_int;
    pub fn ogg_stream_clear(os: *mut ogg_stream_state) -> c_int;
    pub fn ogg_stream_reset(os: *mut ogg_stream_state) -> c_int;
    pub fn ogg_stream_reset_serialno(os: *mut ogg_stream_state, serialno: c_int)-> c_int;
    pub fn ogg_stream_destroy(os: *mut ogg_stream_state) -> c_int;
    pub fn ogg_stream_check(os: *mut ogg_stream_state) -> c_int;
    pub fn ogg_stream_eos(os: *mut ogg_stream_state) -> c_int;

    pub fn ogg_page_checksum_set(og: *mut ogg_page);
    pub fn ogg_page_version(og: *const ogg_page) -> c_int;
    pub fn ogg_page_continued(og: *const ogg_page) -> c_int;
    pub fn ogg_page_bos(og: *const ogg_page) -> c_int;
    pub fn ogg_page_eos(og: *const ogg_page) -> c_int;
    pub fn ogg_page_granulepos(og: *const ogg_page) -> ogg_int64_t;
    pub fn ogg_page_serialno(og: *const ogg_page) -> c_int;
    pub fn ogg_page_pageno(og: *const ogg_page) -> c_long;
    pub fn ogg_page_packets(og: *const ogg_page) -> c_int;

	pub fn ogg_packet_clear(op: *mut ogg_packet);

	pub fn ogg_stream_packetin(os: *mut ogg_stream_state, op: *mut ogg_packet) -> c_int;
	pub fn ogg_stream_pageout(os: *mut ogg_stream_state, og: *mut ogg_page) -> c_int;
	pub fn ogg_stream_flush(os: *mut ogg_stream_state, og: *mut ogg_page) -> c_int;
}


#[link(name = "vorbis")]
extern {
	pub fn vorbis_info_init(vi: *mut vorbis_info);
	pub fn vorbis_info_clear(vi: *mut vorbis_info);
	pub fn vorbis_analysis_init(v: *mut vorbis_dsp_state, vi: *mut vorbis_info) -> c_int;
	pub fn vorbis_comment_init(vc: *mut vorbis_comment);
	pub fn vorbis_comment_add_tag(vc: *mut vorbis_comment, tag: *const c_char, contents: *const c_char);
	pub fn vorbis_comment_clear(vc: *mut vorbis_comment);
	pub fn vorbis_analysis_headerout(v: *mut vorbis_dsp_state, vc: *mut vorbis_comment, op: *mut ogg_packet, op_comm: *mut ogg_packet, op_code: *mut ogg_packet) -> c_int;
	pub fn vorbis_block_init(v: *mut vorbis_dsp_state, vb: *mut vorbis_block) -> c_int;
	pub fn vorbis_block_clear(vb: *mut vorbis_block) -> c_int;
	pub fn vorbis_dsp_clear(v: *mut vorbis_dsp_state);
	pub fn vorbis_analysis_buffer(v: *mut vorbis_dsp_state, vals: c_int) -> *mut *mut c_float;
	pub fn vorbis_analysis_wrote(v: *mut vorbis_dsp_state, vals: c_int) -> c_int;
	pub fn vorbis_analysis_blockout(v: *mut vorbis_dsp_state, vb: *mut vorbis_block) -> c_int;
	pub fn vorbis_analysis(vb: *mut vorbis_block, op: *mut ogg_packet) -> c_int;
	pub fn vorbis_bitrate_addblock(vb: *mut vorbis_block) -> c_int;
    pub fn vorbis_bitrate_flushpacket(v: *mut vorbis_dsp_state, op: *mut ogg_packet) -> c_int;
}


#[link(name = "vorbisenc")]
extern {
	pub fn vorbis_encode_init_vbr(vi: *mut vorbis_info, channels: c_long, rate: c_long, base_quality: c_float) -> c_int;
	pub fn vorbis_encode_setup_init(vi: *mut vorbis_info) -> c_int;
}
