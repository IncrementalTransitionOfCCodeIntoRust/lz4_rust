use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec,
                 __remaining: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn setpriority(__which: __priority_which_t, __who: id_t,
                   __prio: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn RDG_genBuffer(buffer: *mut libc::c_void, size: size_t,
                     matchProba: libc::c_double, litProba: libc::c_double,
                     seed: libc::c_uint);
    #[no_mangle]
    fn LZ4_XXH64(input: *const libc::c_void, length: size_t,
                 seed: libc::c_ulonglong) -> XXH64_hash_t;
    #[no_mangle]
    fn LZ4_compressBound(inputSize: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_compress_fast(src: *const libc::c_char, dst: *mut libc::c_char,
                         srcSize: libc::c_int, dstCapacity: libc::c_int,
                         acceleration: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_createStream() -> *mut LZ4_stream_t;
    #[no_mangle]
    fn LZ4_freeStream(streamPtr: *mut LZ4_stream_t) -> libc::c_int;
    #[no_mangle]
    fn LZ4_resetStream_fast(streamPtr: *mut LZ4_stream_t);
    #[no_mangle]
    fn LZ4_loadDict(streamPtr: *mut LZ4_stream_t,
                    dictionary: *const libc::c_char, dictSize: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4_compress_fast_continue(streamPtr: *mut LZ4_stream_t,
                                  src: *const libc::c_char,
                                  dst: *mut libc::c_char,
                                  srcSize: libc::c_int,
                                  dstCapacity: libc::c_int,
                                  acceleration: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_decompress_safe_usingDict(src: *const libc::c_char,
                                     dst: *mut libc::c_char,
                                     srcSize: libc::c_int,
                                     dstCapcity: libc::c_int,
                                     dictStart: *const libc::c_char,
                                     dictSize: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_attach_dictionary(workingStream: *mut LZ4_stream_t,
                             dictionaryStream: *const LZ4_stream_t);
    #[no_mangle]
    fn LZ4_compress_HC(src: *const libc::c_char, dst: *mut libc::c_char,
                       srcSize: libc::c_int, dstCapacity: libc::c_int,
                       compressionLevel: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_createStreamHC() -> *mut LZ4_streamHC_t;
    #[no_mangle]
    fn LZ4_freeStreamHC(streamHCPtr: *mut LZ4_streamHC_t) -> libc::c_int;
    #[no_mangle]
    fn LZ4_resetStreamHC_fast(streamHCPtr: *mut LZ4_streamHC_t,
                              compressionLevel: libc::c_int);
    #[no_mangle]
    fn LZ4_loadDictHC(streamHCPtr: *mut LZ4_streamHC_t,
                      dictionary: *const libc::c_char, dictSize: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4_compress_HC_continue(streamHCPtr: *mut LZ4_streamHC_t,
                                src: *const libc::c_char,
                                dst: *mut libc::c_char, srcSize: libc::c_int,
                                maxDstSize: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_attach_HC_dictionary(working_stream: *mut LZ4_streamHC_t,
                                dictionary_stream: *const LZ4_streamHC_t);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __id_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type id_t = __id_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type int8_t = __int8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type __priority_which = libc::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type __priority_which_t = libc::c_int;
pub type UTIL_time_t = timespec;
pub type stat_t = stat;
pub type XXH64_hash_t = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockParam_t {
    pub srcPtr: *const libc::c_char,
    pub srcSize: size_t,
    pub cPtr: *mut libc::c_char,
    pub cRoom: size_t,
    pub cSize: size_t,
    pub resPtr: *mut libc::c_char,
    pub resSize: size_t,
}
/*
    bench.c - Demo program to benchmark open-source compression algorithms
    Copyright (C) Yann Collet 2012-2016

    GPL v2 License

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

    You can contact the author at :
    - LZ4 homepage : http://www.lz4.org
    - LZ4 source repository : https://github.com/lz4/lz4
*/
/*-************************************
*  Compiler options
**************************************/
/* Visual Studio */
/* *************************************
*  Includes
***************************************/
/* Compiler options */
/* UTIL_GetFileSize, UTIL_sleep */
/* malloc, free */
/* memset */
/* fprintf, fopen, ftello */
/* clock_t, clock, CLOCKS_PER_SEC */
/* assert */
/* RDG_genBuffer */
/* *************************************
*  Compression parameters and functions
***************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compressionParameters {
    pub cLevel: libc::c_int,
    pub dictBuf: *const libc::c_char,
    pub dictSize: libc::c_int,
    pub LZ4_stream: *mut LZ4_stream_t,
    pub LZ4_dictStream: *mut LZ4_stream_t,
    pub LZ4_streamHC: *mut LZ4_streamHC_t,
    pub LZ4_dictStreamHC: *mut LZ4_streamHC_t,
    pub initFunction: Option<unsafe extern "C" fn(_:
                                                      *mut compressionParameters)
                                 -> ()>,
    pub resetFunction: Option<unsafe extern "C" fn(_:
                                                       *const compressionParameters)
                                  -> ()>,
    pub blockFunction: Option<unsafe extern "C" fn(_:
                                                       *const compressionParameters,
                                                   _: *const libc::c_char,
                                                   _: *mut libc::c_char,
                                                   _: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub cleanupFunction: Option<unsafe extern "C" fn(_:
                                                         *const compressionParameters)
                                    -> ()>,
}
pub type LZ4_streamHC_t = LZ4_streamHC_u;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LZ4_streamHC_u {
    pub table: [size_t; 32775],
    pub internal_donotuse: LZ4HC_CCtx_internal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4HC_CCtx_internal {
    pub hashTable: [uint32_t; 32768],
    pub chainTable: [uint16_t; 65536],
    pub end: *const uint8_t,
    pub base: *const uint8_t,
    pub dictBase: *const uint8_t,
    pub dictLimit: uint32_t,
    pub lowLimit: uint32_t,
    pub nextToUpdate: uint32_t,
    pub compressionLevel: libc::c_short,
    pub favorDecSpeed: int8_t,
    pub dirty: int8_t,
    pub dictCtx: *const LZ4HC_CCtx_internal,
}
pub type LZ4_stream_t = LZ4_stream_u;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LZ4_stream_u {
    pub table: [libc::c_ulonglong; 2052],
    pub internal_donotuse: LZ4_stream_t_internal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4_stream_t_internal {
    pub hashTable: [uint32_t; 4096],
    pub currentOffset: uint32_t,
    pub dirty: uint16_t,
    pub tableType: uint16_t,
    pub dictionary: *const uint8_t,
    pub dictCtx: *const LZ4_stream_t_internal,
    pub dictSize: uint32_t,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn UTIL_getTime() -> UTIL_time_t {
    let mut now: UTIL_time_t = UTIL_time_t{tv_sec: 0, tv_nsec: 0,};
    if clock_gettime(1 as libc::c_int, &mut now) != 0 {
        fprintf(stderr,
                b"ERROR: Failed to get time\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    return now;
}
unsafe extern "C" fn UTIL_getSpanTime(mut begin: UTIL_time_t,
                                      mut end: UTIL_time_t) -> UTIL_time_t {
    let mut diff: UTIL_time_t = UTIL_time_t{tv_sec: 0, tv_nsec: 0,};
    if end.tv_nsec < begin.tv_nsec {
        diff.tv_sec =
            end.tv_sec - 1 as libc::c_int as libc::c_long - begin.tv_sec;
        diff.tv_nsec =
            (end.tv_nsec as
                 libc::c_ulonglong).wrapping_add(1000000000 as
                                                     libc::c_ulonglong).wrapping_sub(begin.tv_nsec
                                                                                         as
                                                                                         libc::c_ulonglong)
                as __syscall_slong_t
    } else {
        diff.tv_sec = end.tv_sec - begin.tv_sec;
        diff.tv_nsec = end.tv_nsec - begin.tv_nsec
    }
    return diff;
}
unsafe extern "C" fn UTIL_getSpanTimeMicro(mut begin: UTIL_time_t,
                                           mut end: UTIL_time_t) -> U64 {
    let diff: UTIL_time_t = UTIL_getSpanTime(begin, end);
    let mut micro: U64 = 0 as libc::c_int as U64;
    micro =
        (micro as
             libc::c_ulonglong).wrapping_add((1000000 as
                                                  libc::c_ulonglong).wrapping_mul(diff.tv_sec
                                                                                      as
                                                                                      libc::c_ulonglong))
            as U64 as U64;
    micro =
        (micro as
             libc::c_ulonglong).wrapping_add((diff.tv_nsec as
                                                  libc::c_ulonglong).wrapping_div(1000
                                                                                      as
                                                                                      libc::c_ulonglong))
            as U64 as U64;
    return micro;
}
unsafe extern "C" fn UTIL_getSpanTimeNano(mut begin: UTIL_time_t,
                                          mut end: UTIL_time_t) -> U64 {
    let diff: UTIL_time_t = UTIL_getSpanTime(begin, end);
    let mut nano: U64 = 0 as libc::c_int as U64;
    nano =
        (nano as
             libc::c_ulonglong).wrapping_add((1000000000 as
                                                  libc::c_ulonglong).wrapping_mul(diff.tv_sec
                                                                                      as
                                                                                      libc::c_ulonglong))
            as U64 as U64;
    nano =
        (nano as libc::c_ulong).wrapping_add(diff.tv_nsec as libc::c_ulong) as
            U64 as U64;
    return nano;
}
unsafe extern "C" fn UTIL_clockSpanMicro(mut clockStart: UTIL_time_t) -> U64 {
    let clockEnd: UTIL_time_t = UTIL_getTime();
    return UTIL_getSpanTimeMicro(clockStart, clockEnd);
}
unsafe extern "C" fn UTIL_clockSpanNano(mut clockStart: UTIL_time_t) -> U64 {
    let clockEnd: UTIL_time_t = UTIL_getTime();
    return UTIL_getSpanTimeNano(clockStart, clockEnd);
}
unsafe extern "C" fn UTIL_waitForNextTick() {
    let clockStart: UTIL_time_t = UTIL_getTime();
    let mut clockEnd: UTIL_time_t = UTIL_time_t{tv_sec: 0, tv_nsec: 0,};
    loop  {
        clockEnd = UTIL_getTime();
        if !(UTIL_getSpanTimeNano(clockStart, clockEnd) ==
                 0 as libc::c_int as libc::c_ulong) {
            break ;
        }
    };
}
unsafe extern "C" fn UTIL_isDirectory(mut infilename: *const libc::c_char)
 -> U32 {
    let mut r: libc::c_int = 0;
    let mut statbuf: stat_t =
        stat_t{st_dev: 0,
               st_ino: 0,
               st_nlink: 0,
               st_mode: 0,
               st_uid: 0,
               st_gid: 0,
               __pad0: 0,
               st_rdev: 0,
               st_size: 0,
               st_blksize: 0,
               st_blocks: 0,
               st_atim: UTIL_time_t{tv_sec: 0, tv_nsec: 0,},
               st_mtim: UTIL_time_t{tv_sec: 0, tv_nsec: 0,},
               st_ctim: UTIL_time_t{tv_sec: 0, tv_nsec: 0,},
               __glibc_reserved: [0; 3],};
    r = stat(infilename, &mut statbuf);
    if r == 0 &&
           statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
               0o40000 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as U32
    }
    return 0 as libc::c_int as U32;
}
unsafe extern "C" fn UTIL_getFileSize(mut infilename: *const libc::c_char)
 -> U64 {
    let mut r: libc::c_int = 0;
    let mut statbuf: stat =
        stat_t{st_dev: 0,
               st_ino: 0,
               st_nlink: 0,
               st_mode: 0,
               st_uid: 0,
               st_gid: 0,
               __pad0: 0,
               st_rdev: 0,
               st_size: 0,
               st_blksize: 0,
               st_blocks: 0,
               st_atim: UTIL_time_t{tv_sec: 0, tv_nsec: 0,},
               st_mtim: UTIL_time_t{tv_sec: 0, tv_nsec: 0,},
               st_ctim: UTIL_time_t{tv_sec: 0, tv_nsec: 0,},
               __glibc_reserved: [0; 3],};
    r = stat(infilename, &mut statbuf);
    if r != 0 ||
           !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                 0o100000 as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int as U64
    }
    return statbuf.st_size as U64;
}
unsafe extern "C" fn UTIL_getTotalFileSize(mut fileNamesTable:
                                               *mut *const libc::c_char,
                                           mut nbFiles: libc::c_uint) -> U64 {
    let mut total: U64 = 0 as libc::c_int as U64;
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    while n < nbFiles {
        total =
            (total as
                 libc::c_ulong).wrapping_add(UTIL_getFileSize(*fileNamesTable.offset(n
                                                                                         as
                                                                                         isize)))
                as U64 as U64;
        n = n.wrapping_add(1)
    }
    return total;
}
unsafe extern "C" fn LZ4_compressInitNoStream(mut pThis:
                                                  *mut compressionParameters) {
    (*pThis).LZ4_stream = 0 as *mut LZ4_stream_t;
    (*pThis).LZ4_dictStream = 0 as *mut LZ4_stream_t;
    (*pThis).LZ4_streamHC = 0 as *mut LZ4_streamHC_t;
    (*pThis).LZ4_dictStreamHC = 0 as *mut LZ4_streamHC_t;
}
unsafe extern "C" fn LZ4_compressInitStream(mut pThis:
                                                *mut compressionParameters) {
    (*pThis).LZ4_stream = LZ4_createStream();
    (*pThis).LZ4_dictStream = LZ4_createStream();
    (*pThis).LZ4_streamHC = 0 as *mut LZ4_streamHC_t;
    (*pThis).LZ4_dictStreamHC = 0 as *mut LZ4_streamHC_t;
    LZ4_loadDict((*pThis).LZ4_dictStream, (*pThis).dictBuf,
                 (*pThis).dictSize);
}
unsafe extern "C" fn LZ4_compressInitStreamHC(mut pThis:
                                                  *mut compressionParameters) {
    (*pThis).LZ4_stream = 0 as *mut LZ4_stream_t;
    (*pThis).LZ4_dictStream = 0 as *mut LZ4_stream_t;
    (*pThis).LZ4_streamHC = LZ4_createStreamHC();
    (*pThis).LZ4_dictStreamHC = LZ4_createStreamHC();
    LZ4_loadDictHC((*pThis).LZ4_dictStreamHC, (*pThis).dictBuf,
                   (*pThis).dictSize);
}
unsafe extern "C" fn LZ4_compressResetNoStream(mut _pThis:
                                                   *const compressionParameters) {
}
unsafe extern "C" fn LZ4_compressResetStream(mut pThis:
                                                 *const compressionParameters) {
    LZ4_resetStream_fast((*pThis).LZ4_stream);
    LZ4_attach_dictionary((*pThis).LZ4_stream, (*pThis).LZ4_dictStream);
}
unsafe extern "C" fn LZ4_compressResetStreamHC(mut pThis:
                                                   *const compressionParameters) {
    LZ4_resetStreamHC_fast((*pThis).LZ4_streamHC, (*pThis).cLevel);
    LZ4_attach_HC_dictionary((*pThis).LZ4_streamHC,
                             (*pThis).LZ4_dictStreamHC);
}
unsafe extern "C" fn LZ4_compressBlockNoStream(mut pThis:
                                                   *const compressionParameters,
                                               mut src: *const libc::c_char,
                                               mut dst: *mut libc::c_char,
                                               mut srcSize: libc::c_int,
                                               mut dstSize: libc::c_int)
 -> libc::c_int {
    let acceleration: libc::c_int =
        if (*pThis).cLevel < 0 as libc::c_int {
            (-(*pThis).cLevel) + 1 as libc::c_int
        } else { 1 as libc::c_int };
    return LZ4_compress_fast(src, dst, srcSize, dstSize, acceleration);
}
unsafe extern "C" fn LZ4_compressBlockNoStreamHC(mut pThis:
                                                     *const compressionParameters,
                                                 mut src: *const libc::c_char,
                                                 mut dst: *mut libc::c_char,
                                                 mut srcSize: libc::c_int,
                                                 mut dstSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC(src, dst, srcSize, dstSize, (*pThis).cLevel);
}
unsafe extern "C" fn LZ4_compressBlockStream(mut pThis:
                                                 *const compressionParameters,
                                             mut src: *const libc::c_char,
                                             mut dst: *mut libc::c_char,
                                             mut srcSize: libc::c_int,
                                             mut dstSize: libc::c_int)
 -> libc::c_int {
    let acceleration: libc::c_int =
        if (*pThis).cLevel < 0 as libc::c_int {
            (-(*pThis).cLevel) + 1 as libc::c_int
        } else { 1 as libc::c_int };
    return LZ4_compress_fast_continue((*pThis).LZ4_stream, src, dst, srcSize,
                                      dstSize, acceleration);
}
unsafe extern "C" fn LZ4_compressBlockStreamHC(mut pThis:
                                                   *const compressionParameters,
                                               mut src: *const libc::c_char,
                                               mut dst: *mut libc::c_char,
                                               mut srcSize: libc::c_int,
                                               mut dstSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC_continue((*pThis).LZ4_streamHC, src, dst, srcSize,
                                    dstSize);
}
unsafe extern "C" fn LZ4_compressCleanupNoStream(mut _pThis:
                                                     *const compressionParameters) {
}
unsafe extern "C" fn LZ4_compressCleanupStream(mut pThis:
                                                   *const compressionParameters) {
    LZ4_freeStream((*pThis).LZ4_stream);
    LZ4_freeStream((*pThis).LZ4_dictStream);
}
unsafe extern "C" fn LZ4_compressCleanupStreamHC(mut pThis:
                                                     *const compressionParameters) {
    LZ4_freeStreamHC((*pThis).LZ4_streamHC);
    LZ4_freeStreamHC((*pThis).LZ4_dictStreamHC);
}
unsafe extern "C" fn LZ4_buildCompressionParameters(mut pParams:
                                                        *mut compressionParameters,
                                                    mut cLevel: libc::c_int,
                                                    mut dictBuf:
                                                        *const libc::c_char,
                                                    mut dictSize:
                                                        libc::c_int) {
    (*pParams).cLevel = cLevel;
    (*pParams).dictBuf = dictBuf;
    (*pParams).dictSize = dictSize;
    if dictSize != 0 {
        if cLevel < 3 as libc::c_int {
            (*pParams).initFunction =
                Some(LZ4_compressInitStream as
                         unsafe extern "C" fn(_: *mut compressionParameters)
                             -> ());
            (*pParams).resetFunction =
                Some(LZ4_compressResetStream as
                         unsafe extern "C" fn(_: *const compressionParameters)
                             -> ());
            (*pParams).blockFunction =
                Some(LZ4_compressBlockStream as
                         unsafe extern "C" fn(_: *const compressionParameters,
                                              _: *const libc::c_char,
                                              _: *mut libc::c_char,
                                              _: libc::c_int, _: libc::c_int)
                             -> libc::c_int);
            (*pParams).cleanupFunction =
                Some(LZ4_compressCleanupStream as
                         unsafe extern "C" fn(_: *const compressionParameters)
                             -> ())
        } else {
            (*pParams).initFunction =
                Some(LZ4_compressInitStreamHC as
                         unsafe extern "C" fn(_: *mut compressionParameters)
                             -> ());
            (*pParams).resetFunction =
                Some(LZ4_compressResetStreamHC as
                         unsafe extern "C" fn(_: *const compressionParameters)
                             -> ());
            (*pParams).blockFunction =
                Some(LZ4_compressBlockStreamHC as
                         unsafe extern "C" fn(_: *const compressionParameters,
                                              _: *const libc::c_char,
                                              _: *mut libc::c_char,
                                              _: libc::c_int, _: libc::c_int)
                             -> libc::c_int);
            (*pParams).cleanupFunction =
                Some(LZ4_compressCleanupStreamHC as
                         unsafe extern "C" fn(_: *const compressionParameters)
                             -> ())
        }
    } else {
        (*pParams).initFunction =
            Some(LZ4_compressInitNoStream as
                     unsafe extern "C" fn(_: *mut compressionParameters)
                         -> ());
        (*pParams).resetFunction =
            Some(LZ4_compressResetNoStream as
                     unsafe extern "C" fn(_: *const compressionParameters)
                         -> ());
        (*pParams).cleanupFunction =
            Some(LZ4_compressCleanupNoStream as
                     unsafe extern "C" fn(_: *const compressionParameters)
                         -> ());
        if cLevel < 3 as libc::c_int {
            (*pParams).blockFunction =
                Some(LZ4_compressBlockNoStream as
                         unsafe extern "C" fn(_: *const compressionParameters,
                                              _: *const libc::c_char,
                                              _: *mut libc::c_char,
                                              _: libc::c_int, _: libc::c_int)
                             -> libc::c_int)
        } else {
            (*pParams).blockFunction =
                Some(LZ4_compressBlockNoStreamHC as
                         unsafe extern "C" fn(_: *const compressionParameters,
                                              _: *const libc::c_char,
                                              _: *mut libc::c_char,
                                              _: libc::c_int, _: libc::c_int)
                             -> libc::c_int)
        }
    };
}
// Initialized in run_static_initializers
static mut maxMemory: size_t = 0;
static mut g_compressibilityDefault: U32 = 50 as libc::c_int as U32;
/* *************************************
*  console display
***************************************/
static mut g_displayLevel: U32 = 2 as libc::c_int as U32;
/* 0 : no display;   1: errors;   2 : + result + interaction + warnings;   3 : + progression;   4 : + information */
static mut refreshRate: clock_t =
    1000000 as libc::c_int as __clock_t * 15 as libc::c_int as libc::c_long /
        100 as libc::c_int as libc::c_long;
static mut g_time: clock_t = 0 as libc::c_int as clock_t;
/* *************************************
*  Benchmark Parameters
***************************************/
static mut g_nbSeconds: U32 = 3 as libc::c_int as U32;
static mut g_blockSize: size_t = 0 as libc::c_int as size_t;
#[no_mangle]
pub static mut g_additionalParam: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut g_benchSeparately: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn BMK_setNotificationLevel(mut level: libc::c_uint) {
    g_displayLevel = level;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setAdditionalParam(mut additionalParam:
                                                    libc::c_int) {
    g_additionalParam = additionalParam;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setNbSeconds(mut nbSeconds: libc::c_uint) {
    g_nbSeconds = nbSeconds;
    if g_displayLevel >= 3 as libc::c_int as libc::c_uint {
        fprintf(stderr,
                b"- test >= %u seconds per compression / decompression -\n\x00"
                    as *const u8 as *const libc::c_char, g_nbSeconds);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setBlockSize(mut blockSize: size_t) {
    g_blockSize = blockSize;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_setBenchSeparately(mut separate: libc::c_int) {
    g_benchSeparately = (separate != 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn BMK_benchMem(mut srcBuffer: *const libc::c_void,
                                  mut srcSize: size_t,
                                  mut displayName: *const libc::c_char,
                                  mut cLevel: libc::c_int,
                                  mut fileSizes: *const size_t,
                                  mut nbFiles: U32,
                                  mut dictBuf: *const libc::c_char,
                                  mut dictSize: libc::c_int) -> libc::c_int {
    let blockSize: size_t =
        (if g_blockSize >= 32 as libc::c_int as libc::c_ulong {
             g_blockSize
         } else {
             srcSize
         }).wrapping_add((srcSize == 0) as libc::c_int as libc::c_ulong);
    let maxNbBlocks: U32 =
        (srcSize.wrapping_add(blockSize.wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong)).wrapping_div(blockSize)
             as U32).wrapping_add(nbFiles);
    let blockTable: *mut blockParam_t =
        malloc((maxNbBlocks as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<blockParam_t>()
                                                    as libc::c_ulong)) as
            *mut blockParam_t;
    /* avoid div by 0 */
    let maxCompressedSize: size_t =
        (LZ4_compressBound(srcSize as libc::c_int) as
             libc::c_uint).wrapping_add(maxNbBlocks.wrapping_mul(1024 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as size_t; /* add some room for safety */
    let compressedBuffer: *mut libc::c_void = malloc(maxCompressedSize);
    let resultBuffer: *mut libc::c_void = malloc(srcSize);
    let mut nbBlocks: U32 = 0;
    let mut compP: compressionParameters =
        compressionParameters{cLevel: 0,
                              dictBuf: 0 as *const libc::c_char,
                              dictSize: 0,
                              LZ4_stream: 0 as *mut LZ4_stream_t,
                              LZ4_dictStream: 0 as *mut LZ4_stream_t,
                              LZ4_streamHC: 0 as *mut LZ4_streamHC_t,
                              LZ4_dictStreamHC: 0 as *mut LZ4_streamHC_t,
                              initFunction: None,
                              resetFunction: None,
                              blockFunction: None,
                              cleanupFunction: None,};
    /* checks */
    if compressedBuffer.is_null() || resultBuffer.is_null() ||
           blockTable.is_null() {
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    31 as libc::c_int); /* can only display 17 characters */
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(31 as libc::c_int);
    }
    if strlen(displayName) > 17 as libc::c_int as libc::c_ulong {
        displayName =
            displayName.offset(strlen(displayName).wrapping_sub(17 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                                   as isize)
    }
    /* init */
    LZ4_buildCompressionParameters(&mut compP, cLevel, dictBuf, dictSize);
    compP.initFunction.expect("non-null function pointer")(&mut compP);
    /* Init blockTable data */
    let mut srcPtr: *const libc::c_char = srcBuffer as *const libc::c_char;
    let mut cPtr: *mut libc::c_char = compressedBuffer as *mut libc::c_char;
    let mut resPtr: *mut libc::c_char = resultBuffer as *mut libc::c_char;
    let mut fileNb: U32 = 0;
    nbBlocks = 0 as libc::c_int as U32;
    fileNb = 0 as libc::c_int as U32;
    while fileNb < nbFiles {
        let mut remaining: size_t = *fileSizes.offset(fileNb as isize);
        let nbBlocksforThisFile: U32 =
            remaining.wrapping_add(blockSize.wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong)).wrapping_div(blockSize)
                as U32;
        let blockEnd: U32 = nbBlocks.wrapping_add(nbBlocksforThisFile);
        while nbBlocks < blockEnd {
            let thisBlockSize: size_t =
                if remaining < blockSize { remaining } else { blockSize };
            let ref mut fresh0 =
                (*blockTable.offset(nbBlocks as isize)).srcPtr;
            *fresh0 = srcPtr;
            let ref mut fresh1 = (*blockTable.offset(nbBlocks as isize)).cPtr;
            *fresh1 = cPtr;
            let ref mut fresh2 =
                (*blockTable.offset(nbBlocks as isize)).resPtr;
            *fresh2 = resPtr;
            (*blockTable.offset(nbBlocks as isize)).srcSize = thisBlockSize;
            (*blockTable.offset(nbBlocks as isize)).cRoom =
                LZ4_compressBound(thisBlockSize as libc::c_int) as size_t;
            srcPtr = srcPtr.offset(thisBlockSize as isize);
            cPtr =
                cPtr.offset((*blockTable.offset(nbBlocks as isize)).cRoom as
                                isize);
            resPtr = resPtr.offset(thisBlockSize as isize);
            remaining =
                (remaining as libc::c_ulong).wrapping_sub(thisBlockSize) as
                    size_t as size_t;
            nbBlocks = nbBlocks.wrapping_add(1)
        }
        fileNb = fileNb.wrapping_add(1)
    }
    /* warmimg up memory */
    RDG_genBuffer(compressedBuffer, maxCompressedSize, 0.10f64, 0.50f64,
                  1 as libc::c_int as libc::c_uint);
    /* Bench */
    let mut fastestC: U64 = -(1 as libc::c_longlong) as U64; /* Bench */
    let mut fastestD: U64 =
        -(1 as libc::c_longlong) as
            U64; /* conservative initial compression speed estimate */
    let crcOrig: U64 =
        LZ4_XXH64(srcBuffer, srcSize, 0 as libc::c_int as libc::c_ulonglong)
            as U64; /* conservative initial decode speed estimate */
    let mut coolTime: UTIL_time_t = UTIL_time_t{tv_sec: 0, tv_nsec: 0,};
    let maxTime: U64 =
        (g_nbSeconds.wrapping_mul(1 as libc::c_int as libc::c_uint) as
             libc::c_ulonglong).wrapping_mul(1000000000 as
                                                 libc::c_ulonglong).wrapping_add(100
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulonglong)
            as U64;
    let mut nbCompressionLoops: U32 =
        (((5 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
              libc::c_ulong).wrapping_div(srcSize.wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong))
             as U32).wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut nbDecodeLoops: U32 =
        (((200 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
              libc::c_ulong).wrapping_div(srcSize.wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong))
             as U32).wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut totalCTime: U64 = 0 as libc::c_int as U64;
    let mut totalDTime: U64 = 0 as libc::c_int as U64;
    let mut cCompleted: U32 = 0 as libc::c_int as U32;
    let mut dCompleted: U32 = 0 as libc::c_int as U32;
    let marks: [*const libc::c_char; 4] =
        [b" |\x00" as *const u8 as *const libc::c_char,
         b" /\x00" as *const u8 as *const libc::c_char,
         b" =\x00" as *const u8 as *const libc::c_char,
         b"\\\x00" as *const u8 as *const libc::c_char];
    let mut markNb: U32 = 0 as libc::c_int as U32;
    let mut cSize: size_t = 0 as libc::c_int as size_t;
    let mut ratio: libc::c_double = 0.0f64;
    coolTime = UTIL_getTime();
    if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    /* CRC Checking */
    while cCompleted == 0 || dCompleted == 0 {
        if UTIL_clockSpanMicro(coolTime) as libc::c_ulonglong >
               (70 as libc::c_int as
                    libc::c_ulonglong).wrapping_mul(1000000 as
                                                        libc::c_ulonglong) {
            if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"\rcooling down ...    \r\x00" as *const u8 as
                            *const libc::c_char); /* for (testNb = 1; testNb <= (g_nbSeconds + !g_nbSeconds); testNb++) */
            }
            sleep(10 as libc::c_int as libc::c_uint);
            coolTime = UTIL_getTime()
        }
        /* overheat protection */
        /* Compression */
        if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"%2s-%-17.17s :%10u ->\r\x00" as *const u8 as
                        *const libc::c_char, marks[markNb as usize],
                    displayName,
                    srcSize as U32); /* warm up and erase result buffer */
        }
        if cCompleted == 0 {
            memset(compressedBuffer, 0xe5 as libc::c_int, maxCompressedSize);
        }
        let mut t: timespec = UTIL_time_t{tv_sec: 0, tv_nsec: 0,};
        t.tv_sec = 0 as libc::c_int as __time_t;
        t.tv_nsec =
            (1 as libc::c_int as
                 libc::c_ulonglong).wrapping_mul(1000000 as libc::c_ulonglong)
                as __syscall_slong_t;
        nanosleep(&mut t, 0 as *mut timespec);
        /* give processor time to other processes */
        UTIL_waitForNextTick();
        if cCompleted == 0 {
            /* still some time to do compression tests */
            let clockStart: UTIL_time_t = UTIL_getTime();
            let mut nbLoops: U32 = 0;
            nbLoops = 0 as libc::c_int as U32;
            while nbLoops < nbCompressionLoops {
                let mut blockNb: U32 = 0;
                compP.resetFunction.expect("non-null function pointer")(&mut compP);
                blockNb = 0 as libc::c_int as U32;
                while blockNb < nbBlocks {
                    let rSize: size_t =
                        compP.blockFunction.expect("non-null function pointer")(&mut compP,
                                                                                (*blockTable.offset(blockNb
                                                                                                        as
                                                                                                        isize)).srcPtr,
                                                                                (*blockTable.offset(blockNb
                                                                                                        as
                                                                                                        isize)).cPtr,
                                                                                (*blockTable.offset(blockNb
                                                                                                        as
                                                                                                        isize)).srcSize
                                                                                    as
                                                                                    libc::c_int,
                                                                                (*blockTable.offset(blockNb
                                                                                                        as
                                                                                                        isize)).cRoom
                                                                                    as
                                                                                    libc::c_int)
                            as size_t;
                    if rSize == 0 as libc::c_int as libc::c_ulong {
                        if g_displayLevel >= 1 as libc::c_int as libc::c_uint
                           {
                            fprintf(stderr,
                                    b"Error %i : \x00" as *const u8 as
                                        *const libc::c_char,
                                    1 as libc::c_int);
                        }
                        if g_displayLevel >= 1 as libc::c_int as libc::c_uint
                           {
                            fprintf(stderr,
                                    b"LZ4 compression failed\x00" as *const u8
                                        as *const libc::c_char);
                        }
                        if g_displayLevel >= 1 as libc::c_int as libc::c_uint
                           {
                            fprintf(stderr,
                                    b"\n\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        exit(1 as libc::c_int);
                    }
                    (*blockTable.offset(blockNb as isize)).cSize = rSize;
                    blockNb = blockNb.wrapping_add(1)
                }
                nbLoops = nbLoops.wrapping_add(1)
            }
            let clockSpan: U64 = UTIL_clockSpanNano(clockStart);
            if clockSpan > 0 as libc::c_int as libc::c_ulong {
                if clockSpan <
                       fastestC.wrapping_mul(nbCompressionLoops as
                                                 libc::c_ulong) {
                    fastestC =
                        clockSpan.wrapping_div(nbCompressionLoops as
                                                   libc::c_ulong)
                }
                if fastestC > 0 as libc::c_int as libc::c_ulong {
                } else {
                    __assert_fail(b"fastestC > 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"bench.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  418 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 98],
                                                            &[libc::c_char; 98]>(b"int BMK_benchMem(const void *, size_t, const char *, int, const size_t *, U32, const char *, int)\x00")).as_ptr());
                }
                nbCompressionLoops =
                    ((1 as libc::c_int as
                          libc::c_ulonglong).wrapping_mul(1000000000 as
                                                              libc::c_ulonglong).wrapping_div(fastestC
                                                                                                  as
                                                                                                  libc::c_ulonglong)
                         as
                         U32).wrapping_add(1 as libc::c_int as libc::c_uint)
                /* aim for ~1sec */
            } else {
                if nbCompressionLoops <
                       40000000 as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"nbCompressionLoops < 40000000\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"bench.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  421 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 98],
                                                            &[libc::c_char; 98]>(b"int BMK_benchMem(const void *, size_t, const char *, int, const size_t *, U32, const char *, int)\x00")).as_ptr()); /* avoid overflow */
                } /* avoid div by 0 */
                nbCompressionLoops =
                    (nbCompressionLoops as
                         libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                        libc::c_uint) as U32
                        as U32
            }
            totalCTime =
                (totalCTime as libc::c_ulong).wrapping_add(clockSpan) as U64
                    as U64;
            cCompleted = (totalCTime > maxTime) as libc::c_int as U32
        }
        cSize = 0 as libc::c_int as size_t;
        let mut blockNb_0: U32 = 0;
        blockNb_0 = 0 as libc::c_int as U32;
        while blockNb_0 < nbBlocks {
            cSize =
                (cSize as
                     libc::c_ulong).wrapping_add((*blockTable.offset(blockNb_0
                                                                         as
                                                                         isize)).cSize)
                    as size_t as size_t;
            blockNb_0 = blockNb_0.wrapping_add(1)
        }
        cSize =
            (cSize as
                 libc::c_ulong).wrapping_add((cSize == 0) as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        ratio = srcSize as libc::c_double / cSize as libc::c_double;
        markNb =
            markNb.wrapping_add(1 as libc::c_int as
                                    libc::c_uint).wrapping_rem(4 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
        if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"%2s-%-17.17s :%10u ->%10u (%5.3f),%6.1f MB/s\r\x00" as
                        *const u8 as *const libc::c_char,
                    marks[markNb as usize], displayName, srcSize as U32,
                    cSize as U32, ratio,
                    srcSize as libc::c_double / fastestC as libc::c_double *
                        1000 as libc::c_int as libc::c_double);
        }
        /*  unused when decompression disabled */
        /* Decompression */
        if dCompleted == 0 {
            memset(resultBuffer, 0xd6 as libc::c_int,
                   srcSize); /* warm result buffer */
        }
        let mut t_0: timespec = UTIL_time_t{tv_sec: 0, tv_nsec: 0,};
        t_0.tv_sec = 0 as libc::c_int as __time_t;
        t_0.tv_nsec =
            (5 as libc::c_int as
                 libc::c_ulonglong).wrapping_mul(1000000 as libc::c_ulonglong)
                as __syscall_slong_t;
        nanosleep(&mut t_0, 0 as *mut timespec);
        /* give processor time to other processes */
        UTIL_waitForNextTick();
        if dCompleted == 0 {
            let clockStart_0: UTIL_time_t = UTIL_getTime();
            let mut nbLoops_0: U32 = 0;
            nbLoops_0 = 0 as libc::c_int as U32;
            while nbLoops_0 < nbDecodeLoops {
                let mut blockNb_1: U32 = 0;
                blockNb_1 = 0 as libc::c_int as U32;
                while blockNb_1 < nbBlocks {
                    let regenSize: libc::c_int =
                        LZ4_decompress_safe_usingDict((*blockTable.offset(blockNb_1
                                                                              as
                                                                              isize)).cPtr,
                                                      (*blockTable.offset(blockNb_1
                                                                              as
                                                                              isize)).resPtr,
                                                      (*blockTable.offset(blockNb_1
                                                                              as
                                                                              isize)).cSize
                                                          as libc::c_int,
                                                      (*blockTable.offset(blockNb_1
                                                                              as
                                                                              isize)).srcSize
                                                          as libc::c_int,
                                                      dictBuf, dictSize);
                    if regenSize < 0 as libc::c_int {
                        fprintf(stderr,
                                b"LZ4_decompress_safe_usingDict() failed on block %u \n\x00"
                                    as *const u8 as *const libc::c_char,
                                blockNb_1);
                        break ;
                    } else {
                        (*blockTable.offset(blockNb_1 as isize)).resSize =
                            regenSize as size_t;
                        blockNb_1 = blockNb_1.wrapping_add(1)
                    }
                }
                nbLoops_0 = nbLoops_0.wrapping_add(1)
            }
            let clockSpan_0: U64 = UTIL_clockSpanNano(clockStart_0);
            if clockSpan_0 > 0 as libc::c_int as libc::c_ulong {
                if clockSpan_0 <
                       fastestD.wrapping_mul(nbDecodeLoops as libc::c_ulong) {
                    fastestD =
                        clockSpan_0.wrapping_div(nbDecodeLoops as
                                                     libc::c_ulong)
                }
                if fastestD > 0 as libc::c_int as libc::c_ulong {
                } else {
                    __assert_fail(b"fastestD > 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"bench.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  465 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 98],
                                                            &[libc::c_char; 98]>(b"int BMK_benchMem(const void *, size_t, const char *, int, const size_t *, U32, const char *, int)\x00")).as_ptr());
                }
                nbDecodeLoops =
                    ((1 as libc::c_int as
                          libc::c_ulonglong).wrapping_mul(1000000000 as
                                                              libc::c_ulonglong).wrapping_div(fastestD
                                                                                                  as
                                                                                                  libc::c_ulonglong)
                         as
                         U32).wrapping_add(1 as libc::c_int as libc::c_uint)
                /* aim for ~1sec */
            } else {
                if nbDecodeLoops < 40000000 as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"nbDecodeLoops < 40000000\x00" as *const u8
                                      as *const libc::c_char,
                                  b"bench.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  468 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 98],
                                                            &[libc::c_char; 98]>(b"int BMK_benchMem(const void *, size_t, const char *, int, const size_t *, U32, const char *, int)\x00")).as_ptr()); /* avoid overflow */
                }
                nbDecodeLoops =
                    (nbDecodeLoops as
                         libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                        libc::c_uint) as U32
                        as U32
            }
            totalDTime =
                (totalDTime as libc::c_ulong).wrapping_add(clockSpan_0) as U64
                    as U64;
            dCompleted =
                (totalDTime >
                     (1 as libc::c_int as
                          libc::c_ulong).wrapping_mul(maxTime)) as libc::c_int
                    as U32
        }
        markNb =
            markNb.wrapping_add(1 as libc::c_int as
                                    libc::c_uint).wrapping_rem(4 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
        if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"%2s-%-17.17s :%10u ->%10u (%5.3f),%6.1f MB/s ,%6.1f MB/s\r\x00"
                        as *const u8 as *const libc::c_char,
                    marks[markNb as usize], displayName, srcSize as U32,
                    cSize as U32, ratio,
                    srcSize as libc::c_double / fastestC as libc::c_double *
                        1000 as libc::c_int as libc::c_double,
                    srcSize as libc::c_double / fastestD as libc::c_double *
                        1000 as libc::c_int as libc::c_double);
        }
        /* CRC Checking */
        let crcCheck: U64 =
            LZ4_XXH64(resultBuffer, srcSize,
                      0 as libc::c_int as libc::c_ulonglong) as U64;
        if !(crcOrig != crcCheck) { continue ; }
        let mut u: size_t = 0;
        fprintf(stderr,
                b"\n!!! WARNING !!! %17s : Invalid Checksum : %x != %x   \n\x00"
                    as *const u8 as *const libc::c_char, displayName,
                crcOrig as libc::c_uint, crcCheck as libc::c_uint);
        u = 0 as libc::c_int as size_t;
        while u < srcSize {
            if *(srcBuffer as *const BYTE).offset(u as isize) as libc::c_int
                   !=
                   *(resultBuffer as *const BYTE).offset(u as isize) as
                       libc::c_int {
                let mut segNb: U32 = 0;
                let mut bNb: U32 = 0;
                let mut pos: U32 = 0;
                let mut bacc: size_t = 0 as libc::c_int as size_t;
                fprintf(stderr,
                        b"Decoding error at pos %u \x00" as *const u8 as
                            *const libc::c_char, u as U32);
                segNb = 0 as libc::c_int as U32;
                while segNb < nbBlocks {
                    if bacc.wrapping_add((*blockTable.offset(segNb as
                                                                 isize)).srcSize)
                           > u {
                        break ;
                    }
                    bacc =
                        (bacc as
                             libc::c_ulong).wrapping_add((*blockTable.offset(segNb
                                                                                 as
                                                                                 isize)).srcSize)
                            as size_t as size_t;
                    segNb = segNb.wrapping_add(1)
                }
                pos = u.wrapping_sub(bacc) as U32;
                bNb =
                    pos.wrapping_div((128 as libc::c_int *
                                          ((1 as libc::c_int) <<
                                               10 as libc::c_int)) as
                                         libc::c_uint);
                fprintf(stderr,
                        b"(block %u, sub %u, pos %u) \n\x00" as *const u8 as
                            *const libc::c_char, segNb, bNb, pos);
                break ;
            } else {
                if u ==
                       srcSize.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                   {
                    /* should never happen */
                    fprintf(stderr,
                            b"no difference detected\n\x00" as *const u8 as
                                *const libc::c_char);
                }
                u = u.wrapping_add(1)
            }
        }
        break ;
    }
    if g_displayLevel == 1 as libc::c_int as libc::c_uint {
        let cSpeed: libc::c_double =
            srcSize as libc::c_double / fastestC as libc::c_double *
                1000 as libc::c_int as libc::c_double;
        let dSpeed: libc::c_double =
            srcSize as libc::c_double / fastestD as libc::c_double *
                1000 as libc::c_int as libc::c_double;
        if g_additionalParam != 0 {
            fprintf(stderr,
                    b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s (param=%d)\n\x00"
                        as *const u8 as *const libc::c_char, cLevel,
                    cSize as libc::c_int, ratio, cSpeed, dSpeed, displayName,
                    g_additionalParam);
        } else {
            fprintf(stderr,
                    b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s\n\x00" as
                        *const u8 as *const libc::c_char, cLevel,
                    cSize as libc::c_int, ratio, cSpeed, dSpeed, displayName);
        }
    }
    if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
        fprintf(stderr, b"%2i#\n\x00" as *const u8 as *const libc::c_char,
                cLevel);
    }
    /* clean up */
    compP.cleanupFunction.expect("non-null function pointer")(&mut compP);
    free(blockTable as *mut libc::c_void);
    free(compressedBuffer);
    free(resultBuffer);
    return 0 as libc::c_int;
}
unsafe extern "C" fn BMK_findMaxMem(mut requiredMem: U64) -> size_t {
    let mut step: size_t =
        (64 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
            size_t;
    let mut testmem: *mut BYTE = 0 as *mut BYTE;
    requiredMem =
        (requiredMem >>
             26 as
                 libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong)
            << 26 as libc::c_int;
    requiredMem =
        (requiredMem as
             libc::c_ulong).wrapping_add((2 as libc::c_int as
                                              libc::c_ulong).wrapping_mul(step))
            as U64 as U64;
    if requiredMem > maxMemory { requiredMem = maxMemory }
    while testmem.is_null() {
        if requiredMem > step {
            requiredMem =
                (requiredMem as libc::c_ulong).wrapping_sub(step) as U64 as
                    U64
        } else { requiredMem >>= 1 as libc::c_int }
        testmem = malloc(requiredMem) as *mut BYTE
    }
    free(testmem as *mut libc::c_void);
    /* keep some space available */
    if requiredMem > step {
        requiredMem =
            (requiredMem as libc::c_ulong).wrapping_sub(step) as U64 as U64
    } else { requiredMem >>= 1 as libc::c_int } /* Windows */
    return requiredMem; /* Linux */
}
unsafe extern "C" fn BMK_benchCLevel(mut srcBuffer: *mut libc::c_void,
                                     mut benchedSize: size_t,
                                     mut displayName: *const libc::c_char,
                                     mut cLevel: libc::c_int,
                                     mut cLevelLast: libc::c_int,
                                     mut fileSizes: *const size_t,
                                     mut nbFiles: libc::c_uint,
                                     mut dictBuf: *const libc::c_char,
                                     mut dictSize: libc::c_int) {
    let mut l: libc::c_int = 0;
    let mut pch: *const libc::c_char = strrchr(displayName, '\\' as i32);
    if pch.is_null() { pch = strrchr(displayName, '/' as i32) }
    if !pch.is_null() { displayName = pch.offset(1 as libc::c_int as isize) }
    setpriority(PRIO_PROCESS as libc::c_int, 0 as libc::c_int as id_t,
                -(20 as libc::c_int));
    if g_displayLevel == 1 as libc::c_int as libc::c_uint &&
           g_additionalParam == 0 {
        fprintf(stderr,
                b"bench %s %s: input %u bytes, %u seconds, %u KB blocks\n\x00"
                    as *const u8 as *const libc::c_char,
                b"1.9.2\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
                benchedSize as U32, g_nbSeconds,
                (g_blockSize >> 10 as libc::c_int) as U32);
    }
    if cLevelLast < cLevel { cLevelLast = cLevel }
    l = cLevel;
    while l <= cLevelLast {
        BMK_benchMem(srcBuffer, benchedSize, displayName, l, fileSizes,
                     nbFiles, dictBuf, dictSize);
        l += 1
    };
}
/* ! BMK_loadFiles() :
    Loads `buffer` with content of files listed within `fileNamesTable`.
    At most, fills `buffer` entirely */
unsafe extern "C" fn BMK_loadFiles(mut buffer: *mut libc::c_void,
                                   mut bufferSize: size_t,
                                   mut fileSizes: *mut size_t,
                                   mut fileNamesTable:
                                       *mut *const libc::c_char,
                                   mut nbFiles: libc::c_uint) {
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut totalSize: size_t = 0 as libc::c_int as size_t;
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    while n < nbFiles {
        let mut f: *mut FILE = 0 as *mut FILE;
        let mut fileSize: U64 =
            UTIL_getFileSize(*fileNamesTable.offset(n as isize));
        if UTIL_isDirectory(*fileNamesTable.offset(n as isize)) != 0 {
            if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Ignoring %s directory...       \n\x00" as *const u8
                            as *const libc::c_char,
                        *fileNamesTable.offset(n as isize));
            }
            *fileSizes.offset(n as isize) = 0 as libc::c_int as size_t
        } else {
            f =
                fopen(*fileNamesTable.offset(n as isize),
                      b"rb\x00" as *const u8 as *const libc::c_char);
            if f.is_null() {
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 10 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"impossible to open file %s\x00" as *const u8 as
                                *const libc::c_char,
                            *fileNamesTable.offset(n as isize));
                }
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"\n\x00" as *const u8 as *const libc::c_char);
                }
                exit(10 as libc::c_int);
            }
            if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
                if clock() - g_time > refreshRate ||
                       g_displayLevel >= 4 as libc::c_int as libc::c_uint {
                    g_time = clock();
                    fprintf(stderr,
                            b"Loading %s...       \r\x00" as *const u8 as
                                *const libc::c_char,
                            *fileNamesTable.offset(n as isize));
                    if g_displayLevel >= 4 as libc::c_int as libc::c_uint {
                        fflush(stdout);
                    }
                }
            }
            if fileSize > bufferSize.wrapping_sub(pos) {
                /* buffer too small - stop after this file */
                fileSize = bufferSize.wrapping_sub(pos);
                nbFiles = n
            }
            let readSize: size_t =
                fread((buffer as *mut libc::c_char).offset(pos as isize) as
                          *mut libc::c_void,
                      1 as libc::c_int as libc::c_ulong, fileSize, f);
            if readSize != fileSize {
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 11 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"could not read %s\x00" as *const u8 as
                                *const libc::c_char,
                            *fileNamesTable.offset(n as isize));
                }
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"\n\x00" as *const u8 as *const libc::c_char);
                }
                exit(11 as libc::c_int);
            }
            pos =
                (pos as libc::c_ulong).wrapping_add(readSize) as size_t as
                    size_t;
            *fileSizes.offset(n as isize) = fileSize;
            totalSize =
                (totalSize as libc::c_ulong).wrapping_add(fileSize) as size_t
                    as size_t;
            fclose(f);
        }
        n = n.wrapping_add(1)
    }
    if totalSize == 0 as libc::c_int as libc::c_ulong {
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    12 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"no data to bench\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(12 as libc::c_int);
    };
}
unsafe extern "C" fn BMK_benchFileTable(mut fileNamesTable:
                                            *mut *const libc::c_char,
                                        mut nbFiles: libc::c_uint,
                                        mut cLevel: libc::c_int,
                                        mut cLevelLast: libc::c_int,
                                        mut dictBuf: *const libc::c_char,
                                        mut dictSize: libc::c_int) {
    let mut srcBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut benchedSize: size_t = 0;
    let mut fileSizes: *mut size_t =
        malloc((nbFiles as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>()
                                                    as libc::c_ulong)) as
            *mut size_t;
    let totalSizeToLoad: U64 = UTIL_getTotalFileSize(fileNamesTable, nbFiles);
    let mut mfName: [libc::c_char; 20] =
        [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0];
    if fileSizes.is_null() {
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    12 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"not enough memory for fileSizes\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(12 as libc::c_int);
    }
    /* Memory allocation & restrictions */
    benchedSize =
        BMK_findMaxMem(totalSizeToLoad.wrapping_mul(3 as libc::c_int as
                                                        libc::c_ulong)).wrapping_div(3
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong); /* avoid alloc of zero */
    if benchedSize == 0 as libc::c_int as libc::c_ulong {
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    12 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"not enough memory\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(12 as libc::c_int);
    }
    if benchedSize > totalSizeToLoad { benchedSize = totalSizeToLoad }
    if benchedSize > 0x7e000000 as libc::c_int as libc::c_ulong {
        benchedSize = 0x7e000000 as libc::c_int as size_t;
        fprintf(stderr,
                b"File(s) bigger than LZ4\'s max input size; testing %u MB only...\n\x00"
                    as *const u8 as *const libc::c_char,
                (benchedSize >> 20 as libc::c_int) as U32);
    } else if benchedSize < totalSizeToLoad {
        fprintf(stderr,
                b"Not enough memory; testing %u MB only...\n\x00" as *const u8
                    as *const libc::c_char,
                (benchedSize >> 20 as libc::c_int) as U32);
    }
    srcBuffer =
        malloc(benchedSize.wrapping_add((benchedSize == 0) as libc::c_int as
                                            libc::c_ulong));
    if srcBuffer.is_null() {
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    12 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"not enough memory\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(12 as libc::c_int);
    }
    /* Load input buffer */
    BMK_loadFiles(srcBuffer, benchedSize, fileSizes, fileNamesTable, nbFiles);
    /* Bench */
    snprintf(mfName.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
             b" %u files\x00" as *const u8 as *const libc::c_char, nbFiles);
    let mut displayName: *const libc::c_char =
        if nbFiles > 1 as libc::c_int as libc::c_uint {
            mfName.as_mut_ptr() as *const libc::c_char
        } else { *fileNamesTable.offset(0 as libc::c_int as isize) };
    BMK_benchCLevel(srcBuffer, benchedSize, displayName, cLevel, cLevelLast,
                    fileSizes, nbFiles, dictBuf, dictSize);
    /* clean up */
    free(srcBuffer);
    free(fileSizes as *mut libc::c_void);
}
unsafe extern "C" fn BMK_syntheticTest(mut cLevel: libc::c_int,
                                       mut cLevelLast: libc::c_int,
                                       mut compressibility: libc::c_double,
                                       mut dictBuf: *const libc::c_char,
                                       mut dictSize: libc::c_int) {
    let mut name: [libc::c_char; 20] =
        [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0];
    let mut benchedSize: size_t = 10000000 as libc::c_int as size_t;
    let srcBuffer: *mut libc::c_void = malloc(benchedSize);
    /* Memory allocation */
    if srcBuffer.is_null() {
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    21 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"not enough memory\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        exit(21 as libc::c_int);
    }
    /* Fill input buffer */
    RDG_genBuffer(srcBuffer, benchedSize, compressibility, 0.0f64,
                  0 as libc::c_int as libc::c_uint);
    /* Bench */
    snprintf(name.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
             b"Synthetic %2u%%\x00" as *const u8 as *const libc::c_char,
             (compressibility * 100 as libc::c_int as libc::c_double) as
                 libc::c_uint);
    BMK_benchCLevel(srcBuffer, benchedSize, name.as_mut_ptr(), cLevel,
                    cLevelLast, &mut benchedSize,
                    1 as libc::c_int as libc::c_uint, dictBuf, dictSize);
    /* clean up */
    free(srcBuffer);
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFilesSeparately(mut fileNamesTable:
                                                      *mut *const libc::c_char,
                                                  mut nbFiles: libc::c_uint,
                                                  mut cLevel: libc::c_int,
                                                  mut cLevelLast: libc::c_int,
                                                  mut dictBuf:
                                                      *const libc::c_char,
                                                  mut dictSize: libc::c_int)
 -> libc::c_int {
    let mut fileNb: libc::c_uint = 0;
    if cLevel > 12 as libc::c_int { cLevel = 12 as libc::c_int }
    if cLevelLast > 12 as libc::c_int { cLevelLast = 12 as libc::c_int }
    if cLevelLast < cLevel { cLevelLast = cLevel }
    if cLevelLast > cLevel {
        if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"Benchmarking levels from %d to %d\n\x00" as *const u8 as
                        *const libc::c_char, cLevel, cLevelLast);
        }
    }
    fileNb = 0 as libc::c_int as libc::c_uint;
    while fileNb < nbFiles {
        BMK_benchFileTable(fileNamesTable.offset(fileNb as isize),
                           1 as libc::c_int as libc::c_uint, cLevel,
                           cLevelLast, dictBuf, dictSize);
        fileNb = fileNb.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFiles(mut fileNamesTable:
                                            *mut *const libc::c_char,
                                        mut nbFiles: libc::c_uint,
                                        mut cLevel: libc::c_int,
                                        mut cLevelLast: libc::c_int,
                                        mut dictFileName: *const libc::c_char)
 -> libc::c_int {
    let compressibility: libc::c_double =
        g_compressibilityDefault as libc::c_double /
            100 as libc::c_int as libc::c_double;
    let mut dictBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dictSize: libc::c_int = 0 as libc::c_int;
    if cLevel > 12 as libc::c_int { cLevel = 12 as libc::c_int }
    if cLevelLast > 12 as libc::c_int { cLevelLast = 12 as libc::c_int }
    if cLevelLast < cLevel { cLevelLast = cLevel }
    if cLevelLast > cLevel {
        if g_displayLevel >= 2 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    b"Benchmarking levels from %d to %d\n\x00" as *const u8 as
                        *const libc::c_char, cLevel, cLevelLast);
        }
    }
    if !dictFileName.is_null() {
        let mut dictFile: *mut FILE = 0 as *mut FILE;
        let mut dictFileSize: U64 = UTIL_getFileSize(dictFileName);
        if dictFileSize == 0 {
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 25 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Dictionary error : could not stat dictionary file\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            exit(25 as libc::c_int);
        }
        dictFile =
            fopen(dictFileName,
                  b"rb\x00" as *const u8 as *const libc::c_char);
        if dictFile.is_null() {
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 25 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Dictionary error : could not open dictionary file\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            exit(25 as libc::c_int);
        }
        if dictFileSize >
               (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                   as libc::c_ulong {
            dictSize =
                64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int);
            if fseek(dictFile,
                     dictFileSize.wrapping_sub(dictSize as libc::c_ulong) as
                         libc::c_long, 0 as libc::c_int) != 0 {
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 25 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"Dictionary error : could not seek dictionary file\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"\n\x00" as *const u8 as *const libc::c_char);
                }
                exit(25 as libc::c_int);
            }
        } else { dictSize = dictFileSize as libc::c_int }
        dictBuf = malloc(dictSize as libc::c_ulong) as *mut libc::c_char;
        if dictBuf.is_null() {
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 25 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Allocation error : not enough memory\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            exit(25 as libc::c_int);
        }
        if fread(dictBuf as *mut libc::c_void,
                 1 as libc::c_int as libc::c_ulong, dictSize as libc::c_ulong,
                 dictFile) != dictSize as size_t {
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 25 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"Dictionary error : could not read dictionary file\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            exit(25 as libc::c_int);
        }
        fclose(dictFile);
    }
    if nbFiles == 0 as libc::c_int as libc::c_uint {
        BMK_syntheticTest(cLevel, cLevelLast, compressibility, dictBuf,
                          dictSize);
    } else if g_benchSeparately != 0 {
        BMK_benchFilesSeparately(fileNamesTable, nbFiles, cLevel, cLevelLast,
                                 dictBuf, dictSize);
    } else {
        BMK_benchFileTable(fileNamesTable, nbFiles, cLevel, cLevelLast,
                           dictBuf, dictSize);
    }
    free(dictBuf as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    maxMemory =
        if ::std::mem::size_of::<size_t>() as libc::c_ulong ==
               4 as libc::c_int as libc::c_ulong {
            (2 as libc::c_int as
                 libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                                30 as
                                                    libc::c_int).wrapping_sub((64
                                                                                   as
                                                                                   libc::c_int
                                                                                   *
                                                                                   ((1
                                                                                         as
                                                                                         libc::c_int)
                                                                                        <<
                                                                                        20
                                                                                            as
                                                                                            libc::c_int))
                                                                                  as
                                                                                  libc::c_uint)
                as libc::c_ulong
        } else {
            ((1 as libc::c_ulonglong) <<
                 (::std::mem::size_of::<size_t>() as
                      libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                      libc::c_ulong).wrapping_sub(31
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                as size_t
        }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
