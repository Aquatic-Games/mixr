#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn abs(_: libc::c_int) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stb_vorbis_alloc {
    pub alloc_buffer: *mut libc::c_char,
    pub alloc_buffer_length_in_bytes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stb_vorbis {
    pub sample_rate: libc::c_uint,
    pub channels: libc::c_int,
    pub setup_memory_required: libc::c_uint,
    pub temp_memory_required: libc::c_uint,
    pub setup_temp_memory_required: libc::c_uint,
    pub vendor: *mut libc::c_char,
    pub comment_list_length: libc::c_int,
    pub comment_list: *mut *mut libc::c_char,
    pub f: *mut FILE,
    pub f_start: uint32,
    pub close_on_free: libc::c_int,
    pub stream: *mut uint8,
    pub stream_start: *mut uint8,
    pub stream_end: *mut uint8,
    pub stream_len: uint32,
    pub push_mode: uint8,
    pub first_audio_page_offset: uint32,
    pub p_first: ProbedPage,
    pub p_last: ProbedPage,
    pub alloc: stb_vorbis_alloc,
    pub setup_offset: libc::c_int,
    pub temp_offset: libc::c_int,
    pub eof: libc::c_int,
    pub error: STBVorbisError,
    pub blocksize: [libc::c_int; 2],
    pub blocksize_0: libc::c_int,
    pub blocksize_1: libc::c_int,
    pub codebook_count: libc::c_int,
    pub codebooks: *mut Codebook,
    pub floor_count: libc::c_int,
    pub floor_types: [uint16; 64],
    pub floor_config: *mut Floor,
    pub residue_count: libc::c_int,
    pub residue_types: [uint16; 64],
    pub residue_config: *mut Residue,
    pub mapping_count: libc::c_int,
    pub mapping: *mut Mapping,
    pub mode_count: libc::c_int,
    pub mode_config: [Mode; 64],
    pub total_samples: uint32,
    pub channel_buffers: [*mut libc::c_float; 16],
    pub outputs: [*mut libc::c_float; 16],
    pub previous_window: [*mut libc::c_float; 16],
    pub previous_length: libc::c_int,
    pub finalY: [*mut int16; 16],
    pub current_loc: uint32,
    pub current_loc_valid: libc::c_int,
    pub A: [*mut libc::c_float; 2],
    pub B: [*mut libc::c_float; 2],
    pub C: [*mut libc::c_float; 2],
    pub window: [*mut libc::c_float; 2],
    pub bit_reverse: [*mut uint16; 2],
    pub serial: uint32,
    pub last_page: libc::c_int,
    pub segment_count: libc::c_int,
    pub segments: [uint8; 255],
    pub page_flag: uint8,
    pub bytes_in_seg: uint8,
    pub first_decode: uint8,
    pub next_seg: libc::c_int,
    pub last_seg: libc::c_int,
    pub last_seg_which: libc::c_int,
    pub acc: uint32,
    pub valid_bits: libc::c_int,
    pub packet_bytes: libc::c_int,
    pub end_seg_with_known_loc: libc::c_int,
    pub known_loc_for_packet: uint32,
    pub discard_samples_deferred: libc::c_int,
    pub samples_output: uint32,
    pub page_crc_tests: libc::c_int,
    pub scan: [CRCscan; 4],
    pub channel_buffer_start: libc::c_int,
    pub channel_buffer_end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CRCscan {
    pub goal_crc: uint32,
    pub bytes_left: libc::c_int,
    pub crc_so_far: uint32,
    pub bytes_done: libc::c_int,
    pub sample_loc: uint32,
}
pub type uint32 = libc::c_uint;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mode {
    pub blockflag: uint8,
    pub mapping: uint8,
    pub windowtype: uint16,
    pub transformtype: uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mapping {
    pub coupling_steps: uint16,
    pub chan: *mut MappingChannel,
    pub submaps: uint8,
    pub submap_floor: [uint8; 15],
    pub submap_residue: [uint8; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MappingChannel {
    pub magnitude: uint8,
    pub angle: uint8,
    pub mux: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Residue {
    pub begin: uint32,
    pub end: uint32,
    pub part_size: uint32,
    pub classifications: uint8,
    pub classbook: uint8,
    pub classdata: *mut *mut uint8,
    pub residue_books: *mut [int16; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Floor {
    pub floor0: Floor0,
    pub floor1: Floor1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Floor1 {
    pub partitions: uint8,
    pub partition_class_list: [uint8; 32],
    pub class_dimensions: [uint8; 16],
    pub class_subclasses: [uint8; 16],
    pub class_masterbooks: [uint8; 16],
    pub subclass_books: [[int16; 8]; 16],
    pub Xlist: [uint16; 250],
    pub sorted_order: [uint8; 250],
    pub neighbors: [[uint8; 2]; 250],
    pub floor1_multiplier: uint8,
    pub rangebits: uint8,
    pub values: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Floor0 {
    pub order: uint8,
    pub rate: uint16,
    pub bark_map_size: uint16,
    pub amplitude_bits: uint8,
    pub amplitude_offset: uint8,
    pub number_of_books: uint8,
    pub book_list: [uint8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Codebook {
    pub dimensions: libc::c_int,
    pub entries: libc::c_int,
    pub codeword_lengths: *mut uint8,
    pub minimum_value: libc::c_float,
    pub delta_value: libc::c_float,
    pub value_bits: uint8,
    pub lookup_type: uint8,
    pub sequence_p: uint8,
    pub sparse: uint8,
    pub lookup_values: uint32,
    pub multiplicands: *mut codetype,
    pub codewords: *mut uint32,
    pub fast_huffman: [int16; 1024],
    pub sorted_codewords: *mut uint32,
    pub sorted_values: *mut libc::c_int,
    pub sorted_entries: libc::c_int,
}
pub type codetype = libc::c_float;
pub type STBVorbisError = libc::c_uint;
pub const VORBIS_ogg_skeleton_not_supported: STBVorbisError = 38;
pub const VORBIS_seek_failed: STBVorbisError = 37;
pub const VORBIS_cant_find_last_page: STBVorbisError = 36;
pub const VORBIS_bad_packet_type: STBVorbisError = 35;
pub const VORBIS_invalid_first_page: STBVorbisError = 34;
pub const VORBIS_incorrect_stream_serial_number: STBVorbisError = 33;
pub const VORBIS_continued_packet_flag_invalid: STBVorbisError = 32;
pub const VORBIS_invalid_stream_structure_version: STBVorbisError = 31;
pub const VORBIS_missing_capture_pattern: STBVorbisError = 30;
pub const VORBIS_invalid_stream: STBVorbisError = 21;
pub const VORBIS_invalid_setup: STBVorbisError = 20;
pub const VORBIS_seek_invalid: STBVorbisError = 11;
pub const VORBIS_unexpected_eof: STBVorbisError = 10;
pub const VORBIS_seek_without_length: STBVorbisError = 7;
pub const VORBIS_file_open_failure: STBVorbisError = 6;
pub const VORBIS_too_many_channels: STBVorbisError = 5;
pub const VORBIS_feature_not_supported: STBVorbisError = 4;
pub const VORBIS_outofmem: STBVorbisError = 3;
pub const VORBIS_invalid_api_mixing: STBVorbisError = 2;
pub const VORBIS_need_more_data: STBVorbisError = 1;
pub const VORBIS__no_error: STBVorbisError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProbedPage {
    pub page_start: uint32,
    pub page_end: uint32,
    pub last_decoded_sample: uint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stb_vorbis_info {
    pub sample_rate: libc::c_uint,
    pub channels: libc::c_int,
    pub setup_memory_required: libc::c_uint,
    pub setup_temp_memory_required: libc::c_uint,
    pub temp_memory_required: libc::c_uint,
    pub max_frame_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stb_vorbis_comment {
    pub vendor: *mut libc::c_char,
    pub comment_list_length: libc::c_int,
    pub comment_list: *mut *mut libc::c_char,
}
pub type vorb = stb_vorbis;
pub type int32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbv__floor_ordering {
    pub x: uint16,
    pub id: uint16,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub const VORBIS_packet_setup: C2RustUnnamed = 5;
pub const VORBIS_packet_comment: C2RustUnnamed = 3;
pub const VORBIS_packet_id: C2RustUnnamed = 1;
pub type YTYPE = int16;
#[derive(Copy, Clone)]
#[repr(C)]
pub union float_conv {
    pub f: libc::c_float,
    pub i: libc::c_int,
}
pub type int8 = libc::c_schar;
pub type C2RustUnnamed = libc::c_uint;
unsafe extern "C" fn error(mut f: *mut vorb, mut e: STBVorbisError) -> libc::c_int {
    (*f).error = e;
    if (*f).eof == 0
        && e as libc::c_uint != VORBIS_need_more_data as libc::c_int as libc::c_uint
    {
        (*f).error = e;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn make_block_array(
    mut mem: *mut libc::c_void,
    mut count: libc::c_int,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut p: *mut *mut libc::c_void = mem as *mut *mut libc::c_void;
    let mut q: *mut libc::c_char = p.offset(count as isize) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < count {
        let ref mut fresh0 = *p.offset(i as isize);
        *fresh0 = q as *mut libc::c_void;
        q = q.offset(size as isize);
        i += 1;
    }
    return p as *mut libc::c_void;
}
unsafe extern "C" fn setup_malloc(
    mut f: *mut vorb,
    mut sz: libc::c_int,
) -> *mut libc::c_void {
    sz = sz + 7 as libc::c_int & !(7 as libc::c_int);
    let ref mut fresh1 = (*f).setup_memory_required;
    *fresh1 = (*fresh1).wrapping_add(sz as libc::c_uint);
    if !((*f).alloc.alloc_buffer).is_null() {
        let mut p: *mut libc::c_void = ((*f).alloc.alloc_buffer)
            .offset((*f).setup_offset as isize) as *mut libc::c_void;
        if (*f).setup_offset + sz > (*f).temp_offset {
            return 0 as *mut libc::c_void;
        }
        (*f).setup_offset += sz;
        return p;
    }
    return if sz != 0 { malloc(sz as libc::c_ulong) } else { 0 as *mut libc::c_void };
}
unsafe extern "C" fn setup_free(mut f: *mut vorb, mut p: *mut libc::c_void) {
    if !((*f).alloc.alloc_buffer).is_null() {
        return;
    }
    free(p);
}
unsafe extern "C" fn setup_temp_malloc(
    mut f: *mut vorb,
    mut sz: libc::c_int,
) -> *mut libc::c_void {
    sz = sz + 7 as libc::c_int & !(7 as libc::c_int);
    if !((*f).alloc.alloc_buffer).is_null() {
        if (*f).temp_offset - sz < (*f).setup_offset {
            return 0 as *mut libc::c_void;
        }
        (*f).temp_offset -= sz;
        return ((*f).alloc.alloc_buffer).offset((*f).temp_offset as isize)
            as *mut libc::c_void;
    }
    return malloc(sz as libc::c_ulong);
}
unsafe extern "C" fn setup_temp_free(
    mut f: *mut vorb,
    mut p: *mut libc::c_void,
    mut sz: libc::c_int,
) {
    if !((*f).alloc.alloc_buffer).is_null() {
        (*f).temp_offset += sz + 7 as libc::c_int & !(7 as libc::c_int);
        return;
    }
    free(p);
}
static mut crc_table: [uint32; 256] = [0; 256];
unsafe extern "C" fn crc32_init() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: uint32 = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        s = (i as uint32) << 24 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            s = s << 1 as libc::c_int
                ^ (if s >= (1 as libc::c_uint) << 31 as libc::c_int {
                    0x4c11db7 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_uint;
            j += 1;
        }
        crc_table[i as usize] = s;
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn crc32_update(mut crc: uint32, mut byte: uint8) -> uint32 {
    return crc << 8 as libc::c_int
        ^ crc_table[(byte as libc::c_uint ^ crc >> 24 as libc::c_int) as usize];
}
unsafe extern "C" fn bit_reverse(mut n: libc::c_uint) -> libc::c_uint {
    n = (n & 0xaaaaaaaa as libc::c_uint) >> 1 as libc::c_int
        | (n & 0x55555555 as libc::c_int as libc::c_uint) << 1 as libc::c_int;
    n = (n & 0xcccccccc as libc::c_uint) >> 2 as libc::c_int
        | (n & 0x33333333 as libc::c_int as libc::c_uint) << 2 as libc::c_int;
    n = (n & 0xf0f0f0f0 as libc::c_uint) >> 4 as libc::c_int
        | (n & 0xf0f0f0f as libc::c_int as libc::c_uint) << 4 as libc::c_int;
    n = (n & 0xff00ff00 as libc::c_uint) >> 8 as libc::c_int
        | (n & 0xff00ff as libc::c_int as libc::c_uint) << 8 as libc::c_int;
    return n >> 16 as libc::c_int | n << 16 as libc::c_int;
}
unsafe extern "C" fn square(mut x: libc::c_float) -> libc::c_float {
    return x * x;
}
unsafe extern "C" fn ilog(mut n: int32) -> libc::c_int {
    static mut log2_4: [libc::c_schar; 16] = [
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
    ];
    if n < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if n < (1 as libc::c_int) << 14 as libc::c_int {
        if n < (1 as libc::c_int) << 4 as libc::c_int {
            return 0 as libc::c_int + log2_4[n as usize] as libc::c_int
        } else if n < (1 as libc::c_int) << 9 as libc::c_int {
            return 5 as libc::c_int
                + log2_4[(n >> 5 as libc::c_int) as usize] as libc::c_int
        } else {
            return 10 as libc::c_int
                + log2_4[(n >> 10 as libc::c_int) as usize] as libc::c_int
        }
    } else if n < (1 as libc::c_int) << 24 as libc::c_int {
        if n < (1 as libc::c_int) << 19 as libc::c_int {
            return 15 as libc::c_int
                + log2_4[(n >> 15 as libc::c_int) as usize] as libc::c_int
        } else {
            return 20 as libc::c_int
                + log2_4[(n >> 20 as libc::c_int) as usize] as libc::c_int
        }
    } else if n < (1 as libc::c_int) << 29 as libc::c_int {
        return 25 as libc::c_int
            + log2_4[(n >> 25 as libc::c_int) as usize] as libc::c_int
    } else {
        return 30 as libc::c_int
            + log2_4[(n >> 30 as libc::c_int) as usize] as libc::c_int
    };
}
unsafe extern "C" fn float32_unpack(mut x: uint32) -> libc::c_float {
    let mut mantissa: uint32 = x & 0x1fffff as libc::c_int as libc::c_uint;
    let mut sign: uint32 = x & 0x80000000 as libc::c_uint;
    let mut exp_0: uint32 = (x & 0x7fe00000 as libc::c_int as libc::c_uint)
        >> 21 as libc::c_int;
    let mut res: libc::c_double = if sign != 0 {
        -(mantissa as libc::c_double)
    } else {
        mantissa as libc::c_double
    };
    return ldexp(
        res as libc::c_float as libc::c_double,
        exp_0 as libc::c_int - 788 as libc::c_int,
    ) as libc::c_float;
}
unsafe extern "C" fn add_entry(
    mut c: *mut Codebook,
    mut huff_code: uint32,
    mut symbol: libc::c_int,
    mut count: libc::c_int,
    mut len: libc::c_int,
    mut values: *mut uint32,
) {
    if (*c).sparse == 0 {
        *((*c).codewords).offset(symbol as isize) = huff_code;
    } else {
        *((*c).codewords).offset(count as isize) = huff_code;
        *((*c).codeword_lengths).offset(count as isize) = len as uint8;
        *values.offset(count as isize) = symbol as uint32;
    };
}
unsafe extern "C" fn compute_codewords(
    mut c: *mut Codebook,
    mut len: *mut uint8,
    mut n: libc::c_int,
    mut values: *mut uint32,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut available: [uint32; 32] = [0; 32];
    memset(
        available.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint32; 32]>() as libc::c_ulong,
    );
    k = 0 as libc::c_int;
    while k < n {
        if (*len.offset(k as isize) as libc::c_int) < 255 as libc::c_int {
            break;
        }
        k += 1;
    }
    if k == n {
        return 1 as libc::c_int;
    }
    let fresh2 = m;
    m = m + 1;
    add_entry(
        c,
        0 as libc::c_int as uint32,
        k,
        fresh2,
        *len.offset(k as isize) as libc::c_int,
        values,
    );
    i = 1 as libc::c_int;
    while i <= *len.offset(k as isize) as libc::c_int {
        available[i as usize] = (1 as libc::c_uint) << 32 as libc::c_int - i;
        i += 1;
    }
    i = k + 1 as libc::c_int;
    while i < n {
        let mut res: uint32 = 0;
        let mut z: libc::c_int = *len.offset(i as isize) as libc::c_int;
        let mut y: libc::c_int = 0;
        if !(z == 255 as libc::c_int) {
            while z > 0 as libc::c_int && available[z as usize] == 0 {
                z -= 1;
            }
            if z == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            res = available[z as usize];
            available[z as usize] = 0 as libc::c_int as uint32;
            let fresh3 = m;
            m = m + 1;
            add_entry(
                c,
                bit_reverse(res),
                i,
                fresh3,
                *len.offset(i as isize) as libc::c_int,
                values,
            );
            if z != *len.offset(i as isize) as libc::c_int {
                y = *len.offset(i as isize) as libc::c_int;
                while y > z {
                    available[y
                        as usize] = res
                        .wrapping_add(
                            ((1 as libc::c_int) << 32 as libc::c_int - y) as libc::c_uint,
                        );
                    y -= 1;
                }
            }
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn compute_accelerated_huffman(mut c: *mut Codebook) {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 10 as libc::c_int {
        (*c).fast_huffman[i as usize] = -(1 as libc::c_int) as int16;
        i += 1;
    }
    len = if (*c).sparse as libc::c_int != 0 {
        (*c).sorted_entries
    } else {
        (*c).entries
    };
    if len > 32767 as libc::c_int {
        len = 32767 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < len {
        if *((*c).codeword_lengths).offset(i as isize) as libc::c_int
            <= 10 as libc::c_int
        {
            let mut z: uint32 = if (*c).sparse as libc::c_int != 0 {
                bit_reverse(*((*c).sorted_codewords).offset(i as isize))
            } else {
                *((*c).codewords).offset(i as isize)
            };
            while z < ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint {
                (*c).fast_huffman[z as usize] = i as int16;
                z = (z as libc::c_uint)
                    .wrapping_add(
                        ((1 as libc::c_int)
                            << *((*c).codeword_lengths).offset(i as isize)
                                as libc::c_int) as libc::c_uint,
                    ) as uint32 as uint32;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn uint32_compare(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    let mut x: uint32 = *(p as *mut uint32);
    let mut y: uint32 = *(q as *mut uint32);
    return if x < y { -(1 as libc::c_int) } else { (x > y) as libc::c_int };
}
unsafe extern "C" fn include_in_sort(
    mut c: *mut Codebook,
    mut len: uint8,
) -> libc::c_int {
    if (*c).sparse != 0 {
        return 1 as libc::c_int;
    }
    if len as libc::c_int == 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    if len as libc::c_int > 10 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compute_sorted_huffman(
    mut c: *mut Codebook,
    mut lengths: *mut uint8,
    mut values: *mut uint32,
) {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if (*c).sparse == 0 {
        let mut k: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*c).entries {
            if include_in_sort(c, *lengths.offset(i as isize)) != 0 {
                let fresh4 = k;
                k = k + 1;
                *((*c).sorted_codewords)
                    .offset(
                        fresh4 as isize,
                    ) = bit_reverse(*((*c).codewords).offset(i as isize));
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*c).sorted_entries {
            *((*c).sorted_codewords)
                .offset(i as isize) = bit_reverse(*((*c).codewords).offset(i as isize));
            i += 1;
        }
    }
    qsort(
        (*c).sorted_codewords as *mut libc::c_void,
        (*c).sorted_entries as size_t,
        ::std::mem::size_of::<uint32>() as libc::c_ulong,
        Some(
            uint32_compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    *((*c).sorted_codewords)
        .offset((*c).sorted_entries as isize) = 0xffffffff as libc::c_uint;
    len = if (*c).sparse as libc::c_int != 0 {
        (*c).sorted_entries
    } else {
        (*c).entries
    };
    i = 0 as libc::c_int;
    while i < len {
        let mut huff_len: libc::c_int = if (*c).sparse as libc::c_int != 0 {
            *lengths.offset(*values.offset(i as isize) as isize) as libc::c_int
        } else {
            *lengths.offset(i as isize) as libc::c_int
        };
        if include_in_sort(c, huff_len as uint8) != 0 {
            let mut code: uint32 = bit_reverse(*((*c).codewords).offset(i as isize));
            let mut x: libc::c_int = 0 as libc::c_int;
            let mut n: libc::c_int = (*c).sorted_entries;
            while n > 1 as libc::c_int {
                let mut m: libc::c_int = x + (n >> 1 as libc::c_int);
                if *((*c).sorted_codewords).offset(m as isize) <= code {
                    x = m;
                    n -= n >> 1 as libc::c_int;
                } else {
                    n >>= 1 as libc::c_int;
                }
            }
            if (*c).sparse != 0 {
                *((*c).sorted_values)
                    .offset(x as isize) = *values.offset(i as isize) as libc::c_int;
                *((*c).codeword_lengths).offset(x as isize) = huff_len as uint8;
            } else {
                *((*c).sorted_values).offset(x as isize) = i;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn vorbis_validate(mut data: *mut uint8) -> libc::c_int {
    static mut vorbis: [uint8; 6] = [
        'v' as i32 as uint8,
        'o' as i32 as uint8,
        'r' as i32 as uint8,
        'b' as i32 as uint8,
        'i' as i32 as uint8,
        's' as i32 as uint8,
    ];
    return (memcmp(
        data as *const libc::c_void,
        vorbis.as_mut_ptr() as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn lookup1_values(
    mut entries: libc::c_int,
    mut dim: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = floor(
        exp(
            (log(entries as libc::c_float as libc::c_double) as libc::c_float
                / dim as libc::c_float) as libc::c_double,
        ),
    ) as libc::c_int;
    if floor(
        pow(
            (r as libc::c_float + 1 as libc::c_int as libc::c_float) as libc::c_double,
            dim as libc::c_double,
        ),
    ) as libc::c_int <= entries
    {
        r += 1;
    }
    if pow(
        (r as libc::c_float + 1 as libc::c_int as libc::c_float) as libc::c_double,
        dim as libc::c_double,
    ) <= entries as libc::c_double
    {
        return -(1 as libc::c_int);
    }
    if floor(pow(r as libc::c_float as libc::c_double, dim as libc::c_double))
        as libc::c_int > entries
    {
        return -(1 as libc::c_int);
    }
    return r;
}
unsafe extern "C" fn compute_twiddle_factors(
    mut n: libc::c_int,
    mut A: *mut libc::c_float,
    mut B: *mut libc::c_float,
    mut C_0: *mut libc::c_float,
) {
    let mut n4: libc::c_int = n >> 2 as libc::c_int;
    let mut n8: libc::c_int = n >> 3 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut k2: libc::c_int = 0;
    k2 = 0 as libc::c_int;
    k = k2;
    while k < n4 {
        *A
            .offset(
                k2 as isize,
            ) = cos(
            (4 as libc::c_int * k) as libc::c_double * 3.14159265358979323846f64
                / n as libc::c_double,
        ) as libc::c_float;
        *A
            .offset(
                (k2 + 1 as libc::c_int) as isize,
            ) = -sin(
            (4 as libc::c_int * k) as libc::c_double * 3.14159265358979323846f64
                / n as libc::c_double,
        ) as libc::c_float;
        *B
            .offset(
                k2 as isize,
            ) = cos(
            (k2 + 1 as libc::c_int) as libc::c_double * 3.14159265358979323846f64
                / n as libc::c_double / 2 as libc::c_int as libc::c_double,
        ) as libc::c_float * 0.5f32;
        *B
            .offset(
                (k2 + 1 as libc::c_int) as isize,
            ) = sin(
            (k2 + 1 as libc::c_int) as libc::c_double * 3.14159265358979323846f64
                / n as libc::c_double / 2 as libc::c_int as libc::c_double,
        ) as libc::c_float * 0.5f32;
        k += 1;
        k2 += 2 as libc::c_int;
    }
    k2 = 0 as libc::c_int;
    k = k2;
    while k < n8 {
        *C_0
            .offset(
                k2 as isize,
            ) = cos(
            (2 as libc::c_int * (k2 + 1 as libc::c_int)) as libc::c_double
                * 3.14159265358979323846f64 / n as libc::c_double,
        ) as libc::c_float;
        *C_0
            .offset(
                (k2 + 1 as libc::c_int) as isize,
            ) = -sin(
            (2 as libc::c_int * (k2 + 1 as libc::c_int)) as libc::c_double
                * 3.14159265358979323846f64 / n as libc::c_double,
        ) as libc::c_float;
        k += 1;
        k2 += 2 as libc::c_int;
    }
}
unsafe extern "C" fn compute_window(mut n: libc::c_int, mut window: *mut libc::c_float) {
    let mut n2: libc::c_int = n >> 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n2 {
        *window
            .offset(
                i as isize,
            ) = sin(
            0.5f64 * 3.14159265358979323846f64
                * square(
                    sin(
                        ((i - 0 as libc::c_int) as libc::c_double + 0.5f64)
                            / n2 as libc::c_double * 0.5f64 * 3.14159265358979323846f64,
                    ) as libc::c_float,
                ) as libc::c_double,
        ) as libc::c_float;
        i += 1;
    }
}
unsafe extern "C" fn compute_bitreverse(mut n: libc::c_int, mut rev: *mut uint16) {
    let mut ld: libc::c_int = ilog(n) - 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut n8: libc::c_int = n >> 3 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n8 {
        *rev
            .offset(
                i as isize,
            ) = ((bit_reverse(i as libc::c_uint)
            >> 32 as libc::c_int - ld + 3 as libc::c_int) << 2 as libc::c_int) as uint16;
        i += 1;
    }
}
unsafe extern "C" fn init_blocksize(
    mut f: *mut vorb,
    mut b: libc::c_int,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut n2: libc::c_int = n >> 1 as libc::c_int;
    let mut n4: libc::c_int = n >> 2 as libc::c_int;
    let mut n8: libc::c_int = n >> 3 as libc::c_int;
    let ref mut fresh5 = (*f).A[b as usize];
    *fresh5 = setup_malloc(
        f,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(n2 as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_float;
    let ref mut fresh6 = (*f).B[b as usize];
    *fresh6 = setup_malloc(
        f,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(n2 as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_float;
    let ref mut fresh7 = (*f).C[b as usize];
    *fresh7 = setup_malloc(
        f,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(n4 as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_float;
    if ((*f).A[b as usize]).is_null() || ((*f).B[b as usize]).is_null()
        || ((*f).C[b as usize]).is_null()
    {
        return error(f, VORBIS_outofmem);
    }
    compute_twiddle_factors(
        n,
        (*f).A[b as usize],
        (*f).B[b as usize],
        (*f).C[b as usize],
    );
    let ref mut fresh8 = (*f).window[b as usize];
    *fresh8 = setup_malloc(
        f,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(n2 as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_float;
    if ((*f).window[b as usize]).is_null() {
        return error(f, VORBIS_outofmem);
    }
    compute_window(n, (*f).window[b as usize]);
    let ref mut fresh9 = (*f).bit_reverse[b as usize];
    *fresh9 = setup_malloc(
        f,
        (::std::mem::size_of::<uint16>() as libc::c_ulong)
            .wrapping_mul(n8 as libc::c_ulong) as libc::c_int,
    ) as *mut uint16;
    if ((*f).bit_reverse[b as usize]).is_null() {
        return error(f, VORBIS_outofmem);
    }
    compute_bitreverse(n, (*f).bit_reverse[b as usize]);
    return 1 as libc::c_int;
}
unsafe extern "C" fn neighbors(
    mut x: *mut uint16,
    mut n: libc::c_int,
    mut plow: *mut libc::c_int,
    mut phigh: *mut libc::c_int,
) {
    let mut low: libc::c_int = -(1 as libc::c_int);
    let mut high: libc::c_int = 65536 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if *x.offset(i as isize) as libc::c_int > low
            && (*x.offset(i as isize) as libc::c_int)
                < *x.offset(n as isize) as libc::c_int
        {
            *plow = i;
            low = *x.offset(i as isize) as libc::c_int;
        }
        if (*x.offset(i as isize) as libc::c_int) < high
            && *x.offset(i as isize) as libc::c_int
                > *x.offset(n as isize) as libc::c_int
        {
            *phigh = i;
            high = *x.offset(i as isize) as libc::c_int;
        }
        i += 1;
    }
}
unsafe extern "C" fn point_compare(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    let mut a: *mut stbv__floor_ordering = p as *mut stbv__floor_ordering;
    let mut b: *mut stbv__floor_ordering = q as *mut stbv__floor_ordering;
    return if ((*a).x as libc::c_int) < (*b).x as libc::c_int {
        -(1 as libc::c_int)
    } else {
        ((*a).x as libc::c_int > (*b).x as libc::c_int) as libc::c_int
    };
}
unsafe extern "C" fn get8(mut z: *mut vorb) -> uint8 {
    if !((*z).stream).is_null() {
        if (*z).stream >= (*z).stream_end {
            (*z).eof = 1 as libc::c_int;
            return 0 as libc::c_int as uint8;
        }
        let ref mut fresh10 = (*z).stream;
        let fresh11 = *fresh10;
        *fresh10 = (*fresh10).offset(1);
        return *fresh11;
    }
    let mut c: libc::c_int = fgetc((*z).f);
    if c == -(1 as libc::c_int) {
        (*z).eof = 1 as libc::c_int;
        return 0 as libc::c_int as uint8;
    }
    return c as uint8;
}
unsafe extern "C" fn get32(mut f: *mut vorb) -> uint32 {
    let mut x: uint32 = 0;
    x = get8(f) as uint32;
    x = (x as libc::c_uint)
        .wrapping_add(((get8(f) as libc::c_int) << 8 as libc::c_int) as libc::c_uint)
        as uint32 as uint32;
    x = (x as libc::c_uint)
        .wrapping_add(((get8(f) as libc::c_int) << 16 as libc::c_int) as libc::c_uint)
        as uint32 as uint32;
    x = (x as libc::c_uint).wrapping_add((get8(f) as uint32) << 24 as libc::c_int)
        as uint32 as uint32;
    return x;
}
unsafe extern "C" fn getn(
    mut z: *mut vorb,
    mut data: *mut uint8,
    mut n: libc::c_int,
) -> libc::c_int {
    if !((*z).stream).is_null() {
        if ((*z).stream).offset(n as isize) > (*z).stream_end {
            (*z).eof = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        memcpy(
            data as *mut libc::c_void,
            (*z).stream as *const libc::c_void,
            n as libc::c_ulong,
        );
        let ref mut fresh12 = (*z).stream;
        *fresh12 = (*fresh12).offset(n as isize);
        return 1 as libc::c_int;
    }
    if fread(data as *mut libc::c_void, n as size_t, 1 as libc::c_int as size_t, (*z).f)
        == 1 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int
    } else {
        (*z).eof = 1 as libc::c_int;
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn skip(mut z: *mut vorb, mut n: libc::c_int) {
    if !((*z).stream).is_null() {
        let ref mut fresh13 = (*z).stream;
        *fresh13 = (*fresh13).offset(n as isize);
        if (*z).stream >= (*z).stream_end {
            (*z).eof = 1 as libc::c_int;
        }
        return;
    }
    let mut x: libc::c_long = ftell((*z).f);
    fseek((*z).f, x + n as libc::c_long, 0 as libc::c_int);
}
unsafe extern "C" fn set_file_offset(
    mut f: *mut stb_vorbis,
    mut loc: libc::c_uint,
) -> libc::c_int {
    if (*f).push_mode != 0 {
        return 0 as libc::c_int;
    }
    (*f).eof = 0 as libc::c_int;
    if !((*f).stream).is_null() {
        if ((*f).stream_start).offset(loc as isize) >= (*f).stream_end
            || ((*f).stream_start).offset(loc as isize) < (*f).stream_start
        {
            let ref mut fresh14 = (*f).stream;
            *fresh14 = (*f).stream_end;
            (*f).eof = 1 as libc::c_int;
            return 0 as libc::c_int;
        } else {
            let ref mut fresh15 = (*f).stream;
            *fresh15 = ((*f).stream_start).offset(loc as isize);
            return 1 as libc::c_int;
        }
    }
    if loc.wrapping_add((*f).f_start) < loc || loc >= 0x80000000 as libc::c_uint {
        loc = 0x7fffffff as libc::c_int as libc::c_uint;
        (*f).eof = 1 as libc::c_int;
    } else {
        loc = loc.wrapping_add((*f).f_start);
    }
    if fseek((*f).f, loc as libc::c_long, 0 as libc::c_int) == 0 {
        return 1 as libc::c_int;
    }
    (*f).eof = 1 as libc::c_int;
    fseek((*f).f, (*f).f_start as libc::c_long, 2 as libc::c_int);
    return 0 as libc::c_int;
}
static mut ogg_page_header: [uint8; 4] = [
    0x4f as libc::c_int as uint8,
    0x67 as libc::c_int as uint8,
    0x67 as libc::c_int as uint8,
    0x53 as libc::c_int as uint8,
];
unsafe extern "C" fn capture_pattern(mut f: *mut vorb) -> libc::c_int {
    if 0x4f as libc::c_int != get8(f) as libc::c_int {
        return 0 as libc::c_int;
    }
    if 0x67 as libc::c_int != get8(f) as libc::c_int {
        return 0 as libc::c_int;
    }
    if 0x67 as libc::c_int != get8(f) as libc::c_int {
        return 0 as libc::c_int;
    }
    if 0x53 as libc::c_int != get8(f) as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn start_page_no_capturepattern(mut f: *mut vorb) -> libc::c_int {
    let mut loc0: uint32 = 0;
    let mut loc1: uint32 = 0;
    let mut n: uint32 = 0;
    if (*f).first_decode as libc::c_int != 0 && (*f).push_mode == 0 {
        (*f)
            .p_first
            .page_start = (stb_vorbis_get_file_offset(f))
            .wrapping_sub(4 as libc::c_int as libc::c_uint);
    }
    if 0 as libc::c_int != get8(f) as libc::c_int {
        return error(f, VORBIS_invalid_stream_structure_version);
    }
    (*f).page_flag = get8(f);
    loc0 = get32(f);
    loc1 = get32(f);
    get32(f);
    n = get32(f);
    (*f).last_page = n as libc::c_int;
    get32(f);
    (*f).segment_count = get8(f) as libc::c_int;
    if getn(f, ((*f).segments).as_mut_ptr(), (*f).segment_count) == 0 {
        return error(f, VORBIS_unexpected_eof);
    }
    (*f).end_seg_with_known_loc = -(2 as libc::c_int);
    if loc0 != !(0 as libc::c_uint) || loc1 != !(0 as libc::c_uint) {
        let mut i: libc::c_int = 0;
        i = (*f).segment_count - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if ((*f).segments[i as usize] as libc::c_int) < 255 as libc::c_int {
                break;
            }
            i -= 1;
        }
        if i >= 0 as libc::c_int {
            (*f).end_seg_with_known_loc = i;
            (*f).known_loc_for_packet = loc0;
        }
    }
    if (*f).first_decode != 0 {
        let mut i_0: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        len = 0 as libc::c_int;
        i_0 = 0 as libc::c_int;
        while i_0 < (*f).segment_count {
            len += (*f).segments[i_0 as usize] as libc::c_int;
            i_0 += 1;
        }
        len += 27 as libc::c_int + (*f).segment_count;
        (*f)
            .p_first
            .page_end = ((*f).p_first.page_start).wrapping_add(len as libc::c_uint);
        (*f).p_first.last_decoded_sample = loc0;
    }
    (*f).next_seg = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn start_page(mut f: *mut vorb) -> libc::c_int {
    if capture_pattern(f) == 0 {
        return error(f, VORBIS_missing_capture_pattern);
    }
    return start_page_no_capturepattern(f);
}
unsafe extern "C" fn start_packet(mut f: *mut vorb) -> libc::c_int {
    while (*f).next_seg == -(1 as libc::c_int) {
        if start_page(f) == 0 {
            return 0 as libc::c_int;
        }
        if (*f).page_flag as libc::c_int & 1 as libc::c_int != 0 {
            return error(f, VORBIS_continued_packet_flag_invalid);
        }
    }
    (*f).last_seg = 0 as libc::c_int;
    (*f).valid_bits = 0 as libc::c_int;
    (*f).packet_bytes = 0 as libc::c_int;
    (*f).bytes_in_seg = 0 as libc::c_int as uint8;
    return 1 as libc::c_int;
}
unsafe extern "C" fn maybe_start_packet(mut f: *mut vorb) -> libc::c_int {
    if (*f).next_seg == -(1 as libc::c_int) {
        let mut x: libc::c_int = get8(f) as libc::c_int;
        if (*f).eof != 0 {
            return 0 as libc::c_int;
        }
        if 0x4f as libc::c_int != x {
            return error(f, VORBIS_missing_capture_pattern);
        }
        if 0x67 as libc::c_int != get8(f) as libc::c_int {
            return error(f, VORBIS_missing_capture_pattern);
        }
        if 0x67 as libc::c_int != get8(f) as libc::c_int {
            return error(f, VORBIS_missing_capture_pattern);
        }
        if 0x53 as libc::c_int != get8(f) as libc::c_int {
            return error(f, VORBIS_missing_capture_pattern);
        }
        if start_page_no_capturepattern(f) == 0 {
            return 0 as libc::c_int;
        }
        if (*f).page_flag as libc::c_int & 1 as libc::c_int != 0 {
            (*f).last_seg = 0 as libc::c_int;
            (*f).bytes_in_seg = 0 as libc::c_int as uint8;
            return error(f, VORBIS_continued_packet_flag_invalid);
        }
    }
    return start_packet(f);
}
unsafe extern "C" fn next_segment(mut f: *mut vorb) -> libc::c_int {
    let mut len: libc::c_int = 0;
    if (*f).last_seg != 0 {
        return 0 as libc::c_int;
    }
    if (*f).next_seg == -(1 as libc::c_int) {
        (*f).last_seg_which = (*f).segment_count - 1 as libc::c_int;
        if start_page(f) == 0 {
            (*f).last_seg = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        if (*f).page_flag as libc::c_int & 1 as libc::c_int == 0 {
            return error(f, VORBIS_continued_packet_flag_invalid);
        }
    }
    let ref mut fresh16 = (*f).next_seg;
    let fresh17 = *fresh16;
    *fresh16 = *fresh16 + 1;
    len = (*f).segments[fresh17 as usize] as libc::c_int;
    if len < 255 as libc::c_int {
        (*f).last_seg = 1 as libc::c_int;
        (*f).last_seg_which = (*f).next_seg - 1 as libc::c_int;
    }
    if (*f).next_seg >= (*f).segment_count {
        (*f).next_seg = -(1 as libc::c_int);
    }
    (*f).bytes_in_seg = len as uint8;
    return len;
}
unsafe extern "C" fn get8_packet_raw(mut f: *mut vorb) -> libc::c_int {
    if (*f).bytes_in_seg == 0 {
        if (*f).last_seg != 0 {
            return -(1 as libc::c_int)
        } else {
            if next_segment(f) == 0 {
                return -(1 as libc::c_int);
            }
        }
    }
    let ref mut fresh18 = (*f).bytes_in_seg;
    *fresh18 = (*fresh18).wrapping_sub(1);
    let ref mut fresh19 = (*f).packet_bytes;
    *fresh19 += 1;
    return get8(f) as libc::c_int;
}
unsafe extern "C" fn get8_packet(mut f: *mut vorb) -> libc::c_int {
    let mut x: libc::c_int = get8_packet_raw(f);
    (*f).valid_bits = 0 as libc::c_int;
    return x;
}
unsafe extern "C" fn get32_packet(mut f: *mut vorb) -> libc::c_int {
    let mut x: uint32 = 0;
    x = get8_packet(f) as uint32;
    x = (x as libc::c_uint)
        .wrapping_add((get8_packet(f) << 8 as libc::c_int) as libc::c_uint) as uint32
        as uint32;
    x = (x as libc::c_uint)
        .wrapping_add((get8_packet(f) << 16 as libc::c_int) as libc::c_uint) as uint32
        as uint32;
    x = (x as libc::c_uint).wrapping_add((get8_packet(f) as uint32) << 24 as libc::c_int)
        as uint32 as uint32;
    return x as libc::c_int;
}
unsafe extern "C" fn flush_packet(mut f: *mut vorb) {
    while get8_packet_raw(f) != -(1 as libc::c_int) {}
}
unsafe extern "C" fn get_bits(mut f: *mut vorb, mut n: libc::c_int) -> uint32 {
    let mut z: uint32 = 0;
    if (*f).valid_bits < 0 as libc::c_int {
        return 0 as libc::c_int as uint32;
    }
    if (*f).valid_bits < n {
        if n > 24 as libc::c_int {
            z = get_bits(f, 24 as libc::c_int);
            z = (z as libc::c_uint)
                .wrapping_add(get_bits(f, n - 24 as libc::c_int) << 24 as libc::c_int)
                as uint32 as uint32;
            return z;
        }
        if (*f).valid_bits == 0 as libc::c_int {
            (*f).acc = 0 as libc::c_int as uint32;
        }
        while (*f).valid_bits < n {
            let mut z_0: libc::c_int = get8_packet_raw(f);
            if z_0 == -(1 as libc::c_int) {
                (*f).valid_bits = -(1 as libc::c_int);
                return 0 as libc::c_int as uint32;
            }
            let ref mut fresh20 = (*f).acc;
            *fresh20 = (*fresh20 as libc::c_uint)
                .wrapping_add((z_0 << (*f).valid_bits) as libc::c_uint) as uint32
                as uint32;
            (*f).valid_bits += 8 as libc::c_int;
        }
    }
    z = (*f).acc & (((1 as libc::c_int) << n) - 1 as libc::c_int) as libc::c_uint;
    (*f).acc >>= n;
    (*f).valid_bits -= n;
    return z;
}
#[inline]
unsafe extern "C" fn prep_huffman(mut f: *mut vorb) {
    if (*f).valid_bits <= 24 as libc::c_int {
        if (*f).valid_bits == 0 as libc::c_int {
            (*f).acc = 0 as libc::c_int as uint32;
        }
        loop {
            let mut z: libc::c_int = 0;
            if (*f).last_seg != 0 && (*f).bytes_in_seg == 0 {
                return;
            }
            z = get8_packet_raw(f);
            if z == -(1 as libc::c_int) {
                return;
            }
            let ref mut fresh21 = (*f).acc;
            *fresh21 = (*fresh21 as libc::c_uint)
                .wrapping_add((z as libc::c_uint) << (*f).valid_bits) as uint32
                as uint32;
            (*f).valid_bits += 8 as libc::c_int;
            if !((*f).valid_bits <= 24 as libc::c_int) {
                break;
            }
        }
    }
}
unsafe extern "C" fn codebook_decode_scalar_raw(
    mut f: *mut vorb,
    mut c: *mut Codebook,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    prep_huffman(f);
    if ((*c).codewords).is_null() && ((*c).sorted_codewords).is_null() {
        return -(1 as libc::c_int);
    }
    if if (*c).entries > 8 as libc::c_int {
        ((*c).sorted_codewords != 0 as *mut libc::c_void as *mut uint32) as libc::c_int
    } else {
        ((*c).codewords).is_null() as libc::c_int
    } != 0
    {
        let mut code: uint32 = bit_reverse((*f).acc);
        let mut x: libc::c_int = 0 as libc::c_int;
        let mut n: libc::c_int = (*c).sorted_entries;
        let mut len: libc::c_int = 0;
        while n > 1 as libc::c_int {
            let mut m: libc::c_int = x + (n >> 1 as libc::c_int);
            if *((*c).sorted_codewords).offset(m as isize) <= code {
                x = m;
                n -= n >> 1 as libc::c_int;
            } else {
                n >>= 1 as libc::c_int;
            }
        }
        if (*c).sparse == 0 {
            x = *((*c).sorted_values).offset(x as isize);
        }
        len = *((*c).codeword_lengths).offset(x as isize) as libc::c_int;
        if (*f).valid_bits >= len {
            (*f).acc >>= len;
            (*f).valid_bits -= len;
            return x;
        }
        (*f).valid_bits = 0 as libc::c_int;
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*c).entries {
        if !(*((*c).codeword_lengths).offset(i as isize) as libc::c_int
            == 255 as libc::c_int)
        {
            if *((*c).codewords).offset(i as isize)
                == (*f).acc
                    & (((1 as libc::c_int)
                        << *((*c).codeword_lengths).offset(i as isize) as libc::c_int)
                        - 1 as libc::c_int) as libc::c_uint
            {
                if (*f).valid_bits
                    >= *((*c).codeword_lengths).offset(i as isize) as libc::c_int
                {
                    (*f).acc
                        >>= *((*c).codeword_lengths).offset(i as isize) as libc::c_int;
                    (*f).valid_bits
                        -= *((*c).codeword_lengths).offset(i as isize) as libc::c_int;
                    return i;
                }
                (*f).valid_bits = 0 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
        i += 1;
    }
    error(f, VORBIS_invalid_stream);
    (*f).valid_bits = 0 as libc::c_int;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn codebook_decode_start(
    mut f: *mut vorb,
    mut c: *mut Codebook,
) -> libc::c_int {
    let mut z: libc::c_int = -(1 as libc::c_int);
    if (*c).lookup_type as libc::c_int == 0 as libc::c_int {
        error(f, VORBIS_invalid_stream);
    } else {
        if (*f).valid_bits < 10 as libc::c_int {
            prep_huffman(f);
        }
        z = ((*f).acc
            & (((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint) as libc::c_int;
        z = (*c).fast_huffman[z as usize] as libc::c_int;
        if z >= 0 as libc::c_int {
            let mut n: libc::c_int = *((*c).codeword_lengths).offset(z as isize)
                as libc::c_int;
            (*f).acc >>= n;
            (*f).valid_bits -= n;
            if (*f).valid_bits < 0 as libc::c_int {
                (*f).valid_bits = 0 as libc::c_int;
                z = -(1 as libc::c_int);
            }
        } else {
            z = codebook_decode_scalar_raw(f, c);
        }
        (*c).sparse != 0;
        if z < 0 as libc::c_int {
            if (*f).bytes_in_seg == 0 {
                if (*f).last_seg != 0 {
                    return z;
                }
            }
            error(f, VORBIS_invalid_stream);
        }
    }
    return z;
}
unsafe extern "C" fn codebook_decode(
    mut f: *mut vorb,
    mut c: *mut Codebook,
    mut output: *mut libc::c_float,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut z: libc::c_int = codebook_decode_start(f, c);
    if z < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if len > (*c).dimensions {
        len = (*c).dimensions;
    }
    z *= (*c).dimensions;
    if (*c).sequence_p != 0 {
        let mut last: libc::c_float = 0 as libc::c_int as libc::c_float;
        i = 0 as libc::c_int;
        while i < len {
            let mut val: libc::c_float = *((*c).multiplicands).offset((z + i) as isize)
                + last;
            *output.offset(i as isize) += val;
            last = val + (*c).minimum_value;
            i += 1;
        }
    } else {
        let mut last_0: libc::c_float = 0 as libc::c_int as libc::c_float;
        i = 0 as libc::c_int;
        while i < len {
            *output.offset(i as isize)
                += *((*c).multiplicands).offset((z + i) as isize) + last_0;
            i += 1;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn codebook_decode_step(
    mut f: *mut vorb,
    mut c: *mut Codebook,
    mut output: *mut libc::c_float,
    mut len: libc::c_int,
    mut step: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut z: libc::c_int = codebook_decode_start(f, c);
    let mut last: libc::c_float = 0 as libc::c_int as libc::c_float;
    if z < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if len > (*c).dimensions {
        len = (*c).dimensions;
    }
    z *= (*c).dimensions;
    i = 0 as libc::c_int;
    while i < len {
        let mut val: libc::c_float = *((*c).multiplicands).offset((z + i) as isize)
            + last;
        *output.offset((i * step) as isize) += val;
        if (*c).sequence_p != 0 {
            last = val;
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn codebook_decode_deinterleave_repeat(
    mut f: *mut vorb,
    mut c: *mut Codebook,
    mut outputs: *mut *mut libc::c_float,
    mut ch: libc::c_int,
    mut c_inter_p: *mut libc::c_int,
    mut p_inter_p: *mut libc::c_int,
    mut len: libc::c_int,
    mut total_decode: libc::c_int,
) -> libc::c_int {
    let mut c_inter: libc::c_int = *c_inter_p;
    let mut p_inter: libc::c_int = *p_inter_p;
    let mut i: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut effective: libc::c_int = (*c).dimensions;
    if (*c).lookup_type as libc::c_int == 0 as libc::c_int {
        return error(f, VORBIS_invalid_stream);
    }
    while total_decode > 0 as libc::c_int {
        let mut last: libc::c_float = 0 as libc::c_int as libc::c_float;
        if (*f).valid_bits < 10 as libc::c_int {
            prep_huffman(f);
        }
        z = ((*f).acc
            & (((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint) as libc::c_int;
        z = (*c).fast_huffman[z as usize] as libc::c_int;
        if z >= 0 as libc::c_int {
            let mut n: libc::c_int = *((*c).codeword_lengths).offset(z as isize)
                as libc::c_int;
            (*f).acc >>= n;
            (*f).valid_bits -= n;
            if (*f).valid_bits < 0 as libc::c_int {
                (*f).valid_bits = 0 as libc::c_int;
                z = -(1 as libc::c_int);
            }
        } else {
            z = codebook_decode_scalar_raw(f, c);
        }
        if z < 0 as libc::c_int {
            if (*f).bytes_in_seg == 0 {
                if (*f).last_seg != 0 {
                    return 0 as libc::c_int;
                }
            }
            return error(f, VORBIS_invalid_stream);
        }
        if c_inter + p_inter * ch + effective > len * ch {
            effective = len * ch - (p_inter * ch - c_inter);
        }
        z *= (*c).dimensions;
        if (*c).sequence_p != 0 {
            i = 0 as libc::c_int;
            while i < effective {
                let mut val: libc::c_float = *((*c).multiplicands)
                    .offset((z + i) as isize) + last;
                if !(*outputs.offset(c_inter as isize)).is_null() {
                    *(*outputs.offset(c_inter as isize)).offset(p_inter as isize) += val;
                }
                c_inter += 1;
                if c_inter == ch {
                    c_inter = 0 as libc::c_int;
                    p_inter += 1;
                }
                last = val;
                i += 1;
            }
        } else {
            i = 0 as libc::c_int;
            while i < effective {
                let mut val_0: libc::c_float = *((*c).multiplicands)
                    .offset((z + i) as isize) + last;
                if !(*outputs.offset(c_inter as isize)).is_null() {
                    *(*outputs.offset(c_inter as isize)).offset(p_inter as isize)
                        += val_0;
                }
                c_inter += 1;
                if c_inter == ch {
                    c_inter = 0 as libc::c_int;
                    p_inter += 1;
                }
                i += 1;
            }
        }
        total_decode -= effective;
    }
    *c_inter_p = c_inter;
    *p_inter_p = p_inter;
    return 1 as libc::c_int;
}
unsafe extern "C" fn predict_point(
    mut x: libc::c_int,
    mut x0: libc::c_int,
    mut x1: libc::c_int,
    mut y0: libc::c_int,
    mut y1: libc::c_int,
) -> libc::c_int {
    let mut dy: libc::c_int = y1 - y0;
    let mut adx: libc::c_int = x1 - x0;
    let mut err: libc::c_int = abs(dy) * (x - x0);
    let mut off: libc::c_int = err / adx;
    return if dy < 0 as libc::c_int { y0 - off } else { y0 + off };
}
static mut inverse_db_table: [libc::c_float; 256] = [
    1.0649863e-07f32,
    1.1341951e-07f32,
    1.2079015e-07f32,
    1.2863978e-07f32,
    1.3699951e-07f32,
    1.4590251e-07f32,
    1.5538408e-07f32,
    1.6548181e-07f32,
    1.7623575e-07f32,
    1.8768855e-07f32,
    1.9988561e-07f32,
    2.1287530e-07f32,
    2.2670913e-07f32,
    2.4144197e-07f32,
    2.5713223e-07f32,
    2.7384213e-07f32,
    2.9163793e-07f32,
    3.1059021e-07f32,
    3.3077411e-07f32,
    3.5226968e-07f32,
    3.7516214e-07f32,
    3.9954229e-07f32,
    4.2550680e-07f32,
    4.5315863e-07f32,
    4.8260743e-07f32,
    5.1396998e-07f32,
    5.4737065e-07f32,
    5.8294187e-07f32,
    6.2082472e-07f32,
    6.6116941e-07f32,
    7.0413592e-07f32,
    7.4989464e-07f32,
    7.9862701e-07f32,
    8.5052630e-07f32,
    9.0579828e-07f32,
    9.6466216e-07f32,
    1.0273513e-06f32,
    1.0941144e-06f32,
    1.1652161e-06f32,
    1.2409384e-06f32,
    1.3215816e-06f32,
    1.4074654e-06f32,
    1.4989305e-06f32,
    1.5963394e-06f32,
    1.7000785e-06f32,
    1.8105592e-06f32,
    1.9282195e-06f32,
    2.0535261e-06f32,
    2.1869758e-06f32,
    2.3290978e-06f32,
    2.4804557e-06f32,
    2.6416497e-06f32,
    2.8133190e-06f32,
    2.9961443e-06f32,
    3.1908506e-06f32,
    3.3982101e-06f32,
    3.6190449e-06f32,
    3.8542308e-06f32,
    4.1047004e-06f32,
    4.3714470e-06f32,
    4.6555282e-06f32,
    4.9580707e-06f32,
    5.2802740e-06f32,
    5.6234160e-06f32,
    5.9888572e-06f32,
    6.3780469e-06f32,
    6.7925283e-06f32,
    7.2339451e-06f32,
    7.7040476e-06f32,
    8.2047000e-06f32,
    8.7378876e-06f32,
    9.3057248e-06f32,
    9.9104632e-06f32,
    1.0554501e-05f32,
    1.1240392e-05f32,
    1.1970856e-05f32,
    1.2748789e-05f32,
    1.3577278e-05f32,
    1.4459606e-05f32,
    1.5399272e-05f32,
    1.6400004e-05f32,
    1.7465768e-05f32,
    1.8600792e-05f32,
    1.9809576e-05f32,
    2.1096914e-05f32,
    2.2467911e-05f32,
    2.3928002e-05f32,
    2.5482978e-05f32,
    2.7139006e-05f32,
    2.8902651e-05f32,
    3.0780908e-05f32,
    3.2781225e-05f32,
    3.4911534e-05f32,
    3.7180282e-05f32,
    3.9596466e-05f32,
    4.2169667e-05f32,
    4.4910090e-05f32,
    4.7828601e-05f32,
    5.0936773e-05f32,
    5.4246931e-05f32,
    5.7772202e-05f32,
    6.1526565e-05f32,
    6.5524908e-05f32,
    6.9783085e-05f32,
    7.4317983e-05f32,
    7.9147585e-05f32,
    8.4291040e-05f32,
    8.9768747e-05f32,
    9.5602426e-05f32,
    0.00010181521f32,
    0.00010843174f32,
    0.00011547824f32,
    0.00012298267f32,
    0.00013097477f32,
    0.00013948625f32,
    0.00014855085f32,
    0.00015820453f32,
    0.00016848555f32,
    0.00017943469f32,
    0.00019109536f32,
    0.00020351382f32,
    0.00021673929f32,
    0.00023082423f32,
    0.00024582449f32,
    0.00026179955f32,
    0.00027881276f32,
    0.00029693158f32,
    0.00031622787f32,
    0.00033677814f32,
    0.00035866388f32,
    0.00038197188f32,
    0.00040679456f32,
    0.00043323036f32,
    0.00046138411f32,
    0.00049136745f32,
    0.00052329927f32,
    0.00055730621f32,
    0.00059352311f32,
    0.00063209358f32,
    0.00067317058f32,
    0.00071691700f32,
    0.00076350630f32,
    0.00081312324f32,
    0.00086596457f32,
    0.00092223983f32,
    0.00098217216f32,
    0.0010459992f32,
    0.0011139742f32,
    0.0011863665f32,
    0.0012634633f32,
    0.0013455702f32,
    0.0014330129f32,
    0.0015261382f32,
    0.0016253153f32,
    0.0017309374f32,
    0.0018434235f32,
    0.0019632195f32,
    0.0020908006f32,
    0.0022266726f32,
    0.0023713743f32,
    0.0025254795f32,
    0.0026895994f32,
    0.0028643847f32,
    0.0030505286f32,
    0.0032487691f32,
    0.0034598925f32,
    0.0036847358f32,
    0.0039241906f32,
    0.0041792066f32,
    0.0044507950f32,
    0.0047400328f32,
    0.0050480668f32,
    0.0053761186f32,
    0.0057254891f32,
    0.0060975636f32,
    0.0064938176f32,
    0.0069158225f32,
    0.0073652516f32,
    0.0078438871f32,
    0.0083536271f32,
    0.0088964928f32,
    0.009474637f32,
    0.010090352f32,
    0.010746080f32,
    0.011444421f32,
    0.012188144f32,
    0.012980198f32,
    0.013823725f32,
    0.014722068f32,
    0.015678791f32,
    0.016697687f32,
    0.017782797f32,
    0.018938423f32,
    0.020169149f32,
    0.021479854f32,
    0.022875735f32,
    0.024362330f32,
    0.025945531f32,
    0.027631618f32,
    0.029427276f32,
    0.031339626f32,
    0.033376252f32,
    0.035545228f32,
    0.037855157f32,
    0.040315199f32,
    0.042935108f32,
    0.045725273f32,
    0.048696758f32,
    0.051861348f32,
    0.055231591f32,
    0.058820850f32,
    0.062643361f32,
    0.066714279f32,
    0.071049749f32,
    0.075666962f32,
    0.080584227f32,
    0.085821044f32,
    0.091398179f32,
    0.097337747f32,
    0.10366330f32,
    0.11039993f32,
    0.11757434f32,
    0.12521498f32,
    0.13335215f32,
    0.14201813f32,
    0.15124727f32,
    0.16107617f32,
    0.17154380f32,
    0.18269168f32,
    0.19456402f32,
    0.20720788f32,
    0.22067342f32,
    0.23501402f32,
    0.25028656f32,
    0.26655159f32,
    0.28387361f32,
    0.30232132f32,
    0.32196786f32,
    0.34289114f32,
    0.36517414f32,
    0.38890521f32,
    0.41417847f32,
    0.44109412f32,
    0.46975890f32,
    0.50028648f32,
    0.53279791f32,
    0.56742212f32,
    0.60429640f32,
    0.64356699f32,
    0.68538959f32,
    0.72993007f32,
    0.77736504f32,
    0.82788260f32,
    0.88168307f32,
    0.9389798f32,
    1.0f32,
];
#[inline]
unsafe extern "C" fn draw_line(
    mut output: *mut libc::c_float,
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut n: libc::c_int,
) {
    let mut dy: libc::c_int = y1 - y0;
    let mut adx: libc::c_int = x1 - x0;
    let mut ady: libc::c_int = abs(dy);
    let mut base: libc::c_int = 0;
    let mut x: libc::c_int = x0;
    let mut y: libc::c_int = y0;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut sy: libc::c_int = 0;
    base = dy / adx;
    if dy < 0 as libc::c_int {
        sy = base - 1 as libc::c_int;
    } else {
        sy = base + 1 as libc::c_int;
    }
    ady -= abs(base) * adx;
    if x1 > n {
        x1 = n;
    }
    if x < x1 {
        *output.offset(x as isize)
            *= inverse_db_table[(y & 255 as libc::c_int) as usize];
        x += 1;
        while x < x1 {
            err += ady;
            if err >= adx {
                err -= adx;
                y += sy;
            } else {
                y += base;
            }
            *output.offset(x as isize)
                *= inverse_db_table[(y & 255 as libc::c_int) as usize];
            x += 1;
        }
    }
}
unsafe extern "C" fn residue_decode(
    mut f: *mut vorb,
    mut book: *mut Codebook,
    mut target: *mut libc::c_float,
    mut offset: libc::c_int,
    mut n: libc::c_int,
    mut rtype: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    if rtype == 0 as libc::c_int {
        let mut step: libc::c_int = n / (*book).dimensions;
        k = 0 as libc::c_int;
        while k < step {
            if codebook_decode_step(
                f,
                book,
                target.offset(offset as isize).offset(k as isize),
                n - offset - k,
                step,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            k += 1;
        }
    } else {
        k = 0 as libc::c_int;
        while k < n {
            if codebook_decode(f, book, target.offset(offset as isize), n - k) == 0 {
                return 0 as libc::c_int;
            }
            k += (*book).dimensions;
            offset += (*book).dimensions;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn decode_residue(
    mut f: *mut vorb,
    mut residue_buffers: *mut *mut libc::c_float,
    mut ch: libc::c_int,
    mut n: libc::c_int,
    mut rn: libc::c_int,
    mut do_not_decode: *mut uint8,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pass: libc::c_int = 0;
    let mut r: *mut Residue = ((*f).residue_config).offset(rn as isize);
    let mut rtype: libc::c_int = (*f).residue_types[rn as usize] as libc::c_int;
    let mut c: libc::c_int = (*r).classbook as libc::c_int;
    let mut classwords: libc::c_int = (*((*f).codebooks).offset(c as isize)).dimensions;
    let mut actual_size: libc::c_uint = (if rtype == 2 as libc::c_int {
        n * 2 as libc::c_int
    } else {
        n
    }) as libc::c_uint;
    let mut limit_r_begin: libc::c_uint = if (*r).begin < actual_size {
        (*r).begin
    } else {
        actual_size
    };
    let mut limit_r_end: libc::c_uint = if (*r).end < actual_size {
        (*r).end
    } else {
        actual_size
    };
    let mut n_read: libc::c_int = limit_r_end.wrapping_sub(limit_r_begin) as libc::c_int;
    let mut part_read: libc::c_int = (n_read as libc::c_uint)
        .wrapping_div((*r).part_size) as libc::c_int;
    let mut temp_alloc_point: libc::c_int = (*f).temp_offset;
    let mut part_classdata: *mut *mut *mut uint8 = make_block_array(
        if !((*f).alloc.alloc_buffer).is_null() {
            setup_temp_malloc(
                f,
                ((*f).channels as libc::c_ulong)
                    .wrapping_mul(
                        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_add(
                                (part_read as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<*mut uint8>() as libc::c_ulong,
                                    ),
                            ),
                    ) as libc::c_int,
            )
        } else {
            let mut fresh22 = ::std::vec::from_elem(
                0,
                ((*f).channels as libc::c_ulong)
                    .wrapping_mul(
                        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_add(
                                (part_read as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<*mut uint8>() as libc::c_ulong,
                                    ),
                            ),
                    ) as usize,
            );
            fresh22.as_mut_ptr()
        },
        (*f).channels,
        (part_read as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut uint8>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut *mut *mut uint8;
    i = 0 as libc::c_int;
    while i < ch {
        if *do_not_decode.offset(i as isize) == 0 {
            memset(
                *residue_buffers.offset(i as isize) as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
        }
        i += 1;
    }
    if rtype == 2 as libc::c_int && ch != 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < ch {
            if *do_not_decode.offset(j as isize) == 0 {
                break;
            }
            j += 1;
        }
        if !(j == ch) {
            pass = 0 as libc::c_int;
            's_90: while pass < 8 as libc::c_int {
                let mut pcount: libc::c_int = 0 as libc::c_int;
                let mut class_set: libc::c_int = 0 as libc::c_int;
                if ch == 2 as libc::c_int {
                    while pcount < part_read {
                        let mut z: libc::c_int = ((*r).begin)
                            .wrapping_add(
                                (pcount as libc::c_uint).wrapping_mul((*r).part_size),
                            ) as libc::c_int;
                        let mut c_inter: libc::c_int = z & 1 as libc::c_int;
                        let mut p_inter: libc::c_int = z >> 1 as libc::c_int;
                        if pass == 0 as libc::c_int {
                            let mut c_0: *mut Codebook = ((*f).codebooks)
                                .offset((*r).classbook as libc::c_int as isize);
                            let mut q: libc::c_int = 0;
                            if (*f).valid_bits < 10 as libc::c_int {
                                prep_huffman(f);
                            }
                            q = ((*f).acc
                                & (((1 as libc::c_int) << 10 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as libc::c_int;
                            q = (*c_0).fast_huffman[q as usize] as libc::c_int;
                            if q >= 0 as libc::c_int {
                                let mut n_0: libc::c_int = *((*c_0).codeword_lengths)
                                    .offset(q as isize) as libc::c_int;
                                (*f).acc >>= n_0;
                                (*f).valid_bits -= n_0;
                                if (*f).valid_bits < 0 as libc::c_int {
                                    (*f).valid_bits = 0 as libc::c_int;
                                    q = -(1 as libc::c_int);
                                }
                            } else {
                                q = codebook_decode_scalar_raw(f, c_0);
                            }
                            if (*c_0).sparse != 0 {
                                q = *((*c_0).sorted_values).offset(q as isize);
                            }
                            if q == -(1 as libc::c_int) {
                                break 's_90;
                            }
                            let ref mut fresh23 = *(*part_classdata
                                .offset(0 as libc::c_int as isize))
                                .offset(class_set as isize);
                            *fresh23 = *((*r).classdata).offset(q as isize);
                        }
                        i = 0 as libc::c_int;
                        while i < classwords && pcount < part_read {
                            let mut z_0: libc::c_int = ((*r).begin)
                                .wrapping_add(
                                    (pcount as libc::c_uint).wrapping_mul((*r).part_size),
                                ) as libc::c_int;
                            let mut c_1: libc::c_int = *(*(*part_classdata
                                .offset(0 as libc::c_int as isize))
                                .offset(class_set as isize))
                                .offset(i as isize) as libc::c_int;
                            let mut b: libc::c_int = (*((*r).residue_books)
                                .offset(c_1 as isize))[pass as usize] as libc::c_int;
                            if b >= 0 as libc::c_int {
                                let mut book: *mut Codebook = ((*f).codebooks)
                                    .offset(b as isize);
                                if codebook_decode_deinterleave_repeat(
                                    f,
                                    book,
                                    residue_buffers,
                                    ch,
                                    &mut c_inter,
                                    &mut p_inter,
                                    n,
                                    (*r).part_size as libc::c_int,
                                ) == 0
                                {
                                    break 's_90;
                                }
                            } else {
                                z_0 = (z_0 as libc::c_uint).wrapping_add((*r).part_size)
                                    as libc::c_int as libc::c_int;
                                c_inter = z_0 & 1 as libc::c_int;
                                p_inter = z_0 >> 1 as libc::c_int;
                            }
                            i += 1;
                            pcount += 1;
                        }
                        class_set += 1;
                    }
                } else if ch > 2 as libc::c_int {
                    while pcount < part_read {
                        let mut z_1: libc::c_int = ((*r).begin)
                            .wrapping_add(
                                (pcount as libc::c_uint).wrapping_mul((*r).part_size),
                            ) as libc::c_int;
                        let mut c_inter_0: libc::c_int = z_1 % ch;
                        let mut p_inter_0: libc::c_int = z_1 / ch;
                        if pass == 0 as libc::c_int {
                            let mut c_2: *mut Codebook = ((*f).codebooks)
                                .offset((*r).classbook as libc::c_int as isize);
                            let mut q_0: libc::c_int = 0;
                            if (*f).valid_bits < 10 as libc::c_int {
                                prep_huffman(f);
                            }
                            q_0 = ((*f).acc
                                & (((1 as libc::c_int) << 10 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as libc::c_int;
                            q_0 = (*c_2).fast_huffman[q_0 as usize] as libc::c_int;
                            if q_0 >= 0 as libc::c_int {
                                let mut n_1: libc::c_int = *((*c_2).codeword_lengths)
                                    .offset(q_0 as isize) as libc::c_int;
                                (*f).acc >>= n_1;
                                (*f).valid_bits -= n_1;
                                if (*f).valid_bits < 0 as libc::c_int {
                                    (*f).valid_bits = 0 as libc::c_int;
                                    q_0 = -(1 as libc::c_int);
                                }
                            } else {
                                q_0 = codebook_decode_scalar_raw(f, c_2);
                            }
                            if (*c_2).sparse != 0 {
                                q_0 = *((*c_2).sorted_values).offset(q_0 as isize);
                            }
                            if q_0 == -(1 as libc::c_int) {
                                break 's_90;
                            }
                            let ref mut fresh24 = *(*part_classdata
                                .offset(0 as libc::c_int as isize))
                                .offset(class_set as isize);
                            *fresh24 = *((*r).classdata).offset(q_0 as isize);
                        }
                        i = 0 as libc::c_int;
                        while i < classwords && pcount < part_read {
                            let mut z_2: libc::c_int = ((*r).begin)
                                .wrapping_add(
                                    (pcount as libc::c_uint).wrapping_mul((*r).part_size),
                                ) as libc::c_int;
                            let mut c_3: libc::c_int = *(*(*part_classdata
                                .offset(0 as libc::c_int as isize))
                                .offset(class_set as isize))
                                .offset(i as isize) as libc::c_int;
                            let mut b_0: libc::c_int = (*((*r).residue_books)
                                .offset(c_3 as isize))[pass as usize] as libc::c_int;
                            if b_0 >= 0 as libc::c_int {
                                let mut book_0: *mut Codebook = ((*f).codebooks)
                                    .offset(b_0 as isize);
                                if codebook_decode_deinterleave_repeat(
                                    f,
                                    book_0,
                                    residue_buffers,
                                    ch,
                                    &mut c_inter_0,
                                    &mut p_inter_0,
                                    n,
                                    (*r).part_size as libc::c_int,
                                ) == 0
                                {
                                    break 's_90;
                                }
                            } else {
                                z_2 = (z_2 as libc::c_uint).wrapping_add((*r).part_size)
                                    as libc::c_int as libc::c_int;
                                c_inter_0 = z_2 % ch;
                                p_inter_0 = z_2 / ch;
                            }
                            i += 1;
                            pcount += 1;
                        }
                        class_set += 1;
                    }
                }
                pass += 1;
            }
        }
    } else {
        pass = 0 as libc::c_int;
        's_488: while pass < 8 as libc::c_int {
            let mut pcount_0: libc::c_int = 0 as libc::c_int;
            let mut class_set_0: libc::c_int = 0 as libc::c_int;
            while pcount_0 < part_read {
                if pass == 0 as libc::c_int {
                    j = 0 as libc::c_int;
                    while j < ch {
                        if *do_not_decode.offset(j as isize) == 0 {
                            let mut c_4: *mut Codebook = ((*f).codebooks)
                                .offset((*r).classbook as libc::c_int as isize);
                            let mut temp: libc::c_int = 0;
                            if (*f).valid_bits < 10 as libc::c_int {
                                prep_huffman(f);
                            }
                            temp = ((*f).acc
                                & (((1 as libc::c_int) << 10 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as libc::c_int;
                            temp = (*c_4).fast_huffman[temp as usize] as libc::c_int;
                            if temp >= 0 as libc::c_int {
                                let mut n_2: libc::c_int = *((*c_4).codeword_lengths)
                                    .offset(temp as isize) as libc::c_int;
                                (*f).acc >>= n_2;
                                (*f).valid_bits -= n_2;
                                if (*f).valid_bits < 0 as libc::c_int {
                                    (*f).valid_bits = 0 as libc::c_int;
                                    temp = -(1 as libc::c_int);
                                }
                            } else {
                                temp = codebook_decode_scalar_raw(f, c_4);
                            }
                            if (*c_4).sparse != 0 {
                                temp = *((*c_4).sorted_values).offset(temp as isize);
                            }
                            if temp == -(1 as libc::c_int) {
                                break 's_488;
                            }
                            let ref mut fresh25 = *(*part_classdata.offset(j as isize))
                                .offset(class_set_0 as isize);
                            *fresh25 = *((*r).classdata).offset(temp as isize);
                        }
                        j += 1;
                    }
                }
                i = 0 as libc::c_int;
                while i < classwords && pcount_0 < part_read {
                    j = 0 as libc::c_int;
                    while j < ch {
                        if *do_not_decode.offset(j as isize) == 0 {
                            let mut c_5: libc::c_int = *(*(*part_classdata
                                .offset(j as isize))
                                .offset(class_set_0 as isize))
                                .offset(i as isize) as libc::c_int;
                            let mut b_1: libc::c_int = (*((*r).residue_books)
                                .offset(c_5 as isize))[pass as usize] as libc::c_int;
                            if b_1 >= 0 as libc::c_int {
                                let mut target: *mut libc::c_float = *residue_buffers
                                    .offset(j as isize);
                                let mut offset: libc::c_int = ((*r).begin)
                                    .wrapping_add(
                                        (pcount_0 as libc::c_uint).wrapping_mul((*r).part_size),
                                    ) as libc::c_int;
                                let mut n_3: libc::c_int = (*r).part_size as libc::c_int;
                                let mut book_1: *mut Codebook = ((*f).codebooks)
                                    .offset(b_1 as isize);
                                if residue_decode(f, book_1, target, offset, n_3, rtype)
                                    == 0
                                {
                                    break 's_488;
                                }
                            }
                        }
                        j += 1;
                    }
                    i += 1;
                    pcount_0 += 1;
                }
                class_set_0 += 1;
            }
            pass += 1;
        }
    }
    (*f).temp_offset = temp_alloc_point;
}
unsafe extern "C" fn imdct_step3_iter0_loop(
    mut n: libc::c_int,
    mut e: *mut libc::c_float,
    mut i_off: libc::c_int,
    mut k_off: libc::c_int,
    mut A: *mut libc::c_float,
) {
    let mut ee0: *mut libc::c_float = e.offset(i_off as isize);
    let mut ee2: *mut libc::c_float = ee0.offset(k_off as isize);
    let mut i: libc::c_int = 0;
    i = n >> 2 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut k00_20: libc::c_float = 0.;
        let mut k01_21: libc::c_float = 0.;
        k00_20 = *ee0.offset(0 as libc::c_int as isize)
            - *ee2.offset(0 as libc::c_int as isize);
        k01_21 = *ee0.offset(-(1 as libc::c_int) as isize)
            - *ee2.offset(-(1 as libc::c_int) as isize);
        *ee0.offset(0 as libc::c_int as isize) += *ee2.offset(0 as libc::c_int as isize);
        *ee0.offset(-(1 as libc::c_int) as isize)
            += *ee2.offset(-(1 as libc::c_int) as isize);
        *ee2
            .offset(
                0 as libc::c_int as isize,
            ) = k00_20 * *A.offset(0 as libc::c_int as isize)
            - k01_21 * *A.offset(1 as libc::c_int as isize);
        *ee2
            .offset(
                -(1 as libc::c_int) as isize,
            ) = k01_21 * *A.offset(0 as libc::c_int as isize)
            + k00_20 * *A.offset(1 as libc::c_int as isize);
        A = A.offset(8 as libc::c_int as isize);
        k00_20 = *ee0.offset(-(2 as libc::c_int) as isize)
            - *ee2.offset(-(2 as libc::c_int) as isize);
        k01_21 = *ee0.offset(-(3 as libc::c_int) as isize)
            - *ee2.offset(-(3 as libc::c_int) as isize);
        *ee0.offset(-(2 as libc::c_int) as isize)
            += *ee2.offset(-(2 as libc::c_int) as isize);
        *ee0.offset(-(3 as libc::c_int) as isize)
            += *ee2.offset(-(3 as libc::c_int) as isize);
        *ee2
            .offset(
                -(2 as libc::c_int) as isize,
            ) = k00_20 * *A.offset(0 as libc::c_int as isize)
            - k01_21 * *A.offset(1 as libc::c_int as isize);
        *ee2
            .offset(
                -(3 as libc::c_int) as isize,
            ) = k01_21 * *A.offset(0 as libc::c_int as isize)
            + k00_20 * *A.offset(1 as libc::c_int as isize);
        A = A.offset(8 as libc::c_int as isize);
        k00_20 = *ee0.offset(-(4 as libc::c_int) as isize)
            - *ee2.offset(-(4 as libc::c_int) as isize);
        k01_21 = *ee0.offset(-(5 as libc::c_int) as isize)
            - *ee2.offset(-(5 as libc::c_int) as isize);
        *ee0.offset(-(4 as libc::c_int) as isize)
            += *ee2.offset(-(4 as libc::c_int) as isize);
        *ee0.offset(-(5 as libc::c_int) as isize)
            += *ee2.offset(-(5 as libc::c_int) as isize);
        *ee2
            .offset(
                -(4 as libc::c_int) as isize,
            ) = k00_20 * *A.offset(0 as libc::c_int as isize)
            - k01_21 * *A.offset(1 as libc::c_int as isize);
        *ee2
            .offset(
                -(5 as libc::c_int) as isize,
            ) = k01_21 * *A.offset(0 as libc::c_int as isize)
            + k00_20 * *A.offset(1 as libc::c_int as isize);
        A = A.offset(8 as libc::c_int as isize);
        k00_20 = *ee0.offset(-(6 as libc::c_int) as isize)
            - *ee2.offset(-(6 as libc::c_int) as isize);
        k01_21 = *ee0.offset(-(7 as libc::c_int) as isize)
            - *ee2.offset(-(7 as libc::c_int) as isize);
        *ee0.offset(-(6 as libc::c_int) as isize)
            += *ee2.offset(-(6 as libc::c_int) as isize);
        *ee0.offset(-(7 as libc::c_int) as isize)
            += *ee2.offset(-(7 as libc::c_int) as isize);
        *ee2
            .offset(
                -(6 as libc::c_int) as isize,
            ) = k00_20 * *A.offset(0 as libc::c_int as isize)
            - k01_21 * *A.offset(1 as libc::c_int as isize);
        *ee2
            .offset(
                -(7 as libc::c_int) as isize,
            ) = k01_21 * *A.offset(0 as libc::c_int as isize)
            + k00_20 * *A.offset(1 as libc::c_int as isize);
        A = A.offset(8 as libc::c_int as isize);
        ee0 = ee0.offset(-(8 as libc::c_int as isize));
        ee2 = ee2.offset(-(8 as libc::c_int as isize));
        i -= 1;
    }
}
unsafe extern "C" fn imdct_step3_inner_r_loop(
    mut lim: libc::c_int,
    mut e: *mut libc::c_float,
    mut d0: libc::c_int,
    mut k_off: libc::c_int,
    mut A: *mut libc::c_float,
    mut k1: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k00_20: libc::c_float = 0.;
    let mut k01_21: libc::c_float = 0.;
    let mut e0: *mut libc::c_float = e.offset(d0 as isize);
    let mut e2: *mut libc::c_float = e0.offset(k_off as isize);
    i = lim >> 2 as libc::c_int;
    while i > 0 as libc::c_int {
        k00_20 = *e0.offset(-(0 as libc::c_int) as isize)
            - *e2.offset(-(0 as libc::c_int) as isize);
        k01_21 = *e0.offset(-(1 as libc::c_int) as isize)
            - *e2.offset(-(1 as libc::c_int) as isize);
        *e0.offset(-(0 as libc::c_int) as isize)
            += *e2.offset(-(0 as libc::c_int) as isize);
        *e0.offset(-(1 as libc::c_int) as isize)
            += *e2.offset(-(1 as libc::c_int) as isize);
        *e2
            .offset(
                -(0 as libc::c_int) as isize,
            ) = k00_20 * *A.offset(0 as libc::c_int as isize)
            - k01_21 * *A.offset(1 as libc::c_int as isize);
        *e2
            .offset(
                -(1 as libc::c_int) as isize,
            ) = k01_21 * *A.offset(0 as libc::c_int as isize)
            + k00_20 * *A.offset(1 as libc::c_int as isize);
        A = A.offset(k1 as isize);
        k00_20 = *e0.offset(-(2 as libc::c_int) as isize)
            - *e2.offset(-(2 as libc::c_int) as isize);
        k01_21 = *e0.offset(-(3 as libc::c_int) as isize)
            - *e2.offset(-(3 as libc::c_int) as isize);
        *e0.offset(-(2 as libc::c_int) as isize)
            += *e2.offset(-(2 as libc::c_int) as isize);
        *e0.offset(-(3 as libc::c_int) as isize)
            += *e2.offset(-(3 as libc::c_int) as isize);
        *e2
            .offset(
                -(2 as libc::c_int) as isize,
            ) = k00_20 * *A.offset(0 as libc::c_int as isize)
            - k01_21 * *A.offset(1 as libc::c_int as isize);
        *e2
            .offset(
                -(3 as libc::c_int) as isize,
            ) = k01_21 * *A.offset(0 as libc::c_int as isize)
            + k00_20 * *A.offset(1 as libc::c_int as isize);
        A = A.offset(k1 as isize);
        k00_20 = *e0.offset(-(4 as libc::c_int) as isize)
            - *e2.offset(-(4 as libc::c_int) as isize);
        k01_21 = *e0.offset(-(5 as libc::c_int) as isize)
            - *e2.offset(-(5 as libc::c_int) as isize);
        *e0.offset(-(4 as libc::c_int) as isize)
            += *e2.offset(-(4 as libc::c_int) as isize);
        *e0.offset(-(5 as libc::c_int) as isize)
            += *e2.offset(-(5 as libc::c_int) as isize);
        *e2
            .offset(
                -(4 as libc::c_int) as isize,
            ) = k00_20 * *A.offset(0 as libc::c_int as isize)
            - k01_21 * *A.offset(1 as libc::c_int as isize);
        *e2
            .offset(
                -(5 as libc::c_int) as isize,
            ) = k01_21 * *A.offset(0 as libc::c_int as isize)
            + k00_20 * *A.offset(1 as libc::c_int as isize);
        A = A.offset(k1 as isize);
        k00_20 = *e0.offset(-(6 as libc::c_int) as isize)
            - *e2.offset(-(6 as libc::c_int) as isize);
        k01_21 = *e0.offset(-(7 as libc::c_int) as isize)
            - *e2.offset(-(7 as libc::c_int) as isize);
        *e0.offset(-(6 as libc::c_int) as isize)
            += *e2.offset(-(6 as libc::c_int) as isize);
        *e0.offset(-(7 as libc::c_int) as isize)
            += *e2.offset(-(7 as libc::c_int) as isize);
        *e2
            .offset(
                -(6 as libc::c_int) as isize,
            ) = k00_20 * *A.offset(0 as libc::c_int as isize)
            - k01_21 * *A.offset(1 as libc::c_int as isize);
        *e2
            .offset(
                -(7 as libc::c_int) as isize,
            ) = k01_21 * *A.offset(0 as libc::c_int as isize)
            + k00_20 * *A.offset(1 as libc::c_int as isize);
        e0 = e0.offset(-(8 as libc::c_int as isize));
        e2 = e2.offset(-(8 as libc::c_int as isize));
        A = A.offset(k1 as isize);
        i -= 1;
    }
}
unsafe extern "C" fn imdct_step3_inner_s_loop(
    mut n: libc::c_int,
    mut e: *mut libc::c_float,
    mut i_off: libc::c_int,
    mut k_off: libc::c_int,
    mut A: *mut libc::c_float,
    mut a_off: libc::c_int,
    mut k0: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut A0: libc::c_float = *A.offset(0 as libc::c_int as isize);
    let mut A1: libc::c_float = *A
        .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    let mut A2: libc::c_float = *A.offset((0 as libc::c_int + a_off) as isize);
    let mut A3: libc::c_float = *A
        .offset((0 as libc::c_int + a_off + 1 as libc::c_int) as isize);
    let mut A4: libc::c_float = *A
        .offset(
            (0 as libc::c_int + a_off * 2 as libc::c_int + 0 as libc::c_int) as isize,
        );
    let mut A5: libc::c_float = *A
        .offset(
            (0 as libc::c_int + a_off * 2 as libc::c_int + 1 as libc::c_int) as isize,
        );
    let mut A6: libc::c_float = *A
        .offset(
            (0 as libc::c_int + a_off * 3 as libc::c_int + 0 as libc::c_int) as isize,
        );
    let mut A7: libc::c_float = *A
        .offset(
            (0 as libc::c_int + a_off * 3 as libc::c_int + 1 as libc::c_int) as isize,
        );
    let mut k00: libc::c_float = 0.;
    let mut k11: libc::c_float = 0.;
    let mut ee0: *mut libc::c_float = e.offset(i_off as isize);
    let mut ee2: *mut libc::c_float = ee0.offset(k_off as isize);
    i = n;
    while i > 0 as libc::c_int {
        k00 = *ee0.offset(0 as libc::c_int as isize)
            - *ee2.offset(0 as libc::c_int as isize);
        k11 = *ee0.offset(-(1 as libc::c_int) as isize)
            - *ee2.offset(-(1 as libc::c_int) as isize);
        *ee0
            .offset(
                0 as libc::c_int as isize,
            ) = *ee0.offset(0 as libc::c_int as isize)
            + *ee2.offset(0 as libc::c_int as isize);
        *ee0
            .offset(
                -(1 as libc::c_int) as isize,
            ) = *ee0.offset(-(1 as libc::c_int) as isize)
            + *ee2.offset(-(1 as libc::c_int) as isize);
        *ee2.offset(0 as libc::c_int as isize) = k00 * A0 - k11 * A1;
        *ee2.offset(-(1 as libc::c_int) as isize) = k11 * A0 + k00 * A1;
        k00 = *ee0.offset(-(2 as libc::c_int) as isize)
            - *ee2.offset(-(2 as libc::c_int) as isize);
        k11 = *ee0.offset(-(3 as libc::c_int) as isize)
            - *ee2.offset(-(3 as libc::c_int) as isize);
        *ee0
            .offset(
                -(2 as libc::c_int) as isize,
            ) = *ee0.offset(-(2 as libc::c_int) as isize)
            + *ee2.offset(-(2 as libc::c_int) as isize);
        *ee0
            .offset(
                -(3 as libc::c_int) as isize,
            ) = *ee0.offset(-(3 as libc::c_int) as isize)
            + *ee2.offset(-(3 as libc::c_int) as isize);
        *ee2.offset(-(2 as libc::c_int) as isize) = k00 * A2 - k11 * A3;
        *ee2.offset(-(3 as libc::c_int) as isize) = k11 * A2 + k00 * A3;
        k00 = *ee0.offset(-(4 as libc::c_int) as isize)
            - *ee2.offset(-(4 as libc::c_int) as isize);
        k11 = *ee0.offset(-(5 as libc::c_int) as isize)
            - *ee2.offset(-(5 as libc::c_int) as isize);
        *ee0
            .offset(
                -(4 as libc::c_int) as isize,
            ) = *ee0.offset(-(4 as libc::c_int) as isize)
            + *ee2.offset(-(4 as libc::c_int) as isize);
        *ee0
            .offset(
                -(5 as libc::c_int) as isize,
            ) = *ee0.offset(-(5 as libc::c_int) as isize)
            + *ee2.offset(-(5 as libc::c_int) as isize);
        *ee2.offset(-(4 as libc::c_int) as isize) = k00 * A4 - k11 * A5;
        *ee2.offset(-(5 as libc::c_int) as isize) = k11 * A4 + k00 * A5;
        k00 = *ee0.offset(-(6 as libc::c_int) as isize)
            - *ee2.offset(-(6 as libc::c_int) as isize);
        k11 = *ee0.offset(-(7 as libc::c_int) as isize)
            - *ee2.offset(-(7 as libc::c_int) as isize);
        *ee0
            .offset(
                -(6 as libc::c_int) as isize,
            ) = *ee0.offset(-(6 as libc::c_int) as isize)
            + *ee2.offset(-(6 as libc::c_int) as isize);
        *ee0
            .offset(
                -(7 as libc::c_int) as isize,
            ) = *ee0.offset(-(7 as libc::c_int) as isize)
            + *ee2.offset(-(7 as libc::c_int) as isize);
        *ee2.offset(-(6 as libc::c_int) as isize) = k00 * A6 - k11 * A7;
        *ee2.offset(-(7 as libc::c_int) as isize) = k11 * A6 + k00 * A7;
        ee0 = ee0.offset(-(k0 as isize));
        ee2 = ee2.offset(-(k0 as isize));
        i -= 1;
    }
}
#[inline]
unsafe extern "C" fn iter_54(mut z: *mut libc::c_float) {
    let mut k00: libc::c_float = 0.;
    let mut k11: libc::c_float = 0.;
    let mut k22: libc::c_float = 0.;
    let mut k33: libc::c_float = 0.;
    let mut y0: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut y3: libc::c_float = 0.;
    k00 = *z.offset(0 as libc::c_int as isize) - *z.offset(-(4 as libc::c_int) as isize);
    y0 = *z.offset(0 as libc::c_int as isize) + *z.offset(-(4 as libc::c_int) as isize);
    y2 = *z.offset(-(2 as libc::c_int) as isize)
        + *z.offset(-(6 as libc::c_int) as isize);
    k22 = *z.offset(-(2 as libc::c_int) as isize)
        - *z.offset(-(6 as libc::c_int) as isize);
    *z.offset(-(0 as libc::c_int) as isize) = y0 + y2;
    *z.offset(-(2 as libc::c_int) as isize) = y0 - y2;
    k33 = *z.offset(-(3 as libc::c_int) as isize)
        - *z.offset(-(7 as libc::c_int) as isize);
    *z.offset(-(4 as libc::c_int) as isize) = k00 + k33;
    *z.offset(-(6 as libc::c_int) as isize) = k00 - k33;
    k11 = *z.offset(-(1 as libc::c_int) as isize)
        - *z.offset(-(5 as libc::c_int) as isize);
    y1 = *z.offset(-(1 as libc::c_int) as isize)
        + *z.offset(-(5 as libc::c_int) as isize);
    y3 = *z.offset(-(3 as libc::c_int) as isize)
        + *z.offset(-(7 as libc::c_int) as isize);
    *z.offset(-(1 as libc::c_int) as isize) = y1 + y3;
    *z.offset(-(3 as libc::c_int) as isize) = y1 - y3;
    *z.offset(-(5 as libc::c_int) as isize) = k11 - k22;
    *z.offset(-(7 as libc::c_int) as isize) = k11 + k22;
}
unsafe extern "C" fn imdct_step3_inner_s_loop_ld654(
    mut n: libc::c_int,
    mut e: *mut libc::c_float,
    mut i_off: libc::c_int,
    mut A: *mut libc::c_float,
    mut base_n: libc::c_int,
) {
    let mut a_off: libc::c_int = base_n >> 3 as libc::c_int;
    let mut A2: libc::c_float = *A.offset((0 as libc::c_int + a_off) as isize);
    let mut z: *mut libc::c_float = e.offset(i_off as isize);
    let mut base: *mut libc::c_float = z.offset(-((16 as libc::c_int * n) as isize));
    while z > base {
        let mut k00: libc::c_float = 0.;
        let mut k11: libc::c_float = 0.;
        let mut l00: libc::c_float = 0.;
        let mut l11: libc::c_float = 0.;
        k00 = *z.offset(-(0 as libc::c_int) as isize)
            - *z.offset(-(8 as libc::c_int) as isize);
        k11 = *z.offset(-(1 as libc::c_int) as isize)
            - *z.offset(-(9 as libc::c_int) as isize);
        l00 = *z.offset(-(2 as libc::c_int) as isize)
            - *z.offset(-(10 as libc::c_int) as isize);
        l11 = *z.offset(-(3 as libc::c_int) as isize)
            - *z.offset(-(11 as libc::c_int) as isize);
        *z
            .offset(
                -(0 as libc::c_int) as isize,
            ) = *z.offset(-(0 as libc::c_int) as isize)
            + *z.offset(-(8 as libc::c_int) as isize);
        *z
            .offset(
                -(1 as libc::c_int) as isize,
            ) = *z.offset(-(1 as libc::c_int) as isize)
            + *z.offset(-(9 as libc::c_int) as isize);
        *z
            .offset(
                -(2 as libc::c_int) as isize,
            ) = *z.offset(-(2 as libc::c_int) as isize)
            + *z.offset(-(10 as libc::c_int) as isize);
        *z
            .offset(
                -(3 as libc::c_int) as isize,
            ) = *z.offset(-(3 as libc::c_int) as isize)
            + *z.offset(-(11 as libc::c_int) as isize);
        *z.offset(-(8 as libc::c_int) as isize) = k00;
        *z.offset(-(9 as libc::c_int) as isize) = k11;
        *z.offset(-(10 as libc::c_int) as isize) = (l00 + l11) * A2;
        *z.offset(-(11 as libc::c_int) as isize) = (l11 - l00) * A2;
        k00 = *z.offset(-(4 as libc::c_int) as isize)
            - *z.offset(-(12 as libc::c_int) as isize);
        k11 = *z.offset(-(5 as libc::c_int) as isize)
            - *z.offset(-(13 as libc::c_int) as isize);
        l00 = *z.offset(-(6 as libc::c_int) as isize)
            - *z.offset(-(14 as libc::c_int) as isize);
        l11 = *z.offset(-(7 as libc::c_int) as isize)
            - *z.offset(-(15 as libc::c_int) as isize);
        *z
            .offset(
                -(4 as libc::c_int) as isize,
            ) = *z.offset(-(4 as libc::c_int) as isize)
            + *z.offset(-(12 as libc::c_int) as isize);
        *z
            .offset(
                -(5 as libc::c_int) as isize,
            ) = *z.offset(-(5 as libc::c_int) as isize)
            + *z.offset(-(13 as libc::c_int) as isize);
        *z
            .offset(
                -(6 as libc::c_int) as isize,
            ) = *z.offset(-(6 as libc::c_int) as isize)
            + *z.offset(-(14 as libc::c_int) as isize);
        *z
            .offset(
                -(7 as libc::c_int) as isize,
            ) = *z.offset(-(7 as libc::c_int) as isize)
            + *z.offset(-(15 as libc::c_int) as isize);
        *z.offset(-(12 as libc::c_int) as isize) = k11;
        *z.offset(-(13 as libc::c_int) as isize) = -k00;
        *z.offset(-(14 as libc::c_int) as isize) = (l11 - l00) * A2;
        *z.offset(-(15 as libc::c_int) as isize) = (l00 + l11) * -A2;
        iter_54(z);
        iter_54(z.offset(-(8 as libc::c_int as isize)));
        z = z.offset(-(16 as libc::c_int as isize));
    }
}
unsafe extern "C" fn inverse_mdct(
    mut buffer: *mut libc::c_float,
    mut n: libc::c_int,
    mut f: *mut vorb,
    mut blocktype: libc::c_int,
) {
    let mut n2: libc::c_int = n >> 1 as libc::c_int;
    let mut n4: libc::c_int = n >> 2 as libc::c_int;
    let mut n8: libc::c_int = n >> 3 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut ld: libc::c_int = 0;
    let mut save_point: libc::c_int = (*f).temp_offset;
    let mut buf2: *mut libc::c_float = (if !((*f).alloc.alloc_buffer).is_null() {
        setup_temp_malloc(
            f,
            (n2 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                as libc::c_int,
        )
    } else {
        let mut fresh26 = ::std::vec::from_elem(
            0,
            (n2 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                as usize,
        );
        fresh26.as_mut_ptr()
    }) as *mut libc::c_float;
    let mut u: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut A: *mut libc::c_float = (*f).A[blocktype as usize];
    let mut d: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut e: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut AA: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut e_stop: *mut libc::c_float = 0 as *mut libc::c_float;
    d = &mut *buf2.offset((n2 - 2 as libc::c_int) as isize) as *mut libc::c_float;
    AA = A;
    e = &mut *buffer.offset(0 as libc::c_int as isize) as *mut libc::c_float;
    e_stop = &mut *buffer.offset(n2 as isize) as *mut libc::c_float;
    while e != e_stop {
        *d
            .offset(
                1 as libc::c_int as isize,
            ) = *e.offset(0 as libc::c_int as isize)
            * *AA.offset(0 as libc::c_int as isize)
            - *e.offset(2 as libc::c_int as isize)
                * *AA.offset(1 as libc::c_int as isize);
        *d
            .offset(
                0 as libc::c_int as isize,
            ) = *e.offset(0 as libc::c_int as isize)
            * *AA.offset(1 as libc::c_int as isize)
            + *e.offset(2 as libc::c_int as isize)
                * *AA.offset(0 as libc::c_int as isize);
        d = d.offset(-(2 as libc::c_int as isize));
        AA = AA.offset(2 as libc::c_int as isize);
        e = e.offset(4 as libc::c_int as isize);
    }
    e = &mut *buffer.offset((n2 - 3 as libc::c_int) as isize) as *mut libc::c_float;
    while d >= buf2 {
        *d
            .offset(
                1 as libc::c_int as isize,
            ) = -*e.offset(2 as libc::c_int as isize)
            * *AA.offset(0 as libc::c_int as isize)
            - -*e.offset(0 as libc::c_int as isize)
                * *AA.offset(1 as libc::c_int as isize);
        *d
            .offset(
                0 as libc::c_int as isize,
            ) = -*e.offset(2 as libc::c_int as isize)
            * *AA.offset(1 as libc::c_int as isize)
            + -*e.offset(0 as libc::c_int as isize)
                * *AA.offset(0 as libc::c_int as isize);
        d = d.offset(-(2 as libc::c_int as isize));
        AA = AA.offset(2 as libc::c_int as isize);
        e = e.offset(-(4 as libc::c_int as isize));
    }
    u = buffer;
    v = buf2;
    let mut AA_0: *mut libc::c_float = &mut *A.offset((n2 - 8 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut d0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut e0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut e1: *mut libc::c_float = 0 as *mut libc::c_float;
    e0 = &mut *v.offset(n4 as isize) as *mut libc::c_float;
    e1 = &mut *v.offset(0 as libc::c_int as isize) as *mut libc::c_float;
    d0 = &mut *u.offset(n4 as isize) as *mut libc::c_float;
    d1 = &mut *u.offset(0 as libc::c_int as isize) as *mut libc::c_float;
    while AA_0 >= A {
        let mut v40_20: libc::c_float = 0.;
        let mut v41_21: libc::c_float = 0.;
        v41_21 = *e0.offset(1 as libc::c_int as isize)
            - *e1.offset(1 as libc::c_int as isize);
        v40_20 = *e0.offset(0 as libc::c_int as isize)
            - *e1.offset(0 as libc::c_int as isize);
        *d0
            .offset(
                1 as libc::c_int as isize,
            ) = *e0.offset(1 as libc::c_int as isize)
            + *e1.offset(1 as libc::c_int as isize);
        *d0
            .offset(
                0 as libc::c_int as isize,
            ) = *e0.offset(0 as libc::c_int as isize)
            + *e1.offset(0 as libc::c_int as isize);
        *d1
            .offset(
                1 as libc::c_int as isize,
            ) = v41_21 * *AA_0.offset(4 as libc::c_int as isize)
            - v40_20 * *AA_0.offset(5 as libc::c_int as isize);
        *d1
            .offset(
                0 as libc::c_int as isize,
            ) = v40_20 * *AA_0.offset(4 as libc::c_int as isize)
            + v41_21 * *AA_0.offset(5 as libc::c_int as isize);
        v41_21 = *e0.offset(3 as libc::c_int as isize)
            - *e1.offset(3 as libc::c_int as isize);
        v40_20 = *e0.offset(2 as libc::c_int as isize)
            - *e1.offset(2 as libc::c_int as isize);
        *d0
            .offset(
                3 as libc::c_int as isize,
            ) = *e0.offset(3 as libc::c_int as isize)
            + *e1.offset(3 as libc::c_int as isize);
        *d0
            .offset(
                2 as libc::c_int as isize,
            ) = *e0.offset(2 as libc::c_int as isize)
            + *e1.offset(2 as libc::c_int as isize);
        *d1
            .offset(
                3 as libc::c_int as isize,
            ) = v41_21 * *AA_0.offset(0 as libc::c_int as isize)
            - v40_20 * *AA_0.offset(1 as libc::c_int as isize);
        *d1
            .offset(
                2 as libc::c_int as isize,
            ) = v40_20 * *AA_0.offset(0 as libc::c_int as isize)
            + v41_21 * *AA_0.offset(1 as libc::c_int as isize);
        AA_0 = AA_0.offset(-(8 as libc::c_int as isize));
        d0 = d0.offset(4 as libc::c_int as isize);
        d1 = d1.offset(4 as libc::c_int as isize);
        e0 = e0.offset(4 as libc::c_int as isize);
        e1 = e1.offset(4 as libc::c_int as isize);
    }
    ld = ilog(n) - 1 as libc::c_int;
    imdct_step3_iter0_loop(
        n >> 4 as libc::c_int,
        u,
        n2 - 1 as libc::c_int - n4 * 0 as libc::c_int,
        -(n >> 3 as libc::c_int),
        A,
    );
    imdct_step3_iter0_loop(
        n >> 4 as libc::c_int,
        u,
        n2 - 1 as libc::c_int - n4 * 1 as libc::c_int,
        -(n >> 3 as libc::c_int),
        A,
    );
    imdct_step3_inner_r_loop(
        n >> 5 as libc::c_int,
        u,
        n2 - 1 as libc::c_int - n8 * 0 as libc::c_int,
        -(n >> 4 as libc::c_int),
        A,
        16 as libc::c_int,
    );
    imdct_step3_inner_r_loop(
        n >> 5 as libc::c_int,
        u,
        n2 - 1 as libc::c_int - n8 * 1 as libc::c_int,
        -(n >> 4 as libc::c_int),
        A,
        16 as libc::c_int,
    );
    imdct_step3_inner_r_loop(
        n >> 5 as libc::c_int,
        u,
        n2 - 1 as libc::c_int - n8 * 2 as libc::c_int,
        -(n >> 4 as libc::c_int),
        A,
        16 as libc::c_int,
    );
    imdct_step3_inner_r_loop(
        n >> 5 as libc::c_int,
        u,
        n2 - 1 as libc::c_int - n8 * 3 as libc::c_int,
        -(n >> 4 as libc::c_int),
        A,
        16 as libc::c_int,
    );
    l = 2 as libc::c_int;
    while l < ld - 3 as libc::c_int >> 1 as libc::c_int {
        let mut k0: libc::c_int = n >> l + 2 as libc::c_int;
        let mut k0_2: libc::c_int = k0 >> 1 as libc::c_int;
        let mut lim: libc::c_int = (1 as libc::c_int) << l + 1 as libc::c_int;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < lim {
            imdct_step3_inner_r_loop(
                n >> l + 4 as libc::c_int,
                u,
                n2 - 1 as libc::c_int - k0 * i,
                -k0_2,
                A,
                (1 as libc::c_int) << l + 3 as libc::c_int,
            );
            i += 1;
        }
        l += 1;
    }
    while l < ld - 6 as libc::c_int {
        let mut k0_0: libc::c_int = n >> l + 2 as libc::c_int;
        let mut k1: libc::c_int = (1 as libc::c_int) << l + 3 as libc::c_int;
        let mut k0_2_0: libc::c_int = k0_0 >> 1 as libc::c_int;
        let mut rlim: libc::c_int = n >> l + 6 as libc::c_int;
        let mut r: libc::c_int = 0;
        let mut lim_0: libc::c_int = (1 as libc::c_int) << l + 1 as libc::c_int;
        let mut i_off: libc::c_int = 0;
        let mut A0: *mut libc::c_float = A;
        i_off = n2 - 1 as libc::c_int;
        r = rlim;
        while r > 0 as libc::c_int {
            imdct_step3_inner_s_loop(lim_0, u, i_off, -k0_2_0, A0, k1, k0_0);
            A0 = A0.offset((k1 * 4 as libc::c_int) as isize);
            i_off -= 8 as libc::c_int;
            r -= 1;
        }
        l += 1;
    }
    imdct_step3_inner_s_loop_ld654(
        n >> 5 as libc::c_int,
        u,
        n2 - 1 as libc::c_int,
        A,
        n,
    );
    let mut bitrev: *mut uint16 = (*f).bit_reverse[blocktype as usize];
    let mut d0_0: *mut libc::c_float = &mut *v.offset((n4 - 4 as libc::c_int) as isize)
        as *mut libc::c_float;
    let mut d1_0: *mut libc::c_float = &mut *v.offset((n2 - 4 as libc::c_int) as isize)
        as *mut libc::c_float;
    while d0_0 >= v {
        let mut k4: libc::c_int = 0;
        k4 = *bitrev.offset(0 as libc::c_int as isize) as libc::c_int;
        *d1_0
            .offset(
                3 as libc::c_int as isize,
            ) = *u.offset((k4 + 0 as libc::c_int) as isize);
        *d1_0
            .offset(
                2 as libc::c_int as isize,
            ) = *u.offset((k4 + 1 as libc::c_int) as isize);
        *d0_0
            .offset(
                3 as libc::c_int as isize,
            ) = *u.offset((k4 + 2 as libc::c_int) as isize);
        *d0_0
            .offset(
                2 as libc::c_int as isize,
            ) = *u.offset((k4 + 3 as libc::c_int) as isize);
        k4 = *bitrev.offset(1 as libc::c_int as isize) as libc::c_int;
        *d1_0
            .offset(
                1 as libc::c_int as isize,
            ) = *u.offset((k4 + 0 as libc::c_int) as isize);
        *d1_0
            .offset(
                0 as libc::c_int as isize,
            ) = *u.offset((k4 + 1 as libc::c_int) as isize);
        *d0_0
            .offset(
                1 as libc::c_int as isize,
            ) = *u.offset((k4 + 2 as libc::c_int) as isize);
        *d0_0
            .offset(
                0 as libc::c_int as isize,
            ) = *u.offset((k4 + 3 as libc::c_int) as isize);
        d0_0 = d0_0.offset(-(4 as libc::c_int as isize));
        d1_0 = d1_0.offset(-(4 as libc::c_int as isize));
        bitrev = bitrev.offset(2 as libc::c_int as isize);
    }
    let mut C_0: *mut libc::c_float = (*f).C[blocktype as usize];
    let mut d_0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut e_0: *mut libc::c_float = 0 as *mut libc::c_float;
    d_0 = v;
    e_0 = v.offset(n2 as isize).offset(-(4 as libc::c_int as isize));
    while d_0 < e_0 {
        let mut a02: libc::c_float = 0.;
        let mut a11: libc::c_float = 0.;
        let mut b0: libc::c_float = 0.;
        let mut b1: libc::c_float = 0.;
        let mut b2: libc::c_float = 0.;
        let mut b3: libc::c_float = 0.;
        a02 = *d_0.offset(0 as libc::c_int as isize)
            - *e_0.offset(2 as libc::c_int as isize);
        a11 = *d_0.offset(1 as libc::c_int as isize)
            + *e_0.offset(3 as libc::c_int as isize);
        b0 = *C_0.offset(1 as libc::c_int as isize) * a02
            + *C_0.offset(0 as libc::c_int as isize) * a11;
        b1 = *C_0.offset(1 as libc::c_int as isize) * a11
            - *C_0.offset(0 as libc::c_int as isize) * a02;
        b2 = *d_0.offset(0 as libc::c_int as isize)
            + *e_0.offset(2 as libc::c_int as isize);
        b3 = *d_0.offset(1 as libc::c_int as isize)
            - *e_0.offset(3 as libc::c_int as isize);
        *d_0.offset(0 as libc::c_int as isize) = b2 + b0;
        *d_0.offset(1 as libc::c_int as isize) = b3 + b1;
        *e_0.offset(2 as libc::c_int as isize) = b2 - b0;
        *e_0.offset(3 as libc::c_int as isize) = b1 - b3;
        a02 = *d_0.offset(2 as libc::c_int as isize)
            - *e_0.offset(0 as libc::c_int as isize);
        a11 = *d_0.offset(3 as libc::c_int as isize)
            + *e_0.offset(1 as libc::c_int as isize);
        b0 = *C_0.offset(3 as libc::c_int as isize) * a02
            + *C_0.offset(2 as libc::c_int as isize) * a11;
        b1 = *C_0.offset(3 as libc::c_int as isize) * a11
            - *C_0.offset(2 as libc::c_int as isize) * a02;
        b2 = *d_0.offset(2 as libc::c_int as isize)
            + *e_0.offset(0 as libc::c_int as isize);
        b3 = *d_0.offset(3 as libc::c_int as isize)
            - *e_0.offset(1 as libc::c_int as isize);
        *d_0.offset(2 as libc::c_int as isize) = b2 + b0;
        *d_0.offset(3 as libc::c_int as isize) = b3 + b1;
        *e_0.offset(0 as libc::c_int as isize) = b2 - b0;
        *e_0.offset(1 as libc::c_int as isize) = b1 - b3;
        C_0 = C_0.offset(4 as libc::c_int as isize);
        d_0 = d_0.offset(4 as libc::c_int as isize);
        e_0 = e_0.offset(-(4 as libc::c_int as isize));
    }
    let mut d0_1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d1_1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d3: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut B: *mut libc::c_float = ((*f).B[blocktype as usize])
        .offset(n2 as isize)
        .offset(-(8 as libc::c_int as isize));
    let mut e_1: *mut libc::c_float = buf2
        .offset(n2 as isize)
        .offset(-(8 as libc::c_int as isize));
    d0_1 = &mut *buffer.offset(0 as libc::c_int as isize) as *mut libc::c_float;
    d1_1 = &mut *buffer.offset((n2 - 4 as libc::c_int) as isize) as *mut libc::c_float;
    d2 = &mut *buffer.offset(n2 as isize) as *mut libc::c_float;
    d3 = &mut *buffer.offset((n - 4 as libc::c_int) as isize) as *mut libc::c_float;
    while e_1 >= v {
        let mut p0: libc::c_float = 0.;
        let mut p1: libc::c_float = 0.;
        let mut p2: libc::c_float = 0.;
        let mut p3: libc::c_float = 0.;
        p3 = *e_1.offset(6 as libc::c_int as isize)
            * *B.offset(7 as libc::c_int as isize)
            - *e_1.offset(7 as libc::c_int as isize)
                * *B.offset(6 as libc::c_int as isize);
        p2 = -*e_1.offset(6 as libc::c_int as isize)
            * *B.offset(6 as libc::c_int as isize)
            - *e_1.offset(7 as libc::c_int as isize)
                * *B.offset(7 as libc::c_int as isize);
        *d0_1.offset(0 as libc::c_int as isize) = p3;
        *d1_1.offset(3 as libc::c_int as isize) = -p3;
        *d2.offset(0 as libc::c_int as isize) = p2;
        *d3.offset(3 as libc::c_int as isize) = p2;
        p1 = *e_1.offset(4 as libc::c_int as isize)
            * *B.offset(5 as libc::c_int as isize)
            - *e_1.offset(5 as libc::c_int as isize)
                * *B.offset(4 as libc::c_int as isize);
        p0 = -*e_1.offset(4 as libc::c_int as isize)
            * *B.offset(4 as libc::c_int as isize)
            - *e_1.offset(5 as libc::c_int as isize)
                * *B.offset(5 as libc::c_int as isize);
        *d0_1.offset(1 as libc::c_int as isize) = p1;
        *d1_1.offset(2 as libc::c_int as isize) = -p1;
        *d2.offset(1 as libc::c_int as isize) = p0;
        *d3.offset(2 as libc::c_int as isize) = p0;
        p3 = *e_1.offset(2 as libc::c_int as isize)
            * *B.offset(3 as libc::c_int as isize)
            - *e_1.offset(3 as libc::c_int as isize)
                * *B.offset(2 as libc::c_int as isize);
        p2 = -*e_1.offset(2 as libc::c_int as isize)
            * *B.offset(2 as libc::c_int as isize)
            - *e_1.offset(3 as libc::c_int as isize)
                * *B.offset(3 as libc::c_int as isize);
        *d0_1.offset(2 as libc::c_int as isize) = p3;
        *d1_1.offset(1 as libc::c_int as isize) = -p3;
        *d2.offset(2 as libc::c_int as isize) = p2;
        *d3.offset(1 as libc::c_int as isize) = p2;
        p1 = *e_1.offset(0 as libc::c_int as isize)
            * *B.offset(1 as libc::c_int as isize)
            - *e_1.offset(1 as libc::c_int as isize)
                * *B.offset(0 as libc::c_int as isize);
        p0 = -*e_1.offset(0 as libc::c_int as isize)
            * *B.offset(0 as libc::c_int as isize)
            - *e_1.offset(1 as libc::c_int as isize)
                * *B.offset(1 as libc::c_int as isize);
        *d0_1.offset(3 as libc::c_int as isize) = p1;
        *d1_1.offset(0 as libc::c_int as isize) = -p1;
        *d2.offset(3 as libc::c_int as isize) = p0;
        *d3.offset(0 as libc::c_int as isize) = p0;
        B = B.offset(-(8 as libc::c_int as isize));
        e_1 = e_1.offset(-(8 as libc::c_int as isize));
        d0_1 = d0_1.offset(4 as libc::c_int as isize);
        d2 = d2.offset(4 as libc::c_int as isize);
        d1_1 = d1_1.offset(-(4 as libc::c_int as isize));
        d3 = d3.offset(-(4 as libc::c_int as isize));
    }
    (*f).temp_offset = save_point;
}
unsafe extern "C" fn get_window(
    mut f: *mut vorb,
    mut len: libc::c_int,
) -> *mut libc::c_float {
    len <<= 1 as libc::c_int;
    if len == (*f).blocksize_0 {
        return (*f).window[0 as libc::c_int as usize];
    }
    if len == (*f).blocksize_1 {
        return (*f).window[1 as libc::c_int as usize];
    }
    return 0 as *mut libc::c_float;
}
unsafe extern "C" fn do_floor(
    mut f: *mut vorb,
    mut map: *mut Mapping,
    mut i: libc::c_int,
    mut n: libc::c_int,
    mut target: *mut libc::c_float,
    mut finalY: *mut YTYPE,
    mut step2_flag: *mut uint8,
) -> libc::c_int {
    let mut n2: libc::c_int = n >> 1 as libc::c_int;
    let mut s: libc::c_int = (*((*map).chan).offset(i as isize)).mux as libc::c_int;
    let mut floor_0: libc::c_int = 0;
    floor_0 = (*map).submap_floor[s as usize] as libc::c_int;
    if (*f).floor_types[floor_0 as usize] as libc::c_int == 0 as libc::c_int {
        return error(f, VORBIS_invalid_stream)
    } else {
        let mut g: *mut Floor1 = &mut (*((*f).floor_config).offset(floor_0 as isize))
            .floor1;
        let mut j: libc::c_int = 0;
        let mut q: libc::c_int = 0;
        let mut lx: libc::c_int = 0 as libc::c_int;
        let mut ly: libc::c_int = *finalY.offset(0 as libc::c_int as isize)
            as libc::c_int * (*g).floor1_multiplier as libc::c_int;
        q = 1 as libc::c_int;
        while q < (*g).values {
            j = (*g).sorted_order[q as usize] as libc::c_int;
            if *finalY.offset(j as isize) as libc::c_int >= 0 as libc::c_int {
                let mut hy: libc::c_int = *finalY.offset(j as isize) as libc::c_int
                    * (*g).floor1_multiplier as libc::c_int;
                let mut hx: libc::c_int = (*g).Xlist[j as usize] as libc::c_int;
                if lx != hx {
                    draw_line(target, lx, ly, hx, hy, n2);
                }
                lx = hx;
                ly = hy;
            }
            q += 1;
        }
        if lx < n2 {
            j = lx;
            while j < n2 {
                *target.offset(j as isize) *= inverse_db_table[ly as usize];
                j += 1;
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vorbis_decode_initial(
    mut f: *mut vorb,
    mut p_left_start: *mut libc::c_int,
    mut p_left_end: *mut libc::c_int,
    mut p_right_start: *mut libc::c_int,
    mut p_right_end: *mut libc::c_int,
    mut mode: *mut libc::c_int,
) -> libc::c_int {
    let mut m: *mut Mode = 0 as *mut Mode;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut window_center: libc::c_int = 0;
    let ref mut fresh27 = (*f).channel_buffer_end;
    *fresh27 = 0 as libc::c_int;
    (*f).channel_buffer_start = *fresh27;
    loop {
        if (*f).eof != 0 {
            return 0 as libc::c_int;
        }
        if maybe_start_packet(f) == 0 {
            return 0 as libc::c_int;
        }
        if !(get_bits(f, 1 as libc::c_int) != 0 as libc::c_int as libc::c_uint) {
            break;
        }
        if (*f).push_mode != 0 {
            return error(f, VORBIS_bad_packet_type);
        }
        while -(1 as libc::c_int) != get8_packet(f) {}
    }
    !((*f).alloc.alloc_buffer).is_null();
    i = get_bits(f, ilog((*f).mode_count - 1 as libc::c_int)) as libc::c_int;
    if i == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if i >= (*f).mode_count {
        return 0 as libc::c_int;
    }
    *mode = i;
    m = ((*f).mode_config).as_mut_ptr().offset(i as isize);
    if (*m).blockflag != 0 {
        n = (*f).blocksize_1;
        prev = get_bits(f, 1 as libc::c_int) as libc::c_int;
        next = get_bits(f, 1 as libc::c_int) as libc::c_int;
    } else {
        next = 0 as libc::c_int;
        prev = next;
        n = (*f).blocksize_0;
    }
    window_center = n >> 1 as libc::c_int;
    if (*m).blockflag as libc::c_int != 0 && prev == 0 {
        *p_left_start = n - (*f).blocksize_0 >> 2 as libc::c_int;
        *p_left_end = n + (*f).blocksize_0 >> 2 as libc::c_int;
    } else {
        *p_left_start = 0 as libc::c_int;
        *p_left_end = window_center;
    }
    if (*m).blockflag as libc::c_int != 0 && next == 0 {
        *p_right_start = n * 3 as libc::c_int - (*f).blocksize_0 >> 2 as libc::c_int;
        *p_right_end = n * 3 as libc::c_int + (*f).blocksize_0 >> 2 as libc::c_int;
    } else {
        *p_right_start = window_center;
        *p_right_end = n;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vorbis_decode_packet_rest(
    mut f: *mut vorb,
    mut len: *mut libc::c_int,
    mut m: *mut Mode,
    mut left_start: libc::c_int,
    mut left_end: libc::c_int,
    mut right_start: libc::c_int,
    mut right_end: libc::c_int,
    mut p_left: *mut libc::c_int,
) -> libc::c_int {
    let mut map: *mut Mapping = 0 as *mut Mapping;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut zero_channel: [libc::c_int; 256] = [0; 256];
    let mut really_zero_channel: [libc::c_int; 256] = [0; 256];
    n = (*f).blocksize[(*m).blockflag as usize];
    map = &mut *((*f).mapping).offset((*m).mapping as isize) as *mut Mapping;
    n2 = n >> 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*f).channels {
        let mut s: libc::c_int = (*((*map).chan).offset(i as isize)).mux as libc::c_int;
        let mut floor_0: libc::c_int = 0;
        zero_channel[i as usize] = 0 as libc::c_int;
        floor_0 = (*map).submap_floor[s as usize] as libc::c_int;
        if (*f).floor_types[floor_0 as usize] as libc::c_int == 0 as libc::c_int {
            return error(f, VORBIS_invalid_stream)
        } else {
            let mut g: *mut Floor1 = &mut (*((*f).floor_config).offset(floor_0 as isize))
                .floor1;
            let mut current_block_92: u64;
            if get_bits(f, 1 as libc::c_int) != 0 {
                let mut finalY: *mut libc::c_short = 0 as *mut libc::c_short;
                let mut step2_flag: [uint8; 256] = [0; 256];
                static mut range_list: [libc::c_int; 4] = [
                    256 as libc::c_int,
                    128 as libc::c_int,
                    86 as libc::c_int,
                    64 as libc::c_int,
                ];
                let mut range: libc::c_int = range_list[((*g).floor1_multiplier
                    as libc::c_int - 1 as libc::c_int) as usize];
                let mut offset: libc::c_int = 2 as libc::c_int;
                finalY = (*f).finalY[i as usize];
                *finalY
                    .offset(
                        0 as libc::c_int as isize,
                    ) = get_bits(f, ilog(range) - 1 as libc::c_int) as libc::c_short;
                *finalY
                    .offset(
                        1 as libc::c_int as isize,
                    ) = get_bits(f, ilog(range) - 1 as libc::c_int) as libc::c_short;
                j = 0 as libc::c_int;
                while j < (*g).partitions as libc::c_int {
                    let mut pclass: libc::c_int = (*g).partition_class_list[j as usize]
                        as libc::c_int;
                    let mut cdim: libc::c_int = (*g).class_dimensions[pclass as usize]
                        as libc::c_int;
                    let mut cbits: libc::c_int = (*g).class_subclasses[pclass as usize]
                        as libc::c_int;
                    let mut csub: libc::c_int = ((1 as libc::c_int) << cbits)
                        - 1 as libc::c_int;
                    let mut cval: libc::c_int = 0 as libc::c_int;
                    if cbits != 0 {
                        let mut c: *mut Codebook = ((*f).codebooks)
                            .offset(
                                (*g).class_masterbooks[pclass as usize] as libc::c_int
                                    as isize,
                            );
                        if (*f).valid_bits < 10 as libc::c_int {
                            prep_huffman(f);
                        }
                        cval = ((*f).acc
                            & (((1 as libc::c_int) << 10 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint) as libc::c_int;
                        cval = (*c).fast_huffman[cval as usize] as libc::c_int;
                        if cval >= 0 as libc::c_int {
                            let mut n_0: libc::c_int = *((*c).codeword_lengths)
                                .offset(cval as isize) as libc::c_int;
                            (*f).acc >>= n_0;
                            (*f).valid_bits -= n_0;
                            if (*f).valid_bits < 0 as libc::c_int {
                                (*f).valid_bits = 0 as libc::c_int;
                                cval = -(1 as libc::c_int);
                            }
                        } else {
                            cval = codebook_decode_scalar_raw(f, c);
                        }
                        if (*c).sparse != 0 {
                            cval = *((*c).sorted_values).offset(cval as isize);
                        }
                    }
                    k = 0 as libc::c_int;
                    while k < cdim {
                        let mut book: libc::c_int = (*g)
                            .subclass_books[pclass as usize][(cval & csub) as usize]
                            as libc::c_int;
                        cval = cval >> cbits;
                        if book >= 0 as libc::c_int {
                            let mut temp: libc::c_int = 0;
                            let mut c_0: *mut Codebook = ((*f).codebooks)
                                .offset(book as isize);
                            if (*f).valid_bits < 10 as libc::c_int {
                                prep_huffman(f);
                            }
                            temp = ((*f).acc
                                & (((1 as libc::c_int) << 10 as libc::c_int)
                                    - 1 as libc::c_int) as libc::c_uint) as libc::c_int;
                            temp = (*c_0).fast_huffman[temp as usize] as libc::c_int;
                            if temp >= 0 as libc::c_int {
                                let mut n_1: libc::c_int = *((*c_0).codeword_lengths)
                                    .offset(temp as isize) as libc::c_int;
                                (*f).acc >>= n_1;
                                (*f).valid_bits -= n_1;
                                if (*f).valid_bits < 0 as libc::c_int {
                                    (*f).valid_bits = 0 as libc::c_int;
                                    temp = -(1 as libc::c_int);
                                }
                            } else {
                                temp = codebook_decode_scalar_raw(f, c_0);
                            }
                            if (*c_0).sparse != 0 {
                                temp = *((*c_0).sorted_values).offset(temp as isize);
                            }
                            let fresh28 = offset;
                            offset = offset + 1;
                            *finalY.offset(fresh28 as isize) = temp as libc::c_short;
                        } else {
                            let fresh29 = offset;
                            offset = offset + 1;
                            *finalY
                                .offset(
                                    fresh29 as isize,
                                ) = 0 as libc::c_int as libc::c_short;
                        }
                        k += 1;
                    }
                    j += 1;
                }
                if (*f).valid_bits == -(1 as libc::c_int) {
                    current_block_92 = 12111971536921421966;
                } else {
                    step2_flag[1 as libc::c_int as usize] = 1 as libc::c_int as uint8;
                    step2_flag[0 as libc::c_int
                        as usize] = step2_flag[1 as libc::c_int as usize];
                    j = 2 as libc::c_int;
                    while j < (*g).values {
                        let mut low: libc::c_int = 0;
                        let mut high: libc::c_int = 0;
                        let mut pred: libc::c_int = 0;
                        let mut highroom: libc::c_int = 0;
                        let mut lowroom: libc::c_int = 0;
                        let mut room: libc::c_int = 0;
                        let mut val: libc::c_int = 0;
                        low = (*g).neighbors[j as usize][0 as libc::c_int as usize]
                            as libc::c_int;
                        high = (*g).neighbors[j as usize][1 as libc::c_int as usize]
                            as libc::c_int;
                        pred = predict_point(
                            (*g).Xlist[j as usize] as libc::c_int,
                            (*g).Xlist[low as usize] as libc::c_int,
                            (*g).Xlist[high as usize] as libc::c_int,
                            *finalY.offset(low as isize) as libc::c_int,
                            *finalY.offset(high as isize) as libc::c_int,
                        );
                        val = *finalY.offset(j as isize) as libc::c_int;
                        highroom = range - pred;
                        lowroom = pred;
                        if highroom < lowroom {
                            room = highroom * 2 as libc::c_int;
                        } else {
                            room = lowroom * 2 as libc::c_int;
                        }
                        if val != 0 {
                            step2_flag[high as usize] = 1 as libc::c_int as uint8;
                            step2_flag[low as usize] = step2_flag[high as usize];
                            step2_flag[j as usize] = 1 as libc::c_int as uint8;
                            if val >= room {
                                if highroom > lowroom {
                                    *finalY
                                        .offset(
                                            j as isize,
                                        ) = (val - lowroom + pred) as libc::c_short;
                                } else {
                                    *finalY
                                        .offset(
                                            j as isize,
                                        ) = (pred - val + highroom - 1 as libc::c_int)
                                        as libc::c_short;
                                }
                            } else if val & 1 as libc::c_int != 0 {
                                *finalY
                                    .offset(
                                        j as isize,
                                    ) = (pred - (val + 1 as libc::c_int >> 1 as libc::c_int))
                                    as libc::c_short;
                            } else {
                                *finalY
                                    .offset(
                                        j as isize,
                                    ) = (pred + (val >> 1 as libc::c_int)) as libc::c_short;
                            }
                        } else {
                            step2_flag[j as usize] = 0 as libc::c_int as uint8;
                            *finalY.offset(j as isize) = pred as libc::c_short;
                        }
                        j += 1;
                    }
                    j = 0 as libc::c_int;
                    while j < (*g).values {
                        if step2_flag[j as usize] == 0 {
                            *finalY
                                .offset(j as isize) = -(1 as libc::c_int) as libc::c_short;
                        }
                        j += 1;
                    }
                    current_block_92 = 4691324637564808323;
                }
            } else {
                current_block_92 = 12111971536921421966;
            }
            match current_block_92 {
                12111971536921421966 => {
                    zero_channel[i as usize] = 1 as libc::c_int;
                }
                _ => {}
            }
        }
        i += 1;
    }
    !((*f).alloc.alloc_buffer).is_null();
    memcpy(
        really_zero_channel.as_mut_ptr() as *mut libc::c_void,
        zero_channel.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*f).channels as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < (*map).coupling_steps as libc::c_int {
        if zero_channel[(*((*map).chan).offset(i as isize)).magnitude as usize] == 0
            || zero_channel[(*((*map).chan).offset(i as isize)).angle as usize] == 0
        {
            zero_channel[(*((*map).chan).offset(i as isize)).angle
                as usize] = 0 as libc::c_int;
            zero_channel[(*((*map).chan).offset(i as isize)).magnitude
                as usize] = zero_channel[(*((*map).chan).offset(i as isize)).angle
                as usize];
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*map).submaps as libc::c_int {
        let mut residue_buffers: [*mut libc::c_float; 16] = [0
            as *mut libc::c_float; 16];
        let mut r: libc::c_int = 0;
        let mut do_not_decode: [uint8; 256] = [0; 256];
        let mut ch: libc::c_int = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*f).channels {
            if (*((*map).chan).offset(j as isize)).mux as libc::c_int == i {
                if zero_channel[j as usize] != 0 {
                    do_not_decode[ch as usize] = 1 as libc::c_int as uint8;
                    residue_buffers[ch as usize] = 0 as *mut libc::c_float;
                } else {
                    do_not_decode[ch as usize] = 0 as libc::c_int as uint8;
                    residue_buffers[ch as usize] = (*f).channel_buffers[j as usize];
                }
                ch += 1;
            }
            j += 1;
        }
        r = (*map).submap_residue[i as usize] as libc::c_int;
        decode_residue(
            f,
            residue_buffers.as_mut_ptr(),
            ch,
            n2,
            r,
            do_not_decode.as_mut_ptr(),
        );
        i += 1;
    }
    !((*f).alloc.alloc_buffer).is_null();
    i = (*map).coupling_steps as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut n2_0: libc::c_int = n >> 1 as libc::c_int;
        let mut m_0: *mut libc::c_float = (*f)
            .channel_buffers[(*((*map).chan).offset(i as isize)).magnitude as usize];
        let mut a: *mut libc::c_float = (*f)
            .channel_buffers[(*((*map).chan).offset(i as isize)).angle as usize];
        j = 0 as libc::c_int;
        while j < n2_0 {
            let mut a2: libc::c_float = 0.;
            let mut m2: libc::c_float = 0.;
            if *m_0.offset(j as isize) > 0 as libc::c_int as libc::c_float {
                if *a.offset(j as isize) > 0 as libc::c_int as libc::c_float {
                    m2 = *m_0.offset(j as isize);
                    a2 = *m_0.offset(j as isize) - *a.offset(j as isize);
                } else {
                    a2 = *m_0.offset(j as isize);
                    m2 = *m_0.offset(j as isize) + *a.offset(j as isize);
                }
            } else if *a.offset(j as isize) > 0 as libc::c_int as libc::c_float {
                m2 = *m_0.offset(j as isize);
                a2 = *m_0.offset(j as isize) + *a.offset(j as isize);
            } else {
                a2 = *m_0.offset(j as isize);
                m2 = *m_0.offset(j as isize) - *a.offset(j as isize);
            }
            *m_0.offset(j as isize) = m2;
            *a.offset(j as isize) = a2;
            j += 1;
        }
        i -= 1;
    }
    i = 0 as libc::c_int;
    while i < (*f).channels {
        if really_zero_channel[i as usize] != 0 {
            memset(
                (*f).channel_buffers[i as usize] as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(n2 as libc::c_ulong),
            );
        } else {
            do_floor(
                f,
                map,
                i,
                n,
                (*f).channel_buffers[i as usize],
                (*f).finalY[i as usize],
                0 as *mut uint8,
            );
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*f).channels {
        inverse_mdct(
            (*f).channel_buffers[i as usize],
            n,
            f,
            (*m).blockflag as libc::c_int,
        );
        i += 1;
    }
    flush_packet(f);
    if (*f).first_decode != 0 {
        (*f).current_loc = (0 as libc::c_uint).wrapping_sub(n2 as libc::c_uint);
        (*f).discard_samples_deferred = n - right_end;
        (*f).current_loc_valid = 1 as libc::c_int;
        (*f).first_decode = 0 as libc::c_int as uint8;
    } else if (*f).discard_samples_deferred != 0 {
        if (*f).discard_samples_deferred >= right_start - left_start {
            (*f).discard_samples_deferred -= right_start - left_start;
            left_start = right_start;
            *p_left = left_start;
        } else {
            left_start += (*f).discard_samples_deferred;
            *p_left = left_start;
            (*f).discard_samples_deferred = 0 as libc::c_int;
        }
    } else {
        (*f).previous_length == 0 as libc::c_int && (*f).current_loc_valid != 0;
    }
    if (*f).last_seg_which == (*f).end_seg_with_known_loc {
        if (*f).current_loc_valid != 0
            && (*f).page_flag as libc::c_int & 4 as libc::c_int != 0
        {
            let mut current_end: uint32 = (*f).known_loc_for_packet;
            if current_end
                < ((*f).current_loc)
                    .wrapping_add((right_end - left_start) as libc::c_uint)
            {
                if current_end < (*f).current_loc {
                    *len = 0 as libc::c_int;
                } else {
                    *len = current_end.wrapping_sub((*f).current_loc) as libc::c_int;
                }
                *len += left_start;
                if *len > right_end {
                    *len = right_end;
                }
                let ref mut fresh30 = (*f).current_loc;
                *fresh30 = (*fresh30 as libc::c_uint).wrapping_add(*len as libc::c_uint)
                    as uint32 as uint32;
                return 1 as libc::c_int;
            }
        }
        (*f)
            .current_loc = ((*f).known_loc_for_packet)
            .wrapping_sub((n2 - left_start) as libc::c_uint);
        (*f).current_loc_valid = 1 as libc::c_int;
    }
    if (*f).current_loc_valid != 0 {
        let ref mut fresh31 = (*f).current_loc;
        *fresh31 = (*fresh31 as libc::c_uint)
            .wrapping_add((right_start - left_start) as libc::c_uint) as uint32
            as uint32;
    }
    !((*f).alloc.alloc_buffer).is_null();
    *len = right_end;
    return 1 as libc::c_int;
}
unsafe extern "C" fn vorbis_decode_packet(
    mut f: *mut vorb,
    mut len: *mut libc::c_int,
    mut p_left: *mut libc::c_int,
    mut p_right: *mut libc::c_int,
) -> libc::c_int {
    let mut mode: libc::c_int = 0;
    let mut left_end: libc::c_int = 0;
    let mut right_end: libc::c_int = 0;
    if vorbis_decode_initial(
        f,
        p_left,
        &mut left_end,
        p_right,
        &mut right_end,
        &mut mode,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return vorbis_decode_packet_rest(
        f,
        len,
        ((*f).mode_config).as_mut_ptr().offset(mode as isize),
        *p_left,
        left_end,
        *p_right,
        right_end,
        p_left,
    );
}
unsafe extern "C" fn vorbis_finish_frame(
    mut f: *mut stb_vorbis,
    mut len: libc::c_int,
    mut left: libc::c_int,
    mut right: libc::c_int,
) -> libc::c_int {
    let mut prev: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*f).previous_length != 0 {
        let mut i_0: libc::c_int = 0;
        let mut j_0: libc::c_int = 0;
        let mut n: libc::c_int = (*f).previous_length;
        let mut w: *mut libc::c_float = get_window(f, n);
        if w.is_null() {
            return 0 as libc::c_int;
        }
        i_0 = 0 as libc::c_int;
        while i_0 < (*f).channels {
            j_0 = 0 as libc::c_int;
            while j_0 < n {
                *((*f).channel_buffers[i_0 as usize])
                    .offset(
                        (left + j_0) as isize,
                    ) = *((*f).channel_buffers[i_0 as usize])
                    .offset((left + j_0) as isize) * *w.offset(j_0 as isize)
                    + *((*f).previous_window[i_0 as usize]).offset(j_0 as isize)
                        * *w.offset((n - 1 as libc::c_int - j_0) as isize);
                j_0 += 1;
            }
            i_0 += 1;
        }
    }
    prev = (*f).previous_length;
    (*f).previous_length = len - right;
    i = 0 as libc::c_int;
    while i < (*f).channels {
        j = 0 as libc::c_int;
        while right + j < len {
            *((*f).previous_window[i as usize])
                .offset(
                    j as isize,
                ) = *((*f).channel_buffers[i as usize]).offset((right + j) as isize);
            j += 1;
        }
        i += 1;
    }
    if prev == 0 {
        return 0 as libc::c_int;
    }
    if len < right {
        right = len;
    }
    let ref mut fresh32 = (*f).samples_output;
    *fresh32 = (*fresh32 as libc::c_uint).wrapping_add((right - left) as libc::c_uint)
        as uint32 as uint32;
    return right - left;
}
unsafe extern "C" fn vorbis_pump_first_frame(mut f: *mut stb_vorbis) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    res = vorbis_decode_packet(f, &mut len, &mut left, &mut right);
    if res != 0 {
        vorbis_finish_frame(f, len, left, right);
    }
    return res;
}
unsafe extern "C" fn is_whole_packet_present(mut f: *mut stb_vorbis) -> libc::c_int {
    let mut s: libc::c_int = (*f).next_seg;
    let mut first: libc::c_int = 1 as libc::c_int;
    let mut p: *mut uint8 = (*f).stream;
    if s != -(1 as libc::c_int) {
        while s < (*f).segment_count {
            p = p.offset((*f).segments[s as usize] as libc::c_int as isize);
            if ((*f).segments[s as usize] as libc::c_int) < 255 as libc::c_int {
                break;
            }
            s += 1;
        }
        if s == (*f).segment_count {
            s = -(1 as libc::c_int);
        }
        if p > (*f).stream_end {
            return error(f, VORBIS_need_more_data);
        }
        first = 0 as libc::c_int;
    }
    while s == -(1 as libc::c_int) {
        let mut q: *mut uint8 = 0 as *mut uint8;
        let mut n: libc::c_int = 0;
        if p.offset(26 as libc::c_int as isize) >= (*f).stream_end {
            return error(f, VORBIS_need_more_data);
        }
        if memcmp(
            p as *const libc::c_void,
            ogg_page_header.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
        {
            return error(f, VORBIS_invalid_stream);
        }
        if *p.offset(4 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
            return error(f, VORBIS_invalid_stream);
        }
        if first != 0 {
            if (*f).previous_length != 0 {
                if *p.offset(5 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int
                    != 0
                {
                    return error(f, VORBIS_invalid_stream);
                }
            }
        } else if *p.offset(5 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int
            == 0
        {
            return error(f, VORBIS_invalid_stream)
        }
        n = *p.offset(26 as libc::c_int as isize) as libc::c_int;
        q = p.offset(27 as libc::c_int as isize);
        p = q.offset(n as isize);
        if p > (*f).stream_end {
            return error(f, VORBIS_need_more_data);
        }
        s = 0 as libc::c_int;
        while s < n {
            p = p.offset(*q.offset(s as isize) as libc::c_int as isize);
            if (*q.offset(s as isize) as libc::c_int) < 255 as libc::c_int {
                break;
            }
            s += 1;
        }
        if s == n {
            s = -(1 as libc::c_int);
        }
        if p > (*f).stream_end {
            return error(f, VORBIS_need_more_data);
        }
        first = 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn start_decoder(mut f: *mut vorb) -> libc::c_int {
    let mut header: [uint8; 6] = [0; 6];
    let mut x: uint8 = 0;
    let mut y: uint8 = 0;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut max_submaps: libc::c_int = 0 as libc::c_int;
    let mut longest_floorlist: libc::c_int = 0 as libc::c_int;
    (*f).first_decode = 1 as libc::c_int as uint8;
    if start_page(f) == 0 {
        return 0 as libc::c_int;
    }
    if (*f).page_flag as libc::c_int & 2 as libc::c_int == 0 {
        return error(f, VORBIS_invalid_first_page);
    }
    if (*f).page_flag as libc::c_int & 4 as libc::c_int != 0 {
        return error(f, VORBIS_invalid_first_page);
    }
    if (*f).page_flag as libc::c_int & 1 as libc::c_int != 0 {
        return error(f, VORBIS_invalid_first_page);
    }
    if (*f).segment_count != 1 as libc::c_int {
        return error(f, VORBIS_invalid_first_page);
    }
    if (*f).segments[0 as libc::c_int as usize] as libc::c_int != 30 as libc::c_int {
        if (*f).segments[0 as libc::c_int as usize] as libc::c_int == 64 as libc::c_int
            && getn(f, header.as_mut_ptr(), 6 as libc::c_int) != 0
            && header[0 as libc::c_int as usize] as libc::c_int == 'f' as i32
            && header[1 as libc::c_int as usize] as libc::c_int == 'i' as i32
            && header[2 as libc::c_int as usize] as libc::c_int == 's' as i32
            && header[3 as libc::c_int as usize] as libc::c_int == 'h' as i32
            && header[4 as libc::c_int as usize] as libc::c_int == 'e' as i32
            && header[5 as libc::c_int as usize] as libc::c_int == 'a' as i32
            && get8(f) as libc::c_int == 'd' as i32
            && get8(f) as libc::c_int == '\0' as i32
        {
            return error(f, VORBIS_ogg_skeleton_not_supported)
        } else {
            return error(f, VORBIS_invalid_first_page)
        }
    }
    if get8(f) as libc::c_int != VORBIS_packet_id as libc::c_int {
        return error(f, VORBIS_invalid_first_page);
    }
    if getn(f, header.as_mut_ptr(), 6 as libc::c_int) == 0 {
        return error(f, VORBIS_unexpected_eof);
    }
    if vorbis_validate(header.as_mut_ptr()) == 0 {
        return error(f, VORBIS_invalid_first_page);
    }
    if get32(f) != 0 as libc::c_int as libc::c_uint {
        return error(f, VORBIS_invalid_first_page);
    }
    (*f).channels = get8(f) as libc::c_int;
    if (*f).channels == 0 {
        return error(f, VORBIS_invalid_first_page);
    }
    if (*f).channels > 16 as libc::c_int {
        return error(f, VORBIS_too_many_channels);
    }
    (*f).sample_rate = get32(f);
    if (*f).sample_rate == 0 {
        return error(f, VORBIS_invalid_first_page);
    }
    get32(f);
    get32(f);
    get32(f);
    x = get8(f);
    let mut log0: libc::c_int = 0;
    let mut log1: libc::c_int = 0;
    log0 = x as libc::c_int & 15 as libc::c_int;
    log1 = x as libc::c_int >> 4 as libc::c_int;
    (*f).blocksize_0 = (1 as libc::c_int) << log0;
    (*f).blocksize_1 = (1 as libc::c_int) << log1;
    if log0 < 6 as libc::c_int || log0 > 13 as libc::c_int {
        return error(f, VORBIS_invalid_setup);
    }
    if log1 < 6 as libc::c_int || log1 > 13 as libc::c_int {
        return error(f, VORBIS_invalid_setup);
    }
    if log0 > log1 {
        return error(f, VORBIS_invalid_setup);
    }
    x = get8(f);
    if x as libc::c_int & 1 as libc::c_int == 0 {
        return error(f, VORBIS_invalid_first_page);
    }
    if start_page(f) == 0 {
        return 0 as libc::c_int;
    }
    if start_packet(f) == 0 {
        return 0 as libc::c_int;
    }
    if next_segment(f) == 0 {
        return 0 as libc::c_int;
    }
    if get8_packet(f) != VORBIS_packet_comment as libc::c_int {
        return error(f, VORBIS_invalid_setup);
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        header[i as usize] = get8_packet(f) as uint8;
        i += 1;
    }
    if vorbis_validate(header.as_mut_ptr()) == 0 {
        return error(f, VORBIS_invalid_setup);
    }
    len = get32_packet(f);
    let ref mut fresh33 = (*f).vendor;
    *fresh33 = setup_malloc(
        f,
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((len + 1 as libc::c_int) as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_char;
    if ((*f).vendor).is_null() {
        return error(f, VORBIS_outofmem);
    }
    i = 0 as libc::c_int;
    while i < len {
        *((*f).vendor).offset(i as isize) = get8_packet(f) as libc::c_char;
        i += 1;
    }
    *((*f).vendor).offset(len as isize) = '\0' as i32 as libc::c_char;
    (*f).comment_list_length = get32_packet(f);
    let ref mut fresh34 = (*f).comment_list;
    *fresh34 = 0 as *mut *mut libc::c_char;
    if (*f).comment_list_length > 0 as libc::c_int {
        let ref mut fresh35 = (*f).comment_list;
        *fresh35 = setup_malloc(
            f,
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*f).comment_list_length as libc::c_ulong) as libc::c_int,
        ) as *mut *mut libc::c_char;
        if ((*f).comment_list).is_null() {
            return error(f, VORBIS_outofmem);
        }
    }
    i = 0 as libc::c_int;
    while i < (*f).comment_list_length {
        len = get32_packet(f);
        let ref mut fresh36 = *((*f).comment_list).offset(i as isize);
        *fresh36 = setup_malloc(
            f,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((len + 1 as libc::c_int) as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        if (*((*f).comment_list).offset(i as isize)).is_null() {
            return error(f, VORBIS_outofmem);
        }
        j = 0 as libc::c_int;
        while j < len {
            *(*((*f).comment_list).offset(i as isize))
                .offset(j as isize) = get8_packet(f) as libc::c_char;
            j += 1;
        }
        *(*((*f).comment_list).offset(i as isize))
            .offset(len as isize) = '\0' as i32 as libc::c_char;
        i += 1;
    }
    x = get8_packet(f) as uint8;
    if x as libc::c_int & 1 as libc::c_int == 0 {
        return error(f, VORBIS_invalid_setup);
    }
    skip(f, (*f).bytes_in_seg as libc::c_int);
    (*f).bytes_in_seg = 0 as libc::c_int as uint8;
    loop {
        len = next_segment(f);
        skip(f, len);
        (*f).bytes_in_seg = 0 as libc::c_int as uint8;
        if !(len != 0) {
            break;
        }
    }
    if start_packet(f) == 0 {
        return 0 as libc::c_int;
    }
    if (*f).push_mode != 0 {
        if is_whole_packet_present(f) == 0 {
            if (*f).error as libc::c_uint
                == VORBIS_invalid_stream as libc::c_int as libc::c_uint
            {
                (*f).error = VORBIS_invalid_setup;
            }
            return 0 as libc::c_int;
        }
    }
    crc32_init();
    if get8_packet(f) != VORBIS_packet_setup as libc::c_int {
        return error(f, VORBIS_invalid_setup);
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        header[i as usize] = get8_packet(f) as uint8;
        i += 1;
    }
    if vorbis_validate(header.as_mut_ptr()) == 0 {
        return error(f, VORBIS_invalid_setup);
    }
    (*f)
        .codebook_count = (get_bits(f, 8 as libc::c_int))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let ref mut fresh37 = (*f).codebooks;
    *fresh37 = setup_malloc(
        f,
        (::std::mem::size_of::<Codebook>() as libc::c_ulong)
            .wrapping_mul((*f).codebook_count as libc::c_ulong) as libc::c_int,
    ) as *mut Codebook;
    if ((*f).codebooks).is_null() {
        return error(f, VORBIS_outofmem);
    }
    memset(
        (*f).codebooks as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<Codebook>() as libc::c_ulong)
            .wrapping_mul((*f).codebook_count as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < (*f).codebook_count {
        let mut values: *mut uint32 = 0 as *mut uint32;
        let mut ordered: libc::c_int = 0;
        let mut sorted_count: libc::c_int = 0;
        let mut total: libc::c_int = 0 as libc::c_int;
        let mut lengths: *mut uint8 = 0 as *mut uint8;
        let mut c: *mut Codebook = ((*f).codebooks).offset(i as isize);
        x = get_bits(f, 8 as libc::c_int) as uint8;
        if x as libc::c_int != 0x42 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        x = get_bits(f, 8 as libc::c_int) as uint8;
        if x as libc::c_int != 0x43 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        x = get_bits(f, 8 as libc::c_int) as uint8;
        if x as libc::c_int != 0x56 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        x = get_bits(f, 8 as libc::c_int) as uint8;
        (*c)
            .dimensions = (get_bits(f, 8 as libc::c_int) << 8 as libc::c_int)
            .wrapping_add(x as libc::c_uint) as libc::c_int;
        x = get_bits(f, 8 as libc::c_int) as uint8;
        y = get_bits(f, 8 as libc::c_int) as uint8;
        (*c)
            .entries = (get_bits(f, 8 as libc::c_int) << 16 as libc::c_int)
            .wrapping_add(((y as libc::c_int) << 8 as libc::c_int) as libc::c_uint)
            .wrapping_add(x as libc::c_uint) as libc::c_int;
        ordered = get_bits(f, 1 as libc::c_int) as libc::c_int;
        (*c)
            .sparse = (if ordered != 0 {
            0 as libc::c_int as libc::c_uint
        } else {
            get_bits(f, 1 as libc::c_int)
        }) as uint8;
        if (*c).dimensions == 0 as libc::c_int && (*c).entries != 0 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        if (*c).sparse != 0 {
            lengths = setup_temp_malloc(f, (*c).entries) as *mut uint8;
        } else {
            let ref mut fresh38 = (*c).codeword_lengths;
            *fresh38 = setup_malloc(f, (*c).entries) as *mut uint8;
            lengths = *fresh38;
        }
        if lengths.is_null() {
            return error(f, VORBIS_outofmem);
        }
        if ordered != 0 {
            let mut current_entry: libc::c_int = 0 as libc::c_int;
            let mut current_length: libc::c_int = (get_bits(f, 5 as libc::c_int))
                .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            while current_entry < (*c).entries {
                let mut limit: libc::c_int = (*c).entries - current_entry;
                let mut n: libc::c_int = get_bits(f, ilog(limit)) as libc::c_int;
                if current_length >= 32 as libc::c_int {
                    return error(f, VORBIS_invalid_setup);
                }
                if current_entry + n > (*c).entries {
                    return error(f, VORBIS_invalid_setup);
                }
                memset(
                    lengths.offset(current_entry as isize) as *mut libc::c_void,
                    current_length,
                    n as libc::c_ulong,
                );
                current_entry += n;
                current_length += 1;
            }
        } else {
            j = 0 as libc::c_int;
            while j < (*c).entries {
                let mut present: libc::c_int = (if (*c).sparse as libc::c_int != 0 {
                    get_bits(f, 1 as libc::c_int)
                } else {
                    1 as libc::c_int as libc::c_uint
                }) as libc::c_int;
                if present != 0 {
                    *lengths
                        .offset(
                            j as isize,
                        ) = (get_bits(f, 5 as libc::c_int))
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint8;
                    total += 1;
                    if *lengths.offset(j as isize) as libc::c_int == 32 as libc::c_int {
                        return error(f, VORBIS_invalid_setup);
                    }
                } else {
                    *lengths.offset(j as isize) = 255 as libc::c_int as uint8;
                }
                j += 1;
            }
        }
        if (*c).sparse as libc::c_int != 0 && total >= (*c).entries >> 2 as libc::c_int {
            if (*c).entries > (*f).setup_temp_memory_required as libc::c_int {
                (*f).setup_temp_memory_required = (*c).entries as libc::c_uint;
            }
            let ref mut fresh39 = (*c).codeword_lengths;
            *fresh39 = setup_malloc(f, (*c).entries) as *mut uint8;
            if ((*c).codeword_lengths).is_null() {
                return error(f, VORBIS_outofmem);
            }
            memcpy(
                (*c).codeword_lengths as *mut libc::c_void,
                lengths as *const libc::c_void,
                (*c).entries as libc::c_ulong,
            );
            setup_temp_free(f, lengths as *mut libc::c_void, (*c).entries);
            lengths = (*c).codeword_lengths;
            (*c).sparse = 0 as libc::c_int as uint8;
        }
        if (*c).sparse != 0 {
            sorted_count = total;
        } else {
            sorted_count = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*c).entries {
                if *lengths.offset(j as isize) as libc::c_int > 10 as libc::c_int
                    && *lengths.offset(j as isize) as libc::c_int != 255 as libc::c_int
                {
                    sorted_count += 1;
                }
                j += 1;
            }
        }
        (*c).sorted_entries = sorted_count;
        values = 0 as *mut uint32;
        if (*c).sparse == 0 {
            let ref mut fresh40 = (*c).codewords;
            *fresh40 = setup_malloc(
                f,
                (::std::mem::size_of::<uint32>() as libc::c_ulong)
                    .wrapping_mul((*c).entries as libc::c_ulong) as libc::c_int,
            ) as *mut uint32;
            if ((*c).codewords).is_null() {
                return error(f, VORBIS_outofmem);
            }
        } else {
            let mut size: libc::c_uint = 0;
            if (*c).sorted_entries != 0 {
                let ref mut fresh41 = (*c).codeword_lengths;
                *fresh41 = setup_malloc(f, (*c).sorted_entries) as *mut uint8;
                if ((*c).codeword_lengths).is_null() {
                    return error(f, VORBIS_outofmem);
                }
                let ref mut fresh42 = (*c).codewords;
                *fresh42 = setup_temp_malloc(
                    f,
                    (::std::mem::size_of::<uint32>() as libc::c_ulong)
                        .wrapping_mul((*c).sorted_entries as libc::c_ulong)
                        as libc::c_int,
                ) as *mut uint32;
                if ((*c).codewords).is_null() {
                    return error(f, VORBIS_outofmem);
                }
                values = setup_temp_malloc(
                    f,
                    (::std::mem::size_of::<uint32>() as libc::c_ulong)
                        .wrapping_mul((*c).sorted_entries as libc::c_ulong)
                        as libc::c_int,
                ) as *mut uint32;
                if values.is_null() {
                    return error(f, VORBIS_outofmem);
                }
            }
            size = ((*c).entries as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<uint32>() as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<uint32>() as libc::c_ulong)
                        .wrapping_mul((*c).sorted_entries as libc::c_ulong),
                ) as libc::c_uint;
            if size > (*f).setup_temp_memory_required {
                (*f).setup_temp_memory_required = size;
            }
        }
        if compute_codewords(c, lengths, (*c).entries, values) == 0 {
            if (*c).sparse != 0 {
                setup_temp_free(f, values as *mut libc::c_void, 0 as libc::c_int);
            }
            return error(f, VORBIS_invalid_setup);
        }
        if (*c).sorted_entries != 0 {
            let ref mut fresh43 = (*c).sorted_codewords;
            *fresh43 = setup_malloc(
                f,
                (::std::mem::size_of::<uint32>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*c).sorted_entries + 1 as libc::c_int) as libc::c_ulong,
                    ) as libc::c_int,
            ) as *mut uint32;
            if ((*c).sorted_codewords).is_null() {
                return error(f, VORBIS_outofmem);
            }
            let ref mut fresh44 = (*c).sorted_values;
            *fresh44 = setup_malloc(
                f,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*c).sorted_entries + 1 as libc::c_int) as libc::c_ulong,
                    ) as libc::c_int,
            ) as *mut libc::c_int;
            if ((*c).sorted_values).is_null() {
                return error(f, VORBIS_outofmem);
            }
            let ref mut fresh45 = (*c).sorted_values;
            *fresh45 = (*fresh45).offset(1);
            *((*c).sorted_values)
                .offset(-(1 as libc::c_int) as isize) = -(1 as libc::c_int);
            compute_sorted_huffman(c, lengths, values);
        }
        if (*c).sparse != 0 {
            setup_temp_free(
                f,
                values as *mut libc::c_void,
                (::std::mem::size_of::<uint32>() as libc::c_ulong)
                    .wrapping_mul((*c).sorted_entries as libc::c_ulong) as libc::c_int,
            );
            setup_temp_free(
                f,
                (*c).codewords as *mut libc::c_void,
                (::std::mem::size_of::<uint32>() as libc::c_ulong)
                    .wrapping_mul((*c).sorted_entries as libc::c_ulong) as libc::c_int,
            );
            setup_temp_free(f, lengths as *mut libc::c_void, (*c).entries);
            let ref mut fresh46 = (*c).codewords;
            *fresh46 = 0 as *mut uint32;
        }
        compute_accelerated_huffman(c);
        (*c).lookup_type = get_bits(f, 4 as libc::c_int) as uint8;
        if (*c).lookup_type as libc::c_int > 2 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        if (*c).lookup_type as libc::c_int > 0 as libc::c_int {
            let mut current_block_305: u64;
            let mut mults: *mut uint16 = 0 as *mut uint16;
            (*c).minimum_value = float32_unpack(get_bits(f, 32 as libc::c_int));
            (*c).delta_value = float32_unpack(get_bits(f, 32 as libc::c_int));
            (*c)
                .value_bits = (get_bits(f, 4 as libc::c_int))
                .wrapping_add(1 as libc::c_int as libc::c_uint) as uint8;
            (*c).sequence_p = get_bits(f, 1 as libc::c_int) as uint8;
            if (*c).lookup_type as libc::c_int == 1 as libc::c_int {
                let mut values_0: libc::c_int = lookup1_values(
                    (*c).entries,
                    (*c).dimensions,
                );
                if values_0 < 0 as libc::c_int {
                    return error(f, VORBIS_invalid_setup);
                }
                (*c).lookup_values = values_0 as uint32;
            } else {
                (*c).lookup_values = ((*c).entries * (*c).dimensions) as uint32;
            }
            if (*c).lookup_values == 0 as libc::c_int as libc::c_uint {
                return error(f, VORBIS_invalid_setup);
            }
            mults = setup_temp_malloc(
                f,
                (::std::mem::size_of::<uint16>() as libc::c_ulong)
                    .wrapping_mul((*c).lookup_values as libc::c_ulong) as libc::c_int,
            ) as *mut uint16;
            if mults.is_null() {
                return error(f, VORBIS_outofmem);
            }
            j = 0 as libc::c_int;
            while j < (*c).lookup_values as libc::c_int {
                let mut q: libc::c_int = get_bits(f, (*c).value_bits as libc::c_int)
                    as libc::c_int;
                if q == -(1 as libc::c_int) {
                    setup_temp_free(
                        f,
                        mults as *mut libc::c_void,
                        (::std::mem::size_of::<uint16>() as libc::c_ulong)
                            .wrapping_mul((*c).lookup_values as libc::c_ulong)
                            as libc::c_int,
                    );
                    return error(f, VORBIS_invalid_setup);
                }
                *mults.offset(j as isize) = q as uint16;
                j += 1;
            }
            if (*c).lookup_type as libc::c_int == 1 as libc::c_int {
                let mut len_0: libc::c_int = 0;
                let mut sparse: libc::c_int = (*c).sparse as libc::c_int;
                let mut last: libc::c_float = 0 as libc::c_int as libc::c_float;
                if sparse != 0 {
                    if (*c).sorted_entries == 0 as libc::c_int {
                        current_block_305 = 4241823157621550342;
                    } else {
                        let ref mut fresh47 = (*c).multiplicands;
                        *fresh47 = setup_malloc(
                            f,
                            (::std::mem::size_of::<codetype>() as libc::c_ulong)
                                .wrapping_mul((*c).sorted_entries as libc::c_ulong)
                                .wrapping_mul((*c).dimensions as libc::c_ulong)
                                as libc::c_int,
                        ) as *mut codetype;
                        current_block_305 = 2794870492977015345;
                    }
                } else {
                    let ref mut fresh48 = (*c).multiplicands;
                    *fresh48 = setup_malloc(
                        f,
                        (::std::mem::size_of::<codetype>() as libc::c_ulong)
                            .wrapping_mul((*c).entries as libc::c_ulong)
                            .wrapping_mul((*c).dimensions as libc::c_ulong)
                            as libc::c_int,
                    ) as *mut codetype;
                    current_block_305 = 2794870492977015345;
                }
                match current_block_305 {
                    4241823157621550342 => {}
                    _ => {
                        if ((*c).multiplicands).is_null() {
                            setup_temp_free(
                                f,
                                mults as *mut libc::c_void,
                                (::std::mem::size_of::<uint16>() as libc::c_ulong)
                                    .wrapping_mul((*c).lookup_values as libc::c_ulong)
                                    as libc::c_int,
                            );
                            return error(f, VORBIS_outofmem);
                        }
                        len_0 = if sparse != 0 {
                            (*c).sorted_entries
                        } else {
                            (*c).entries
                        };
                        j = 0 as libc::c_int;
                        while j < len_0 {
                            let mut z: libc::c_uint = (if sparse != 0 {
                                *((*c).sorted_values).offset(j as isize)
                            } else {
                                j
                            }) as libc::c_uint;
                            let mut div: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                            k = 0 as libc::c_int;
                            while k < (*c).dimensions {
                                let mut off: libc::c_int = z
                                    .wrapping_div(div)
                                    .wrapping_rem((*c).lookup_values) as libc::c_int;
                                let mut val: libc::c_float = *mults.offset(off as isize)
                                    as libc::c_int as libc::c_float * (*c).delta_value
                                    + (*c).minimum_value + last;
                                *((*c).multiplicands)
                                    .offset((j * (*c).dimensions + k) as isize) = val;
                                if (*c).sequence_p != 0 {
                                    last = val;
                                }
                                if (k + 1 as libc::c_int) < (*c).dimensions {
                                    if div
                                        > (2147483647 as libc::c_int as libc::c_uint)
                                            .wrapping_mul(2 as libc::c_uint)
                                            .wrapping_add(1 as libc::c_uint)
                                            .wrapping_div((*c).lookup_values)
                                    {
                                        setup_temp_free(
                                            f,
                                            mults as *mut libc::c_void,
                                            (::std::mem::size_of::<uint16>() as libc::c_ulong)
                                                .wrapping_mul((*c).lookup_values as libc::c_ulong)
                                                as libc::c_int,
                                        );
                                        return error(f, VORBIS_invalid_setup);
                                    }
                                    div = div.wrapping_mul((*c).lookup_values);
                                }
                                k += 1;
                            }
                            j += 1;
                        }
                        (*c).lookup_type = 2 as libc::c_int as uint8;
                    }
                }
            } else {
                let mut last_0: libc::c_float = 0 as libc::c_int as libc::c_float;
                let ref mut fresh49 = (*c).multiplicands;
                *fresh49 = setup_malloc(
                    f,
                    (::std::mem::size_of::<codetype>() as libc::c_ulong)
                        .wrapping_mul((*c).lookup_values as libc::c_ulong) as libc::c_int,
                ) as *mut codetype;
                if ((*c).multiplicands).is_null() {
                    setup_temp_free(
                        f,
                        mults as *mut libc::c_void,
                        (::std::mem::size_of::<uint16>() as libc::c_ulong)
                            .wrapping_mul((*c).lookup_values as libc::c_ulong)
                            as libc::c_int,
                    );
                    return error(f, VORBIS_outofmem);
                }
                j = 0 as libc::c_int;
                while j < (*c).lookup_values as libc::c_int {
                    let mut val_0: libc::c_float = *mults.offset(j as isize)
                        as libc::c_int as libc::c_float * (*c).delta_value
                        + (*c).minimum_value + last_0;
                    *((*c).multiplicands).offset(j as isize) = val_0;
                    if (*c).sequence_p != 0 {
                        last_0 = val_0;
                    }
                    j += 1;
                }
            }
            setup_temp_free(
                f,
                mults as *mut libc::c_void,
                (::std::mem::size_of::<uint16>() as libc::c_ulong)
                    .wrapping_mul((*c).lookup_values as libc::c_ulong) as libc::c_int,
            );
        }
        i += 1;
    }
    x = (get_bits(f, 6 as libc::c_int)).wrapping_add(1 as libc::c_int as libc::c_uint)
        as uint8;
    i = 0 as libc::c_int;
    while i < x as libc::c_int {
        let mut z_0: uint32 = get_bits(f, 16 as libc::c_int);
        if z_0 != 0 as libc::c_int as libc::c_uint {
            return error(f, VORBIS_invalid_setup);
        }
        i += 1;
    }
    (*f)
        .floor_count = (get_bits(f, 6 as libc::c_int))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let ref mut fresh50 = (*f).floor_config;
    *fresh50 = setup_malloc(
        f,
        ((*f).floor_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Floor>() as libc::c_ulong) as libc::c_int,
    ) as *mut Floor;
    if ((*f).floor_config).is_null() {
        return error(f, VORBIS_outofmem);
    }
    i = 0 as libc::c_int;
    while i < (*f).floor_count {
        (*f).floor_types[i as usize] = get_bits(f, 16 as libc::c_int) as uint16;
        if (*f).floor_types[i as usize] as libc::c_int > 1 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        if (*f).floor_types[i as usize] as libc::c_int == 0 as libc::c_int {
            let mut g: *mut Floor0 = &mut (*((*f).floor_config).offset(i as isize))
                .floor0;
            (*g).order = get_bits(f, 8 as libc::c_int) as uint8;
            (*g).rate = get_bits(f, 16 as libc::c_int) as uint16;
            (*g).bark_map_size = get_bits(f, 16 as libc::c_int) as uint16;
            (*g).amplitude_bits = get_bits(f, 6 as libc::c_int) as uint8;
            (*g).amplitude_offset = get_bits(f, 8 as libc::c_int) as uint8;
            (*g)
                .number_of_books = (get_bits(f, 4 as libc::c_int))
                .wrapping_add(1 as libc::c_int as libc::c_uint) as uint8;
            j = 0 as libc::c_int;
            while j < (*g).number_of_books as libc::c_int {
                (*g).book_list[j as usize] = get_bits(f, 8 as libc::c_int) as uint8;
                j += 1;
            }
            return error(f, VORBIS_feature_not_supported);
        } else {
            let mut p: [stbv__floor_ordering; 250] = [stbv__floor_ordering {
                x: 0,
                id: 0,
            }; 250];
            let mut g_0: *mut Floor1 = &mut (*((*f).floor_config).offset(i as isize))
                .floor1;
            let mut max_class: libc::c_int = -(1 as libc::c_int);
            (*g_0).partitions = get_bits(f, 5 as libc::c_int) as uint8;
            j = 0 as libc::c_int;
            while j < (*g_0).partitions as libc::c_int {
                (*g_0)
                    .partition_class_list[j
                    as usize] = get_bits(f, 4 as libc::c_int) as uint8;
                if (*g_0).partition_class_list[j as usize] as libc::c_int > max_class {
                    max_class = (*g_0).partition_class_list[j as usize] as libc::c_int;
                }
                j += 1;
            }
            j = 0 as libc::c_int;
            while j <= max_class {
                (*g_0)
                    .class_dimensions[j
                    as usize] = (get_bits(f, 3 as libc::c_int))
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as uint8;
                (*g_0)
                    .class_subclasses[j
                    as usize] = get_bits(f, 2 as libc::c_int) as uint8;
                if (*g_0).class_subclasses[j as usize] != 0 {
                    (*g_0)
                        .class_masterbooks[j
                        as usize] = get_bits(f, 8 as libc::c_int) as uint8;
                    if (*g_0).class_masterbooks[j as usize] as libc::c_int
                        >= (*f).codebook_count
                    {
                        return error(f, VORBIS_invalid_setup);
                    }
                }
                k = 0 as libc::c_int;
                while k
                    < (1 as libc::c_int)
                        << (*g_0).class_subclasses[j as usize] as libc::c_int
                {
                    (*g_0)
                        .subclass_books[j
                        as usize][k
                        as usize] = (get_bits(f, 8 as libc::c_int) as int16
                        as libc::c_int - 1 as libc::c_int) as int16;
                    if (*g_0).subclass_books[j as usize][k as usize] as libc::c_int
                        >= (*f).codebook_count
                    {
                        return error(f, VORBIS_invalid_setup);
                    }
                    k += 1;
                }
                j += 1;
            }
            (*g_0)
                .floor1_multiplier = (get_bits(f, 2 as libc::c_int))
                .wrapping_add(1 as libc::c_int as libc::c_uint) as uint8;
            (*g_0).rangebits = get_bits(f, 4 as libc::c_int) as uint8;
            (*g_0).Xlist[0 as libc::c_int as usize] = 0 as libc::c_int as uint16;
            (*g_0)
                .Xlist[1 as libc::c_int
                as usize] = ((1 as libc::c_int) << (*g_0).rangebits as libc::c_int)
                as uint16;
            (*g_0).values = 2 as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*g_0).partitions as libc::c_int {
                let mut c_0: libc::c_int = (*g_0).partition_class_list[j as usize]
                    as libc::c_int;
                k = 0 as libc::c_int;
                while k < (*g_0).class_dimensions[c_0 as usize] as libc::c_int {
                    (*g_0)
                        .Xlist[(*g_0).values
                        as usize] = get_bits(f, (*g_0).rangebits as libc::c_int)
                        as uint16;
                    let ref mut fresh51 = (*g_0).values;
                    *fresh51 += 1;
                    k += 1;
                }
                j += 1;
            }
            j = 0 as libc::c_int;
            while j < (*g_0).values {
                p[j as usize].x = (*g_0).Xlist[j as usize];
                p[j as usize].id = j as uint16;
                j += 1;
            }
            qsort(
                p.as_mut_ptr() as *mut libc::c_void,
                (*g_0).values as size_t,
                ::std::mem::size_of::<stbv__floor_ordering>() as libc::c_ulong,
                Some(
                    point_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            j = 0 as libc::c_int;
            while j < (*g_0).values - 1 as libc::c_int {
                if p[j as usize].x as libc::c_int
                    == p[(j + 1 as libc::c_int) as usize].x as libc::c_int
                {
                    return error(f, VORBIS_invalid_setup);
                }
                j += 1;
            }
            j = 0 as libc::c_int;
            while j < (*g_0).values {
                (*g_0).sorted_order[j as usize] = p[j as usize].id as uint8;
                j += 1;
            }
            j = 2 as libc::c_int;
            while j < (*g_0).values {
                let mut low: libc::c_int = 0 as libc::c_int;
                let mut hi: libc::c_int = 0 as libc::c_int;
                neighbors(((*g_0).Xlist).as_mut_ptr(), j, &mut low, &mut hi);
                (*g_0).neighbors[j as usize][0 as libc::c_int as usize] = low as uint8;
                (*g_0).neighbors[j as usize][1 as libc::c_int as usize] = hi as uint8;
                j += 1;
            }
            if (*g_0).values > longest_floorlist {
                longest_floorlist = (*g_0).values;
            }
        }
        i += 1;
    }
    (*f)
        .residue_count = (get_bits(f, 6 as libc::c_int))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let ref mut fresh52 = (*f).residue_config;
    *fresh52 = setup_malloc(
        f,
        ((*f).residue_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Residue>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut Residue;
    if ((*f).residue_config).is_null() {
        return error(f, VORBIS_outofmem);
    }
    memset(
        (*f).residue_config as *mut libc::c_void,
        0 as libc::c_int,
        ((*f).residue_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Residue>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < (*f).residue_count {
        let mut residue_cascade: [uint8; 64] = [0; 64];
        let mut r: *mut Residue = ((*f).residue_config).offset(i as isize);
        (*f).residue_types[i as usize] = get_bits(f, 16 as libc::c_int) as uint16;
        if (*f).residue_types[i as usize] as libc::c_int > 2 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        (*r).begin = get_bits(f, 24 as libc::c_int);
        (*r).end = get_bits(f, 24 as libc::c_int);
        if (*r).end < (*r).begin {
            return error(f, VORBIS_invalid_setup);
        }
        (*r)
            .part_size = (get_bits(f, 24 as libc::c_int))
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        (*r)
            .classifications = (get_bits(f, 6 as libc::c_int))
            .wrapping_add(1 as libc::c_int as libc::c_uint) as uint8;
        (*r).classbook = get_bits(f, 8 as libc::c_int) as uint8;
        if (*r).classbook as libc::c_int >= (*f).codebook_count {
            return error(f, VORBIS_invalid_setup);
        }
        j = 0 as libc::c_int;
        while j < (*r).classifications as libc::c_int {
            let mut high_bits: uint8 = 0 as libc::c_int as uint8;
            let mut low_bits: uint8 = get_bits(f, 3 as libc::c_int) as uint8;
            if get_bits(f, 1 as libc::c_int) != 0 {
                high_bits = get_bits(f, 5 as libc::c_int) as uint8;
            }
            residue_cascade[j
                as usize] = (high_bits as libc::c_int * 8 as libc::c_int
                + low_bits as libc::c_int) as uint8;
            j += 1;
        }
        let ref mut fresh53 = (*r).residue_books;
        *fresh53 = setup_malloc(
            f,
            (::std::mem::size_of::<[int16; 8]>() as libc::c_ulong)
                .wrapping_mul((*r).classifications as libc::c_ulong) as libc::c_int,
        ) as *mut [libc::c_short; 8];
        if ((*r).residue_books).is_null() {
            return error(f, VORBIS_outofmem);
        }
        j = 0 as libc::c_int;
        while j < (*r).classifications as libc::c_int {
            k = 0 as libc::c_int;
            while k < 8 as libc::c_int {
                if residue_cascade[j as usize] as libc::c_int & (1 as libc::c_int) << k
                    != 0
                {
                    (*((*r).residue_books)
                        .offset(
                            j as isize,
                        ))[k as usize] = get_bits(f, 8 as libc::c_int) as int16;
                    if (*((*r).residue_books).offset(j as isize))[k as usize]
                        as libc::c_int >= (*f).codebook_count
                    {
                        return error(f, VORBIS_invalid_setup);
                    }
                } else {
                    (*((*r).residue_books)
                        .offset(j as isize))[k as usize] = -(1 as libc::c_int) as int16;
                }
                k += 1;
            }
            j += 1;
        }
        let ref mut fresh54 = (*r).classdata;
        *fresh54 = setup_malloc(
            f,
            (::std::mem::size_of::<*mut uint8>() as libc::c_ulong)
                .wrapping_mul(
                    (*((*f).codebooks).offset((*r).classbook as isize)).entries
                        as libc::c_ulong,
                ) as libc::c_int,
        ) as *mut *mut uint8;
        if ((*r).classdata).is_null() {
            return error(f, VORBIS_outofmem);
        }
        memset(
            (*r).classdata as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<*mut uint8>() as libc::c_ulong)
                .wrapping_mul(
                    (*((*f).codebooks).offset((*r).classbook as isize)).entries
                        as libc::c_ulong,
                ),
        );
        j = 0 as libc::c_int;
        while j < (*((*f).codebooks).offset((*r).classbook as isize)).entries {
            let mut classwords: libc::c_int = (*((*f).codebooks)
                .offset((*r).classbook as isize))
                .dimensions;
            let mut temp: libc::c_int = j;
            let ref mut fresh55 = *((*r).classdata).offset(j as isize);
            *fresh55 = setup_malloc(
                f,
                (::std::mem::size_of::<uint8>() as libc::c_ulong)
                    .wrapping_mul(classwords as libc::c_ulong) as libc::c_int,
            ) as *mut uint8;
            if (*((*r).classdata).offset(j as isize)).is_null() {
                return error(f, VORBIS_outofmem);
            }
            k = classwords - 1 as libc::c_int;
            while k >= 0 as libc::c_int {
                *(*((*r).classdata).offset(j as isize))
                    .offset(
                        k as isize,
                    ) = (temp % (*r).classifications as libc::c_int) as uint8;
                temp /= (*r).classifications as libc::c_int;
                k -= 1;
            }
            j += 1;
        }
        i += 1;
    }
    (*f)
        .mapping_count = (get_bits(f, 6 as libc::c_int))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let ref mut fresh56 = (*f).mapping;
    *fresh56 = setup_malloc(
        f,
        ((*f).mapping_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Mapping>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut Mapping;
    if ((*f).mapping).is_null() {
        return error(f, VORBIS_outofmem);
    }
    memset(
        (*f).mapping as *mut libc::c_void,
        0 as libc::c_int,
        ((*f).mapping_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Mapping>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < (*f).mapping_count {
        let mut m: *mut Mapping = ((*f).mapping).offset(i as isize);
        let mut mapping_type: libc::c_int = get_bits(f, 16 as libc::c_int)
            as libc::c_int;
        if mapping_type != 0 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        let ref mut fresh57 = (*m).chan;
        *fresh57 = setup_malloc(
            f,
            ((*f).channels as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<MappingChannel>() as libc::c_ulong)
                as libc::c_int,
        ) as *mut MappingChannel;
        if ((*m).chan).is_null() {
            return error(f, VORBIS_outofmem);
        }
        if get_bits(f, 1 as libc::c_int) != 0 {
            (*m)
                .submaps = (get_bits(f, 4 as libc::c_int))
                .wrapping_add(1 as libc::c_int as libc::c_uint) as uint8;
        } else {
            (*m).submaps = 1 as libc::c_int as uint8;
        }
        if (*m).submaps as libc::c_int > max_submaps {
            max_submaps = (*m).submaps as libc::c_int;
        }
        if get_bits(f, 1 as libc::c_int) != 0 {
            (*m)
                .coupling_steps = (get_bits(f, 8 as libc::c_int))
                .wrapping_add(1 as libc::c_int as libc::c_uint) as uint16;
            if (*m).coupling_steps as libc::c_int > (*f).channels {
                return error(f, VORBIS_invalid_setup);
            }
            k = 0 as libc::c_int;
            while k < (*m).coupling_steps as libc::c_int {
                (*((*m).chan).offset(k as isize))
                    .magnitude = get_bits(f, ilog((*f).channels - 1 as libc::c_int))
                    as uint8;
                (*((*m).chan).offset(k as isize))
                    .angle = get_bits(f, ilog((*f).channels - 1 as libc::c_int))
                    as uint8;
                if (*((*m).chan).offset(k as isize)).magnitude as libc::c_int
                    >= (*f).channels
                {
                    return error(f, VORBIS_invalid_setup);
                }
                if (*((*m).chan).offset(k as isize)).angle as libc::c_int
                    >= (*f).channels
                {
                    return error(f, VORBIS_invalid_setup);
                }
                if (*((*m).chan).offset(k as isize)).magnitude as libc::c_int
                    == (*((*m).chan).offset(k as isize)).angle as libc::c_int
                {
                    return error(f, VORBIS_invalid_setup);
                }
                k += 1;
            }
        } else {
            (*m).coupling_steps = 0 as libc::c_int as uint16;
        }
        if get_bits(f, 2 as libc::c_int) != 0 {
            return error(f, VORBIS_invalid_setup);
        }
        if (*m).submaps as libc::c_int > 1 as libc::c_int {
            j = 0 as libc::c_int;
            while j < (*f).channels {
                (*((*m).chan).offset(j as isize))
                    .mux = get_bits(f, 4 as libc::c_int) as uint8;
                if (*((*m).chan).offset(j as isize)).mux as libc::c_int
                    >= (*m).submaps as libc::c_int
                {
                    return error(f, VORBIS_invalid_setup);
                }
                j += 1;
            }
        } else {
            j = 0 as libc::c_int;
            while j < (*f).channels {
                (*((*m).chan).offset(j as isize)).mux = 0 as libc::c_int as uint8;
                j += 1;
            }
        }
        j = 0 as libc::c_int;
        while j < (*m).submaps as libc::c_int {
            get_bits(f, 8 as libc::c_int);
            (*m).submap_floor[j as usize] = get_bits(f, 8 as libc::c_int) as uint8;
            (*m).submap_residue[j as usize] = get_bits(f, 8 as libc::c_int) as uint8;
            if (*m).submap_floor[j as usize] as libc::c_int >= (*f).floor_count {
                return error(f, VORBIS_invalid_setup);
            }
            if (*m).submap_residue[j as usize] as libc::c_int >= (*f).residue_count {
                return error(f, VORBIS_invalid_setup);
            }
            j += 1;
        }
        i += 1;
    }
    (*f)
        .mode_count = (get_bits(f, 6 as libc::c_int))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*f).mode_count {
        let mut m_0: *mut Mode = ((*f).mode_config).as_mut_ptr().offset(i as isize);
        (*m_0).blockflag = get_bits(f, 1 as libc::c_int) as uint8;
        (*m_0).windowtype = get_bits(f, 16 as libc::c_int) as uint16;
        (*m_0).transformtype = get_bits(f, 16 as libc::c_int) as uint16;
        (*m_0).mapping = get_bits(f, 8 as libc::c_int) as uint8;
        if (*m_0).windowtype as libc::c_int != 0 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        if (*m_0).transformtype as libc::c_int != 0 as libc::c_int {
            return error(f, VORBIS_invalid_setup);
        }
        if (*m_0).mapping as libc::c_int >= (*f).mapping_count {
            return error(f, VORBIS_invalid_setup);
        }
        i += 1;
    }
    flush_packet(f);
    (*f).previous_length = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*f).channels {
        let ref mut fresh58 = (*f).channel_buffers[i as usize];
        *fresh58 = setup_malloc(
            f,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul((*f).blocksize_1 as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_float;
        let ref mut fresh59 = (*f).previous_window[i as usize];
        *fresh59 = setup_malloc(
            f,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul((*f).blocksize_1 as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_float;
        let ref mut fresh60 = (*f).finalY[i as usize];
        *fresh60 = setup_malloc(
            f,
            (::std::mem::size_of::<int16>() as libc::c_ulong)
                .wrapping_mul(longest_floorlist as libc::c_ulong) as libc::c_int,
        ) as *mut int16;
        if ((*f).channel_buffers[i as usize]).is_null()
            || ((*f).previous_window[i as usize]).is_null()
            || ((*f).finalY[i as usize]).is_null()
        {
            return error(f, VORBIS_outofmem);
        }
        memset(
            (*f).channel_buffers[i as usize] as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul((*f).blocksize_1 as libc::c_ulong),
        );
        i += 1;
    }
    if init_blocksize(f, 0 as libc::c_int, (*f).blocksize_0) == 0 {
        return 0 as libc::c_int;
    }
    if init_blocksize(f, 1 as libc::c_int, (*f).blocksize_1) == 0 {
        return 0 as libc::c_int;
    }
    (*f).blocksize[0 as libc::c_int as usize] = (*f).blocksize_0;
    (*f).blocksize[1 as libc::c_int as usize] = (*f).blocksize_1;
    let mut imdct_mem: uint32 = (((*f).blocksize_1 as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
        >> 1 as libc::c_int) as uint32;
    let mut classify_mem: uint32 = 0;
    let mut i_0: libc::c_int = 0;
    let mut max_part_read: libc::c_int = 0 as libc::c_int;
    i_0 = 0 as libc::c_int;
    while i_0 < (*f).residue_count {
        let mut r_0: *mut Residue = ((*f).residue_config).offset(i_0 as isize);
        let mut actual_size: libc::c_uint = ((*f).blocksize_1 / 2 as libc::c_int)
            as libc::c_uint;
        let mut limit_r_begin: libc::c_uint = if (*r_0).begin < actual_size {
            (*r_0).begin
        } else {
            actual_size
        };
        let mut limit_r_end: libc::c_uint = if (*r_0).end < actual_size {
            (*r_0).end
        } else {
            actual_size
        };
        let mut n_read: libc::c_int = limit_r_end.wrapping_sub(limit_r_begin)
            as libc::c_int;
        let mut part_read: libc::c_int = (n_read as libc::c_uint)
            .wrapping_div((*r_0).part_size) as libc::c_int;
        if part_read > max_part_read {
            max_part_read = part_read;
        }
        i_0 += 1;
    }
    classify_mem = ((*f).channels as libc::c_ulong)
        .wrapping_mul(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_add(
                    (max_part_read as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut uint8>() as libc::c_ulong,
                        ),
                ),
        ) as uint32;
    (*f).temp_memory_required = classify_mem;
    if imdct_mem > (*f).temp_memory_required {
        (*f).temp_memory_required = imdct_mem;
    }
    if !((*f).alloc.alloc_buffer).is_null() {
        if ((*f).setup_offset as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<vorb>() as libc::c_ulong)
            .wrapping_add((*f).temp_memory_required as libc::c_ulong)
            > (*f).temp_offset as libc::c_uint as libc::c_ulong
        {
            return error(f, VORBIS_outofmem);
        }
    }
    if (*f).next_seg == -(1 as libc::c_int) {
        (*f).first_audio_page_offset = stb_vorbis_get_file_offset(f);
    } else {
        (*f).first_audio_page_offset = 0 as libc::c_int as uint32;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vorbis_deinit(mut p: *mut stb_vorbis) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    setup_free(p, (*p).vendor as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < (*p).comment_list_length {
        setup_free(p, *((*p).comment_list).offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    setup_free(p, (*p).comment_list as *mut libc::c_void);
    if !((*p).residue_config).is_null() {
        i = 0 as libc::c_int;
        while i < (*p).residue_count {
            let mut r: *mut Residue = ((*p).residue_config).offset(i as isize);
            if !((*r).classdata).is_null() {
                j = 0 as libc::c_int;
                while j < (*((*p).codebooks).offset((*r).classbook as isize)).entries {
                    setup_free(
                        p,
                        *((*r).classdata).offset(j as isize) as *mut libc::c_void,
                    );
                    j += 1;
                }
                setup_free(p, (*r).classdata as *mut libc::c_void);
            }
            setup_free(p, (*r).residue_books as *mut libc::c_void);
            i += 1;
        }
    }
    if !((*p).codebooks).is_null() {
        i = 0 as libc::c_int;
        while i < (*p).codebook_count {
            let mut c: *mut Codebook = ((*p).codebooks).offset(i as isize);
            setup_free(p, (*c).codeword_lengths as *mut libc::c_void);
            setup_free(p, (*c).multiplicands as *mut libc::c_void);
            setup_free(p, (*c).codewords as *mut libc::c_void);
            setup_free(p, (*c).sorted_codewords as *mut libc::c_void);
            setup_free(
                p,
                (if !((*c).sorted_values).is_null() {
                    ((*c).sorted_values).offset(-(1 as libc::c_int as isize))
                } else {
                    0 as *mut libc::c_int
                }) as *mut libc::c_void,
            );
            i += 1;
        }
        setup_free(p, (*p).codebooks as *mut libc::c_void);
    }
    setup_free(p, (*p).floor_config as *mut libc::c_void);
    setup_free(p, (*p).residue_config as *mut libc::c_void);
    if !((*p).mapping).is_null() {
        i = 0 as libc::c_int;
        while i < (*p).mapping_count {
            setup_free(
                p,
                (*((*p).mapping).offset(i as isize)).chan as *mut libc::c_void,
            );
            i += 1;
        }
        setup_free(p, (*p).mapping as *mut libc::c_void);
    }
    i = 0 as libc::c_int;
    while i < (*p).channels && i < 16 as libc::c_int {
        setup_free(p, (*p).channel_buffers[i as usize] as *mut libc::c_void);
        setup_free(p, (*p).previous_window[i as usize] as *mut libc::c_void);
        setup_free(p, (*p).finalY[i as usize] as *mut libc::c_void);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        setup_free(p, (*p).A[i as usize] as *mut libc::c_void);
        setup_free(p, (*p).B[i as usize] as *mut libc::c_void);
        setup_free(p, (*p).C[i as usize] as *mut libc::c_void);
        setup_free(p, (*p).window[i as usize] as *mut libc::c_void);
        setup_free(p, (*p).bit_reverse[i as usize] as *mut libc::c_void);
        i += 1;
    }
    if (*p).close_on_free != 0 {
        fclose((*p).f);
    }
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_close(mut p: *mut stb_vorbis) {
    if p.is_null() {
        return;
    }
    vorbis_deinit(p);
    setup_free(p, p as *mut libc::c_void);
}
unsafe extern "C" fn vorbis_init(
    mut p: *mut stb_vorbis,
    mut z: *const stb_vorbis_alloc,
) {
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<stb_vorbis>() as libc::c_ulong,
    );
    if !z.is_null() {
        (*p).alloc = *z;
        (*p).alloc.alloc_buffer_length_in_bytes &= !(7 as libc::c_int);
        (*p).temp_offset = (*p).alloc.alloc_buffer_length_in_bytes;
    }
    (*p).eof = 0 as libc::c_int;
    (*p).error = VORBIS__no_error;
    let ref mut fresh61 = (*p).stream;
    *fresh61 = 0 as *mut uint8;
    let ref mut fresh62 = (*p).codebooks;
    *fresh62 = 0 as *mut Codebook;
    (*p).page_crc_tests = -(1 as libc::c_int);
    (*p).close_on_free = 0 as libc::c_int;
    let ref mut fresh63 = (*p).f;
    *fresh63 = 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_sample_offset(
    mut f: *mut stb_vorbis,
) -> libc::c_int {
    if (*f).current_loc_valid != 0 {
        return (*f).current_loc as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_info(mut f: *mut stb_vorbis) -> stb_vorbis_info {
    let mut d: stb_vorbis_info = stb_vorbis_info {
        sample_rate: 0,
        channels: 0,
        setup_memory_required: 0,
        setup_temp_memory_required: 0,
        temp_memory_required: 0,
        max_frame_size: 0,
    };
    d.channels = (*f).channels;
    d.sample_rate = (*f).sample_rate;
    d.setup_memory_required = (*f).setup_memory_required;
    d.setup_temp_memory_required = (*f).setup_temp_memory_required;
    d.temp_memory_required = (*f).temp_memory_required;
    d.max_frame_size = (*f).blocksize_1 >> 1 as libc::c_int;
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_comment(
    mut f: *mut stb_vorbis,
) -> stb_vorbis_comment {
    let mut d: stb_vorbis_comment = stb_vorbis_comment {
        vendor: 0 as *mut libc::c_char,
        comment_list_length: 0,
        comment_list: 0 as *mut *mut libc::c_char,
    };
    d.vendor = (*f).vendor;
    d.comment_list_length = (*f).comment_list_length;
    d.comment_list = (*f).comment_list;
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_error(mut f: *mut stb_vorbis) -> libc::c_int {
    let mut e: libc::c_int = (*f).error as libc::c_int;
    (*f).error = VORBIS__no_error;
    return e;
}
unsafe extern "C" fn vorbis_alloc(mut f: *mut stb_vorbis) -> *mut stb_vorbis {
    let mut p: *mut stb_vorbis = setup_malloc(
        f,
        ::std::mem::size_of::<stb_vorbis>() as libc::c_ulong as libc::c_int,
    ) as *mut stb_vorbis;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_flush_pushdata(mut f: *mut stb_vorbis) {
    (*f).previous_length = 0 as libc::c_int;
    (*f).page_crc_tests = 0 as libc::c_int;
    (*f).discard_samples_deferred = 0 as libc::c_int;
    (*f).current_loc_valid = 0 as libc::c_int;
    (*f).first_decode = 0 as libc::c_int as uint8;
    (*f).samples_output = 0 as libc::c_int as uint32;
    (*f).channel_buffer_start = 0 as libc::c_int;
    (*f).channel_buffer_end = 0 as libc::c_int;
}
unsafe extern "C" fn vorbis_search_for_page_pushdata(
    mut f: *mut vorb,
    mut data: *mut uint8,
    mut data_len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*f).page_crc_tests {
        (*f).scan[i as usize].bytes_done = 0 as libc::c_int;
        i += 1;
    }
    if (*f).page_crc_tests < 4 as libc::c_int {
        if data_len < 4 as libc::c_int {
            return 0 as libc::c_int;
        }
        data_len -= 3 as libc::c_int;
        i = 0 as libc::c_int;
        while i < data_len {
            if *data.offset(i as isize) as libc::c_int == 0x4f as libc::c_int {
                if 0 as libc::c_int
                    == memcmp(
                        data.offset(i as isize) as *const libc::c_void,
                        ogg_page_header.as_mut_ptr() as *const libc::c_void,
                        4 as libc::c_int as libc::c_ulong,
                    )
                {
                    let mut j: libc::c_int = 0;
                    let mut len: libc::c_int = 0;
                    let mut crc: uint32 = 0;
                    if i + 26 as libc::c_int >= data_len
                        || i + 27 as libc::c_int
                            + *data.offset((i + 26 as libc::c_int) as isize)
                                as libc::c_int >= data_len
                    {
                        data_len = i;
                        break;
                    } else {
                        len = 27 as libc::c_int
                            + *data.offset((i + 26 as libc::c_int) as isize)
                                as libc::c_int;
                        j = 0 as libc::c_int;
                        while j
                            < *data.offset((i + 26 as libc::c_int) as isize)
                                as libc::c_int
                        {
                            len
                                += *data.offset((i + 27 as libc::c_int + j) as isize)
                                    as libc::c_int;
                            j += 1;
                        }
                        crc = 0 as libc::c_int as uint32;
                        j = 0 as libc::c_int;
                        while j < 22 as libc::c_int {
                            crc = crc32_update(crc, *data.offset((i + j) as isize));
                            j += 1;
                        }
                        while j < 26 as libc::c_int {
                            crc = crc32_update(crc, 0 as libc::c_int as uint8);
                            j += 1;
                        }
                        let ref mut fresh64 = (*f).page_crc_tests;
                        let fresh65 = *fresh64;
                        *fresh64 = *fresh64 + 1;
                        n = fresh65;
                        (*f).scan[n as usize].bytes_left = len - j;
                        (*f).scan[n as usize].crc_so_far = crc;
                        (*f)
                            .scan[n as usize]
                            .goal_crc = (*data.offset((i + 22 as libc::c_int) as isize)
                            as libc::c_int
                            + ((*data.offset((i + 23 as libc::c_int) as isize)
                                as libc::c_int) << 8 as libc::c_int)
                            + ((*data.offset((i + 24 as libc::c_int) as isize)
                                as libc::c_int) << 16 as libc::c_int)
                            + ((*data.offset((i + 25 as libc::c_int) as isize)
                                as libc::c_int) << 24 as libc::c_int)) as uint32;
                        if *data
                            .offset(
                                (i + 27 as libc::c_int
                                    + *data.offset((i + 26 as libc::c_int) as isize)
                                        as libc::c_int - 1 as libc::c_int) as isize,
                            ) as libc::c_int == 255 as libc::c_int
                        {
                            (*f)
                                .scan[n as usize]
                                .sample_loc = !(0 as libc::c_int) as uint32;
                        } else {
                            (*f)
                                .scan[n as usize]
                                .sample_loc = (*data.offset((i + 6 as libc::c_int) as isize)
                                as libc::c_int
                                + ((*data.offset((i + 7 as libc::c_int) as isize)
                                    as libc::c_int) << 8 as libc::c_int)
                                + ((*data.offset((i + 8 as libc::c_int) as isize)
                                    as libc::c_int) << 16 as libc::c_int)
                                + ((*data.offset((i + 9 as libc::c_int) as isize)
                                    as libc::c_int) << 24 as libc::c_int)) as uint32;
                        }
                        (*f).scan[n as usize].bytes_done = i + j;
                        if (*f).page_crc_tests == 4 as libc::c_int {
                            break;
                        }
                    }
                }
            }
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < (*f).page_crc_tests {
        let mut crc_0: uint32 = 0;
        let mut j_0: libc::c_int = 0;
        let mut n_0: libc::c_int = (*f).scan[i as usize].bytes_done;
        let mut m: libc::c_int = (*f).scan[i as usize].bytes_left;
        if m > data_len - n_0 {
            m = data_len - n_0;
        }
        crc_0 = (*f).scan[i as usize].crc_so_far;
        j_0 = 0 as libc::c_int;
        while j_0 < m {
            crc_0 = crc32_update(crc_0, *data.offset((n_0 + j_0) as isize));
            j_0 += 1;
        }
        (*f).scan[i as usize].bytes_left -= m;
        (*f).scan[i as usize].crc_so_far = crc_0;
        if (*f).scan[i as usize].bytes_left == 0 as libc::c_int {
            if (*f).scan[i as usize].crc_so_far == (*f).scan[i as usize].goal_crc {
                data_len = n_0 + m;
                (*f).page_crc_tests = -(1 as libc::c_int);
                (*f).previous_length = 0 as libc::c_int;
                (*f).next_seg = -(1 as libc::c_int);
                (*f).current_loc = (*f).scan[i as usize].sample_loc;
                (*f)
                    .current_loc_valid = ((*f).current_loc != !(0 as libc::c_uint))
                    as libc::c_int;
                return data_len;
            }
            let ref mut fresh66 = (*f).page_crc_tests;
            *fresh66 -= 1;
            (*f).scan[i as usize] = (*f).scan[*fresh66 as usize];
        } else {
            i += 1;
        }
    }
    return data_len;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_decode_frame_pushdata(
    mut f: *mut stb_vorbis,
    mut data: *const uint8,
    mut data_len: libc::c_int,
    mut channels: *mut libc::c_int,
    mut output: *mut *mut *mut libc::c_float,
    mut samples: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    if (*f).push_mode == 0 {
        return error(f, VORBIS_invalid_api_mixing);
    }
    if (*f).page_crc_tests >= 0 as libc::c_int {
        *samples = 0 as libc::c_int;
        return vorbis_search_for_page_pushdata(f, data as *mut uint8, data_len);
    }
    let ref mut fresh67 = (*f).stream;
    *fresh67 = data as *mut uint8;
    let ref mut fresh68 = (*f).stream_end;
    *fresh68 = (data as *mut uint8).offset(data_len as isize);
    (*f).error = VORBIS__no_error;
    if is_whole_packet_present(f) == 0 {
        *samples = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    if vorbis_decode_packet(f, &mut len, &mut left, &mut right) == 0 {
        let mut error_0: STBVorbisError = (*f).error;
        if error_0 as libc::c_uint
            == VORBIS_bad_packet_type as libc::c_int as libc::c_uint
        {
            (*f).error = VORBIS__no_error;
            while get8_packet(f) != -(1 as libc::c_int) {
                if (*f).eof != 0 {
                    break;
                }
            }
            *samples = 0 as libc::c_int;
            return ((*f).stream).offset_from(data) as libc::c_long as libc::c_int;
        }
        if error_0 as libc::c_uint
            == VORBIS_continued_packet_flag_invalid as libc::c_int as libc::c_uint
        {
            if (*f).previous_length == 0 as libc::c_int {
                (*f).error = VORBIS__no_error;
                while get8_packet(f) != -(1 as libc::c_int) {
                    if (*f).eof != 0 {
                        break;
                    }
                }
                *samples = 0 as libc::c_int;
                return ((*f).stream).offset_from(data) as libc::c_long as libc::c_int;
            }
        }
        stb_vorbis_flush_pushdata(f);
        (*f).error = error_0;
        *samples = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    len = vorbis_finish_frame(f, len, left, right);
    i = 0 as libc::c_int;
    while i < (*f).channels {
        let ref mut fresh69 = (*f).outputs[i as usize];
        *fresh69 = ((*f).channel_buffers[i as usize]).offset(left as isize);
        i += 1;
    }
    if !channels.is_null() {
        *channels = (*f).channels;
    }
    *samples = len;
    *output = ((*f).outputs).as_mut_ptr();
    return ((*f).stream).offset_from(data) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_open_pushdata(
    mut data: *const libc::c_uchar,
    mut data_len: libc::c_int,
    mut data_used: *mut libc::c_int,
    mut error_0: *mut libc::c_int,
    mut alloc: *const stb_vorbis_alloc,
) -> *mut stb_vorbis {
    let mut f: *mut stb_vorbis = 0 as *mut stb_vorbis;
    let mut p: stb_vorbis = stb_vorbis {
        sample_rate: 0,
        channels: 0,
        setup_memory_required: 0,
        temp_memory_required: 0,
        setup_temp_memory_required: 0,
        vendor: 0 as *mut libc::c_char,
        comment_list_length: 0,
        comment_list: 0 as *mut *mut libc::c_char,
        f: 0 as *mut FILE,
        f_start: 0,
        close_on_free: 0,
        stream: 0 as *mut uint8,
        stream_start: 0 as *mut uint8,
        stream_end: 0 as *mut uint8,
        stream_len: 0,
        push_mode: 0,
        first_audio_page_offset: 0,
        p_first: ProbedPage {
            page_start: 0,
            page_end: 0,
            last_decoded_sample: 0,
        },
        p_last: ProbedPage {
            page_start: 0,
            page_end: 0,
            last_decoded_sample: 0,
        },
        alloc: stb_vorbis_alloc {
            alloc_buffer: 0 as *mut libc::c_char,
            alloc_buffer_length_in_bytes: 0,
        },
        setup_offset: 0,
        temp_offset: 0,
        eof: 0,
        error: VORBIS__no_error,
        blocksize: [0; 2],
        blocksize_0: 0,
        blocksize_1: 0,
        codebook_count: 0,
        codebooks: 0 as *mut Codebook,
        floor_count: 0,
        floor_types: [0; 64],
        floor_config: 0 as *mut Floor,
        residue_count: 0,
        residue_types: [0; 64],
        residue_config: 0 as *mut Residue,
        mapping_count: 0,
        mapping: 0 as *mut Mapping,
        mode_count: 0,
        mode_config: [Mode {
            blockflag: 0,
            mapping: 0,
            windowtype: 0,
            transformtype: 0,
        }; 64],
        total_samples: 0,
        channel_buffers: [0 as *mut libc::c_float; 16],
        outputs: [0 as *mut libc::c_float; 16],
        previous_window: [0 as *mut libc::c_float; 16],
        previous_length: 0,
        finalY: [0 as *mut int16; 16],
        current_loc: 0,
        current_loc_valid: 0,
        A: [0 as *mut libc::c_float; 2],
        B: [0 as *mut libc::c_float; 2],
        C: [0 as *mut libc::c_float; 2],
        window: [0 as *mut libc::c_float; 2],
        bit_reverse: [0 as *mut uint16; 2],
        serial: 0,
        last_page: 0,
        segment_count: 0,
        segments: [0; 255],
        page_flag: 0,
        bytes_in_seg: 0,
        first_decode: 0,
        next_seg: 0,
        last_seg: 0,
        last_seg_which: 0,
        acc: 0,
        valid_bits: 0,
        packet_bytes: 0,
        end_seg_with_known_loc: 0,
        known_loc_for_packet: 0,
        discard_samples_deferred: 0,
        samples_output: 0,
        page_crc_tests: 0,
        scan: [CRCscan {
            goal_crc: 0,
            bytes_left: 0,
            crc_so_far: 0,
            bytes_done: 0,
            sample_loc: 0,
        }; 4],
        channel_buffer_start: 0,
        channel_buffer_end: 0,
    };
    vorbis_init(&mut p, alloc);
    p.stream = data as *mut uint8;
    p.stream_end = (data as *mut uint8).offset(data_len as isize);
    p.push_mode = 1 as libc::c_int as uint8;
    if start_decoder(&mut p) == 0 {
        if p.eof != 0 {
            *error_0 = VORBIS_need_more_data as libc::c_int;
        } else {
            *error_0 = p.error as libc::c_int;
        }
        vorbis_deinit(&mut p);
        return 0 as *mut stb_vorbis;
    }
    f = vorbis_alloc(&mut p);
    if !f.is_null() {
        *f = p;
        *data_used = ((*f).stream).offset_from(data) as libc::c_long as libc::c_int;
        *error_0 = 0 as libc::c_int;
        return f;
    } else {
        vorbis_deinit(&mut p);
        return 0 as *mut stb_vorbis;
    };
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_file_offset(
    mut f: *mut stb_vorbis,
) -> libc::c_uint {
    if (*f).push_mode != 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    if !((*f).stream).is_null() {
        return ((*f).stream).offset_from((*f).stream_start) as libc::c_long
            as libc::c_uint;
    }
    return (ftell((*f).f) - (*f).f_start as libc::c_long) as libc::c_uint;
}
unsafe extern "C" fn vorbis_find_page(
    mut f: *mut stb_vorbis,
    mut end: *mut uint32,
    mut last: *mut uint32,
) -> uint32 {
    loop {
        let mut n: libc::c_int = 0;
        if (*f).eof != 0 {
            return 0 as libc::c_int as uint32;
        }
        n = get8(f) as libc::c_int;
        if n == 0x4f as libc::c_int {
            let mut retry_loc: libc::c_uint = stb_vorbis_get_file_offset(f);
            let mut i: libc::c_int = 0;
            if retry_loc.wrapping_sub(25 as libc::c_int as libc::c_uint)
                > (*f).stream_len
            {
                return 0 as libc::c_int as uint32;
            }
            i = 1 as libc::c_int;
            while i < 4 as libc::c_int {
                if get8(f) as libc::c_int != ogg_page_header[i as usize] as libc::c_int {
                    break;
                }
                i += 1;
            }
            if (*f).eof != 0 {
                return 0 as libc::c_int as uint32;
            }
            if i == 4 as libc::c_int {
                let mut header: [uint8; 27] = [0; 27];
                let mut i_0: uint32 = 0;
                let mut crc: uint32 = 0;
                let mut goal: uint32 = 0;
                let mut len: uint32 = 0;
                i_0 = 0 as libc::c_int as uint32;
                while i_0 < 4 as libc::c_int as libc::c_uint {
                    header[i_0 as usize] = ogg_page_header[i_0 as usize];
                    i_0 = i_0.wrapping_add(1);
                }
                while i_0 < 27 as libc::c_int as libc::c_uint {
                    header[i_0 as usize] = get8(f);
                    i_0 = i_0.wrapping_add(1);
                }
                if (*f).eof != 0 {
                    return 0 as libc::c_int as uint32;
                }
                if !(header[4 as libc::c_int as usize] as libc::c_int
                    != 0 as libc::c_int)
                {
                    goal = ((header[22 as libc::c_int as usize] as libc::c_int
                        + ((header[23 as libc::c_int as usize] as libc::c_int)
                            << 8 as libc::c_int)
                        + ((header[24 as libc::c_int as usize] as libc::c_int)
                            << 16 as libc::c_int)) as libc::c_uint)
                        .wrapping_add(
                            (header[25 as libc::c_int as usize] as uint32)
                                << 24 as libc::c_int,
                        );
                    i_0 = 22 as libc::c_int as uint32;
                    while i_0 < 26 as libc::c_int as libc::c_uint {
                        header[i_0 as usize] = 0 as libc::c_int as uint8;
                        i_0 = i_0.wrapping_add(1);
                    }
                    crc = 0 as libc::c_int as uint32;
                    i_0 = 0 as libc::c_int as uint32;
                    while i_0 < 27 as libc::c_int as libc::c_uint {
                        crc = crc32_update(crc, header[i_0 as usize]);
                        i_0 = i_0.wrapping_add(1);
                    }
                    len = 0 as libc::c_int as uint32;
                    i_0 = 0 as libc::c_int as uint32;
                    while i_0 < header[26 as libc::c_int as usize] as libc::c_uint {
                        let mut s: libc::c_int = get8(f) as libc::c_int;
                        crc = crc32_update(crc, s as uint8);
                        len = (len as libc::c_uint).wrapping_add(s as libc::c_uint)
                            as uint32 as uint32;
                        i_0 = i_0.wrapping_add(1);
                    }
                    if len != 0 && (*f).eof != 0 {
                        return 0 as libc::c_int as uint32;
                    }
                    i_0 = 0 as libc::c_int as uint32;
                    while i_0 < len {
                        crc = crc32_update(crc, get8(f));
                        i_0 = i_0.wrapping_add(1);
                    }
                    if crc == goal {
                        if !end.is_null() {
                            *end = stb_vorbis_get_file_offset(f);
                        }
                        if !last.is_null() {
                            if header[5 as libc::c_int as usize] as libc::c_int
                                & 0x4 as libc::c_int != 0
                            {
                                *last = 1 as libc::c_int as uint32;
                            } else {
                                *last = 0 as libc::c_int as uint32;
                            }
                        }
                        set_file_offset(
                            f,
                            retry_loc.wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                        return 1 as libc::c_int as uint32;
                    }
                }
            }
            set_file_offset(f, retry_loc);
        }
    };
}
unsafe extern "C" fn get_seek_page_info(
    mut f: *mut stb_vorbis,
    mut z: *mut ProbedPage,
) -> libc::c_int {
    let mut header: [uint8; 27] = [0; 27];
    let mut lacing: [uint8; 255] = [0; 255];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    (*z).page_start = stb_vorbis_get_file_offset(f);
    getn(f, header.as_mut_ptr(), 27 as libc::c_int);
    if header[0 as libc::c_int as usize] as libc::c_int != 'O' as i32
        || header[1 as libc::c_int as usize] as libc::c_int != 'g' as i32
        || header[2 as libc::c_int as usize] as libc::c_int != 'g' as i32
        || header[3 as libc::c_int as usize] as libc::c_int != 'S' as i32
    {
        return 0 as libc::c_int;
    }
    getn(f, lacing.as_mut_ptr(), header[26 as libc::c_int as usize] as libc::c_int);
    len = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < header[26 as libc::c_int as usize] as libc::c_int {
        len += lacing[i as usize] as libc::c_int;
        i += 1;
    }
    (*z)
        .page_end = ((*z).page_start)
        .wrapping_add(27 as libc::c_int as libc::c_uint)
        .wrapping_add(header[26 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(len as libc::c_uint);
    (*z)
        .last_decoded_sample = (header[6 as libc::c_int as usize] as libc::c_int
        + ((header[7 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int)
        + ((header[8 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int)
        + ((header[9 as libc::c_int as usize] as libc::c_int) << 24 as libc::c_int))
        as uint32;
    set_file_offset(f, (*z).page_start);
    return 1 as libc::c_int;
}
unsafe extern "C" fn go_to_page_before(
    mut f: *mut stb_vorbis,
    mut limit_offset: libc::c_uint,
) -> libc::c_int {
    let mut previous_safe: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    if limit_offset >= 65536 as libc::c_int as libc::c_uint
        && limit_offset.wrapping_sub(65536 as libc::c_int as libc::c_uint)
            >= (*f).first_audio_page_offset
    {
        previous_safe = limit_offset.wrapping_sub(65536 as libc::c_int as libc::c_uint);
    } else {
        previous_safe = (*f).first_audio_page_offset;
    }
    set_file_offset(f, previous_safe);
    while vorbis_find_page(f, &mut end, 0 as *mut uint32) != 0 {
        if end >= limit_offset && stb_vorbis_get_file_offset(f) < limit_offset {
            return 1 as libc::c_int;
        }
        set_file_offset(f, end);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn seek_to_sample_coarse(
    mut f: *mut stb_vorbis,
    mut sample_number: uint32,
) -> libc::c_int {
    let mut current_block: u64;
    let mut left: ProbedPage = ProbedPage {
        page_start: 0,
        page_end: 0,
        last_decoded_sample: 0,
    };
    let mut right: ProbedPage = ProbedPage {
        page_start: 0,
        page_end: 0,
        last_decoded_sample: 0,
    };
    let mut mid: ProbedPage = ProbedPage {
        page_start: 0,
        page_end: 0,
        last_decoded_sample: 0,
    };
    let mut i: libc::c_int = 0;
    let mut start_seg_with_known_loc: libc::c_int = 0;
    let mut end_pos: libc::c_int = 0;
    let mut page_start: libc::c_int = 0;
    let mut delta: uint32 = 0;
    let mut stream_length: uint32 = 0;
    let mut padding: uint32 = 0;
    let mut last_sample_limit: uint32 = 0;
    let mut offset: libc::c_double = 0.0f64;
    let mut bytes_per_sample: libc::c_double = 0.0f64;
    let mut probe: libc::c_int = 0 as libc::c_int;
    stream_length = stb_vorbis_stream_length_in_samples(f);
    if stream_length == 0 as libc::c_int as libc::c_uint {
        return error(f, VORBIS_seek_without_length);
    }
    if sample_number > stream_length {
        return error(f, VORBIS_seek_invalid);
    }
    padding = ((*f).blocksize_1 - (*f).blocksize_0 >> 2 as libc::c_int) as uint32;
    if sample_number < padding {
        last_sample_limit = 0 as libc::c_int as uint32;
    } else {
        last_sample_limit = sample_number.wrapping_sub(padding);
    }
    left = (*f).p_first;
    loop {
        if !(left.last_decoded_sample == !(0 as libc::c_uint)) {
            current_block = 8236137900636309791;
            break;
        }
        set_file_offset(f, left.page_end);
        if get_seek_page_info(f, &mut left) == 0 {
            current_block = 2518365774060181242;
            break;
        }
    }
    match current_block {
        8236137900636309791 => {
            right = (*f).p_last;
            if last_sample_limit <= left.last_decoded_sample {
                if stb_vorbis_seek_start(f) != 0 {
                    if (*f).current_loc > sample_number {
                        return error(f, VORBIS_seek_failed);
                    }
                    return 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
            's_117: loop {
                if !(left.page_end != right.page_start) {
                    current_block = 4888910987971495881;
                    break;
                }
                delta = (right.page_start).wrapping_sub(left.page_end);
                if delta <= 65536 as libc::c_int as libc::c_uint {
                    set_file_offset(f, left.page_end);
                } else {
                    if probe < 2 as libc::c_int {
                        if probe == 0 as libc::c_int {
                            let mut data_bytes: libc::c_double = (right.page_end)
                                .wrapping_sub(left.page_start) as libc::c_double;
                            bytes_per_sample = data_bytes
                                / right.last_decoded_sample as libc::c_double;
                            offset = left.page_start as libc::c_double
                                + bytes_per_sample
                                    * last_sample_limit.wrapping_sub(left.last_decoded_sample)
                                        as libc::c_double;
                        } else {
                            let mut error_0: libc::c_double = (last_sample_limit
                                as libc::c_double
                                - mid.last_decoded_sample as libc::c_double)
                                * bytes_per_sample;
                            if error_0 >= 0 as libc::c_int as libc::c_double
                                && error_0 < 8000 as libc::c_int as libc::c_double
                            {
                                error_0 = 8000 as libc::c_int as libc::c_double;
                            }
                            if error_0 < 0 as libc::c_int as libc::c_double
                                && error_0 > -(8000 as libc::c_int) as libc::c_double
                            {
                                error_0 = -(8000 as libc::c_int) as libc::c_double;
                            }
                            offset += error_0 * 2 as libc::c_int as libc::c_double;
                        }
                        if offset < left.page_end as libc::c_double {
                            offset = left.page_end as libc::c_double;
                        }
                        if offset
                            > (right.page_start)
                                .wrapping_sub(65536 as libc::c_int as libc::c_uint)
                                as libc::c_double
                        {
                            offset = (right.page_start)
                                .wrapping_sub(65536 as libc::c_int as libc::c_uint)
                                as libc::c_double;
                        }
                        set_file_offset(f, offset as libc::c_uint);
                    } else {
                        set_file_offset(
                            f,
                            (left.page_end)
                                .wrapping_add(
                                    delta.wrapping_div(2 as libc::c_int as libc::c_uint),
                                )
                                .wrapping_sub(32768 as libc::c_int as libc::c_uint),
                        );
                    }
                    if vorbis_find_page(f, 0 as *mut uint32, 0 as *mut uint32) == 0 {
                        current_block = 2518365774060181242;
                        break;
                    }
                }
                loop {
                    if get_seek_page_info(f, &mut mid) == 0 {
                        current_block = 2518365774060181242;
                        break 's_117;
                    }
                    if mid.last_decoded_sample != !(0 as libc::c_uint) {
                        break;
                    }
                    set_file_offset(f, mid.page_end);
                }
                if mid.page_start == right.page_start {
                    if probe >= 2 as libc::c_int
                        || delta <= 65536 as libc::c_int as libc::c_uint
                    {
                        current_block = 4888910987971495881;
                        break;
                    }
                } else if last_sample_limit < mid.last_decoded_sample {
                    right = mid;
                } else {
                    left = mid;
                }
                probe += 1;
            }
            match current_block {
                2518365774060181242 => {}
                _ => {
                    page_start = left.page_start as libc::c_int;
                    set_file_offset(f, page_start as libc::c_uint);
                    if start_page(f) == 0 {
                        return error(f, VORBIS_seek_failed);
                    }
                    end_pos = (*f).end_seg_with_known_loc;
                    loop {
                        i = end_pos;
                        while i > 0 as libc::c_int {
                            if (*f).segments[(i - 1 as libc::c_int) as usize]
                                as libc::c_int != 255 as libc::c_int
                            {
                                break;
                            }
                            i -= 1;
                        }
                        start_seg_with_known_loc = i;
                        if start_seg_with_known_loc > 0 as libc::c_int
                            || (*f).page_flag as libc::c_int & 1 as libc::c_int == 0
                        {
                            current_block = 6033931424626438518;
                            break;
                        }
                        if go_to_page_before(f, page_start as libc::c_uint) == 0 {
                            current_block = 2518365774060181242;
                            break;
                        }
                        page_start = stb_vorbis_get_file_offset(f) as libc::c_int;
                        if start_page(f) == 0 {
                            current_block = 2518365774060181242;
                            break;
                        }
                        end_pos = (*f).segment_count - 1 as libc::c_int;
                    }
                    match current_block {
                        2518365774060181242 => {}
                        _ => {
                            (*f).current_loc_valid = 0 as libc::c_int;
                            (*f).last_seg = 0 as libc::c_int;
                            (*f).valid_bits = 0 as libc::c_int;
                            (*f).packet_bytes = 0 as libc::c_int;
                            (*f).bytes_in_seg = 0 as libc::c_int as uint8;
                            (*f).previous_length = 0 as libc::c_int;
                            (*f).next_seg = start_seg_with_known_loc;
                            i = 0 as libc::c_int;
                            while i < start_seg_with_known_loc {
                                skip(f, (*f).segments[i as usize] as libc::c_int);
                                i += 1;
                            }
                            if vorbis_pump_first_frame(f) == 0 {
                                return 0 as libc::c_int;
                            }
                            if (*f).current_loc > sample_number {
                                return error(f, VORBIS_seek_failed);
                            }
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    stb_vorbis_seek_start(f);
    return error(f, VORBIS_seek_failed);
}
unsafe extern "C" fn peek_decode_initial(
    mut f: *mut vorb,
    mut p_left_start: *mut libc::c_int,
    mut p_left_end: *mut libc::c_int,
    mut p_right_start: *mut libc::c_int,
    mut p_right_end: *mut libc::c_int,
    mut mode: *mut libc::c_int,
) -> libc::c_int {
    let mut bits_read: libc::c_int = 0;
    let mut bytes_read: libc::c_int = 0;
    if vorbis_decode_initial(
        f,
        p_left_start,
        p_left_end,
        p_right_start,
        p_right_end,
        mode,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    bits_read = 1 as libc::c_int + ilog((*f).mode_count - 1 as libc::c_int);
    if (*f).mode_config[*mode as usize].blockflag != 0 {
        bits_read += 2 as libc::c_int;
    }
    bytes_read = (bits_read + 7 as libc::c_int) / 8 as libc::c_int;
    let ref mut fresh70 = (*f).bytes_in_seg;
    *fresh70 = (*fresh70 as libc::c_int + bytes_read) as uint8;
    (*f).packet_bytes -= bytes_read;
    skip(f, -bytes_read);
    if (*f).next_seg == -(1 as libc::c_int) {
        (*f).next_seg = (*f).segment_count - 1 as libc::c_int;
    } else {
        let ref mut fresh71 = (*f).next_seg;
        *fresh71 -= 1;
    }
    (*f).valid_bits = 0 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_seek_frame(
    mut f: *mut stb_vorbis,
    mut sample_number: libc::c_uint,
) -> libc::c_int {
    let mut max_frame_samples: uint32 = 0;
    if (*f).push_mode != 0 {
        return error(f, VORBIS_invalid_api_mixing);
    }
    if seek_to_sample_coarse(f, sample_number) == 0 {
        return 0 as libc::c_int;
    }
    max_frame_samples = ((*f).blocksize_1 * 3 as libc::c_int - (*f).blocksize_0
        >> 2 as libc::c_int) as uint32;
    while (*f).current_loc < sample_number {
        let mut left_start: libc::c_int = 0;
        let mut left_end: libc::c_int = 0;
        let mut right_start: libc::c_int = 0;
        let mut right_end: libc::c_int = 0;
        let mut mode: libc::c_int = 0;
        let mut frame_samples: libc::c_int = 0;
        if peek_decode_initial(
            f,
            &mut left_start,
            &mut left_end,
            &mut right_start,
            &mut right_end,
            &mut mode,
        ) == 0
        {
            return error(f, VORBIS_seek_failed);
        }
        frame_samples = right_start - left_start;
        if ((*f).current_loc).wrapping_add(frame_samples as libc::c_uint) > sample_number
        {
            return 1 as libc::c_int
        } else {
            if ((*f).current_loc)
                .wrapping_add(frame_samples as libc::c_uint)
                .wrapping_add(max_frame_samples) > sample_number
            {
                vorbis_pump_first_frame(f);
            } else {
                let ref mut fresh72 = (*f).current_loc;
                *fresh72 = (*fresh72 as libc::c_uint)
                    .wrapping_add(frame_samples as libc::c_uint) as uint32 as uint32;
                (*f).previous_length = 0 as libc::c_int;
                maybe_start_packet(f);
                flush_packet(f);
            }
        }
    }
    if (*f).current_loc != sample_number {
        return error(f, VORBIS_seek_failed);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_seek(
    mut f: *mut stb_vorbis,
    mut sample_number: libc::c_uint,
) -> libc::c_int {
    if stb_vorbis_seek_frame(f, sample_number) == 0 {
        return 0 as libc::c_int;
    }
    if sample_number != (*f).current_loc {
        let mut n: libc::c_int = 0;
        let mut frame_start: uint32 = (*f).current_loc;
        stb_vorbis_get_frame_float(f, &mut n, 0 as *mut *mut *mut libc::c_float);
        let ref mut fresh73 = (*f).channel_buffer_start;
        *fresh73 = (*fresh73 as libc::c_uint)
            .wrapping_add(sample_number.wrapping_sub(frame_start)) as libc::c_int
            as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_seek_start(mut f: *mut stb_vorbis) -> libc::c_int {
    if (*f).push_mode != 0 {
        return error(f, VORBIS_invalid_api_mixing);
    }
    set_file_offset(f, (*f).first_audio_page_offset);
    (*f).previous_length = 0 as libc::c_int;
    (*f).first_decode = 1 as libc::c_int as uint8;
    (*f).next_seg = -(1 as libc::c_int);
    return vorbis_pump_first_frame(f);
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_stream_length_in_samples(
    mut f: *mut stb_vorbis,
) -> libc::c_uint {
    let mut restore_offset: libc::c_uint = 0;
    let mut previous_safe: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    let mut last_page_loc: libc::c_uint = 0;
    if (*f).push_mode != 0 {
        return error(f, VORBIS_invalid_api_mixing) as libc::c_uint;
    }
    if (*f).total_samples == 0 {
        let mut last: libc::c_uint = 0;
        let mut lo: uint32 = 0;
        let mut hi: uint32 = 0;
        let mut header: [libc::c_char; 6] = [0; 6];
        restore_offset = stb_vorbis_get_file_offset(f);
        if (*f).stream_len >= 65536 as libc::c_int as libc::c_uint
            && ((*f).stream_len).wrapping_sub(65536 as libc::c_int as libc::c_uint)
                >= (*f).first_audio_page_offset
        {
            previous_safe = ((*f).stream_len)
                .wrapping_sub(65536 as libc::c_int as libc::c_uint);
        } else {
            previous_safe = (*f).first_audio_page_offset;
        }
        set_file_offset(f, previous_safe);
        if vorbis_find_page(f, &mut end, &mut last) == 0 {
            (*f).error = VORBIS_cant_find_last_page;
            (*f).total_samples = 0xffffffff as libc::c_uint;
        } else {
            last_page_loc = stb_vorbis_get_file_offset(f);
            while last == 0 {
                set_file_offset(f, end);
                if vorbis_find_page(f, &mut end, &mut last) == 0 {
                    break;
                }
                last_page_loc = stb_vorbis_get_file_offset(f);
            }
            set_file_offset(f, last_page_loc);
            getn(f, header.as_mut_ptr() as *mut libc::c_uchar, 6 as libc::c_int);
            lo = get32(f);
            hi = get32(f);
            if lo == 0xffffffff as libc::c_uint && hi == 0xffffffff as libc::c_uint {
                (*f).error = VORBIS_cant_find_last_page;
                (*f).total_samples = 0xffffffff as libc::c_uint;
            } else {
                if hi != 0 {
                    lo = 0xfffffffe as libc::c_uint;
                }
                (*f).total_samples = lo;
                (*f).p_last.page_start = last_page_loc;
                (*f).p_last.page_end = end;
                (*f).p_last.last_decoded_sample = lo;
            }
        }
        set_file_offset(f, restore_offset);
    }
    return if (*f).total_samples == 0xffffffff as libc::c_uint {
        0 as libc::c_int as libc::c_uint
    } else {
        (*f).total_samples
    };
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_stream_length_in_seconds(
    mut f: *mut stb_vorbis,
) -> libc::c_float {
    return stb_vorbis_stream_length_in_samples(f) as libc::c_float
        / (*f).sample_rate as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_frame_float(
    mut f: *mut stb_vorbis,
    mut channels: *mut libc::c_int,
    mut output: *mut *mut *mut libc::c_float,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if (*f).push_mode != 0 {
        return error(f, VORBIS_invalid_api_mixing);
    }
    if vorbis_decode_packet(f, &mut len, &mut left, &mut right) == 0 {
        let ref mut fresh74 = (*f).channel_buffer_end;
        *fresh74 = 0 as libc::c_int;
        (*f).channel_buffer_start = *fresh74;
        return 0 as libc::c_int;
    }
    len = vorbis_finish_frame(f, len, left, right);
    i = 0 as libc::c_int;
    while i < (*f).channels {
        let ref mut fresh75 = (*f).outputs[i as usize];
        *fresh75 = ((*f).channel_buffers[i as usize]).offset(left as isize);
        i += 1;
    }
    (*f).channel_buffer_start = left;
    (*f).channel_buffer_end = left + len;
    if !channels.is_null() {
        *channels = (*f).channels;
    }
    if !output.is_null() {
        *output = ((*f).outputs).as_mut_ptr();
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_open_file_section(
    mut file: *mut FILE,
    mut close_on_free: libc::c_int,
    mut error_0: *mut libc::c_int,
    mut alloc: *const stb_vorbis_alloc,
    mut length: libc::c_uint,
) -> *mut stb_vorbis {
    let mut f: *mut stb_vorbis = 0 as *mut stb_vorbis;
    let mut p: stb_vorbis = stb_vorbis {
        sample_rate: 0,
        channels: 0,
        setup_memory_required: 0,
        temp_memory_required: 0,
        setup_temp_memory_required: 0,
        vendor: 0 as *mut libc::c_char,
        comment_list_length: 0,
        comment_list: 0 as *mut *mut libc::c_char,
        f: 0 as *mut FILE,
        f_start: 0,
        close_on_free: 0,
        stream: 0 as *mut uint8,
        stream_start: 0 as *mut uint8,
        stream_end: 0 as *mut uint8,
        stream_len: 0,
        push_mode: 0,
        first_audio_page_offset: 0,
        p_first: ProbedPage {
            page_start: 0,
            page_end: 0,
            last_decoded_sample: 0,
        },
        p_last: ProbedPage {
            page_start: 0,
            page_end: 0,
            last_decoded_sample: 0,
        },
        alloc: stb_vorbis_alloc {
            alloc_buffer: 0 as *mut libc::c_char,
            alloc_buffer_length_in_bytes: 0,
        },
        setup_offset: 0,
        temp_offset: 0,
        eof: 0,
        error: VORBIS__no_error,
        blocksize: [0; 2],
        blocksize_0: 0,
        blocksize_1: 0,
        codebook_count: 0,
        codebooks: 0 as *mut Codebook,
        floor_count: 0,
        floor_types: [0; 64],
        floor_config: 0 as *mut Floor,
        residue_count: 0,
        residue_types: [0; 64],
        residue_config: 0 as *mut Residue,
        mapping_count: 0,
        mapping: 0 as *mut Mapping,
        mode_count: 0,
        mode_config: [Mode {
            blockflag: 0,
            mapping: 0,
            windowtype: 0,
            transformtype: 0,
        }; 64],
        total_samples: 0,
        channel_buffers: [0 as *mut libc::c_float; 16],
        outputs: [0 as *mut libc::c_float; 16],
        previous_window: [0 as *mut libc::c_float; 16],
        previous_length: 0,
        finalY: [0 as *mut int16; 16],
        current_loc: 0,
        current_loc_valid: 0,
        A: [0 as *mut libc::c_float; 2],
        B: [0 as *mut libc::c_float; 2],
        C: [0 as *mut libc::c_float; 2],
        window: [0 as *mut libc::c_float; 2],
        bit_reverse: [0 as *mut uint16; 2],
        serial: 0,
        last_page: 0,
        segment_count: 0,
        segments: [0; 255],
        page_flag: 0,
        bytes_in_seg: 0,
        first_decode: 0,
        next_seg: 0,
        last_seg: 0,
        last_seg_which: 0,
        acc: 0,
        valid_bits: 0,
        packet_bytes: 0,
        end_seg_with_known_loc: 0,
        known_loc_for_packet: 0,
        discard_samples_deferred: 0,
        samples_output: 0,
        page_crc_tests: 0,
        scan: [CRCscan {
            goal_crc: 0,
            bytes_left: 0,
            crc_so_far: 0,
            bytes_done: 0,
            sample_loc: 0,
        }; 4],
        channel_buffer_start: 0,
        channel_buffer_end: 0,
    };
    vorbis_init(&mut p, alloc);
    p.f = file;
    p.f_start = ftell(file) as uint32;
    p.stream_len = length;
    p.close_on_free = close_on_free;
    if start_decoder(&mut p) != 0 {
        f = vorbis_alloc(&mut p);
        if !f.is_null() {
            *f = p;
            vorbis_pump_first_frame(f);
            return f;
        }
    }
    if !error_0.is_null() {
        *error_0 = p.error as libc::c_int;
    }
    vorbis_deinit(&mut p);
    return 0 as *mut stb_vorbis;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_open_file(
    mut file: *mut FILE,
    mut close_on_free: libc::c_int,
    mut error_0: *mut libc::c_int,
    mut alloc: *const stb_vorbis_alloc,
) -> *mut stb_vorbis {
    let mut len: libc::c_uint = 0;
    let mut start: libc::c_uint = 0;
    start = ftell(file) as libc::c_uint;
    fseek(file, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    len = (ftell(file) - start as libc::c_long) as libc::c_uint;
    fseek(file, start as libc::c_long, 0 as libc::c_int);
    return stb_vorbis_open_file_section(file, close_on_free, error_0, alloc, len);
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_open_filename(
    mut filename: *const libc::c_char,
    mut error_0: *mut libc::c_int,
    mut alloc: *const stb_vorbis_alloc,
) -> *mut stb_vorbis {
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        return stb_vorbis_open_file(f, 1 as libc::c_int, error_0, alloc);
    }
    if !error_0.is_null() {
        *error_0 = VORBIS_file_open_failure as libc::c_int;
    }
    return 0 as *mut stb_vorbis;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_open_memory(
    mut data: *const libc::c_uchar,
    mut len: libc::c_int,
    mut error_0: *mut libc::c_int,
    mut alloc: *const stb_vorbis_alloc,
) -> *mut stb_vorbis {
    let mut f: *mut stb_vorbis = 0 as *mut stb_vorbis;
    let mut p: stb_vorbis = stb_vorbis {
        sample_rate: 0,
        channels: 0,
        setup_memory_required: 0,
        temp_memory_required: 0,
        setup_temp_memory_required: 0,
        vendor: 0 as *mut libc::c_char,
        comment_list_length: 0,
        comment_list: 0 as *mut *mut libc::c_char,
        f: 0 as *mut FILE,
        f_start: 0,
        close_on_free: 0,
        stream: 0 as *mut uint8,
        stream_start: 0 as *mut uint8,
        stream_end: 0 as *mut uint8,
        stream_len: 0,
        push_mode: 0,
        first_audio_page_offset: 0,
        p_first: ProbedPage {
            page_start: 0,
            page_end: 0,
            last_decoded_sample: 0,
        },
        p_last: ProbedPage {
            page_start: 0,
            page_end: 0,
            last_decoded_sample: 0,
        },
        alloc: stb_vorbis_alloc {
            alloc_buffer: 0 as *mut libc::c_char,
            alloc_buffer_length_in_bytes: 0,
        },
        setup_offset: 0,
        temp_offset: 0,
        eof: 0,
        error: VORBIS__no_error,
        blocksize: [0; 2],
        blocksize_0: 0,
        blocksize_1: 0,
        codebook_count: 0,
        codebooks: 0 as *mut Codebook,
        floor_count: 0,
        floor_types: [0; 64],
        floor_config: 0 as *mut Floor,
        residue_count: 0,
        residue_types: [0; 64],
        residue_config: 0 as *mut Residue,
        mapping_count: 0,
        mapping: 0 as *mut Mapping,
        mode_count: 0,
        mode_config: [Mode {
            blockflag: 0,
            mapping: 0,
            windowtype: 0,
            transformtype: 0,
        }; 64],
        total_samples: 0,
        channel_buffers: [0 as *mut libc::c_float; 16],
        outputs: [0 as *mut libc::c_float; 16],
        previous_window: [0 as *mut libc::c_float; 16],
        previous_length: 0,
        finalY: [0 as *mut int16; 16],
        current_loc: 0,
        current_loc_valid: 0,
        A: [0 as *mut libc::c_float; 2],
        B: [0 as *mut libc::c_float; 2],
        C: [0 as *mut libc::c_float; 2],
        window: [0 as *mut libc::c_float; 2],
        bit_reverse: [0 as *mut uint16; 2],
        serial: 0,
        last_page: 0,
        segment_count: 0,
        segments: [0; 255],
        page_flag: 0,
        bytes_in_seg: 0,
        first_decode: 0,
        next_seg: 0,
        last_seg: 0,
        last_seg_which: 0,
        acc: 0,
        valid_bits: 0,
        packet_bytes: 0,
        end_seg_with_known_loc: 0,
        known_loc_for_packet: 0,
        discard_samples_deferred: 0,
        samples_output: 0,
        page_crc_tests: 0,
        scan: [CRCscan {
            goal_crc: 0,
            bytes_left: 0,
            crc_so_far: 0,
            bytes_done: 0,
            sample_loc: 0,
        }; 4],
        channel_buffer_start: 0,
        channel_buffer_end: 0,
    };
    if data.is_null() {
        if !error_0.is_null() {
            *error_0 = VORBIS_unexpected_eof as libc::c_int;
        }
        return 0 as *mut stb_vorbis;
    }
    vorbis_init(&mut p, alloc);
    p.stream = data as *mut uint8;
    p.stream_end = (data as *mut uint8).offset(len as isize);
    p.stream_start = p.stream;
    p.stream_len = len as uint32;
    p.push_mode = 0 as libc::c_int as uint8;
    if start_decoder(&mut p) != 0 {
        f = vorbis_alloc(&mut p);
        if !f.is_null() {
            *f = p;
            vorbis_pump_first_frame(f);
            if !error_0.is_null() {
                *error_0 = VORBIS__no_error as libc::c_int;
            }
            return f;
        }
    }
    if !error_0.is_null() {
        *error_0 = p.error as libc::c_int;
    }
    vorbis_deinit(&mut p);
    return 0 as *mut stb_vorbis;
}
static mut channel_position: [[int8; 6]; 7] = [
    [0 as libc::c_int as int8, 0, 0, 0, 0, 0],
    [(2 as libc::c_int | 4 as libc::c_int | 1 as libc::c_int) as int8, 0, 0, 0, 0, 0],
    [
        (2 as libc::c_int | 1 as libc::c_int) as int8,
        (4 as libc::c_int | 1 as libc::c_int) as int8,
        0,
        0,
        0,
        0,
    ],
    [
        (2 as libc::c_int | 1 as libc::c_int) as int8,
        (2 as libc::c_int | 4 as libc::c_int | 1 as libc::c_int) as int8,
        (4 as libc::c_int | 1 as libc::c_int) as int8,
        0,
        0,
        0,
    ],
    [
        (2 as libc::c_int | 1 as libc::c_int) as int8,
        (4 as libc::c_int | 1 as libc::c_int) as int8,
        (2 as libc::c_int | 1 as libc::c_int) as int8,
        (4 as libc::c_int | 1 as libc::c_int) as int8,
        0,
        0,
    ],
    [
        (2 as libc::c_int | 1 as libc::c_int) as int8,
        (2 as libc::c_int | 4 as libc::c_int | 1 as libc::c_int) as int8,
        (4 as libc::c_int | 1 as libc::c_int) as int8,
        (2 as libc::c_int | 1 as libc::c_int) as int8,
        (4 as libc::c_int | 1 as libc::c_int) as int8,
        0,
    ],
    [
        (2 as libc::c_int | 1 as libc::c_int) as int8,
        (2 as libc::c_int | 4 as libc::c_int | 1 as libc::c_int) as int8,
        (4 as libc::c_int | 1 as libc::c_int) as int8,
        (2 as libc::c_int | 1 as libc::c_int) as int8,
        (4 as libc::c_int | 1 as libc::c_int) as int8,
        (2 as libc::c_int | 4 as libc::c_int | 1 as libc::c_int) as int8,
    ],
];
unsafe extern "C" fn copy_samples(
    mut dest: *mut libc::c_short,
    mut src: *mut libc::c_float,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        let mut temp: float_conv = float_conv { f: 0. };
        temp
            .f = *src.offset(i as isize)
            + (1.5f32
                * ((1 as libc::c_int) << 23 as libc::c_int - 15 as libc::c_int)
                    as libc::c_float
                + 0.5f32 / ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_float);
        let mut v: libc::c_int = temp.i
            - (((150 as libc::c_int - 15 as libc::c_int) << 23 as libc::c_int)
                + ((1 as libc::c_int) << 22 as libc::c_int));
        if (v + 32768 as libc::c_int) as libc::c_uint
            > 65535 as libc::c_int as libc::c_uint
        {
            v = if v < 0 as libc::c_int {
                -(32768 as libc::c_int)
            } else {
                32767 as libc::c_int
            };
        }
        *dest.offset(i as isize) = v as libc::c_short;
        i += 1;
    }
}
unsafe extern "C" fn compute_samples(
    mut mask: libc::c_int,
    mut output: *mut libc::c_short,
    mut num_c: libc::c_int,
    mut data: *mut *mut libc::c_float,
    mut d_offset: libc::c_int,
    mut len: libc::c_int,
) {
    let mut buffer: [libc::c_float; 32] = [0.; 32];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut n: libc::c_int = 32 as libc::c_int;
    o = 0 as libc::c_int;
    while o < len {
        memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_float; 32]>() as libc::c_ulong,
        );
        if o + n > len {
            n = len - o;
        }
        j = 0 as libc::c_int;
        while j < num_c {
            if channel_position[num_c as usize][j as usize] as libc::c_int & mask != 0 {
                i = 0 as libc::c_int;
                while i < n {
                    buffer[i as usize]
                        += *(*data.offset(j as isize))
                            .offset((d_offset + o + i) as isize);
                    i += 1;
                }
            }
            j += 1;
        }
        i = 0 as libc::c_int;
        while i < n {
            let mut temp: float_conv = float_conv { f: 0. };
            temp
                .f = buffer[i as usize]
                + (1.5f32
                    * ((1 as libc::c_int) << 23 as libc::c_int - 15 as libc::c_int)
                        as libc::c_float
                    + 0.5f32
                        / ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_float);
            let mut v: libc::c_int = temp.i
                - (((150 as libc::c_int - 15 as libc::c_int) << 23 as libc::c_int)
                    + ((1 as libc::c_int) << 22 as libc::c_int));
            if (v + 32768 as libc::c_int) as libc::c_uint
                > 65535 as libc::c_int as libc::c_uint
            {
                v = if v < 0 as libc::c_int {
                    -(32768 as libc::c_int)
                } else {
                    32767 as libc::c_int
                };
            }
            *output.offset((o + i) as isize) = v as libc::c_short;
            i += 1;
        }
        o += 32 as libc::c_int;
    }
}
unsafe extern "C" fn compute_stereo_samples(
    mut output: *mut libc::c_short,
    mut num_c: libc::c_int,
    mut data: *mut *mut libc::c_float,
    mut d_offset: libc::c_int,
    mut len: libc::c_int,
) {
    let mut buffer: [libc::c_float; 32] = [0.; 32];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut n: libc::c_int = 32 as libc::c_int >> 1 as libc::c_int;
    o = 0 as libc::c_int;
    while o < len {
        let mut o2: libc::c_int = o << 1 as libc::c_int;
        memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_float; 32]>() as libc::c_ulong,
        );
        if o + n > len {
            n = len - o;
        }
        j = 0 as libc::c_int;
        while j < num_c {
            let mut m: libc::c_int = channel_position[num_c as usize][j as usize]
                as libc::c_int & (2 as libc::c_int | 4 as libc::c_int);
            if m == 2 as libc::c_int | 4 as libc::c_int {
                i = 0 as libc::c_int;
                while i < n {
                    buffer[(i * 2 as libc::c_int + 0 as libc::c_int) as usize]
                        += *(*data.offset(j as isize))
                            .offset((d_offset + o + i) as isize);
                    buffer[(i * 2 as libc::c_int + 1 as libc::c_int) as usize]
                        += *(*data.offset(j as isize))
                            .offset((d_offset + o + i) as isize);
                    i += 1;
                }
            } else if m == 2 as libc::c_int {
                i = 0 as libc::c_int;
                while i < n {
                    buffer[(i * 2 as libc::c_int + 0 as libc::c_int) as usize]
                        += *(*data.offset(j as isize))
                            .offset((d_offset + o + i) as isize);
                    i += 1;
                }
            } else if m == 4 as libc::c_int {
                i = 0 as libc::c_int;
                while i < n {
                    buffer[(i * 2 as libc::c_int + 1 as libc::c_int) as usize]
                        += *(*data.offset(j as isize))
                            .offset((d_offset + o + i) as isize);
                    i += 1;
                }
            }
            j += 1;
        }
        i = 0 as libc::c_int;
        while i < n << 1 as libc::c_int {
            let mut temp: float_conv = float_conv { f: 0. };
            temp
                .f = buffer[i as usize]
                + (1.5f32
                    * ((1 as libc::c_int) << 23 as libc::c_int - 15 as libc::c_int)
                        as libc::c_float
                    + 0.5f32
                        / ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_float);
            let mut v: libc::c_int = temp.i
                - (((150 as libc::c_int - 15 as libc::c_int) << 23 as libc::c_int)
                    + ((1 as libc::c_int) << 22 as libc::c_int));
            if (v + 32768 as libc::c_int) as libc::c_uint
                > 65535 as libc::c_int as libc::c_uint
            {
                v = if v < 0 as libc::c_int {
                    -(32768 as libc::c_int)
                } else {
                    32767 as libc::c_int
                };
            }
            *output.offset((o2 + i) as isize) = v as libc::c_short;
            i += 1;
        }
        o += 32 as libc::c_int >> 1 as libc::c_int;
    }
}
unsafe extern "C" fn convert_samples_short(
    mut buf_c: libc::c_int,
    mut buffer: *mut *mut libc::c_short,
    mut b_offset: libc::c_int,
    mut data_c: libc::c_int,
    mut data: *mut *mut libc::c_float,
    mut d_offset: libc::c_int,
    mut samples: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if buf_c != data_c && buf_c <= 2 as libc::c_int && data_c <= 6 as libc::c_int {
        static mut channel_selector: [[libc::c_int; 2]; 3] = [
            [0 as libc::c_int, 0],
            [1 as libc::c_int, 0],
            [2 as libc::c_int, 4 as libc::c_int],
        ];
        i = 0 as libc::c_int;
        while i < buf_c {
            compute_samples(
                channel_selector[buf_c as usize][i as usize],
                (*buffer.offset(i as isize)).offset(b_offset as isize),
                data_c,
                data,
                d_offset,
                samples,
            );
            i += 1;
        }
    } else {
        let mut limit: libc::c_int = if buf_c < data_c { buf_c } else { data_c };
        i = 0 as libc::c_int;
        while i < limit {
            copy_samples(
                (*buffer.offset(i as isize)).offset(b_offset as isize),
                (*data.offset(i as isize)).offset(d_offset as isize),
                samples,
            );
            i += 1;
        }
        while i < buf_c {
            memset(
                (*buffer.offset(i as isize)).offset(b_offset as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<libc::c_short>() as libc::c_ulong)
                    .wrapping_mul(samples as libc::c_ulong),
            );
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_frame_short(
    mut f: *mut stb_vorbis,
    mut num_c: libc::c_int,
    mut buffer: *mut *mut libc::c_short,
    mut num_samples: libc::c_int,
) -> libc::c_int {
    let mut output: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut len: libc::c_int = stb_vorbis_get_frame_float(
        f,
        0 as *mut libc::c_int,
        &mut output,
    );
    if len > num_samples {
        len = num_samples;
    }
    if len != 0 {
        convert_samples_short(
            num_c,
            buffer,
            0 as libc::c_int,
            (*f).channels,
            output,
            0 as libc::c_int,
            len,
        );
    }
    return len;
}
unsafe extern "C" fn convert_channels_short_interleaved(
    mut buf_c: libc::c_int,
    mut buffer: *mut libc::c_short,
    mut data_c: libc::c_int,
    mut data: *mut *mut libc::c_float,
    mut d_offset: libc::c_int,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if buf_c != data_c && buf_c <= 2 as libc::c_int && data_c <= 6 as libc::c_int {
        i = 0 as libc::c_int;
        while i < buf_c {
            compute_stereo_samples(buffer, data_c, data, d_offset, len);
            i += 1;
        }
    } else {
        let mut limit: libc::c_int = if buf_c < data_c { buf_c } else { data_c };
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < len {
            i = 0 as libc::c_int;
            while i < limit {
                let mut temp: float_conv = float_conv { f: 0. };
                let mut f: libc::c_float = *(*data.offset(i as isize))
                    .offset((d_offset + j) as isize);
                temp
                    .f = f
                    + (1.5f32
                        * ((1 as libc::c_int) << 23 as libc::c_int - 15 as libc::c_int)
                            as libc::c_float
                        + 0.5f32
                            / ((1 as libc::c_int) << 15 as libc::c_int)
                                as libc::c_float);
                let mut v: libc::c_int = temp.i
                    - (((150 as libc::c_int - 15 as libc::c_int) << 23 as libc::c_int)
                        + ((1 as libc::c_int) << 22 as libc::c_int));
                if (v + 32768 as libc::c_int) as libc::c_uint
                    > 65535 as libc::c_int as libc::c_uint
                {
                    v = if v < 0 as libc::c_int {
                        -(32768 as libc::c_int)
                    } else {
                        32767 as libc::c_int
                    };
                }
                let fresh76 = buffer;
                buffer = buffer.offset(1);
                *fresh76 = v as libc::c_short;
                i += 1;
            }
            while i < buf_c {
                let fresh77 = buffer;
                buffer = buffer.offset(1);
                *fresh77 = 0 as libc::c_int as libc::c_short;
                i += 1;
            }
            j += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_frame_short_interleaved(
    mut f: *mut stb_vorbis,
    mut num_c: libc::c_int,
    mut buffer: *mut libc::c_short,
    mut num_shorts: libc::c_int,
) -> libc::c_int {
    let mut output: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut len: libc::c_int = 0;
    if num_c == 1 as libc::c_int {
        return stb_vorbis_get_frame_short(f, num_c, &mut buffer, num_shorts);
    }
    len = stb_vorbis_get_frame_float(f, 0 as *mut libc::c_int, &mut output);
    if len != 0 {
        if len * num_c > num_shorts {
            len = num_shorts / num_c;
        }
        convert_channels_short_interleaved(
            num_c,
            buffer,
            (*f).channels,
            output,
            0 as libc::c_int,
            len,
        );
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_samples_short_interleaved(
    mut f: *mut stb_vorbis,
    mut channels: libc::c_int,
    mut buffer: *mut libc::c_short,
    mut num_shorts: libc::c_int,
) -> libc::c_int {
    let mut outputs: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut len: libc::c_int = num_shorts / channels;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < len {
        let mut k: libc::c_int = (*f).channel_buffer_end - (*f).channel_buffer_start;
        if n + k >= len {
            k = len - n;
        }
        if k != 0 {
            convert_channels_short_interleaved(
                channels,
                buffer,
                (*f).channels,
                ((*f).channel_buffers).as_mut_ptr(),
                (*f).channel_buffer_start,
                k,
            );
        }
        buffer = buffer.offset((k * channels) as isize);
        n += k;
        (*f).channel_buffer_start += k;
        if n == len {
            break;
        }
        if stb_vorbis_get_frame_float(f, 0 as *mut libc::c_int, &mut outputs) == 0 {
            break;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_samples_short(
    mut f: *mut stb_vorbis,
    mut channels: libc::c_int,
    mut buffer: *mut *mut libc::c_short,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut outputs: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < len {
        let mut k: libc::c_int = (*f).channel_buffer_end - (*f).channel_buffer_start;
        if n + k >= len {
            k = len - n;
        }
        if k != 0 {
            convert_samples_short(
                channels,
                buffer,
                n,
                (*f).channels,
                ((*f).channel_buffers).as_mut_ptr(),
                (*f).channel_buffer_start,
                k,
            );
        }
        n += k;
        (*f).channel_buffer_start += k;
        if n == len {
            break;
        }
        if stb_vorbis_get_frame_float(f, 0 as *mut libc::c_int, &mut outputs) == 0 {
            break;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_decode_filename(
    mut filename: *const libc::c_char,
    mut channels: *mut libc::c_int,
    mut sample_rate: *mut libc::c_int,
    mut output: *mut *mut libc::c_short,
) -> libc::c_int {
    let mut data_len: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut limit: libc::c_int = 0;
    let mut error_0: libc::c_int = 0;
    let mut data: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut v: *mut stb_vorbis = stb_vorbis_open_filename(
        filename,
        &mut error_0,
        0 as *const stb_vorbis_alloc,
    );
    if v.is_null() {
        return -(1 as libc::c_int);
    }
    limit = (*v).channels * 4096 as libc::c_int;
    *channels = (*v).channels;
    if !sample_rate.is_null() {
        *sample_rate = (*v).sample_rate as libc::c_int;
    }
    data_len = 0 as libc::c_int;
    offset = data_len;
    total = limit;
    data = malloc(
        (total as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_short;
    if data.is_null() {
        stb_vorbis_close(v);
        return -(2 as libc::c_int);
    }
    loop {
        let mut n: libc::c_int = stb_vorbis_get_frame_short_interleaved(
            v,
            (*v).channels,
            data.offset(offset as isize),
            total - offset,
        );
        if n == 0 as libc::c_int {
            break;
        }
        data_len += n;
        offset += n * (*v).channels;
        if offset + limit > total {
            let mut data2: *mut libc::c_short = 0 as *mut libc::c_short;
            total *= 2 as libc::c_int;
            data2 = realloc(
                data as *mut libc::c_void,
                (total as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_short;
            if data2.is_null() {
                free(data as *mut libc::c_void);
                stb_vorbis_close(v);
                return -(2 as libc::c_int);
            }
            data = data2;
        }
    }
    *output = data;
    stb_vorbis_close(v);
    return data_len;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_decode_memory(
    mut mem: *const uint8,
    mut len: libc::c_int,
    mut channels: *mut libc::c_int,
    mut sample_rate: *mut libc::c_int,
    mut output: *mut *mut libc::c_short,
) -> libc::c_int {
    let mut data_len: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut limit: libc::c_int = 0;
    let mut error_0: libc::c_int = 0;
    let mut data: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut v: *mut stb_vorbis = stb_vorbis_open_memory(
        mem,
        len,
        &mut error_0,
        0 as *const stb_vorbis_alloc,
    );
    if v.is_null() {
        return -(1 as libc::c_int);
    }
    limit = (*v).channels * 4096 as libc::c_int;
    *channels = (*v).channels;
    if !sample_rate.is_null() {
        *sample_rate = (*v).sample_rate as libc::c_int;
    }
    data_len = 0 as libc::c_int;
    offset = data_len;
    total = limit;
    data = malloc(
        (total as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
    ) as *mut libc::c_short;
    if data.is_null() {
        stb_vorbis_close(v);
        return -(2 as libc::c_int);
    }
    loop {
        let mut n: libc::c_int = stb_vorbis_get_frame_short_interleaved(
            v,
            (*v).channels,
            data.offset(offset as isize),
            total - offset,
        );
        if n == 0 as libc::c_int {
            break;
        }
        data_len += n;
        offset += n * (*v).channels;
        if offset + limit > total {
            let mut data2: *mut libc::c_short = 0 as *mut libc::c_short;
            total *= 2 as libc::c_int;
            data2 = realloc(
                data as *mut libc::c_void,
                (total as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_short;
            if data2.is_null() {
                free(data as *mut libc::c_void);
                stb_vorbis_close(v);
                return -(2 as libc::c_int);
            }
            data = data2;
        }
    }
    *output = data;
    stb_vorbis_close(v);
    return data_len;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_samples_float_interleaved(
    mut f: *mut stb_vorbis,
    mut channels: libc::c_int,
    mut buffer: *mut libc::c_float,
    mut num_floats: libc::c_int,
) -> libc::c_int {
    let mut outputs: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut len: libc::c_int = num_floats / channels;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut z: libc::c_int = (*f).channels;
    if z > channels {
        z = channels;
    }
    while n < len {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = (*f).channel_buffer_end - (*f).channel_buffer_start;
        if n + k >= len {
            k = len - n;
        }
        j = 0 as libc::c_int;
        while j < k {
            i = 0 as libc::c_int;
            while i < z {
                let fresh78 = buffer;
                buffer = buffer.offset(1);
                *fresh78 = *((*f).channel_buffers[i as usize])
                    .offset(((*f).channel_buffer_start + j) as isize);
                i += 1;
            }
            while i < channels {
                let fresh79 = buffer;
                buffer = buffer.offset(1);
                *fresh79 = 0 as libc::c_int as libc::c_float;
                i += 1;
            }
            j += 1;
        }
        n += k;
        (*f).channel_buffer_start += k;
        if n == len {
            break;
        }
        if stb_vorbis_get_frame_float(f, 0 as *mut libc::c_int, &mut outputs) == 0 {
            break;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn stb_vorbis_get_samples_float(
    mut f: *mut stb_vorbis,
    mut channels: libc::c_int,
    mut buffer: *mut *mut libc::c_float,
    mut num_samples: libc::c_int,
) -> libc::c_int {
    let mut outputs: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut z: libc::c_int = (*f).channels;
    if z > channels {
        z = channels;
    }
    while n < num_samples {
        let mut i: libc::c_int = 0;
        let mut k: libc::c_int = (*f).channel_buffer_end - (*f).channel_buffer_start;
        if n + k >= num_samples {
            k = num_samples - n;
        }
        if k != 0 {
            i = 0 as libc::c_int;
            while i < z {
                memcpy(
                    (*buffer.offset(i as isize)).offset(n as isize) as *mut libc::c_void,
                    ((*f).channel_buffers[i as usize])
                        .offset((*f).channel_buffer_start as isize)
                        as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(k as libc::c_ulong),
                );
                i += 1;
            }
            while i < channels {
                memset(
                    (*buffer.offset(i as isize)).offset(n as isize) as *mut libc::c_void,
                    0 as libc::c_int,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(k as libc::c_ulong),
                );
                i += 1;
            }
        }
        n += k;
        (*f).channel_buffer_start += k;
        if n == num_samples {
            break;
        }
        if stb_vorbis_get_frame_float(f, 0 as *mut libc::c_int, &mut outputs) == 0 {
            break;
        }
    }
    return n;
}
