use ::libc;
use ::f128;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LZ4F_cctx_s;
    pub type LZ4F_dctx_s;
    pub type LZ4F_CDict_s;
    #[no_mangle]
    fn chown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t)
     -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
              _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn utimensat(__fd: libc::c_int, __path: *const libc::c_char,
                 __times: *const timespec, __flags: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4_decompress_safe(src: *const libc::c_char, dst: *mut libc::c_char,
                           compressedSize: libc::c_int,
                           dstCapacity: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_compressBound(inputSize: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_compress_fast(src: *const libc::c_char, dst: *mut libc::c_char,
                         srcSize: libc::c_int, dstCapacity: libc::c_int,
                         acceleration: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_compress_HC(src: *const libc::c_char, dst: *mut libc::c_char,
                       srcSize: libc::c_int, dstCapacity: libc::c_int,
                       compressionLevel: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4F_isError(code: LZ4F_errorCode_t) -> libc::c_uint;
    #[no_mangle]
    fn LZ4F_getErrorName(code: LZ4F_errorCode_t) -> *const libc::c_char;
    #[no_mangle]
    fn LZ4F_compressFrameBound(srcSize: size_t,
                               preferencesPtr: *const LZ4F_preferences_t)
     -> size_t;
    #[no_mangle]
    fn LZ4F_createCompressionContext(cctxPtr: *mut *mut LZ4F_cctx,
                                     version: libc::c_uint)
     -> LZ4F_errorCode_t;
    #[no_mangle]
    fn LZ4F_freeCompressionContext(cctx: *mut LZ4F_cctx) -> LZ4F_errorCode_t;
    #[no_mangle]
    fn LZ4F_compressUpdate(cctx: *mut LZ4F_cctx, dstBuffer: *mut libc::c_void,
                           dstCapacity: size_t,
                           srcBuffer: *const libc::c_void, srcSize: size_t,
                           cOptPtr: *const LZ4F_compressOptions_t) -> size_t;
    #[no_mangle]
    fn LZ4F_compressEnd(cctx: *mut LZ4F_cctx, dstBuffer: *mut libc::c_void,
                        dstCapacity: size_t,
                        cOptPtr: *const LZ4F_compressOptions_t) -> size_t;
    #[no_mangle]
    fn LZ4F_createDecompressionContext(dctxPtr: *mut *mut LZ4F_dctx,
                                       version: libc::c_uint)
     -> LZ4F_errorCode_t;
    #[no_mangle]
    fn LZ4F_freeDecompressionContext(dctx: *mut LZ4F_dctx)
     -> LZ4F_errorCode_t;
    #[no_mangle]
    fn LZ4F_headerSize(src: *const libc::c_void, srcSize: size_t) -> size_t;
    #[no_mangle]
    fn LZ4F_getFrameInfo(dctx: *mut LZ4F_dctx,
                         frameInfoPtr: *mut LZ4F_frameInfo_t,
                         srcBuffer: *const libc::c_void,
                         srcSizePtr: *mut size_t) -> size_t;
    #[no_mangle]
    fn LZ4F_createCDict(dictBuffer: *const libc::c_void, dictSize: size_t)
     -> *mut LZ4F_CDict;
    #[no_mangle]
    fn LZ4F_freeCDict(CDict: *mut LZ4F_CDict);
    #[no_mangle]
    fn LZ4F_compressFrame_usingCDict(cctx: *mut LZ4F_cctx,
                                     dst: *mut libc::c_void,
                                     dstCapacity: size_t,
                                     src: *const libc::c_void,
                                     srcSize: size_t,
                                     cdict: *const LZ4F_CDict,
                                     preferencesPtr:
                                         *const LZ4F_preferences_t) -> size_t;
    #[no_mangle]
    fn LZ4F_compressBegin_usingCDict(cctx: *mut LZ4F_cctx,
                                     dstBuffer: *mut libc::c_void,
                                     dstCapacity: size_t,
                                     cdict: *const LZ4F_CDict,
                                     prefsPtr: *const LZ4F_preferences_t)
     -> size_t;
    #[no_mangle]
    fn LZ4F_decompress_usingDict(dctxPtr: *mut LZ4F_dctx,
                                 dstBuffer: *mut libc::c_void,
                                 dstSizePtr: *mut size_t,
                                 srcBuffer: *const libc::c_void,
                                 srcSizePtr: *mut size_t,
                                 dict: *const libc::c_void, dictSize: size_t,
                                 decompressOptionsPtr:
                                     *const LZ4F_decompressOptions_t)
     -> size_t;
}
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
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
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
pub type uint64_t = __uint64_t;
pub type U64 = uint64_t;
pub type UTIL_time_t = timespec;
pub type stat_t = stat;
pub type LZ4F_errorCode_t = size_t;
pub type LZ4F_blockSizeID_t = libc::c_uint;
pub const LZ4F_max4MB: LZ4F_blockSizeID_t = 7;
pub const LZ4F_max1MB: LZ4F_blockSizeID_t = 6;
pub const LZ4F_max256KB: LZ4F_blockSizeID_t = 5;
pub const LZ4F_max64KB: LZ4F_blockSizeID_t = 4;
pub const LZ4F_default: LZ4F_blockSizeID_t = 0;
pub type LZ4F_blockMode_t = libc::c_uint;
pub const LZ4F_blockIndependent: LZ4F_blockMode_t = 1;
pub const LZ4F_blockLinked: LZ4F_blockMode_t = 0;
pub type LZ4F_contentChecksum_t = libc::c_uint;
pub const LZ4F_contentChecksumEnabled: LZ4F_contentChecksum_t = 1;
pub const LZ4F_noContentChecksum: LZ4F_contentChecksum_t = 0;
pub type LZ4F_blockChecksum_t = libc::c_uint;
pub const LZ4F_blockChecksumEnabled: LZ4F_blockChecksum_t = 1;
pub const LZ4F_noBlockChecksum: LZ4F_blockChecksum_t = 0;
pub type LZ4F_frameType_t = libc::c_uint;
pub const LZ4F_skippableFrame: LZ4F_frameType_t = 1;
pub const LZ4F_frame: LZ4F_frameType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_frameInfo_t {
    pub blockSizeID: LZ4F_blockSizeID_t,
    pub blockMode: LZ4F_blockMode_t,
    pub contentChecksumFlag: LZ4F_contentChecksum_t,
    pub frameType: LZ4F_frameType_t,
    pub contentSize: libc::c_ulonglong,
    pub dictID: libc::c_uint,
    pub blockChecksumFlag: LZ4F_blockChecksum_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_preferences_t {
    pub frameInfo: LZ4F_frameInfo_t,
    pub compressionLevel: libc::c_int,
    pub autoFlush: libc::c_uint,
    pub favorDecSpeed: libc::c_uint,
    pub reserved: [libc::c_uint; 3],
}
pub type LZ4F_cctx = LZ4F_cctx_s;
pub type LZ4F_compressionContext_t = *mut LZ4F_cctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_compressOptions_t {
    pub stableSrc: libc::c_uint,
    pub reserved: [libc::c_uint; 3],
}
pub type LZ4F_dctx = LZ4F_dctx_s;
pub type LZ4F_decompressionContext_t = *mut LZ4F_dctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_decompressOptions_t {
    pub stableDst: libc::c_uint,
    pub reserved: [libc::c_uint; 3],
}
pub type LZ4F_CDict = LZ4F_CDict_s;
/* use after variable declarations */
/* *************************************
*  Local Parameters
**************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4IO_prefs_s {
    pub passThrough: libc::c_int,
    pub overwrite: libc::c_int,
    pub testMode: libc::c_int,
    pub blockSizeId: libc::c_int,
    pub blockSize: size_t,
    pub blockChecksum: libc::c_int,
    pub streamChecksum: libc::c_int,
    pub blockIndependence: libc::c_int,
    pub sparseFileSupport: libc::c_int,
    pub contentSizeFlag: libc::c_int,
    pub useDictionary: libc::c_int,
    pub favorDecSpeed: libc::c_uint,
    pub dictionaryFilename: *const libc::c_char,
    pub removeSrcFile: libc::c_int,
}
pub type LZ4IO_prefs_t = LZ4IO_prefs_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cRess_t {
    pub srcBuffer: *mut libc::c_void,
    pub srcBufferSize: size_t,
    pub dstBuffer: *mut libc::c_void,
    pub dstBufferSize: size_t,
    pub ctx: LZ4F_compressionContext_t,
    pub cdict: *mut LZ4F_CDict,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dRess_t {
    pub srcBuffer: *mut libc::c_void,
    pub srcBufferSize: size_t,
    pub dstBuffer: *mut libc::c_void,
    pub dstBufferSize: size_t,
    pub dstFile: *mut FILE,
    pub dCtx: LZ4F_decompressionContext_t,
    pub dictBuffer: *mut libc::c_void,
    pub dictBufferSize: size_t,
}
pub type LZ4IO_blockMode_t = libc::c_uint;
pub const LZ4IO_blockIndependent: LZ4IO_blockMode_t = 1;
pub const LZ4IO_blockLinked: LZ4IO_blockMode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4IO_cFileInfo_t {
    pub fileName: *const libc::c_char,
    pub fileSize: libc::c_ulonglong,
    pub frameCount: libc::c_ulonglong,
    pub frameSummary: LZ4IO_frameInfo_t,
    pub eqFrameTypes: libc::c_ushort,
    pub eqBlockTypes: libc::c_ushort,
    pub allContentSize: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4IO_frameInfo_t {
    pub lz4FrameInfo: LZ4F_frameInfo_t,
    pub frameType: LZ4IO_frameType_t,
}
pub type LZ4IO_frameType_t = libc::c_uint;
pub const skippableFrame: LZ4IO_frameType_t = 2;
pub const legacyFrame: LZ4IO_frameType_t = 1;
pub const lz4Frame: LZ4IO_frameType_t = 0;
pub const LZ4IO_format_not_known: LZ4IO_infoResult = 1;
pub type LZ4IO_infoResult = libc::c_uint;
pub const LZ4IO_not_a_file: LZ4IO_infoResult = 2;
pub const LZ4IO_LZ4F_OK: LZ4IO_infoResult = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const LZ4IO_static_assert: C2RustUnnamed = 1;
pub type compress_f
    =
    Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_char,
                                _: libc::c_int, _: libc::c_int,
                                _: libc::c_int) -> libc::c_int>;
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int { return getc(stdin); }
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
unsafe extern "C" fn UTIL_clockSpanNano(mut clockStart: UTIL_time_t) -> U64 {
    let clockEnd: UTIL_time_t = UTIL_getTime();
    return UTIL_getSpanTimeNano(clockStart, clockEnd);
}
unsafe extern "C" fn UTIL_isRegFile(mut infilename: *const libc::c_char)
 -> libc::c_int {
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
    return UTIL_getFileStat(infilename, &mut statbuf);
}
unsafe extern "C" fn UTIL_getFileStat(mut infilename: *const libc::c_char,
                                      mut statbuf: *mut stat_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = stat(infilename, statbuf);
    if r != 0 ||
           !((*statbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint ==
                 0o100000 as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn UTIL_setFileStat(mut filename: *const libc::c_char,
                                      mut statbuf: *mut stat_t)
 -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    if UTIL_isRegFile(filename) == 0 { return -(1 as libc::c_int) }
    let mut timebuf: [timespec; 2] =
        [UTIL_time_t{tv_sec: 0, tv_nsec: 0,},
         UTIL_time_t{tv_sec: 0, tv_nsec: 0,}];
    timebuf[0 as libc::c_int as usize].tv_nsec =
        ((1 as libc::c_long) << 30 as libc::c_int) - 1 as libc::c_long;
    timebuf[1 as libc::c_int as usize].tv_sec = (*statbuf).st_mtim.tv_sec;
    res +=
        utimensat(-(100 as libc::c_int), filename,
                  timebuf.as_mut_ptr() as *const timespec, 0 as libc::c_int);
    res += chown(filename, (*statbuf).st_uid, (*statbuf).st_gid);
    res +=
        chmod(filename,
              (*statbuf).st_mode & 0o7777 as libc::c_int as libc::c_uint);
    *__errno_location() = 0 as libc::c_int;
    return -res;
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
static mut stdinmark: [libc::c_char; 6] = [115, 116, 100, 105, 110, 0];
static mut stdoutmark: [libc::c_char; 7] = [115, 116, 100, 111, 117, 116, 0];
static mut nulmark: [libc::c_char; 10] =
    [47, 100, 101, 118, 47, 110, 117, 108, 108, 0];
/* *************************************
*  Macros
**************************************/
static mut g_displayLevel: libc::c_int = 0 as libc::c_int;
/* 0 : no display  ; 1: errors  ; 2 : + result + interaction + warnings ; 3 : + progression; 4 : + information */
static mut refreshRate: clock_t =
    1000000 as libc::c_int as __clock_t / 6 as libc::c_int as libc::c_long;
static mut g_time: clock_t = 0 as libc::c_int as clock_t;
/* *************************************
*  Version modifiers
**************************************/
/* ************************************************** */
/* ****************** Parameters ******************** */
/* ************************************************** */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_defaultPreferences() -> *mut LZ4IO_prefs_t {
    let ret: *mut LZ4IO_prefs_t =
        malloc(::std::mem::size_of::<LZ4IO_prefs_t>() as libc::c_ulong) as
            *mut LZ4IO_prefs_t;
    if ret.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    21 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(21 as libc::c_int);
    }
    (*ret).passThrough = 0 as libc::c_int;
    (*ret).overwrite = 1 as libc::c_int;
    (*ret).testMode = 0 as libc::c_int;
    (*ret).blockSizeId = 7 as libc::c_int;
    (*ret).blockSize = 0 as libc::c_int as size_t;
    (*ret).blockChecksum = 0 as libc::c_int;
    (*ret).streamChecksum = 1 as libc::c_int;
    (*ret).blockIndependence = 1 as libc::c_int;
    (*ret).sparseFileSupport = 1 as libc::c_int;
    (*ret).contentSizeFlag = 0 as libc::c_int;
    (*ret).useDictionary = 0 as libc::c_int;
    (*ret).favorDecSpeed = 0 as libc::c_int as libc::c_uint;
    (*ret).dictionaryFilename = 0 as *const libc::c_char;
    (*ret).removeSrcFile = 0 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_freePreferences(prefs: *mut LZ4IO_prefs_t) {
    free(prefs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setDictionaryFilename(prefs:
                                                         *mut LZ4IO_prefs_t,
                                                     mut dictionaryFilename:
                                                         *const libc::c_char)
 -> libc::c_int {
    (*prefs).dictionaryFilename = dictionaryFilename;
    (*prefs).useDictionary =
        (dictionaryFilename != 0 as *mut libc::c_void as *const libc::c_char)
            as libc::c_int;
    return (*prefs).useDictionary;
}
/* Default setting : passThrough = 0; return : passThrough mode (0/1) */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setPassThrough(prefs: *mut LZ4IO_prefs_t,
                                              mut yes: libc::c_int)
 -> libc::c_int {
    (*prefs).passThrough = (yes != 0 as libc::c_int) as libc::c_int;
    return (*prefs).passThrough;
}
/* Default setting : overwrite = 1; return : overwrite mode (0/1) */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setOverwrite(prefs: *mut LZ4IO_prefs_t,
                                            mut yes: libc::c_int)
 -> libc::c_int {
    (*prefs).overwrite = (yes != 0 as libc::c_int) as libc::c_int;
    return (*prefs).overwrite;
}
/* Default setting : testMode = 0; return : testMode (0/1) */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setTestMode(prefs: *mut LZ4IO_prefs_t,
                                           mut yes: libc::c_int)
 -> libc::c_int {
    (*prefs).testMode = (yes != 0 as libc::c_int) as libc::c_int;
    return (*prefs).testMode;
}
/* blockSizeID : valid values : 4-5-6-7 */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setBlockSizeID(prefs: *mut LZ4IO_prefs_t,
                                              mut bsid: libc::c_uint)
 -> size_t {
    static mut blockSizeTable: [size_t; 4] =
        [(64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
             size_t,
         (256 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
             size_t,
         (1 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
             size_t,
         (4 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
             size_t];
    static mut minBlockSizeID: libc::c_uint =
        4 as libc::c_int as libc::c_uint;
    static mut maxBlockSizeID: libc::c_uint =
        7 as libc::c_int as libc::c_uint;
    if bsid < minBlockSizeID || bsid > maxBlockSizeID {
        return 0 as libc::c_int as size_t
    }
    (*prefs).blockSizeId = bsid as libc::c_int;
    (*prefs).blockSize =
        blockSizeTable[((*prefs).blockSizeId as
                            libc::c_uint).wrapping_sub(minBlockSizeID) as
                           usize];
    return (*prefs).blockSize;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setBlockSize(prefs: *mut LZ4IO_prefs_t,
                                            mut blockSize: size_t) -> size_t {
    static mut minBlockSize: size_t = 32 as libc::c_int as size_t;
    static mut maxBlockSize: size_t =
        (4 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
            size_t;
    let mut bsid: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if blockSize < minBlockSize { blockSize = minBlockSize }
    if blockSize > maxBlockSize { blockSize = maxBlockSize }
    (*prefs).blockSize = blockSize;
    blockSize = blockSize.wrapping_sub(1);
    loop 
         /* find which of { 64k, 256k, 1MB, 4MB } is closest to blockSize */
         {
        blockSize >>= 2 as libc::c_int;
        if !(blockSize != 0) { break ; }
        bsid = bsid.wrapping_add(1)
    }
    if bsid < 7 as libc::c_int as libc::c_uint {
        bsid = 7 as libc::c_int as libc::c_uint
    }
    (*prefs).blockSizeId =
        bsid.wrapping_sub(3 as libc::c_int as libc::c_uint) as libc::c_int;
    return (*prefs).blockSize;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setBlockMode(prefs: *mut LZ4IO_prefs_t,
                                            mut blockMode: LZ4IO_blockMode_t)
 -> libc::c_int {
    (*prefs).blockIndependence =
        (blockMode as libc::c_uint ==
             LZ4IO_blockIndependent as libc::c_int as libc::c_uint) as
            libc::c_int;
    return (*prefs).blockIndependence;
}
/* Default setting : no block checksum */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setBlockChecksumMode(prefs: *mut LZ4IO_prefs_t,
                                                    mut enable: libc::c_int)
 -> libc::c_int {
    (*prefs).blockChecksum = (enable != 0 as libc::c_int) as libc::c_int;
    return (*prefs).blockChecksum;
}
/* Default setting : checksum enabled */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setStreamChecksumMode(prefs:
                                                         *mut LZ4IO_prefs_t,
                                                     mut enable: libc::c_int)
 -> libc::c_int {
    (*prefs).streamChecksum = (enable != 0 as libc::c_int) as libc::c_int;
    return (*prefs).streamChecksum;
}
/* Default setting : 0 (no notification) */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setNotificationLevel(mut level: libc::c_int)
 -> libc::c_int {
    g_displayLevel = level;
    return g_displayLevel;
}
/* Default setting : 0 (disabled) */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setSparseFile(prefs: *mut LZ4IO_prefs_t,
                                             mut enable: libc::c_int)
 -> libc::c_int {
    (*prefs).sparseFileSupport = (enable != 0 as libc::c_int) as libc::c_int;
    return (*prefs).sparseFileSupport;
}
/* Default setting : 0 (disabled) */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setContentSize(prefs: *mut LZ4IO_prefs_t,
                                              mut enable: libc::c_int)
 -> libc::c_int {
    (*prefs).contentSizeFlag = (enable != 0 as libc::c_int) as libc::c_int;
    return (*prefs).contentSizeFlag;
}
/* Default setting : 0 (disabled) */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_favorDecSpeed(prefs: *mut LZ4IO_prefs_t,
                                             mut favor: libc::c_int) {
    (*prefs).favorDecSpeed =
        (favor != 0 as libc::c_int) as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_setRemoveSrcFile(prefs: *mut LZ4IO_prefs_t,
                                                mut flag: libc::c_uint) {
    (*prefs).removeSrcFile =
        (flag > 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
/* ************************************************************************ **
** ********************** LZ4 File / Pipe compression ********************* **
** ************************************************************************ */
unsafe extern "C" fn LZ4IO_isSkippableMagicNumber(mut magic: libc::c_uint)
 -> libc::c_int {
    return (magic & 0xfffffff0 as libc::c_uint ==
                0x184d2a50 as libc::c_int as libc::c_uint) as libc::c_int;
}
/* * LZ4IO_openSrcFile() :
 * condition : `srcFileName` must be non-NULL.
 * @result : FILE* to `dstFileName`, or NULL if it fails */
unsafe extern "C" fn LZ4IO_openSrcFile(mut srcFileName: *const libc::c_char)
 -> *mut FILE {
    let mut f: *mut FILE = 0 as *mut FILE;
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        if g_displayLevel >= 4 as libc::c_int {
            fprintf(stderr,
                    b"Using stdin for input\n\x00" as *const u8 as
                        *const libc::c_char);
        }
        f = stdin
    } else {
        f = fopen(srcFileName, b"rb\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"%s: %s \n\x00" as *const u8 as *const libc::c_char,
                        srcFileName, strerror(*__errno_location()));
            }
        }
    }
    return f;
}
/* * FIO_openDstFile() :
 *  condition : `dstFileName` must be non-NULL.
 * @result : FILE* to `dstFileName`, or NULL if it fails */
unsafe extern "C" fn LZ4IO_openDstFile(prefs: *mut LZ4IO_prefs_t,
                                       mut dstFileName: *const libc::c_char)
 -> *mut FILE {
    let mut f: *mut FILE = 0 as *mut FILE;
    if !dstFileName.is_null() {
    } else {
        __assert_fail(b"dstFileName != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"lz4io.c\x00" as *const u8 as *const libc::c_char,
                      333 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 60],
                                                &[libc::c_char; 60]>(b"FILE *LZ4IO_openDstFile(LZ4IO_prefs_t *const, const char *)\x00")).as_ptr());
    }
    if strcmp(dstFileName, stdoutmark.as_ptr()) == 0 {
        if g_displayLevel >= 4 as libc::c_int {
            fprintf(stderr,
                    b"Using stdout for output\n\x00" as *const u8 as
                        *const libc::c_char);
        }
        f = stdout;
        if (*prefs).sparseFileSupport == 1 as libc::c_int {
            (*prefs).sparseFileSupport = 0 as libc::c_int;
            if g_displayLevel >= 4 as libc::c_int {
                fprintf(stderr,
                        b"Sparse File Support is automatically disabled on stdout ; try --sparse \n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
    } else {
        if (*prefs).overwrite == 0 &&
               strcmp(dstFileName, nulmark.as_ptr()) != 0 {
            /* Check if destination file already exists */
            f =
                fopen(dstFileName,
                      b"rb\x00" as *const u8 as *const libc::c_char);
            if !f.is_null() {
                /* dest exists, prompt for overwrite authorization */
                fclose(f);
                if g_displayLevel <= 1 as libc::c_int {
                    /* No interaction possible */
                    fprintf(stderr,
                            b"%s already exists; not overwritten  \n\x00" as
                                *const u8 as *const libc::c_char,
                            dstFileName);
                    return 0 as *mut FILE
                }
                fprintf(stderr,
                        b"%s already exists; do you wish to overwrite (y/N) ? \x00"
                            as *const u8 as *const libc::c_char, dstFileName);
                let mut ch: libc::c_int = getchar();
                if ch != 'Y' as i32 && ch != 'y' as i32 {
                    fprintf(stderr,
                            b"    not overwritten  \n\x00" as *const u8 as
                                *const libc::c_char);
                    return 0 as *mut FILE
                }
                while ch != -(1 as libc::c_int) && ch != '\n' as i32 {
                    ch = getchar()
                }
                /* flush rest of input line */
            }
        }
        f = fopen(dstFileName, b"wb\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
                        dstFileName, strerror(*__errno_location()));
            }
        }
    }
    /* sparse file */
    // todo

    return f;
}
/* **************************************
*   Legacy Compression
***************************************/
/* unoptimized version; solves endianess & alignment issues */
unsafe extern "C" fn LZ4IO_writeLE32(mut p: *mut libc::c_void,
                                     mut value32: libc::c_uint) {
    let dstPtr: *mut libc::c_uchar = p as *mut libc::c_uchar;
    *dstPtr.offset(0 as libc::c_int as isize) = value32 as libc::c_uchar;
    *dstPtr.offset(1 as libc::c_int as isize) =
        (value32 >> 8 as libc::c_int) as libc::c_uchar;
    *dstPtr.offset(2 as libc::c_int as isize) =
        (value32 >> 16 as libc::c_int) as libc::c_uchar;
    *dstPtr.offset(3 as libc::c_int as isize) =
        (value32 >> 24 as libc::c_int) as libc::c_uchar;
}
unsafe extern "C" fn LZ4IO_LZ4_compress(mut src: *const libc::c_char,
                                        mut dst: *mut libc::c_char,
                                        mut srcSize: libc::c_int,
                                        mut dstSize: libc::c_int,
                                        mut _cLevel: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_fast(src, dst, srcSize, dstSize, 1 as libc::c_int);
}
/* LZ4IO_compressFilename_Legacy :
 * This function is intentionally "hidden" (not published in .h)
 * It generates compressed streams using the old 'legacy' format */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_compressFilename_Legacy(prefs:
                                                           *mut LZ4IO_prefs_t,
                                                       mut input_filename:
                                                           *const libc::c_char,
                                                       mut output_filename:
                                                           *const libc::c_char,
                                                       mut compressionlevel:
                                                           libc::c_int)
 -> libc::c_int {
    let compressionFunction: compress_f =
        if compressionlevel < 3 as libc::c_int {
            Some(LZ4IO_LZ4_compress as
                     unsafe extern "C" fn(_: *const libc::c_char,
                                          _: *mut libc::c_char,
                                          _: libc::c_int, _: libc::c_int,
                                          _: libc::c_int) -> libc::c_int)
        } else {
            Some(LZ4_compress_HC as
                     unsafe extern "C" fn(_: *const libc::c_char,
                                          _: *mut libc::c_char,
                                          _: libc::c_int, _: libc::c_int,
                                          _: libc::c_int) -> libc::c_int)
        };
    let mut filesize: libc::c_ulonglong =
        0 as libc::c_int as libc::c_ulonglong;
    let mut compressedfilesize: libc::c_ulonglong =
        4 as libc::c_int as libc::c_ulonglong;
    let mut in_buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out_buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let outBuffSize: libc::c_int =
        LZ4_compressBound(8 as libc::c_int *
                              ((1 as libc::c_int) << 20 as libc::c_int));
    let finput: *mut FILE = LZ4IO_openSrcFile(input_filename);
    let mut foutput: *mut FILE = 0 as *mut FILE;
    let mut clockEnd: clock_t = 0;
    /* Init */
    let clockStart: clock_t = clock();
    if finput.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    20 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s : open file error \x00" as *const u8 as
                        *const libc::c_char, input_filename);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(20 as libc::c_int);
    }
    foutput = LZ4IO_openDstFile(prefs, output_filename);
    if foutput.is_null() {
        fclose(finput);
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    20 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"%s : open file error \x00" as *const u8 as
                        *const libc::c_char, input_filename);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(20 as libc::c_int);
    }
    /* Allocate Memory */
    in_buff =
        malloc((8 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int))
                   as libc::c_ulong) as *mut libc::c_char;
    out_buff =
        malloc((outBuffSize as
                    size_t).wrapping_add(4 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    if in_buff.is_null() || out_buff.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    21 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(21 as libc::c_int);
    }
    /* Write Archive Header */
    LZ4IO_writeLE32(out_buff as *mut libc::c_void,
                    0x184c2102 as libc::c_int as libc::c_uint);
    let writeSize: size_t =
        fwrite(out_buff as *const libc::c_void,
               1 as libc::c_int as libc::c_ulong,
               4 as libc::c_int as libc::c_ulong, foutput);
    if writeSize != 4 as libc::c_int as libc::c_ulong {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    22 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Write error : cannot write header\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(22 as libc::c_int);
    }
    loop 
         /* Main Loop */
         {
        let mut outSize: libc::c_int = 0;
        /* Read Block */
        let inSize: size_t =
            fread(in_buff as *mut libc::c_void, 1 as libc::c_int as size_t,
                  (8 as libc::c_int *
                       ((1 as libc::c_int) << 20 as libc::c_int)) as size_t,
                  finput);
        if inSize <=
               (8 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int))
                   as libc::c_ulong {
        } else {
            __assert_fail(b"inSize <= LEGACY_BLOCKSIZE\x00" as *const u8 as
                              *const libc::c_char,
                          b"lz4io.c\x00" as *const u8 as *const libc::c_char,
                          437 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 89],
                                                    &[libc::c_char; 89]>(b"int LZ4IO_compressFilename_Legacy(LZ4IO_prefs_t *const, const char *, const char *, int)\x00")).as_ptr());
        }
        if inSize == 0 as libc::c_int as libc::c_ulong { break ; }
        filesize = filesize.wrapping_add(inSize as libc::c_ulonglong);
        /* Compress Block */
        outSize =
            compressionFunction.expect("non-null function pointer")(in_buff,
                                                                    out_buff.offset(4
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize),
                                                                    inSize as
                                                                        libc::c_int,
                                                                    outBuffSize,
                                                                    compressionlevel);
        compressedfilesize =
            compressedfilesize.wrapping_add((outSize + 4 as libc::c_int) as
                                                libc::c_ulonglong);
        if g_displayLevel >= 2 as libc::c_int {
            if clock() - g_time > refreshRate ||
                   g_displayLevel >= 4 as libc::c_int {
                g_time = clock();
                fprintf(stderr,
                        b"\rRead : %i MB  ==> %.2f%%   \x00" as *const u8 as
                            *const libc::c_char,
                        (filesize >> 20 as libc::c_int) as libc::c_int,
                        compressedfilesize as libc::c_double /
                            filesize as libc::c_double *
                            100 as libc::c_int as libc::c_double);
                if g_displayLevel >= 4 as libc::c_int { fflush(stderr); }
            }
        }
        /* Write Block */
        if outSize > 0 as libc::c_int {
        } else {
            __assert_fail(b"outSize > 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"lz4io.c\x00" as *const u8 as *const libc::c_char,
                          448 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 89],
                                                    &[libc::c_char; 89]>(b"int LZ4IO_compressFilename_Legacy(LZ4IO_prefs_t *const, const char *, const char *, int)\x00")).as_ptr());
        }
        if outSize < outBuffSize {
        } else {
            __assert_fail(b"outSize < outBuffSize\x00" as *const u8 as
                              *const libc::c_char,
                          b"lz4io.c\x00" as *const u8 as *const libc::c_char,
                          449 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 89],
                                                    &[libc::c_char; 89]>(b"int LZ4IO_compressFilename_Legacy(LZ4IO_prefs_t *const, const char *, const char *, int)\x00")).as_ptr());
        }
        LZ4IO_writeLE32(out_buff as *mut libc::c_void,
                        outSize as libc::c_uint);
        let writeSize_0: size_t =
            fwrite(out_buff as *const libc::c_void,
                   1 as libc::c_int as libc::c_ulong,
                   (outSize + 4 as libc::c_int) as libc::c_ulong, foutput);
        if writeSize_0 != (outSize + 4 as libc::c_int) as size_t {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 24 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Write error : cannot write compressed block\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(24 as libc::c_int);
        }
    }
    if ferror(finput) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    25 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error while reading %s \x00" as *const u8 as
                        *const libc::c_char, input_filename);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(25 as libc::c_int);
    }
    /* Status */
    clockEnd = clock(); /* avoid division by zero (speed) */
    if clockEnd == clockStart {
        clockEnd += 1 as libc::c_int as libc::c_long
    } /* avoid division by zero (ratio) */
    filesize =
        filesize.wrapping_add((filesize == 0) as libc::c_int as
                                  libc::c_ulonglong);
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    /* blank line */
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(stderr,
                b"Compressed %llu bytes into %llu bytes ==> %.2f%%\n\x00" as
                    *const u8 as *const libc::c_char, filesize,
                compressedfilesize,
                compressedfilesize as libc::c_double /
                    filesize as libc::c_double *
                    100 as libc::c_int as libc::c_double);
    }
    let seconds: libc::c_double =
        (clockEnd - clockStart) as libc::c_double /
            1000000 as libc::c_int as __clock_t as libc::c_double;
    if g_displayLevel >= 4 as libc::c_int {
        fprintf(stderr,
                b"Done in %.2f s ==> %.2f MB/s\n\x00" as *const u8 as
                    *const libc::c_char, seconds,
                filesize as libc::c_double / seconds /
                    1024 as libc::c_int as libc::c_double /
                    1024 as libc::c_int as libc::c_double);
    }
    /* Close & Free */
    free(in_buff as *mut libc::c_void); /* do not close stdout */
    free(out_buff as *mut libc::c_void);
    fclose(finput);
    if strcmp(output_filename, stdoutmark.as_ptr()) != 0 { fclose(foutput); }
    return 0 as libc::c_int;
}
/* LZ4IO_compressMultipleFilenames_Legacy :
 * This function is intentionally "hidden" (not published in .h)
 * It generates multiple compressed streams using the old 'legacy' format */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_compressMultipleFilenames_Legacy(prefs:
                                                                    *mut LZ4IO_prefs_t,
                                                                mut inFileNamesTable:
                                                                    *mut *const libc::c_char,
                                                                mut ifntSize:
                                                                    libc::c_int,
                                                                mut suffix:
                                                                    *const libc::c_char,
                                                                mut compressionLevel:
                                                                    libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0; /* not enough memory */
    let mut missed_files: libc::c_int = 0 as libc::c_int;
    let mut dstFileName: *mut libc::c_char =
        malloc(30 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    let mut ofnSize: size_t = 30 as libc::c_int as size_t;
    let suffixSize: size_t = strlen(suffix);
    if dstFileName.is_null() { return ifntSize }
    /* loop on each file */
    i = 0 as libc::c_int;
    while i < ifntSize {
        let ifnSize: size_t = strlen(*inFileNamesTable.offset(i as isize));
        if strcmp(suffix, stdoutmark.as_ptr()) == 0 {
            missed_files +=
                LZ4IO_compressFilename_Legacy(prefs,
                                              *inFileNamesTable.offset(i as
                                                                           isize),
                                              stdoutmark.as_ptr(),
                                              compressionLevel)
        } else {
            if ofnSize <=
                   ifnSize.wrapping_add(suffixSize).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
               {
                free(dstFileName as *mut libc::c_void);
                ofnSize =
                    ifnSize.wrapping_add(20 as libc::c_int as libc::c_ulong);
                dstFileName = malloc(ofnSize) as *mut libc::c_char;
                if dstFileName.is_null() { return ifntSize }
            }
            strcpy(dstFileName, *inFileNamesTable.offset(i as isize));
            strcat(dstFileName, suffix);
            missed_files +=
                LZ4IO_compressFilename_Legacy(prefs,
                                              *inFileNamesTable.offset(i as
                                                                           isize),
                                              dstFileName, compressionLevel)
        }
        i += 1
    }
    /* Close & Free */
    free(dstFileName as *mut libc::c_void);
    return missed_files;
}
unsafe extern "C" fn LZ4IO_createDict(prefs: *mut LZ4IO_prefs_t,
                                      mut dictSize: *mut size_t)
 -> *mut libc::c_void {
    let mut readSize: size_t = 0;
    let mut dictEnd: size_t = 0 as libc::c_int as size_t;
    let mut dictLen: size_t = 0 as libc::c_int as size_t;
    let mut dictStart: size_t = 0;
    let mut circularBufSize: size_t =
        (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
            size_t;
    let mut circularBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dictBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dictFilename: *const libc::c_char = (*prefs).dictionaryFilename;
    let mut dictFile: *mut FILE = 0 as *mut FILE;
    if dictFilename.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    25 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Dictionary error : no filename provided\x00" as
                        *const u8 as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(25 as libc::c_int);
    }
    circularBuf = malloc(circularBufSize) as *mut libc::c_char;
    if circularBuf.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    25 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(25 as libc::c_int);
    }
    dictFile = LZ4IO_openSrcFile(dictFilename);
    if dictFile.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    25 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Dictionary error : could not open dictionary file\x00"
                        as *const u8 as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(25 as libc::c_int);
    }
    /* opportunistically seek to the part of the file we care about. If this */
    /* fails it's not a problem since we'll just read everything anyways.    */
    if strcmp(dictFilename, stdinmark.as_ptr()) != 0 {
        fseek(dictFile,
              -(64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                  as libc::c_long, 2 as libc::c_int);
    }
    loop  {
        readSize =
            fread(circularBuf.offset(dictEnd as isize) as *mut libc::c_void,
                  1 as libc::c_int as libc::c_ulong,
                  circularBufSize.wrapping_sub(dictEnd), dictFile);
        dictEnd =
            dictEnd.wrapping_add(readSize).wrapping_rem(circularBufSize);
        dictLen =
            (dictLen as libc::c_ulong).wrapping_add(readSize) as size_t as
                size_t;
        if !(readSize > 0 as libc::c_int as libc::c_ulong) { break ; }
    }
    if dictLen >
           (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
               libc::c_ulong {
        dictLen =
            (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
                size_t
    }
    *dictSize = dictLen;
    dictStart =
        circularBufSize.wrapping_add(dictEnd).wrapping_sub(dictLen).wrapping_rem(circularBufSize);
    if dictStart == 0 as libc::c_int as libc::c_ulong {
        /* We're in the simple case where the dict starts at the beginning of our circular buffer. */
        dictBuf = circularBuf;
        circularBuf = 0 as *mut libc::c_char
    } else {
        /* Otherwise, we will alloc a new buffer and copy our dict into that. */
        dictBuf =
            malloc(if dictLen != 0 {
                       dictLen
                   } else { 1 as libc::c_int as libc::c_ulong }) as
                *mut libc::c_char;
        if dictBuf.is_null() {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 25 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Allocation error : not enough memory\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(25 as libc::c_int);
        }
        memcpy(dictBuf as *mut libc::c_void,
               circularBuf.offset(dictStart as isize) as *const libc::c_void,
               circularBufSize.wrapping_sub(dictStart));
        memcpy(dictBuf.offset(circularBufSize as
                                  isize).offset(-(dictStart as isize)) as
                   *mut libc::c_void, circularBuf as *const libc::c_void,
               dictLen.wrapping_sub(circularBufSize.wrapping_sub(dictStart)));
    }
    fclose(dictFile);
    free(circularBuf as *mut libc::c_void);
    return dictBuf as *mut libc::c_void;
}
unsafe extern "C" fn LZ4IO_createCDict(prefs: *mut LZ4IO_prefs_t)
 -> *mut LZ4F_CDict {
    let mut dictionarySize: size_t = 0;
    let mut dictionaryBuffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cdict: *mut LZ4F_CDict = 0 as *mut LZ4F_CDict;
    if (*prefs).useDictionary == 0 { return 0 as *mut LZ4F_CDict }
    dictionaryBuffer = LZ4IO_createDict(prefs, &mut dictionarySize);
    if dictionaryBuffer.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    25 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Dictionary error : could not create dictionary\x00" as
                        *const u8 as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(25 as libc::c_int);
    }
    cdict = LZ4F_createCDict(dictionaryBuffer, dictionarySize);
    free(dictionaryBuffer);
    return cdict;
}
unsafe extern "C" fn LZ4IO_createCResources(prefs: *mut LZ4IO_prefs_t)
 -> cRess_t {
    let blockSize: size_t = (*prefs).blockSize;
    let mut ress: cRess_t =
        cRess_t{srcBuffer: 0 as *mut libc::c_void,
                srcBufferSize: 0,
                dstBuffer: 0 as *mut libc::c_void,
                dstBufferSize: 0,
                ctx: 0 as *mut LZ4F_cctx,
                cdict: 0 as *mut LZ4F_CDict,};
    let errorCode: LZ4F_errorCode_t =
        LZ4F_createCompressionContext(&mut ress.ctx,
                                      100 as libc::c_int as libc::c_uint);
    if LZ4F_isError(errorCode) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    30 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Allocation error : can\'t create LZ4F context : %s\x00"
                        as *const u8 as *const libc::c_char,
                    LZ4F_getErrorName(errorCode));
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(30 as libc::c_int);
    }
    /* Allocate Memory */
    ress.srcBuffer = malloc(blockSize); /* cover worst case */
    ress.srcBufferSize = blockSize;
    ress.dstBufferSize =
        LZ4F_compressFrameBound(blockSize, 0 as *const LZ4F_preferences_t);
    ress.dstBuffer = malloc(ress.dstBufferSize);
    if ress.srcBuffer.is_null() || ress.dstBuffer.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    31 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(31 as libc::c_int);
    }
    ress.cdict = LZ4IO_createCDict(prefs);
    return ress;
}
unsafe extern "C" fn LZ4IO_freeCResources(mut ress: cRess_t) {
    free(ress.srcBuffer);
    free(ress.dstBuffer);
    LZ4F_freeCDict(ress.cdict);
    ress.cdict = 0 as *mut LZ4F_CDict;
    let errorCode: LZ4F_errorCode_t = LZ4F_freeCompressionContext(ress.ctx);
    if LZ4F_isError(errorCode) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    38 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error : can\'t free LZ4F context resource : %s\x00" as
                        *const u8 as *const libc::c_char,
                    LZ4F_getErrorName(errorCode));
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(38 as libc::c_int);
    };
}
/*
 * LZ4IO_compressFilename_extRess()
 * result : 0 : compression completed correctly
 *          1 : missing or pb opening srcFileName
 */
unsafe extern "C" fn LZ4IO_compressFilename_extRess(io_prefs:
                                                        *mut LZ4IO_prefs_t,
                                                    mut ress: cRess_t,
                                                    mut srcFileName:
                                                        *const libc::c_char,
                                                    mut dstFileName:
                                                        *const libc::c_char,
                                                    mut compressionLevel:
                                                        libc::c_int)
 -> libc::c_int {
    let mut filesize: libc::c_ulonglong =
        0 as libc::c_int as libc::c_ulonglong; /* just a pointer */
    let mut compressedfilesize: libc::c_ulonglong =
        0 as libc::c_int as libc::c_ulonglong;
    let mut srcFile: *mut FILE = 0 as *mut FILE;
    let mut dstFile: *mut FILE = 0 as *mut FILE;
    let srcBuffer: *mut libc::c_void = ress.srcBuffer;
    let dstBuffer: *mut libc::c_void = ress.dstBuffer;
    let dstBufferSize: size_t = ress.dstBufferSize;
    let blockSize: size_t = (*io_prefs).blockSize;
    let mut readSize: size_t = 0;
    let mut ctx: LZ4F_compressionContext_t = ress.ctx;
    let mut prefs: LZ4F_preferences_t =
        LZ4F_preferences_t{frameInfo:
                               LZ4F_frameInfo_t{blockSizeID: LZ4F_default,
                                                blockMode: LZ4F_blockLinked,
                                                contentChecksumFlag:
                                                    LZ4F_noContentChecksum,
                                                frameType: LZ4F_frame,
                                                contentSize: 0,
                                                dictID: 0,
                                                blockChecksumFlag:
                                                    LZ4F_noBlockChecksum,},
                           compressionLevel: 0,
                           autoFlush: 0,
                           favorDecSpeed: 0,
                           reserved: [0; 3],};
    /* Init */
    srcFile = LZ4IO_openSrcFile(srcFileName);
    if srcFile.is_null() { return 1 as libc::c_int }
    dstFile = LZ4IO_openDstFile(io_prefs, dstFileName);
    if dstFile.is_null() { fclose(srcFile); return 1 as libc::c_int }
    memset(&mut prefs as *mut LZ4F_preferences_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<LZ4F_preferences_t>() as libc::c_ulong);
    /* Set compression parameters */
    prefs.autoFlush =
        1 as libc::c_int as libc::c_uint; /* == 0 if input == stdin */
    prefs.compressionLevel = compressionLevel;
    prefs.frameInfo.blockMode =
        (*io_prefs).blockIndependence as LZ4F_blockMode_t;
    prefs.frameInfo.blockSizeID =
        (*io_prefs).blockSizeId as LZ4F_blockSizeID_t;
    prefs.frameInfo.blockChecksumFlag =
        (*io_prefs).blockChecksum as LZ4F_blockChecksum_t;
    prefs.frameInfo.contentChecksumFlag =
        (*io_prefs).streamChecksum as LZ4F_contentChecksum_t;
    prefs.favorDecSpeed = (*io_prefs).favorDecSpeed;
    if (*io_prefs).contentSizeFlag != 0 {
        let fileSize: U64 = UTIL_getFileSize(srcFileName);
        prefs.frameInfo.contentSize = fileSize as libc::c_ulonglong;
        if fileSize == 0 as libc::c_int as libc::c_ulong {
            if g_displayLevel >= 3 as libc::c_int {
                fprintf(stderr,
                        b"Warning : cannot determine input content size \n\x00"
                            as *const u8 as *const libc::c_char);
            }
        }
    }
    /* read first block */
    readSize =
        fread(srcBuffer, 1 as libc::c_int as size_t, blockSize, srcFile);
    if ferror(srcFile) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    30 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error reading %s \x00" as *const u8 as
                        *const libc::c_char, srcFileName);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(30 as libc::c_int);
    }
    filesize = filesize.wrapping_add(readSize as libc::c_ulonglong);
    /* single-block file */
    if readSize < blockSize {
        /* Compress in single pass */
        let mut cSize: size_t =
            LZ4F_compressFrame_usingCDict(ctx, dstBuffer, dstBufferSize,
                                          srcBuffer, readSize, ress.cdict,
                                          &mut prefs);
        if LZ4F_isError(cSize) != 0 {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 31 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Compression failed : %s\x00" as *const u8 as
                            *const libc::c_char, LZ4F_getErrorName(cSize));
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(31 as libc::c_int);
        }
        compressedfilesize = cSize as libc::c_ulonglong;
        if g_displayLevel >= 2 as libc::c_int {
            if clock() - g_time > refreshRate ||
                   g_displayLevel >= 4 as libc::c_int {
                g_time = clock();
                fprintf(stderr,
                        b"\rRead : %u MB   ==> %.2f%%   \x00" as *const u8 as
                            *const libc::c_char,
                        (filesize >> 20 as libc::c_int) as libc::c_uint,
                        compressedfilesize as libc::c_double /
                            filesize.wrapping_add((filesize == 0) as
                                                      libc::c_int as
                                                      libc::c_ulonglong) as
                                libc::c_double *
                            100 as libc::c_int as libc::c_double);
                if g_displayLevel >= 4 as libc::c_int { fflush(stderr); }
            }
        }
        /* avoid division by zero */
        /* Write Block */
        let sizeCheck: size_t =
            fwrite(dstBuffer, 1 as libc::c_int as libc::c_ulong, cSize,
                   dstFile);
        if sizeCheck != cSize {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 32 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Write error : cannot write compressed block\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(32 as libc::c_int);
        }
    } else {
        /* multiple-blocks file */
        /* Write Archive Header */
        let mut headerSize: size_t =
            LZ4F_compressBegin_usingCDict(ctx, dstBuffer, dstBufferSize,
                                          ress.cdict, &mut prefs);
        if LZ4F_isError(headerSize) != 0 {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 33 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"File header generation failed : %s\x00" as *const u8
                            as *const libc::c_char,
                        LZ4F_getErrorName(headerSize));
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(33 as libc::c_int);
        }
        let sizeCheck_0: size_t =
            fwrite(dstBuffer, 1 as libc::c_int as libc::c_ulong, headerSize,
                   dstFile);
        if sizeCheck_0 != headerSize {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 34 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Write error : cannot write header\x00" as *const u8
                            as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(34 as libc::c_int);
        }
        compressedfilesize =
            compressedfilesize.wrapping_add(headerSize as libc::c_ulonglong);
        while readSize > 0 as libc::c_int as libc::c_ulong
              /* Main Loop */
              {
            let mut outSize: size_t = 0;
            /* Compress Block */
            outSize =
                LZ4F_compressUpdate(ctx, dstBuffer, dstBufferSize, srcBuffer,
                                    readSize,
                                    0 as *const LZ4F_compressOptions_t);
            if LZ4F_isError(outSize) != 0 {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 35 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Compression failed : %s\x00" as *const u8 as
                                *const libc::c_char,
                            LZ4F_getErrorName(outSize));
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(35 as libc::c_int);
            }
            compressedfilesize =
                compressedfilesize.wrapping_add(outSize as libc::c_ulonglong);
            if g_displayLevel >= 2 as libc::c_int {
                if clock() - g_time > refreshRate ||
                       g_displayLevel >= 4 as libc::c_int {
                    g_time = clock();
                    fprintf(stderr,
                            b"\rRead : %u MB   ==> %.2f%%   \x00" as *const u8
                                as *const libc::c_char,
                            (filesize >> 20 as libc::c_int) as libc::c_uint,
                            compressedfilesize as libc::c_double /
                                filesize as libc::c_double *
                                100 as libc::c_int as libc::c_double);
                    if g_displayLevel >= 4 as libc::c_int { fflush(stderr); }
                }
            }
            /* Write Block */
            let sizeCheck_1: size_t =
                fwrite(dstBuffer, 1 as libc::c_int as libc::c_ulong, outSize,
                       dstFile);
            if sizeCheck_1 != outSize {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 36 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Write error : cannot write compressed block\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(36 as libc::c_int);
            }
            /* Read next block */
            readSize =
                fread(srcBuffer, 1 as libc::c_int as size_t, blockSize,
                      srcFile);
            filesize = filesize.wrapping_add(readSize as libc::c_ulonglong)
        }
        if ferror(srcFile) != 0 {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 37 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error reading %s \x00" as *const u8 as
                            *const libc::c_char, srcFileName);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(37 as libc::c_int);
        }
        headerSize =
            LZ4F_compressEnd(ctx, dstBuffer, dstBufferSize,
                             0 as *const LZ4F_compressOptions_t);
        if LZ4F_isError(headerSize) != 0 {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 38 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"End of file generation failed : %s\x00" as *const u8
                            as *const libc::c_char,
                        LZ4F_getErrorName(headerSize));
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(38 as libc::c_int);
        }
        let sizeCheck_2: size_t =
            fwrite(dstBuffer, 1 as libc::c_int as libc::c_ulong, headerSize,
                   dstFile);
        if sizeCheck_2 != headerSize {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 39 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Write error : cannot write end of stream\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(39 as libc::c_int);
        }
        compressedfilesize =
            compressedfilesize.wrapping_add(headerSize as libc::c_ulonglong)
    }
    /* End of Stream mark */
    /* Release file handlers */
    fclose(srcFile); /* do not close stdout */
    if strcmp(dstFileName, stdoutmark.as_ptr()) != 0 { fclose(dstFile); }
    /* Copy owner, file permissions and modification time */
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
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 &&
           strcmp(dstFileName, stdoutmark.as_ptr()) != 0 &&
           strcmp(dstFileName, nulmark.as_ptr()) != 0 &&
           UTIL_getFileStat(srcFileName, &mut statbuf) != 0 {
        UTIL_setFileStat(dstFileName, &mut statbuf);
    }
    if (*io_prefs).removeSrcFile != 0 {
        /* remove source file : --rm */
        if remove(srcFileName) != 0 {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 40 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Remove error : %s: %s\x00" as *const u8 as
                            *const libc::c_char, srcFileName,
                        strerror(*__errno_location()));
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(40 as libc::c_int);
        }
    }
    /* Final Status */
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(stderr,
                b"Compressed %llu bytes into %llu bytes ==> %.2f%%\n\x00" as
                    *const u8 as *const libc::c_char, filesize,
                compressedfilesize,
                compressedfilesize as libc::c_double /
                    filesize.wrapping_add((filesize == 0) as libc::c_int as
                                              libc::c_ulonglong) as
                        libc::c_double *
                    100 as libc::c_int as libc::c_double);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_compressFilename(prefs: *mut LZ4IO_prefs_t,
                                                mut srcFileName:
                                                    *const libc::c_char,
                                                mut dstFileName:
                                                    *const libc::c_char,
                                                mut compressionLevel:
                                                    libc::c_int)
 -> libc::c_int {
    let timeStart: UTIL_time_t = UTIL_getTime();
    let cpuStart: clock_t = clock();
    let ress: cRess_t = LZ4IO_createCResources(prefs);
    let result: libc::c_int =
        LZ4IO_compressFilename_extRess(prefs, ress, srcFileName, dstFileName,
                                       compressionLevel);
    /* Free resources */
    LZ4IO_freeCResources(ress);
    /* Final Status */
    let cpuEnd: clock_t = clock(); /* not enough memory */
    let cpuLoad_s: libc::c_double =
        (cpuEnd - cpuStart) as libc::c_double /
            1000000 as libc::c_int as __clock_t as libc::c_double;
    let timeLength_ns: U64 = UTIL_clockSpanNano(timeStart);
    let timeLength_s: libc::c_double =
        timeLength_ns as libc::c_double /
            1000000000 as libc::c_int as libc::c_double;
    if g_displayLevel >= 4 as libc::c_int {
        fprintf(stderr,
                b"Completed in %.2f sec  (cpu load : %.0f%%)\n\x00" as
                    *const u8 as *const libc::c_char, timeLength_s,
                cpuLoad_s / timeLength_s *
                    100 as libc::c_int as libc::c_double);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_compressMultipleFilenames(prefs:
                                                             *mut LZ4IO_prefs_t,
                                                         mut inFileNamesTable:
                                                             *mut *const libc::c_char,
                                                         mut ifntSize:
                                                             libc::c_int,
                                                         mut suffix:
                                                             *const libc::c_char,
                                                         mut compressionLevel:
                                                             libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut missed_files: libc::c_int = 0 as libc::c_int;
    let mut dstFileName: *mut libc::c_char =
        malloc(30 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    let mut ofnSize: size_t = 30 as libc::c_int as size_t;
    let suffixSize: size_t = strlen(suffix);
    let mut ress: cRess_t =
        cRess_t{srcBuffer: 0 as *mut libc::c_void,
                srcBufferSize: 0,
                dstBuffer: 0 as *mut libc::c_void,
                dstBufferSize: 0,
                ctx: 0 as *mut LZ4F_cctx,
                cdict: 0 as *mut LZ4F_CDict,};
    if dstFileName.is_null() { return ifntSize }
    ress = LZ4IO_createCResources(prefs);
    /* loop on each file */
    i = 0 as libc::c_int;
    while i < ifntSize {
        let ifnSize: size_t = strlen(*inFileNamesTable.offset(i as isize));
        if strcmp(suffix, stdoutmark.as_ptr()) == 0 {
            missed_files +=
                LZ4IO_compressFilename_extRess(prefs, ress,
                                               *inFileNamesTable.offset(i as
                                                                            isize),
                                               stdoutmark.as_ptr(),
                                               compressionLevel)
        } else {
            if ofnSize <=
                   ifnSize.wrapping_add(suffixSize).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
               {
                free(dstFileName as *mut libc::c_void);
                ofnSize =
                    ifnSize.wrapping_add(20 as libc::c_int as libc::c_ulong);
                dstFileName = malloc(ofnSize) as *mut libc::c_char;
                if dstFileName.is_null() {
                    LZ4IO_freeCResources(ress);
                    return ifntSize
                }
            }
            strcpy(dstFileName, *inFileNamesTable.offset(i as isize));
            strcat(dstFileName, suffix);
            missed_files +=
                LZ4IO_compressFilename_extRess(prefs, ress,
                                               *inFileNamesTable.offset(i as
                                                                            isize),
                                               dstFileName, compressionLevel)
        }
        i += 1
    }
    /* Close & Free */
    LZ4IO_freeCResources(ress);
    free(dstFileName as *mut libc::c_void);
    return missed_files;
}
/* ********************************************************************* */
/* ********************** LZ4 file-stream Decompression **************** */
/* ********************************************************************* */
/* It's presumed that s points to a memory space of size >= 4 */
unsafe extern "C" fn LZ4IO_readLE32(mut s: *const libc::c_void)
 -> libc::c_uint {
    let srcPtr: *const libc::c_uchar =
        s as
            *const libc::c_uchar; /* Buffer is supposed malloc'ed, hence aligned on size_t */
    let mut value32: libc::c_uint =
        *srcPtr.offset(0 as libc::c_int as isize) as libc::c_uint;
    value32 =
        value32.wrapping_add((*srcPtr.offset(1 as libc::c_int as isize) as
                                  libc::c_uint) << 8 as libc::c_int);
    value32 =
        value32.wrapping_add((*srcPtr.offset(2 as libc::c_int as isize) as
                                  libc::c_uint) << 16 as libc::c_int);
    value32 =
        value32.wrapping_add((*srcPtr.offset(3 as libc::c_int as isize) as
                                  libc::c_uint) << 24 as libc::c_int);
    return value32;
}
unsafe extern "C" fn LZ4IO_fwriteSparse(prefs: *mut LZ4IO_prefs_t,
                                        mut file: *mut FILE,
                                        mut buffer: *const libc::c_void,
                                        mut bufferSize: size_t,
                                        mut storedSkips: libc::c_uint)
 -> libc::c_uint {
    let sizeT: size_t = ::std::mem::size_of::<size_t>() as libc::c_ulong;
    let maskT: size_t = sizeT.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let bufferT: *const size_t = buffer as *const size_t;
    let mut ptrT: *const size_t = bufferT;
    let mut bufferSizeT: size_t = bufferSize.wrapping_div(sizeT);
    let bufferTEnd: *const size_t = bufferT.offset(bufferSizeT as isize);
    let segmentSizeT: size_t =
        ((32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
             libc::c_ulong).wrapping_div(sizeT);
    if (*prefs).sparseFileSupport == 0 {
        /* normal write */
        let sizeCheck: size_t =
            fwrite(buffer, 1 as libc::c_int as libc::c_ulong, bufferSize,
                   file);
        if sizeCheck != bufferSize {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 70 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Write error : cannot write decoded block\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(70 as libc::c_int);
        }
        return 0 as libc::c_int as libc::c_uint
    }
    /* avoid int overflow */
    if storedSkips >
           (1 as libc::c_int as
                libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                               30 as libc::c_int) {
        let seekResult: libc::c_int =
            fseek(file,
                  (1 as libc::c_int as
                       libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                                      30 as libc::c_int) as
                      libc::c_long, 1 as libc::c_int);
        if seekResult != 0 as libc::c_int {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 71 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"1 GB skip error (sparse file support)\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(71 as libc::c_int);
        }
        storedSkips =
            storedSkips.wrapping_sub((1 as libc::c_int as
                                          libc::c_uint).wrapping_mul((1 as
                                                                          libc::c_uint)
                                                                         <<
                                                                         30 as
                                                                             libc::c_int))
    }
    while ptrT < bufferTEnd {
        let mut seg0SizeT: size_t = segmentSizeT;
        let mut nb0T: size_t = 0;
        /* count leading zeros */
        if seg0SizeT > bufferSizeT { seg0SizeT = bufferSizeT }
        bufferSizeT =
            (bufferSizeT as libc::c_ulong).wrapping_sub(seg0SizeT) as size_t
                as size_t;
        nb0T = 0 as libc::c_int as size_t;
        while nb0T < seg0SizeT &&
                  *ptrT.offset(nb0T as isize) ==
                      0 as libc::c_int as libc::c_ulong {
            nb0T = nb0T.wrapping_add(1)
        }
        storedSkips =
            storedSkips.wrapping_add(nb0T.wrapping_mul(sizeT) as
                                         libc::c_uint);
        if nb0T != seg0SizeT {
            /* not all 0s */
            *__errno_location() = 0 as libc::c_int;
            let seekResult_0: libc::c_int =
                fseek(file, storedSkips as libc::c_long, 1 as libc::c_int);
            if seekResult_0 != 0 {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 72 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Sparse skip error(%d): %s ; try --no-sparse\x00"
                                as *const u8 as *const libc::c_char,
                            *__errno_location(),
                            strerror(*__errno_location()));
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(72 as libc::c_int);
            }
            storedSkips = 0 as libc::c_int as libc::c_uint;
            seg0SizeT =
                (seg0SizeT as libc::c_ulong).wrapping_sub(nb0T) as size_t as
                    size_t;
            ptrT = ptrT.offset(nb0T as isize);
            let sizeCheck_0: size_t =
                fwrite(ptrT as *const libc::c_void, sizeT, seg0SizeT, file);
            if sizeCheck_0 != seg0SizeT {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 73 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Write error : cannot write decoded block\x00" as
                                *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(73 as libc::c_int);
            }
        }
        ptrT = ptrT.offset(seg0SizeT as isize)
    }
    if bufferSize & maskT != 0 {
        /* size not multiple of sizeT : implies end of block */
        let restStart: *const libc::c_char =
            bufferTEnd as *const libc::c_char;
        let mut restPtr: *const libc::c_char = restStart;
        let restSize: size_t = bufferSize & maskT;
        let restEnd: *const libc::c_char =
            restStart.offset(restSize as isize);
        while restPtr < restEnd && *restPtr as libc::c_int == 0 as libc::c_int
              {
            restPtr = restPtr.offset(1)
        }
        storedSkips =
            storedSkips.wrapping_add(restPtr.wrapping_offset_from(restStart)
                                         as libc::c_long as libc::c_uint);
        if restPtr != restEnd {
            let seekResult_1: libc::c_int =
                fseek(file, storedSkips as libc::c_long, 1 as libc::c_int);
            if seekResult_1 != 0 {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 74 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Sparse skip error ; try --no-sparse\x00" as
                                *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(74 as libc::c_int);
            }
            storedSkips = 0 as libc::c_int as libc::c_uint;
            let sizeCheck_1: size_t =
                fwrite(restPtr as *const libc::c_void,
                       1 as libc::c_int as libc::c_ulong,
                       restEnd.wrapping_offset_from(restPtr) as libc::c_long
                           as libc::c_ulong, file);
            if sizeCheck_1 !=
                   restEnd.wrapping_offset_from(restPtr) as libc::c_long as
                       size_t {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 75 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Write error : cannot write decoded end of block\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(75 as libc::c_int);
            }
        }
    }
    return storedSkips;
}
unsafe extern "C" fn LZ4IO_fwriteSparseEnd(mut file: *mut FILE,
                                           mut storedSkips: libc::c_uint) {
    if storedSkips > 0 as libc::c_int as libc::c_uint {
        /* implies g_sparseFileSupport>0 */
        let seekResult: libc::c_int =
            fseek(file,
                  storedSkips.wrapping_sub(1 as libc::c_int as libc::c_uint)
                      as libc::c_long, 1 as libc::c_int);
        if seekResult != 0 as libc::c_int {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 69 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Final skip error (sparse file)\n\x00" as *const u8
                            as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(69 as libc::c_int);
        }
        let lastZeroByte: [libc::c_char; 1] =
            [0 as libc::c_int as libc::c_char];
        let sizeCheck: size_t =
            fwrite(lastZeroByte.as_ptr() as *const libc::c_void,
                   1 as libc::c_int as libc::c_ulong,
                   1 as libc::c_int as libc::c_ulong, file);
        if sizeCheck != 1 as libc::c_int as libc::c_ulong {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 69 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Write error : cannot write last zero\n\x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(69 as libc::c_int);
        }
    };
}
static mut g_magicRead: libc::c_uint = 0 as libc::c_int as libc::c_uint;
/* out-parameter of LZ4IO_decodeLegacyStream() */
unsafe extern "C" fn LZ4IO_decodeLegacyStream(prefs: *mut LZ4IO_prefs_t,
                                              mut finput: *mut FILE,
                                              mut foutput: *mut FILE)
 -> libc::c_ulonglong {
    let mut streamSize: libc::c_ulonglong =
        0 as libc::c_int as libc::c_ulonglong;
    let mut storedSkips: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    /* Allocate Memory */
    let in_buff: *mut libc::c_char =
        malloc(LZ4_compressBound(8 as libc::c_int *
                                     ((1 as libc::c_int) <<
                                          20 as libc::c_int)) as size_t) as
            *mut libc::c_char;
    let out_buff: *mut libc::c_char =
        malloc((8 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int))
                   as libc::c_ulong) as *mut libc::c_char;
    if in_buff.is_null() || out_buff.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    51 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(51 as libc::c_int);
    }
    loop 
         /* Main Loop */
         {
        let mut blockSize: libc::c_uint = 0;
        /* Block Size */
        let sizeCheck: size_t =
            fread(in_buff as *mut libc::c_void,
                  1 as libc::c_int as libc::c_ulong,
                  4 as libc::c_int as libc::c_ulong,
                  finput); /* Nothing to read : file read is completed */
        if sizeCheck == 0 as libc::c_int as libc::c_ulong {
            break ; /* Convert to Little Endian */
        }
        if sizeCheck != 4 as libc::c_int as libc::c_ulong {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 52 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Read error : cannot access block size \x00" as
                            *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(52 as libc::c_int);
        }
        blockSize = LZ4IO_readLE32(in_buff as *const libc::c_void);
        if blockSize >
               (if (8 as libc::c_int *
                        ((1 as libc::c_int) << 20 as libc::c_int)) as
                       libc::c_uint >
                       0x7e000000 as libc::c_int as libc::c_uint {
                    0 as libc::c_int
                } else {
                    (8 as libc::c_int *
                         ((1 as libc::c_int) << 20 as libc::c_int) +
                         8 as libc::c_int *
                             ((1 as libc::c_int) << 20 as libc::c_int) /
                             255 as libc::c_int) + 16 as libc::c_int
                }) as libc::c_uint {
            /* Cannot read next block : maybe new stream ? */
            g_magicRead = blockSize;
            break ;
        } else {
            /* Read Block */
            let sizeCheck_0: size_t =
                fread(in_buff as *mut libc::c_void,
                      1 as libc::c_int as libc::c_ulong,
                      blockSize as libc::c_ulong, finput);
            if sizeCheck_0 != blockSize as libc::c_ulong {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 52 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Read error : cannot access compressed block !\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(52 as libc::c_int);
            }
            /* Decode Block */
            let decodeSize: libc::c_int =
                LZ4_decompress_safe(in_buff, out_buff,
                                    blockSize as libc::c_int,
                                    8 as libc::c_int *
                                        ((1 as libc::c_int) <<
                                             20 as libc::c_int));
            if decodeSize < 0 as libc::c_int {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 53 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Decoding Failed ! Corrupted input detected !\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(53 as libc::c_int);
            }
            streamSize =
                streamSize.wrapping_add(decodeSize as libc::c_ulonglong);
            /* success or die */
            storedSkips =
                LZ4IO_fwriteSparse(prefs, foutput,
                                   out_buff as *const libc::c_void,
                                   decodeSize as size_t, storedSkips)
        }
    }
    if ferror(finput) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    54 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Read error : ferror\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(54 as libc::c_int);
    }
    LZ4IO_fwriteSparseEnd(foutput, storedSkips);
    /* Write Block */
    /* Free */
    free(in_buff as *mut libc::c_void);
    free(out_buff as *mut libc::c_void);
    return streamSize;
}
unsafe extern "C" fn LZ4IO_loadDDict(prefs: *mut LZ4IO_prefs_t,
                                     mut ress: *mut dRess_t) {
    if (*prefs).useDictionary == 0 {
        (*ress).dictBuffer = 0 as *mut libc::c_void;
        (*ress).dictBufferSize = 0 as libc::c_int as size_t;
        return
    }
    (*ress).dictBuffer = LZ4IO_createDict(prefs, &mut (*ress).dictBufferSize);
    if (*ress).dictBuffer.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    25 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Dictionary error : could not create dictionary\x00" as
                        *const u8 as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(25 as libc::c_int);
    };
}
static mut LZ4IO_dBufferSize: size_t =
    (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as size_t;
unsafe extern "C" fn LZ4IO_createDResources(prefs: *mut LZ4IO_prefs_t)
 -> dRess_t {
    let mut ress: dRess_t =
        dRess_t{srcBuffer: 0 as *mut libc::c_void,
                srcBufferSize: 0,
                dstBuffer: 0 as *mut libc::c_void,
                dstBufferSize: 0,
                dstFile: 0 as *mut FILE,
                dCtx: 0 as *mut LZ4F_dctx,
                dictBuffer: 0 as *mut libc::c_void,
                dictBufferSize: 0,};
    /* init */
    let errorCode: LZ4F_errorCode_t =
        LZ4F_createDecompressionContext(&mut ress.dCtx,
                                        100 as libc::c_int as libc::c_uint);
    if LZ4F_isError(errorCode) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    60 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Can\'t create LZ4F context : %s\x00" as *const u8 as
                        *const libc::c_char, LZ4F_getErrorName(errorCode));
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(60 as libc::c_int);
    }
    /* Allocate Memory */
    ress.srcBufferSize = LZ4IO_dBufferSize;
    ress.srcBuffer = malloc(ress.srcBufferSize);
    ress.dstBufferSize = LZ4IO_dBufferSize;
    ress.dstBuffer = malloc(ress.dstBufferSize);
    if ress.srcBuffer.is_null() || ress.dstBuffer.is_null() {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    61 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Allocation error : not enough memory\x00" as *const u8
                        as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(61 as libc::c_int);
    }
    LZ4IO_loadDDict(prefs, &mut ress);
    ress.dstFile = 0 as *mut FILE;
    return ress;
}
unsafe extern "C" fn LZ4IO_freeDResources(mut ress: dRess_t) {
    let mut errorCode: LZ4F_errorCode_t =
        LZ4F_freeDecompressionContext(ress.dCtx);
    if LZ4F_isError(errorCode) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    69 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error : can\'t free LZ4F context resource : %s\x00" as
                        *const u8 as *const libc::c_char,
                    LZ4F_getErrorName(errorCode));
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(69 as libc::c_int);
    }
    free(ress.srcBuffer);
    free(ress.dstBuffer);
    free(ress.dictBuffer);
}
unsafe extern "C" fn LZ4IO_decompressLZ4F(prefs: *mut LZ4IO_prefs_t,
                                          mut ress: dRess_t,
                                          mut srcFile: *mut FILE,
                                          mut dstFile: *mut FILE)
 -> libc::c_ulonglong {
    let mut filesize: libc::c_ulonglong =
        0 as libc::c_int as libc::c_ulonglong;
    let mut nextToLoad: LZ4F_errorCode_t = 0;
    let mut storedSkips: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    /* Init feed with magic number (already consumed from FILE* sFile) */
    let mut inSize: size_t = 4 as libc::c_int as size_t;
    let mut outSize: size_t = 0 as libc::c_int as size_t;
    LZ4IO_writeLE32(ress.srcBuffer,
                    0x184d2204 as libc::c_int as libc::c_uint);
    nextToLoad =
        LZ4F_decompress_usingDict(ress.dCtx, ress.dstBuffer, &mut outSize,
                                  ress.srcBuffer, &mut inSize,
                                  ress.dictBuffer, ress.dictBufferSize,
                                  0 as *const LZ4F_decompressOptions_t);
    if LZ4F_isError(nextToLoad) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    62 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Header error : %s\x00" as *const u8 as
                        *const libc::c_char, LZ4F_getErrorName(nextToLoad));
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(62 as libc::c_int);
    }
    /* Main Loop */
    while nextToLoad != 0 {
        let mut readSize: size_t = 0;
        let mut pos: size_t = 0 as libc::c_int as size_t;
        let mut decodedBytes: size_t = ress.dstBufferSize;
        /* Read input */
        if nextToLoad > ress.srcBufferSize {
            nextToLoad = ress.srcBufferSize
        } /* reached end of file or stream */
        readSize =
            fread(ress.srcBuffer, 1 as libc::c_int as libc::c_ulong,
                  nextToLoad, srcFile);
        if readSize == 0 { break ; }
        while pos < readSize || decodedBytes == ress.dstBufferSize {
            /* still to read, or still to flush */
            /* Decode Input (at least partially) */
            let mut remaining: size_t = readSize.wrapping_sub(pos);
            decodedBytes = ress.dstBufferSize;
            nextToLoad =
                LZ4F_decompress_usingDict(ress.dCtx, ress.dstBuffer,
                                          &mut decodedBytes,
                                          (ress.srcBuffer as
                                               *mut libc::c_char).offset(pos
                                                                             as
                                                                             isize)
                                              as *const libc::c_void,
                                          &mut remaining, ress.dictBuffer,
                                          ress.dictBufferSize,
                                          0 as
                                              *const LZ4F_decompressOptions_t);
            if LZ4F_isError(nextToLoad) != 0 {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 66 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Decompression error : %s\x00" as *const u8 as
                                *const libc::c_char,
                            LZ4F_getErrorName(nextToLoad));
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(66 as libc::c_int);
            }
            pos =
                (pos as libc::c_ulong).wrapping_add(remaining) as size_t as
                    size_t;
            /* Write Block */
            if decodedBytes != 0 {
                if (*prefs).testMode == 0 {
                    storedSkips =
                        LZ4IO_fwriteSparse(prefs, dstFile, ress.dstBuffer,
                                           decodedBytes, storedSkips)
                }
                filesize =
                    filesize.wrapping_add(decodedBytes as libc::c_ulonglong);
                if g_displayLevel >= 2 as libc::c_int {
                    if clock() - g_time > refreshRate ||
                           g_displayLevel >= 4 as libc::c_int {
                        g_time = clock();
                        fprintf(stderr,
                                b"\rDecompressed : %u MB  \x00" as *const u8
                                    as *const libc::c_char,
                                (filesize >> 20 as libc::c_int) as
                                    libc::c_uint);
                        if g_displayLevel >= 4 as libc::c_int {
                            fflush(stderr);
                        }
                    }
                }
            }
            if nextToLoad == 0 { break ; }
        }
    }
    /* can be out because readSize == 0, which could be an fread() error */
    if ferror(srcFile) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    67 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Read error\x00" as *const u8 as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(67 as libc::c_int);
    }
    if (*prefs).testMode == 0 { LZ4IO_fwriteSparseEnd(dstFile, storedSkips); }
    if nextToLoad != 0 as libc::c_int as libc::c_ulong {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    68 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Unfinished stream\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(68 as libc::c_int);
    }
    return filesize;
}
unsafe extern "C" fn LZ4IO_passThrough(prefs: *mut LZ4IO_prefs_t,
                                       mut finput: *mut FILE,
                                       mut foutput: *mut FILE,
                                       mut MNstore: *mut libc::c_uchar)
 -> libc::c_ulonglong {
    let mut buffer: [size_t; 8192] = [0; 8192];
    let mut readBytes: size_t = 1 as libc::c_int as size_t;
    let mut total: libc::c_ulonglong = 4 as libc::c_int as libc::c_ulonglong;
    let mut storedSkips: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let sizeCheck: size_t =
        fwrite(MNstore as *const libc::c_void,
               1 as libc::c_int as libc::c_ulong,
               4 as libc::c_int as libc::c_ulong, foutput);
    if sizeCheck != 4 as libc::c_int as libc::c_ulong {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    50 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Pass-through write error\x00" as *const u8 as
                        *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(50 as libc::c_int);
    }
    while readBytes != 0 {
        readBytes =
            fread(buffer.as_mut_ptr() as *mut libc::c_void,
                  1 as libc::c_int as libc::c_ulong,
                  (64 as libc::c_int *
                       ((1 as libc::c_int) << 10 as libc::c_int)) as
                      libc::c_ulong, finput);
        total = total.wrapping_add(readBytes as libc::c_ulonglong);
        storedSkips =
            LZ4IO_fwriteSparse(prefs, foutput,
                               buffer.as_mut_ptr() as *const libc::c_void,
                               readBytes, storedSkips)
    }
    if ferror(finput) != 0 {
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Error %i : \x00" as *const u8 as *const libc::c_char,
                    51 as libc::c_int);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr,
                    b"Read Error\x00" as *const u8 as *const libc::c_char);
        }
        if g_displayLevel >= 1 as libc::c_int {
            fprintf(stderr, b" \n\x00" as *const u8 as *const libc::c_char);
        }
        exit(51 as libc::c_int);
    }
    LZ4IO_fwriteSparseEnd(foutput, storedSkips);
    return total;
}
/* * Safely handle cases when (unsigned)offset > LONG_MAX */
unsafe extern "C" fn fseek_u32(mut fp: *mut FILE, mut offset: libc::c_uint,
                               mut where_0: libc::c_int) -> libc::c_int {
    let stepMax: libc::c_uint =
        (1 as libc::c_uint) << 30 as libc::c_int; /* Only allows SEEK_CUR */
    let mut errorNb: libc::c_int = 0 as libc::c_int;
    if where_0 != 1 as libc::c_int { return -(1 as libc::c_int) }
    while offset > 0 as libc::c_int as libc::c_uint {
        let mut s: libc::c_uint = offset;
        if s > stepMax { s = stepMax }
        errorNb = fseek(fp, s as libc::c_long, 1 as libc::c_int);
        if errorNb != 0 as libc::c_int { break ; }
        offset = offset.wrapping_sub(s)
    }
    return errorNb;
}
unsafe extern "C" fn selectDecoder(prefs: *mut LZ4IO_prefs_t,
                                   mut ress: dRess_t, mut finput: *mut FILE,
                                   mut foutput: *mut FILE)
 -> libc::c_ulonglong {
    let mut MNstore: [libc::c_uchar; 4] = [0; 4];
    let mut magicNumber: libc::c_uint = 0;
    static mut nbFrames: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    /* init */
    nbFrames = nbFrames.wrapping_add(1);
    /* Check Archive Header */
    if g_magicRead != 0 {
        /* magic number already read from finput (see legacy frame)*/
        magicNumber = g_magicRead;
        g_magicRead = 0 as libc::c_int as libc::c_uint
    } else {
        let nbReadBytes: size_t =
            fread(MNstore.as_mut_ptr() as *mut libc::c_void,
                  1 as libc::c_int as libc::c_ulong,
                  4 as libc::c_int as libc::c_ulong, finput);
        /* Little Endian format */
        if nbReadBytes == 0 as libc::c_int as libc::c_ulong {
            nbFrames = 0 as libc::c_int as libc::c_uint; /* EOF */
            return -(1 as libc::c_int) as libc::c_ulonglong
        } /* fold skippable magic numbers */
        if nbReadBytes != 4 as libc::c_int as libc::c_ulong {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 40 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Unrecognized header : Magic Number unreadable\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(40 as libc::c_int);
        }
        magicNumber =
            LZ4IO_readLE32(MNstore.as_mut_ptr() as *const libc::c_void)
    }
    if LZ4IO_isSkippableMagicNumber(magicNumber) != 0 {
        magicNumber = 0x184d2a50 as libc::c_int as libc::c_uint
    }
    match magicNumber {
        407708164 => {
            return LZ4IO_decompressLZ4F(prefs, ress, finput, foutput)
        }
        407642370 => {
            if g_displayLevel >= 4 as libc::c_int {
                fprintf(stderr,
                        b"Detected : Legacy format \n\x00" as *const u8 as
                            *const libc::c_char);
            }
            return LZ4IO_decodeLegacyStream(prefs, finput, foutput)
        }
        407710288 => {
            if g_displayLevel >= 4 as libc::c_int {
                fprintf(stderr,
                        b"Skipping detected skippable area \n\x00" as
                            *const u8 as *const libc::c_char);
            }
            let nbReadBytes_0: size_t =
                fread(MNstore.as_mut_ptr() as *mut libc::c_void,
                      1 as libc::c_int as libc::c_ulong,
                      4 as libc::c_int as libc::c_ulong, finput);
            if nbReadBytes_0 != 4 as libc::c_int as libc::c_ulong {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 42 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Stream error : skippable size unreadable\x00" as
                                *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(42 as libc::c_int);
            }
            let size: libc::c_uint =
                LZ4IO_readLE32(MNstore.as_mut_ptr() as *const libc::c_void);
            let errorNb: libc::c_int =
                fseek_u32(finput, size, 1 as libc::c_int);
            if errorNb != 0 as libc::c_int {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 43 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Stream error : cannot skip skippable area\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(43 as libc::c_int);
            }
            return 0 as libc::c_int as libc::c_ulonglong
        }
        _ => {
            /* macro extension for custom formats */
            if nbFrames == 1 as libc::c_int as libc::c_uint {
                /* just started */
                /* Wrong magic number at the beginning of 1st stream */
                if (*prefs).testMode == 0 && (*prefs).overwrite != 0 &&
                       (*prefs).passThrough != 0 {
                    nbFrames =
                        0 as libc::c_int as
                            libc::c_uint; /* only works for files < 2 GB */
                    return LZ4IO_passThrough(prefs, finput, foutput,
                                             MNstore.as_mut_ptr())
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Error %i : \x00" as *const u8 as
                                *const libc::c_char, 44 as libc::c_int);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"Unrecognized header : file cannot be decoded\x00"
                                as *const u8 as *const libc::c_char);
                }
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b" \n\x00" as *const u8 as *const libc::c_char);
                }
                exit(44 as libc::c_int);
            }
            let position: libc::c_long = ftell(finput);
            if g_displayLevel >= 2 as libc::c_int {
                fprintf(stderr,
                        b"Stream followed by undecodable data \x00" as
                            *const u8 as *const libc::c_char);
            }
            if position != -(1 as libc::c_long) {
                if g_displayLevel >= 2 as libc::c_int {
                    fprintf(stderr,
                            b"at position %i \x00" as *const u8 as
                                *const libc::c_char, position as libc::c_int);
                }
            }
            if g_displayLevel >= 2 as libc::c_int {
                fprintf(stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            return -(1 as libc::c_int) as libc::c_ulonglong
        }
    };
}
unsafe extern "C" fn LZ4IO_decompressSrcFile(prefs: *mut LZ4IO_prefs_t,
                                             mut ress: dRess_t,
                                             mut input_filename:
                                                 *const libc::c_char,
                                             mut _output_filename:
                                                 *const libc::c_char)
 -> libc::c_int {
    let foutput: *mut FILE = ress.dstFile;
    let mut filesize: libc::c_ulonglong =
        0 as libc::c_int as libc::c_ulonglong;
    /* Init */
    let finput: *mut FILE = LZ4IO_openSrcFile(input_filename);
    if finput.is_null() { return 1 as libc::c_int }
    /* Loop over multiple streams */
    loop 
         /* endless loop, see break condition */
         {
        let decodedSize: libc::c_ulonglong =
            selectDecoder(prefs, ress, finput, foutput);
        if decodedSize == -(1 as libc::c_int) as libc::c_ulonglong { break ; }
        filesize = filesize.wrapping_add(decodedSize)
    }
    /* Close input */
    fclose(finput);
    if (*prefs).removeSrcFile != 0 {
        /* --rm */
        if remove(input_filename) != 0 {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 45 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Remove error : %s: %s\x00" as *const u8 as
                            *const libc::c_char, input_filename,
                        strerror(*__errno_location()));
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(45 as libc::c_int);
        }
    }
    /* Final Status */
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(stderr, b"\r%79s\r\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char); /* failure */
    }
    if g_displayLevel >= 2 as libc::c_int {
        fprintf(stderr,
                b"%-20.20s : decoded %llu bytes \n\x00" as *const u8 as
                    *const libc::c_char, input_filename, filesize);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn LZ4IO_decompressDstFile(prefs: *mut LZ4IO_prefs_t,
                                             mut ress: dRess_t,
                                             mut input_filename:
                                                 *const libc::c_char,
                                             mut output_filename:
                                                 *const libc::c_char)
 -> libc::c_int {
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
    let mut stat_result: libc::c_int = 0 as libc::c_int;
    let foutput: *mut FILE = LZ4IO_openDstFile(prefs, output_filename);
    if foutput.is_null() { return 1 as libc::c_int }
    if strcmp(input_filename, stdinmark.as_ptr()) != 0 &&
           UTIL_getFileStat(input_filename, &mut statbuf) != 0 {
        stat_result = 1 as libc::c_int
    }
    ress.dstFile = foutput;
    LZ4IO_decompressSrcFile(prefs, ress, input_filename, output_filename);
    fclose(foutput);
    /* Copy owner, file permissions and modification time */
    if stat_result != 0 as libc::c_int &&
           strcmp(output_filename, stdoutmark.as_ptr()) != 0 &&
           strcmp(output_filename, nulmark.as_ptr()) != 0 {
        UTIL_setFileStat(output_filename, &mut statbuf);
        /* should return value be read ? or is silent fail good enough ? */
    } /* not enough memory */
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_decompressFilename(prefs: *mut LZ4IO_prefs_t,
                                                  mut input_filename:
                                                      *const libc::c_char,
                                                  mut output_filename:
                                                      *const libc::c_char)
 -> libc::c_int {
    let ress: dRess_t = LZ4IO_createDResources(prefs);
    let start: clock_t = clock();
    let missingFiles: libc::c_int =
        LZ4IO_decompressDstFile(prefs, ress, input_filename, output_filename);
    let end: clock_t = clock();
    let seconds: libc::c_double =
        (end - start) as libc::c_double /
            1000000 as libc::c_int as __clock_t as libc::c_double;
    if g_displayLevel >= 4 as libc::c_int {
        fprintf(stderr,
                b"Done in %.2f sec  \n\x00" as *const u8 as
                    *const libc::c_char, seconds);
    }
    LZ4IO_freeDResources(ress);
    return missingFiles;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_decompressMultipleFilenames(prefs:
                                                               *mut LZ4IO_prefs_t,
                                                           mut inFileNamesTable:
                                                               *mut *const libc::c_char,
                                                           mut ifntSize:
                                                               libc::c_int,
                                                           mut suffix:
                                                               *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut skippedFiles: libc::c_int = 0 as libc::c_int;
    let mut missingFiles: libc::c_int = 0 as libc::c_int;
    let mut outFileName: *mut libc::c_char =
        malloc(30 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    let mut ofnSize: size_t = 30 as libc::c_int as size_t;
    let suffixSize: size_t = strlen(suffix);
    let mut ress: dRess_t = LZ4IO_createDResources(prefs);
    if outFileName.is_null() { return ifntSize }
    ress.dstFile = LZ4IO_openDstFile(prefs, stdoutmark.as_ptr());
    i = 0 as libc::c_int;
    while i < ifntSize {
        let ifnSize: size_t = strlen(*inFileNamesTable.offset(i as isize));
        let suffixPtr: *const libc::c_char =
            (*inFileNamesTable.offset(i as
                                          isize)).offset(ifnSize as
                                                             isize).offset(-(suffixSize
                                                                                 as
                                                                                 isize));
        if strcmp(suffix, stdoutmark.as_ptr()) == 0 {
            missingFiles +=
                LZ4IO_decompressSrcFile(prefs, ress,
                                        *inFileNamesTable.offset(i as isize),
                                        stdoutmark.as_ptr())
        } else {
            if ofnSize <=
                   ifnSize.wrapping_sub(suffixSize).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
               {
                free(outFileName as *mut libc::c_void);
                ofnSize =
                    ifnSize.wrapping_add(20 as libc::c_int as libc::c_ulong);
                outFileName = malloc(ofnSize) as *mut libc::c_char;
                if outFileName.is_null() { return ifntSize }
            }
            if ifnSize <= suffixSize ||
                   strcmp(suffixPtr, suffix) != 0 as libc::c_int {
                if g_displayLevel >= 1 as libc::c_int {
                    fprintf(stderr,
                            b"File extension doesn\'t match expected LZ4_EXTENSION (%4s); will not process file: %s\n\x00"
                                as *const u8 as *const libc::c_char, suffix,
                            *inFileNamesTable.offset(i as isize));
                }
                skippedFiles += 1
            } else {
                memcpy(outFileName as *mut libc::c_void,
                       *inFileNamesTable.offset(i as isize) as
                           *const libc::c_void,
                       ifnSize.wrapping_sub(suffixSize));
                *outFileName.offset(ifnSize.wrapping_sub(suffixSize) as isize)
                    = '\u{0}' as i32 as libc::c_char;
                missingFiles +=
                    LZ4IO_decompressDstFile(prefs, ress,
                                            *inFileNamesTable.offset(i as
                                                                         isize),
                                            outFileName)
            }
        }
        i += 1
    }
    LZ4IO_freeDResources(ress);
    free(outFileName as *mut libc::c_void);
    return missingFiles + skippedFiles;
}
static mut LZ4IO_frameTypeNames: [*const libc::c_char; 3] =
    [b"LZ4Frame\x00" as *const u8 as *const libc::c_char,
     b"LegacyFrame\x00" as *const u8 as *const libc::c_char,
     b"SkippableFrame\x00" as *const u8 as *const libc::c_char];
/* Read block headers and skip block data
   Return total blocks size for this frame including block headers,
   block checksums and content checksums.
   returns 0 in case it can't succesfully skip block data.
   Assumes SEEK_CUR after frame header.
 */
unsafe extern "C" fn LZ4IO_skipBlocksData(mut finput: *mut FILE,
                                          blockChecksumFlag:
                                              LZ4F_blockChecksum_t,
                                          contentChecksumFlag:
                                              LZ4F_contentChecksum_t)
 -> libc::c_ulonglong {
    let mut blockInfo: [libc::c_uchar; 4] = [0; 4];
    let mut totalBlocksSize: libc::c_ulonglong =
        0 as libc::c_int as libc::c_ulonglong;
    loop  {
        if fread(blockInfo.as_mut_ptr() as *mut libc::c_void,
                 1 as libc::c_int as libc::c_ulong,
                 4 as libc::c_int as libc::c_ulong, finput) == 0 {
            if feof(finput) != 0 { return totalBlocksSize }
            return 0 as libc::c_int as libc::c_ulonglong
        }
        totalBlocksSize =
            totalBlocksSize.wrapping_add(4 as libc::c_int as
                                             libc::c_ulonglong);
        let nextCBlockSize: libc::c_ulong =
            (LZ4IO_readLE32(&mut blockInfo as *mut [libc::c_uchar; 4] as
                                *const libc::c_void) &
                 0x7fffffff as libc::c_uint) as libc::c_ulong;
        let nextBlock: libc::c_ulong =
            nextCBlockSize.wrapping_add((blockChecksumFlag as
                                             libc::c_uint).wrapping_mul(4 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                            as libc::c_ulong);
        if nextCBlockSize == 0 as libc::c_int as libc::c_ulong {
            /* Reached EndMark */
            if contentChecksumFlag as u64 != 0 {
                /* Skip content checksum */
                if fseek(finput, 4 as libc::c_int as libc::c_long,
                         1 as libc::c_int) != 0 as libc::c_int {
                    return 0 as libc::c_int as libc::c_ulonglong
                }
                totalBlocksSize =
                    totalBlocksSize.wrapping_add(4 as libc::c_int as
                                                     libc::c_ulonglong)
            }
            break ;
        } else {
            totalBlocksSize =
                totalBlocksSize.wrapping_add(nextBlock as libc::c_ulonglong);
            /* skip to the next block */
            if fseek(finput, nextBlock as libc::c_long, 1 as libc::c_int) !=
                   0 as libc::c_int {
                return 0 as libc::c_int as libc::c_ulonglong
            }
        }
    }
    return totalBlocksSize;
}
/* For legacy frames only.
   Read block headers and skip block data.
   Return total blocks size for this frame including block headers.
   or 0 in case it can't succesfully skip block data.
   This works as long as legacy block header size = magic number size.
   Assumes SEEK_CUR after frame header.
 */
unsafe extern "C" fn LZ4IO_skipLegacyBlocksData(mut finput: *mut FILE)
 -> libc::c_ulonglong {
    let mut blockInfo: [libc::c_uchar; 4] = [0; 4];
    let mut totalBlocksSize: libc::c_ulonglong =
        0 as libc::c_int as libc::c_ulonglong;
    loop  {
        if fread(blockInfo.as_mut_ptr() as *mut libc::c_void,
                 1 as libc::c_int as libc::c_ulong,
                 4 as libc::c_int as libc::c_ulong, finput) == 0 {
            if feof(finput) != 0 { return totalBlocksSize }
            return 0 as libc::c_int as libc::c_ulonglong
        }
        let nextCBlockSize: libc::c_uint =
            LZ4IO_readLE32(&mut blockInfo as *mut [libc::c_uchar; 4] as
                               *const libc::c_void);
        if nextCBlockSize == 0x184c2102 as libc::c_int as libc::c_uint ||
               nextCBlockSize == 0x184d2204 as libc::c_int as libc::c_uint ||
               LZ4IO_isSkippableMagicNumber(nextCBlockSize) != 0 {
            /* Rewind back. we want cursor at the begining of next frame.*/
            if fseek(finput, -(4 as libc::c_int) as libc::c_long,
                     1 as libc::c_int) != 0 as libc::c_int {
                return 0 as libc::c_int as libc::c_ulonglong
            }
            break ;
        } else {
            totalBlocksSize =
                totalBlocksSize.wrapping_add((4 as libc::c_int as
                                                  libc::c_uint).wrapping_add(nextCBlockSize)
                                                 as libc::c_ulonglong);
            /* skip to the next block */
            if fseek(finput, nextCBlockSize as libc::c_long, 1 as libc::c_int)
                   != 0 as libc::c_int {
                return 0 as libc::c_int as libc::c_ulonglong
            }
        }
    }
    return totalBlocksSize;
}
/* buffer : must be a valid memory area of at least 4 bytes */
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_blockTypeID(mut sizeID: libc::c_int,
                                           mut blockMode: libc::c_int,
                                           mut buffer: *mut libc::c_char)
 -> *const libc::c_char {
    *buffer.offset(0 as libc::c_int as isize) = 'B' as i32 as libc::c_char;
    if sizeID >= 4 as libc::c_int {
    } else {
        __assert_fail(b"sizeID >= 4\x00" as *const u8 as *const libc::c_char,
                      b"lz4io.c\x00" as *const u8 as *const libc::c_char,
                      1440 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"const char *LZ4IO_blockTypeID(int, int, char *)\x00")).as_ptr());
    }
    if sizeID <= 7 as libc::c_int {
    } else {
        __assert_fail(b"sizeID <= 7\x00" as *const u8 as *const libc::c_char,
                      b"lz4io.c\x00" as *const u8 as *const libc::c_char,
                      1440 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"const char *LZ4IO_blockTypeID(int, int, char *)\x00")).as_ptr());
    }
    *buffer.offset(1 as libc::c_int as isize) =
        (sizeID + '0' as i32) as libc::c_char;
    *buffer.offset(2 as libc::c_int as isize) =
        if blockMode == LZ4F_blockIndependent as libc::c_int {
            'I' as i32
        } else { 'D' as i32 } as libc::c_char;
    *buffer.offset(3 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
    return buffer;
}
/* buffer : must be valid memory area of at least 10 bytes */
unsafe extern "C" fn LZ4IO_toHuman(mut size: f128::f128,
                                   mut buf: *mut libc::c_char)
 -> *const libc::c_char {
    let units: [libc::c_char; 10] =
        *::std::mem::transmute::<&[u8; 10],
                                 &[libc::c_char; 10]>(b"\x00KMGTPEZY\x00");
    let mut i: size_t = 0 as libc::c_int as size_t;
    while size >= f128::f128::new(1024 as libc::c_int) {
        size /= f128::f128::new(1024 as libc::c_int);
        i = i.wrapping_add(1)
    }
    sprintf(buf, b"%.2Lf%c\x00" as *const u8 as *const libc::c_char, size,
            units[i as usize] as libc::c_int);
    return buf;
}
/* Get filename without path prefix */
unsafe extern "C" fn LZ4IO_baseName(mut input_filename: *const libc::c_char)
 -> *const libc::c_char {
    let mut b: *const libc::c_char = strrchr(input_filename, '/' as i32);
    if b.is_null() { b = strrchr(input_filename, '\\' as i32) }
    if b.is_null() { return input_filename }
    return if !b.is_null() { b.offset(1 as libc::c_int as isize) } else { b };
}
/* Report frame/s information in verbose mode.
 * Will populate file info with fileName and frameSummary where applicable.
 * - TODO :
 *  + report nb of blocks, hence max. possible decompressed size (when not reported in header)
 */
unsafe extern "C" fn LZ4IO_getCompressedFileInfo(mut cfinfo:
                                                     *mut LZ4IO_cFileInfo_t,
                                                 mut input_filename:
                                                     *const libc::c_char)
 -> LZ4IO_infoResult {
    let mut result: LZ4IO_infoResult =
        LZ4IO_format_not_known; /* default result (error) */
    let mut buffer: [libc::c_uchar; 19] = [0; 19];
    let finput: *mut FILE = LZ4IO_openSrcFile(input_filename);
    (*cfinfo).fileSize =
        UTIL_getFileSize(input_filename) as libc::c_ulonglong;
    while feof(finput) == 0 {
        let mut frameInfo: LZ4IO_frameInfo_t =
            {
                let mut init =
                    LZ4IO_frameInfo_t{lz4FrameInfo:
                                          {
                                              let mut init =
                                                  LZ4F_frameInfo_t{blockSizeID:
                                                                       LZ4F_default,
                                                                   blockMode:
                                                                       LZ4F_blockLinked,
                                                                   contentChecksumFlag:
                                                                       LZ4F_noContentChecksum,
                                                                   frameType:
                                                                       LZ4F_frame,
                                                                   contentSize:
                                                                       0 as
                                                                           libc::c_ulonglong,
                                                                   dictID:
                                                                       0 as
                                                                           libc::c_uint,
                                                                   blockChecksumFlag:
                                                                       LZ4F_noBlockChecksum,};
                                              init
                                          },
                                      frameType: lz4Frame,};
                init
            };
        let mut magicNumber: libc::c_uint = 0;
        /* Get MagicNumber */
        let mut nbReadBytes: size_t =
            fread(buffer.as_mut_ptr() as *mut libc::c_void,
                  1 as libc::c_int as libc::c_ulong,
                  4 as libc::c_int as libc::c_ulong, finput); /* EOF */
        if nbReadBytes == 0 as libc::c_int as libc::c_ulong {
            break ; /* default result (error) */
        } /* Little Endian format */
        result = LZ4IO_format_not_known; /* fold skippable magic numbers */
        if nbReadBytes != 4 as libc::c_int as libc::c_ulong {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Error %i : \x00" as *const u8 as
                            *const libc::c_char, 40 as libc::c_int);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"Unrecognized header : Magic Number unreadable\x00"
                            as *const u8 as *const libc::c_char);
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b" \n\x00" as *const u8 as *const libc::c_char);
            }
            exit(40 as libc::c_int);
        }
        magicNumber =
            LZ4IO_readLE32(buffer.as_mut_ptr() as *const libc::c_void);
        if LZ4IO_isSkippableMagicNumber(magicNumber) != 0 {
            magicNumber = 0x184d2a50 as libc::c_int as libc::c_uint
        }
        match magicNumber {
            407708164 => {
                if (*cfinfo).frameSummary.frameType as libc::c_uint !=
                       lz4Frame as libc::c_int as libc::c_uint {
                    (*cfinfo).eqFrameTypes =
                        0 as libc::c_int as libc::c_ushort
                }
                /* Get frame info */
                let readBytes: size_t =
                    fread(buffer.as_mut_ptr().offset(4 as libc::c_int as
                                                         isize) as
                              *mut libc::c_void,
                          1 as libc::c_int as libc::c_ulong,
                          (7 as libc::c_int - 4 as libc::c_int) as
                              libc::c_ulong, finput);
                if readBytes == 0 || ferror(finput) != 0 {
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b"Error %i : \x00" as *const u8 as
                                    *const libc::c_char, 71 as libc::c_int);
                    }
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b"Error reading %s\x00" as *const u8 as
                                    *const libc::c_char, input_filename);
                    }
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    exit(71 as libc::c_int);
                }
                let mut hSize: size_t =
                    LZ4F_headerSize(&mut buffer as *mut [libc::c_uchar; 19] as
                                        *const libc::c_void,
                                    7 as libc::c_int as size_t);
                if LZ4F_isError(hSize) == 0 {
                    if hSize >
                           (7 as libc::c_int + 4 as libc::c_int) as
                               libc::c_ulong {
                        /* We've already read LZ4F_HEADER_SIZE_MIN so read any extra until hSize*/
                        let readBytes_0: size_t =
                            fread(buffer.as_mut_ptr().offset(7 as libc::c_int
                                                                 as isize) as
                                      *mut libc::c_void,
                                  1 as libc::c_int as libc::c_ulong,
                                  hSize.wrapping_sub(7 as libc::c_int as
                                                         libc::c_ulong),
                                  finput);
                        if readBytes_0 == 0 || ferror(finput) != 0 {
                            if g_displayLevel >= 1 as libc::c_int {
                                fprintf(stderr,
                                        b"Error %i : \x00" as *const u8 as
                                            *const libc::c_char,
                                        72 as libc::c_int);
                            }
                            if g_displayLevel >= 1 as libc::c_int {
                                fprintf(stderr,
                                        b"Error reading %s\x00" as *const u8
                                            as *const libc::c_char,
                                        input_filename);
                            }
                            if g_displayLevel >= 1 as libc::c_int {
                                fprintf(stderr,
                                        b" \n\x00" as *const u8 as
                                            *const libc::c_char);
                            }
                            exit(72 as libc::c_int);
                        }
                    }
                    /* Create decompression context */
                    let mut dctx: *mut LZ4F_dctx = 0 as *mut LZ4F_dctx;
                    let mut isError: libc::c_uint =
                        LZ4F_isError(LZ4F_createDecompressionContext(&mut dctx,
                                                                     100 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint));
                    if isError == 0 {
                        isError =
                            LZ4F_isError(LZ4F_getFrameInfo(dctx,
                                                           &mut frameInfo.lz4FrameInfo,
                                                           buffer.as_mut_ptr()
                                                               as
                                                               *const libc::c_void,
                                                           &mut hSize));
                        LZ4F_freeDecompressionContext(dctx);
                        if isError == 0 {
                            if ((*cfinfo).frameSummary.lz4FrameInfo.blockSizeID
                                    as libc::c_uint !=
                                    frameInfo.lz4FrameInfo.blockSizeID as
                                        libc::c_uint ||
                                    (*cfinfo).frameSummary.lz4FrameInfo.blockMode
                                        as libc::c_uint !=
                                        frameInfo.lz4FrameInfo.blockMode as
                                            libc::c_uint) &&
                                   (*cfinfo).frameCount !=
                                       0 as libc::c_int as libc::c_ulonglong {
                                (*cfinfo).eqBlockTypes =
                                    0 as libc::c_int as libc::c_ushort
                            }
                            let totalBlocksSize: libc::c_ulonglong =
                                LZ4IO_skipBlocksData(finput,
                                                     frameInfo.lz4FrameInfo.blockChecksumFlag,
                                                     frameInfo.lz4FrameInfo.contentChecksumFlag);
                            if totalBlocksSize != 0 {
                                let mut bTypeBuffer: [libc::c_char; 5] =
                                    [0; 5];
                                LZ4IO_blockTypeID(frameInfo.lz4FrameInfo.blockSizeID
                                                      as libc::c_int,
                                                  frameInfo.lz4FrameInfo.blockMode
                                                      as libc::c_int,
                                                  bTypeBuffer.as_mut_ptr());
                                if g_displayLevel >= 3 as libc::c_int {
                                    fprintf(stderr,
                                            b"    %6llu %14s %5s %8s\x00" as
                                                *const u8 as
                                                *const libc::c_char,
                                            (*cfinfo).frameCount.wrapping_add(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulonglong),
                                            LZ4IO_frameTypeNames[frameInfo.frameType
                                                                     as
                                                                     usize],
                                            bTypeBuffer.as_mut_ptr(),
                                            if frameInfo.lz4FrameInfo.contentChecksumFlag
                                                   as libc::c_uint != 0 {
                                                b"XXH32\x00" as *const u8 as
                                                    *const libc::c_char
                                            } else {
                                                b"-\x00" as *const u8 as
                                                    *const libc::c_char
                                            });
                                }
                                if frameInfo.lz4FrameInfo.contentSize != 0 {
                                    let ratio: libc::c_double =
                                        totalBlocksSize.wrapping_add(hSize as
                                                                         libc::c_ulonglong)
                                            as libc::c_double /
                                            frameInfo.lz4FrameInfo.contentSize
                                                as libc::c_double *
                                            100 as libc::c_int as
                                                libc::c_double;
                                    if g_displayLevel >= 3 as libc::c_int {
                                        fprintf(stderr,
                                                b" %20llu %20llu %9.2f%%\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                totalBlocksSize.wrapping_add(hSize
                                                                                 as
                                                                                 libc::c_ulonglong),
                                                frameInfo.lz4FrameInfo.contentSize,
                                                ratio);
                                    }
                                    /* Now we've consumed frameInfo we can use it to store the total contentSize */
                                    frameInfo.lz4FrameInfo.contentSize =
                                        frameInfo.lz4FrameInfo.contentSize.wrapping_add((*cfinfo).frameSummary.lz4FrameInfo.contentSize)
                                } else {
                                    if g_displayLevel >= 3 as libc::c_int {
                                        fprintf(stderr,
                                                b" %20llu %20s %9s \n\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                totalBlocksSize.wrapping_add(hSize
                                                                                 as
                                                                                 libc::c_ulonglong),
                                                b"-\x00" as *const u8 as
                                                    *const libc::c_char,
                                                b"-\x00" as *const u8 as
                                                    *const libc::c_char); /* only works for files < 2 GB */
                                    }
                                    (*cfinfo).allContentSize =
                                        0 as libc::c_int as libc::c_ushort
                                }
                                result = LZ4IO_LZ4F_OK
                            }
                        }
                    }
                }
            }
            407642370 => {
                frameInfo.frameType = legacyFrame;
                if (*cfinfo).frameSummary.frameType as libc::c_uint !=
                       legacyFrame as libc::c_int as libc::c_uint &&
                       (*cfinfo).frameCount !=
                           0 as libc::c_int as libc::c_ulonglong {
                    (*cfinfo).eqFrameTypes =
                        0 as libc::c_int as libc::c_ushort
                }
                (*cfinfo).eqBlockTypes = 0 as libc::c_int as libc::c_ushort;
                (*cfinfo).allContentSize = 0 as libc::c_int as libc::c_ushort;
                let totalBlocksSize_0: libc::c_ulonglong =
                    LZ4IO_skipLegacyBlocksData(finput);
                if totalBlocksSize_0 != 0 {
                    if g_displayLevel >= 3 as libc::c_int {
                        fprintf(stderr,
                                b"    %6llu %14s %5s %8s %20llu %20s %9s\n\x00"
                                    as *const u8 as *const libc::c_char,
                                (*cfinfo).frameCount.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulonglong),
                                LZ4IO_frameTypeNames[frameInfo.frameType as
                                                         usize],
                                b"-\x00" as *const u8 as *const libc::c_char,
                                b"-\x00" as *const u8 as *const libc::c_char,
                                totalBlocksSize_0.wrapping_add(4 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulonglong),
                                b"-\x00" as *const u8 as *const libc::c_char,
                                b"-\x00" as *const u8 as *const libc::c_char);
                    }
                    result = LZ4IO_LZ4F_OK
                }
            }
            407710288 => {
                frameInfo.frameType = skippableFrame;
                if (*cfinfo).frameSummary.frameType as libc::c_uint !=
                       skippableFrame as libc::c_int as libc::c_uint &&
                       (*cfinfo).frameCount !=
                           0 as libc::c_int as libc::c_ulonglong {
                    (*cfinfo).eqFrameTypes =
                        0 as libc::c_int as libc::c_ushort
                }
                (*cfinfo).eqBlockTypes = 0 as libc::c_int as libc::c_ushort;
                (*cfinfo).allContentSize = 0 as libc::c_int as libc::c_ushort;
                nbReadBytes =
                    fread(buffer.as_mut_ptr() as *mut libc::c_void,
                          1 as libc::c_int as libc::c_ulong,
                          4 as libc::c_int as libc::c_ulong, finput);
                if nbReadBytes != 4 as libc::c_int as libc::c_ulong {
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b"Error %i : \x00" as *const u8 as
                                    *const libc::c_char, 42 as libc::c_int);
                    }
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b"Stream error : skippable size unreadable\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    exit(42 as libc::c_int);
                }
                let size: libc::c_uint =
                    LZ4IO_readLE32(buffer.as_mut_ptr() as
                                       *const libc::c_void);
                let errorNb: libc::c_int =
                    fseek_u32(finput, size, 1 as libc::c_int);
                if errorNb != 0 as libc::c_int {
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b"Error %i : \x00" as *const u8 as
                                    *const libc::c_char, 43 as libc::c_int);
                    }
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b"Stream error : cannot skip skippable area\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if g_displayLevel >= 1 as libc::c_int {
                        fprintf(stderr,
                                b" \n\x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    exit(43 as libc::c_int);
                }
                if g_displayLevel >= 3 as libc::c_int {
                    fprintf(stderr,
                            b"    %6llu %14s %5s %8s %20u %20s %9s\n\x00" as
                                *const u8 as *const libc::c_char,
                            (*cfinfo).frameCount.wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_ulonglong),
                            b"SkippableFrame\x00" as *const u8 as
                                *const libc::c_char,
                            b"-\x00" as *const u8 as *const libc::c_char,
                            b"-\x00" as *const u8 as *const libc::c_char,
                            size.wrapping_add(8 as libc::c_int as
                                                  libc::c_uint),
                            b"-\x00" as *const u8 as *const libc::c_char,
                            b"-\x00" as *const u8 as *const libc::c_char);
                }
                result = LZ4IO_LZ4F_OK
            }
            _ => {
                let position: libc::c_long = ftell(finput);
                if g_displayLevel >= 3 as libc::c_int {
                    fprintf(stderr,
                            b"Stream followed by undecodable data \x00" as
                                *const u8 as *const libc::c_char);
                }
                if position != -(1 as libc::c_long) {
                    if g_displayLevel >= 3 as libc::c_int {
                        fprintf(stderr,
                                b"at position %i \x00" as *const u8 as
                                    *const libc::c_char,
                                position as libc::c_int);
                    }
                }
                if g_displayLevel >= 3 as libc::c_int {
                    fprintf(stderr,
                            b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
        if result as libc::c_uint !=
               LZ4IO_LZ4F_OK as libc::c_int as libc::c_uint {
            break ;
        }
        (*cfinfo).frameSummary = frameInfo;
        (*cfinfo).frameCount = (*cfinfo).frameCount.wrapping_add(1)
    }
    fclose(finput);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4IO_displayCompressedFilesInfo(mut inFileNames:
                                                              *mut *const libc::c_char,
                                                          mut ifnIdx: size_t)
 -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut idx: size_t = 0 as libc::c_int as size_t;
    if g_displayLevel < 3 as libc::c_int {
        fprintf(stdout,
                b"%10s %14s %5s %11s %13s %9s   %s\n\x00" as *const u8 as
                    *const libc::c_char,
                b"Frames\x00" as *const u8 as *const libc::c_char,
                b"Type\x00" as *const u8 as *const libc::c_char,
                b"Block\x00" as *const u8 as *const libc::c_char,
                b"Compressed\x00" as *const u8 as *const libc::c_char,
                b"Uncompressed\x00" as *const u8 as *const libc::c_char,
                b"Ratio\x00" as *const u8 as *const libc::c_char,
                b"Filename\x00" as *const u8 as *const libc::c_char);
    }
    while idx < ifnIdx {
        /* Get file info */
        let mut cfinfo: LZ4IO_cFileInfo_t =
            {
                let mut init =
                    LZ4IO_cFileInfo_t{fileName: 0 as *const libc::c_char,
                                      fileSize: 0 as libc::c_ulonglong,
                                      frameCount:
                                          0 as libc::c_int as
                                              libc::c_ulonglong,
                                      frameSummary:
                                          {
                                              let mut init =
                                                  LZ4IO_frameInfo_t{lz4FrameInfo:
                                                                        {
                                                                            let mut init =
                                                                                LZ4F_frameInfo_t{blockSizeID:
                                                                                                     LZ4F_default,
                                                                                                 blockMode:
                                                                                                     LZ4F_blockLinked,
                                                                                                 contentChecksumFlag:
                                                                                                     LZ4F_noContentChecksum,
                                                                                                 frameType:
                                                                                                     LZ4F_frame,
                                                                                                 contentSize:
                                                                                                     0
                                                                                                         as
                                                                                                         libc::c_ulonglong,
                                                                                                 dictID:
                                                                                                     0
                                                                                                         as
                                                                                                         libc::c_uint,
                                                                                                 blockChecksumFlag:
                                                                                                     LZ4F_noBlockChecksum,};
                                                                            init
                                                                        },
                                                                    frameType:
                                                                        lz4Frame,};
                                              init
                                          },
                                      eqFrameTypes:
                                          1 as libc::c_int as libc::c_ushort,
                                      eqBlockTypes:
                                          1 as libc::c_int as libc::c_ushort,
                                      allContentSize:
                                          1 as libc::c_int as
                                              libc::c_ushort,};
                init
            };
        cfinfo.fileName = LZ4IO_baseName(*inFileNames.offset(idx as isize));
        if UTIL_isRegFile(*inFileNames.offset(idx as isize)) == 0 {
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"lz4: %s is not a regular file \n\x00" as *const u8
                            as *const libc::c_char,
                        *inFileNames.offset(idx as isize));
            }
            return 0 as libc::c_int
        }
        if g_displayLevel >= 3 as libc::c_int {
            fprintf(stderr,
                    b"%s(%llu/%llu)\n\x00" as *const u8 as
                        *const libc::c_char, cfinfo.fileName,
                    (idx as
                         libc::c_ulonglong).wrapping_add(1 as libc::c_int as
                                                             libc::c_ulonglong),
                    ifnIdx as libc::c_ulonglong);
        }
        if g_displayLevel >= 3 as libc::c_int {
            fprintf(stderr,
                    b"    %6s %14s %5s %8s %20s %20s %9s\n\x00" as *const u8
                        as *const libc::c_char,
                    b"Frame\x00" as *const u8 as *const libc::c_char,
                    b"Type\x00" as *const u8 as *const libc::c_char,
                    b"Block\x00" as *const u8 as *const libc::c_char,
                    b"Checksum\x00" as *const u8 as *const libc::c_char,
                    b"Compressed\x00" as *const u8 as *const libc::c_char,
                    b"Uncompressed\x00" as *const u8 as *const libc::c_char,
                    b"Ratio\x00" as *const u8 as *const libc::c_char);
        }
        let op_result: LZ4IO_infoResult =
            LZ4IO_getCompressedFileInfo(&mut cfinfo,
                                        *inFileNames.offset(idx as isize));
        if op_result as libc::c_uint !=
               LZ4IO_LZ4F_OK as libc::c_int as libc::c_uint {
            if op_result as libc::c_uint ==
                   LZ4IO_format_not_known as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(b"op_result == LZ4IO_format_not_known\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lz4io.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1628 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 60],
                                                        &[libc::c_char; 60]>(b"int LZ4IO_displayCompressedFilesInfo(const char **, size_t)\x00")).as_ptr());
            }
            if g_displayLevel >= 1 as libc::c_int {
                fprintf(stderr,
                        b"lz4: %s: File format not recognized \n\x00" as
                            *const u8 as *const libc::c_char,
                        *inFileNames.offset(idx as isize));
            }
            return 0 as libc::c_int
        }
        if g_displayLevel >= 3 as libc::c_int {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
        if g_displayLevel < 3 as libc::c_int {
            /* Display Summary */
            let mut buffers: [[libc::c_char; 10]; 3] = [[0; 10]; 3];
            fprintf(stdout,
                    b"%10llu %14s %5s %11s %13s \x00" as *const u8 as
                        *const libc::c_char, cfinfo.frameCount,
                    if cfinfo.eqFrameTypes as libc::c_int != 0 {
                        LZ4IO_frameTypeNames[cfinfo.frameSummary.frameType as
                                                 usize]
                    } else { b"-\x00" as *const u8 as *const libc::c_char },
                    if cfinfo.eqBlockTypes as libc::c_int != 0 {
                        LZ4IO_blockTypeID(cfinfo.frameSummary.lz4FrameInfo.blockSizeID
                                              as libc::c_int,
                                          cfinfo.frameSummary.lz4FrameInfo.blockMode
                                              as libc::c_int,
                                          buffers[0 as libc::c_int as
                                                      usize].as_mut_ptr())
                    } else { b"-\x00" as *const u8 as *const libc::c_char },
                    LZ4IO_toHuman(f128::f128::new(cfinfo.fileSize),
                                  buffers[1 as libc::c_int as
                                              usize].as_mut_ptr()),
                    if cfinfo.allContentSize as libc::c_int != 0 {
                        LZ4IO_toHuman(f128::f128::new(cfinfo.frameSummary.lz4FrameInfo.contentSize),
                                      buffers[2 as libc::c_int as
                                                  usize].as_mut_ptr())
                    } else { b"-\x00" as *const u8 as *const libc::c_char });
            if cfinfo.allContentSize != 0 {
                let ratio: libc::c_double =
                    cfinfo.fileSize as libc::c_double /
                        cfinfo.frameSummary.lz4FrameInfo.contentSize as
                            libc::c_double *
                        100 as libc::c_int as libc::c_double;
                fprintf(stdout,
                        b"%9.2f%%  %s \n\x00" as *const u8 as
                            *const libc::c_char, ratio, cfinfo.fileName);
            } else {
                fprintf(stdout,
                        b"%9s   %s\n\x00" as *const u8 as *const libc::c_char,
                        b"-\x00" as *const u8 as *const libc::c_char,
                        cfinfo.fileName);
            }
        }
        idx = idx.wrapping_add(1)
    }
    return result;
}
