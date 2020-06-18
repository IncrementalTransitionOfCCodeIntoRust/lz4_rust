#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value, main,
           ptr_wrapping_offset_from, register_tool)]
#[allow(unused_imports)]
use ::lz4_rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type LZ4IO_prefs_s;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn BMK_benchFiles(fileNamesTable: *mut *const libc::c_char,
                      nbFiles: libc::c_uint, cLevel: libc::c_int,
                      cLevelLast: libc::c_int,
                      dictFileName: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn BMK_setNbSeconds(nbLoops: libc::c_uint);
    #[no_mangle]
    fn BMK_setBlockSize(blockSize: size_t);
    #[no_mangle]
    fn BMK_setNotificationLevel(level: libc::c_uint);
    #[no_mangle]
    fn BMK_setBenchSeparately(separate: libc::c_int);
    #[no_mangle]
    fn LZ4IO_defaultPreferences() -> *mut LZ4IO_prefs_t;
    #[no_mangle]
    fn LZ4IO_freePreferences(prefs: *mut LZ4IO_prefs_t);
    #[no_mangle]
    fn LZ4IO_compressFilename(prefs: *mut LZ4IO_prefs_t,
                              input_filename: *const libc::c_char,
                              output_filename: *const libc::c_char,
                              compressionlevel: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_decompressFilename(prefs: *mut LZ4IO_prefs_t,
                                input_filename: *const libc::c_char,
                                output_filename: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_compressMultipleFilenames(prefs: *mut LZ4IO_prefs_t,
                                       inFileNamesTable:
                                           *mut *const libc::c_char,
                                       ifntSize: libc::c_int,
                                       suffix: *const libc::c_char,
                                       compressionlevel: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_decompressMultipleFilenames(prefs: *mut LZ4IO_prefs_t,
                                         inFileNamesTable:
                                             *mut *const libc::c_char,
                                         ifntSize: libc::c_int,
                                         suffix: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setDictionaryFilename(prefs: *mut LZ4IO_prefs_t,
                                   dictionaryFilename: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setPassThrough(prefs: *mut LZ4IO_prefs_t, yes: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setOverwrite(prefs: *mut LZ4IO_prefs_t, yes: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setTestMode(prefs: *mut LZ4IO_prefs_t, yes: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setBlockSizeID(prefs: *mut LZ4IO_prefs_t,
                            blockSizeID: libc::c_uint) -> size_t;
    #[no_mangle]
    fn LZ4IO_setBlockSize(prefs: *mut LZ4IO_prefs_t, blockSize: size_t)
     -> size_t;
    #[no_mangle]
    fn LZ4IO_setBlockMode(prefs: *mut LZ4IO_prefs_t,
                          blockMode: LZ4IO_blockMode_t) -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setBlockChecksumMode(prefs: *mut LZ4IO_prefs_t,
                                  xxhash: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setStreamChecksumMode(prefs: *mut LZ4IO_prefs_t,
                                   xxhash: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setNotificationLevel(level: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setSparseFile(prefs: *mut LZ4IO_prefs_t, enable: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setContentSize(prefs: *mut LZ4IO_prefs_t, enable: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4IO_setRemoveSrcFile(prefs: *mut LZ4IO_prefs_t, flag: libc::c_uint);
    #[no_mangle]
    fn LZ4IO_favorDecSpeed(prefs: *mut LZ4IO_prefs_t, favor: libc::c_int);
    #[no_mangle]
    fn LZ4IO_displayCompressedFilesInfo(inFileNames: *mut *const libc::c_char,
                                        ifnIdx: size_t) -> libc::c_int;
    #[no_mangle]
    fn LZ4_versionString() -> *const libc::c_char;
    #[no_mangle]
    fn LZ4IO_compressFilename_Legacy(prefs: *mut LZ4IO_prefs_t,
                                     input_filename: *const libc::c_char,
                                     output_filename: *const libc::c_char,
                                     compressionlevel: libc::c_int)
     -> libc::c_int;
    /* hidden function */
    #[no_mangle]
    fn LZ4IO_compressMultipleFilenames_Legacy(prefs: *mut LZ4IO_prefs_t,
                                              inFileNamesTable:
                                                  *mut *const libc::c_char,
                                              ifntSize: libc::c_int,
                                              suffix: *const libc::c_char,
                                              compressionLevel: libc::c_int)
     -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
pub type uint32_t = __uint32_t;
pub type U32 = uint32_t;
pub type stat_t = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type LZ4IO_prefs_t = LZ4IO_prefs_s;
pub type LZ4IO_blockMode_t = libc::c_uint;
pub const LZ4IO_blockIndependent: LZ4IO_blockMode_t = 1;
pub const LZ4IO_blockLinked: LZ4IO_blockMode_t = 0;
pub type operationMode_e = libc::c_uint;
pub const om_list: operationMode_e = 5;
pub const om_bench: operationMode_e = 4;
pub const om_test: operationMode_e = 3;
pub const om_decompress: operationMode_e = 2;
pub const om_compress: operationMode_e = 1;
pub const om_auto: operationMode_e = 0;
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int { return getc(stdin); }
unsafe extern "C" fn UTIL_freeFileList(mut filenameTable:
                                           *mut *const libc::c_char,
                                       mut allocatedBuffer:
                                           *mut libc::c_char) {
    if !allocatedBuffer.is_null() {
        free(allocatedBuffer as *mut libc::c_void);
    }
    if !filenameTable.is_null() { free(filenameTable as *mut libc::c_void); };
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
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
               st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
               st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
               st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
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
               st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
               st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
               st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
               __glibc_reserved: [0; 3],};
    r = stat(infilename, &mut statbuf);
    if r == 0 &&
           statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
               0o40000 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as U32
    }
    return 0 as libc::c_int as U32;
}
unsafe extern "C" fn UTIL_realloc(mut ptr: *mut libc::c_void,
                                  mut size: size_t) -> *mut libc::c_void {
    let newptr: *mut libc::c_void = realloc(ptr, size);
    if !newptr.is_null() { return newptr }
    free(ptr);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn UTIL_prepareFileList(mut dirName: *const libc::c_char,
                                          mut bufStart:
                                              *mut *mut libc::c_char,
                                          mut pos: *mut size_t,
                                          mut bufEnd: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut dirLength: libc::c_int = 0;
    let mut nbFiles: libc::c_int = 0 as libc::c_int;
    dir = opendir(dirName);
    if dir.is_null() {
        fprintf(stderr,
                b"Cannot open directory \'%s\': %s\n\x00" as *const u8 as
                    *const libc::c_char, dirName,
                strerror(*__errno_location()));
        return 0 as libc::c_int
    }
    dirLength = strlen(dirName) as libc::c_int;
    *__errno_location() = 0 as libc::c_int;
    loop  {
        entry = readdir(dir);
        if entry.is_null() { break ; }
        let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fnameLength: libc::c_int = 0;
        let mut pathLength: libc::c_int = 0;
        if strcmp((*entry).d_name.as_mut_ptr(),
                  b"..\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int ||
               strcmp((*entry).d_name.as_mut_ptr(),
                      b".\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
            continue ;
        }
        fnameLength = strlen((*entry).d_name.as_mut_ptr()) as libc::c_int;
        path =
            malloc((dirLength + fnameLength + 2 as libc::c_int) as
                       libc::c_ulong) as *mut libc::c_char;
        if path.is_null() { closedir(dir); return 0 as libc::c_int }
        memcpy(path as *mut libc::c_void, dirName as *const libc::c_void,
               dirLength as libc::c_ulong);
        *path.offset(dirLength as isize) = '/' as i32 as libc::c_char;
        memcpy(path.offset(dirLength as
                               isize).offset(1 as libc::c_int as isize) as
                   *mut libc::c_void,
               (*entry).d_name.as_mut_ptr() as *const libc::c_void,
               fnameLength as libc::c_ulong);
        pathLength = dirLength + 1 as libc::c_int + fnameLength;
        *path.offset(pathLength as isize) = 0 as libc::c_int as libc::c_char;
        if UTIL_isDirectory(path) != 0 {
            nbFiles += UTIL_prepareFileList(path, bufStart, pos, bufEnd);
            if (*bufStart).is_null() {
                free(path as *mut libc::c_void);
                closedir(dir);
                return 0 as libc::c_int
            }
        } else {
            if (*bufStart).offset(*pos as isize).offset(pathLength as isize)
                   >= *bufEnd {
                let mut newListSize: ptrdiff_t =
                    (*bufEnd).wrapping_offset_from(*bufStart) as libc::c_long
                        +
                        (8 as libc::c_int * 1024 as libc::c_int) as
                            libc::c_long;
                *bufStart =
                    UTIL_realloc(*bufStart as *mut libc::c_void,
                                 newListSize as size_t) as *mut libc::c_char;
                *bufEnd = (*bufStart).offset(newListSize as isize);
                if (*bufStart).is_null() {
                    free(path as *mut libc::c_void);
                    closedir(dir);
                    return 0 as libc::c_int
                }
            }
            if (*bufStart).offset(*pos as isize).offset(pathLength as isize) <
                   *bufEnd {
                strncpy((*bufStart).offset(*pos as isize), path,
                        (*bufEnd).wrapping_offset_from((*bufStart).offset(*pos
                                                                              as
                                                                              isize))
                            as libc::c_long as libc::c_ulong);
                *pos =
                    (*pos as
                         libc::c_ulong).wrapping_add((pathLength +
                                                          1 as libc::c_int) as
                                                         libc::c_ulong) as
                        size_t as size_t;
                nbFiles += 1
            }
        }
        free(path as *mut libc::c_void);
        *__errno_location() = 0 as libc::c_int
    }
    if *__errno_location() != 0 as libc::c_int {
        fprintf(stderr,
                b"readdir(%s) error: %s\n\x00" as *const u8 as
                    *const libc::c_char, dirName,
                strerror(*__errno_location()));
        free(*bufStart as *mut libc::c_void);
        *bufStart = 0 as *mut libc::c_char
    }
    closedir(dir);
    return nbFiles;
}
unsafe extern "C" fn UTIL_createFileList(mut inputNames:
                                             *mut *const libc::c_char,
                                         mut inputNamesNb: libc::c_uint,
                                         mut allocatedBuffer:
                                             *mut *mut libc::c_char,
                                         mut allocatedNamesNb:
                                             *mut libc::c_uint)
 -> *mut *const libc::c_char {
    let mut pos: size_t = 0;
    let mut i: libc::c_uint = 0;
    let mut nbFiles: libc::c_uint = 0;
    let mut buf: *mut libc::c_char =
        malloc((8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong) as
            *mut libc::c_char;
    let mut bufSize: size_t =
        (8 as libc::c_int * 1024 as libc::c_int) as size_t;
    let mut fileTable: *mut *const libc::c_char =
        0 as *mut *const libc::c_char;
    if buf.is_null() { return 0 as *mut *const libc::c_char }
    i = 0 as libc::c_int as libc::c_uint;
    pos = 0 as libc::c_int as size_t;
    nbFiles = 0 as libc::c_int as libc::c_uint;
    while i < inputNamesNb {
        if UTIL_isDirectory(*inputNames.offset(i as isize)) == 0 {
            let len: size_t = strlen(*inputNames.offset(i as isize));
            if pos.wrapping_add(len) >= bufSize {
                while pos.wrapping_add(len) >= bufSize {
                    bufSize =
                        (bufSize as
                             libc::c_ulong).wrapping_add((8 as libc::c_int *
                                                              1024 as
                                                                  libc::c_int)
                                                             as libc::c_ulong)
                            as size_t as size_t
                }
                buf =
                    UTIL_realloc(buf as *mut libc::c_void, bufSize) as
                        *mut libc::c_char;
                if buf.is_null() { return 0 as *mut *const libc::c_char }
            }
            if pos.wrapping_add(len) < bufSize {
            } else {
                __assert_fail(b"pos + len < bufSize\x00" as *const u8 as
                                  *const libc::c_char,
                              b"./util.h\x00" as *const u8 as
                                  *const libc::c_char,
                              565 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 87],
                                                        &[libc::c_char; 87]>(b"const char **UTIL_createFileList(const char **, unsigned int, char **, unsigned int *)\x00")).as_ptr());
            }
            strncpy(buf.offset(pos as isize), *inputNames.offset(i as isize),
                    bufSize.wrapping_sub(pos));
            pos =
                (pos as
                     libc::c_ulong).wrapping_add(len.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong))
                    as size_t as size_t;
            nbFiles = nbFiles.wrapping_add(1)
        } else {
            let mut bufend: *mut libc::c_char = buf.offset(bufSize as isize);
            nbFiles =
                nbFiles.wrapping_add(UTIL_prepareFileList(*inputNames.offset(i
                                                                                 as
                                                                                 isize),
                                                          &mut buf, &mut pos,
                                                          &mut bufend) as
                                         libc::c_uint);
            if buf.is_null() { return 0 as *mut *const libc::c_char }
            if bufend > buf {
            } else {
                __assert_fail(b"bufend > buf\x00" as *const u8 as
                                  *const libc::c_char,
                              b"./util.h\x00" as *const u8 as
                                  *const libc::c_char,
                              573 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 87],
                                                        &[libc::c_char; 87]>(b"const char **UTIL_createFileList(const char **, unsigned int, char **, unsigned int *)\x00")).as_ptr());
            }
            bufSize =
                bufend.wrapping_offset_from(buf) as libc::c_long as size_t
        }
        i = i.wrapping_add(1)
    }
    if nbFiles == 0 as libc::c_int as libc::c_uint {
        free(buf as *mut libc::c_void);
        return 0 as *mut *const libc::c_char
    }
    fileTable =
        malloc((nbFiles as
                    size_t).wrapping_add(1 as libc::c_int as
                                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const libc::c_char>()
                                                                             as
                                                                             libc::c_ulong))
            as *mut *const libc::c_char;
    if fileTable.is_null() {
        free(buf as *mut libc::c_void);
        return 0 as *mut *const libc::c_char
    }
    i = 0 as libc::c_int as libc::c_uint;
    pos = 0 as libc::c_int as size_t;
    while i < nbFiles {
        let ref mut fresh0 = *fileTable.offset(i as isize);
        *fresh0 = buf.offset(pos as isize);
        pos =
            (pos as
                 libc::c_ulong).wrapping_add(strlen(*fileTable.offset(i as
                                                                          isize)).wrapping_add(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong))
                as size_t as size_t;
        i = i.wrapping_add(1)
    }
    if pos > bufSize {
        free(buf as *mut libc::c_void);
        free(fileTable as *mut libc::c_void);
        return 0 as *mut *const libc::c_char
    }
    *allocatedBuffer = buf;
    *allocatedNamesNb = nbFiles;
    return fileTable;
}
static mut stdinmark: [libc::c_char; 6] = [115, 116, 100, 105, 110, 0];
static mut stdoutmark: [libc::c_char; 7] = [115, 116, 100, 111, 117, 116, 0];
static mut nulmark: [libc::c_char; 10] =
    [47, 100, 101, 118, 47, 110, 117, 108, 108, 0];
static mut g_lz4c_legacy_commands: libc::c_int = 0 as libc::c_int;
/*-************************************
*  Macros
***************************************/
static mut displayLevel: libc::c_uint = 2 as libc::c_int as libc::c_uint;
/*-***************************
*  Functions
*****************************/
unsafe extern "C" fn usage(mut exeName: *const libc::c_char) -> libc::c_int {
    fprintf(stderr,
            b"Usage : \n\x00" as *const u8 as
                *const libc::c_char); /* windows */
    fprintf(stderr,
            b"      %s [arg] [input] [output] \n\x00" as *const u8 as
                *const libc::c_char, exeName);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"input   : a filename \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"          with no FILE, or when FILE is - or %s, read standard input\n\x00"
                as *const u8 as *const libc::c_char, stdinmark.as_ptr());
    fprintf(stderr,
            b"Arguments : \n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -1     : Fast compression (default) \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -9     : High compression \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -d     : decompression (default for %s extension)\n\x00" as
                *const u8 as *const libc::c_char,
            b".lz4\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -z     : force compression \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -D FILE: use FILE as dictionary \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -f     : overwrite output without prompting \n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -k     : preserve source files(s)  (default) \n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"--rm    : remove source file(s) after successful de/compression \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -h/-H  : display help/long help and exit \n\x00" as *const u8
                as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage_advanced(mut exeName: *const libc::c_char)
 -> libc::c_int {
    fprintf(stderr,
            b"*** %s %i-bits v%s, by %s ***\n\x00" as *const u8 as
                *const libc::c_char,
            b"LZ4 command line interface\x00" as *const u8 as
                *const libc::c_char,
            (::std::mem::size_of::<*mut libc::c_void>() as
                 libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                 libc::c_ulong) as
                libc::c_int, LZ4_versionString(),
            b"Yann Collet\x00" as *const u8 as *const libc::c_char);
    usage(exeName);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"Advanced arguments :\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -V     : display Version number and exit \n\x00" as *const u8
                as *const libc::c_char);
    fprintf(stderr,
            b" -v     : verbose mode \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -q     : suppress warnings; specify twice to suppress errors too\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -c     : force write to standard output, even if it is the console\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -t     : test compressed file integrity\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -m     : multiple input files (implies automatic output filenames)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -r     : operate recursively on directories (sets also -m) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -l     : compress using Legacy format (Linux kernel compression)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -B#    : cut file into blocks of size # bytes [32+] \n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"                     or predefined block size [4-7] (default: 7) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -BI    : Block Independence (default) \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -BD    : Block dependency (improves compression ratio) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -BX    : enable block checksum (default:disabled) \n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"--no-frame-crc : disable stream checksum (default:enabled) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"--content-size : compressed frame includes original size (default:not present)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"--list FILE : lists information about .lz4 files (useful for files compressed with --content-size flag)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"--[no-]sparse  : sparse mode (default:enabled on file, disabled on stdout)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"--favor-decSpeed: compressed files decompress faster, but are less compressed \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"--fast[=#]: switch to ultra fast compression level (default: %i)\n\x00"
                as *const u8 as *const libc::c_char, 1 as libc::c_int);
    fprintf(stderr,
            b"--best  : same as -%d\n\x00" as *const u8 as
                *const libc::c_char, 12 as libc::c_int);
    fprintf(stderr,
            b"Benchmark arguments : \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b" -b#    : benchmark file(s), using # compression level (default : 1) \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -e#    : test all compression levels from -bX to # (default : 1)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b" -i#    : minimum evaluation time in seconds (default : 3s) \n\x00"
                as *const u8 as *const libc::c_char);
    if g_lz4c_legacy_commands != 0 {
        fprintf(stderr,
                b"Legacy arguments : \n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr,
                b" -c0    : fast compression \n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr,
                b" -c1    : high compression \n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr,
                b" -c2,-hc: very high compression \n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr,
                b" -y     : overwrite output without prompting \n\x00" as
                    *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage_longhelp(mut exeName: *const libc::c_char)
 -> libc::c_int {
    usage_advanced(exeName);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"****************************\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"***** Advanced comment *****\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"****************************\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"Which values can [output] have ? \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"---------------------------------\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"[output] : a filename \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"          \'%s\', or \'-\' for standard output (pipe mode)\n\x00"
                as *const u8 as *const libc::c_char, stdoutmark.as_ptr());
    fprintf(stderr,
            b"          \'%s\' to discard output (test mode) \n\x00" as
                *const u8 as *const libc::c_char,
            b"null\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"[output] can be left empty. In this case, it receives the following value :\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"          - if stdout is not the console, then [output] = stdout \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"          - if stdout is console : \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"               + for compression, output to filename%s \n\x00"
                as *const u8 as *const libc::c_char,
            b".lz4\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"               + for decompression, output to filename without \'%s\'\n\x00"
                as *const u8 as *const libc::c_char,
            b".lz4\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"                    > if input filename has no \'%s\' extension : error \n\x00"
                as *const u8 as *const libc::c_char,
            b".lz4\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"Compression levels : \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"---------------------\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"-0 ... -2  => Fast compression, all identicals\n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"-3 ... -%d => High compression; higher number == more compression but slower\n\x00"
                as *const u8 as *const libc::c_char, 12 as libc::c_int);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"stdin, stdout and the console : \n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"--------------------------------\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"To protect the console from binary flooding (bad argument mistake)\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"%s will refuse to read from console, or write to console \n\x00"
                as *const u8 as *const libc::c_char, exeName);
    fprintf(stderr,
            b"except if \'-c\' command is specified, to force output to console \n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"Simple example :\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"----------------\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"1 : compress \'filename\' fast, using default output name \'filename.lz4\'\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"          %s filename\n\x00" as *const u8 as
                *const libc::c_char, exeName);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"Short arguments can be aggregated. For example :\n\x00" as
                *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"----------------------------------\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"2 : compress \'filename\' in high compression mode, overwrite output if exists\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"          %s -9 -f filename \n\x00" as *const u8 as
                *const libc::c_char, exeName);
    fprintf(stderr,
            b"    is equivalent to :\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"          %s -9f filename \n\x00" as *const u8 as
                *const libc::c_char, exeName);
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"%s can be used in \'pure pipe mode\'. For example :\n\x00" as
                *const u8 as *const libc::c_char, exeName);
    fprintf(stderr,
            b"-------------------------------------\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(stderr,
            b"3 : compress data stream from \'generator\', send result to \'consumer\'\n\x00"
                as *const u8 as *const libc::c_char);
    fprintf(stderr,
            b"          generator | %s | consumer \n\x00" as *const u8 as
                *const libc::c_char, exeName);
    if g_lz4c_legacy_commands != 0 {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"***** Warning  ***** \n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr,
                b"Legacy arguments take precedence. Therefore : \n\x00" as
                    *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"--------------------------------- \n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr,
                b"          %s -hc filename \n\x00" as *const u8 as
                    *const libc::c_char, exeName);
        fprintf(stderr,
                b"means \'compress filename in high compression mode\' \n\x00"
                    as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"It is not equivalent to : \n\x00" as *const u8 as
                    *const libc::c_char);
        fprintf(stderr,
                b"          %s -h -c filename \n\x00" as *const u8 as
                    *const libc::c_char, exeName);
        fprintf(stderr,
                b"which displays help text and exits \n\x00" as *const u8 as
                    *const libc::c_char);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn badusage(mut exeName: *const libc::c_char)
 -> libc::c_int {
    if displayLevel >= 1 as libc::c_int as libc::c_uint {
        fprintf(stderr,
                b"Incorrect parameters\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    if displayLevel >= 1 as libc::c_int as libc::c_uint { usage(exeName); }
    exit(1 as libc::c_int);
}
unsafe extern "C" fn waitEnter() {
    fprintf(stderr,
            b"Press enter to continue...\n\x00" as *const u8 as
                *const libc::c_char);
    getchar();
}
unsafe extern "C" fn lastNameFromPath(mut path: *const libc::c_char)
 -> *const libc::c_char {
    let mut name: *const libc::c_char = path;
    if !strrchr(name, '/' as i32).is_null() {
        name = strrchr(name, '/' as i32).offset(1 as libc::c_int as isize)
    }
    if !strrchr(name, '\\' as i32).is_null() {
        name = strrchr(name, '\\' as i32).offset(1 as libc::c_int as isize)
    }
    return name;
}
/* ! exeNameMatch() :
    @return : a non-zero value if exeName matches test, excluding the extension
   */
unsafe extern "C" fn exeNameMatch(mut exeName: *const libc::c_char,
                                  mut test: *const libc::c_char)
 -> libc::c_int {
    return (strncmp(exeName, test, strlen(test)) == 0 &&
                (*exeName.offset(strlen(test) as isize) as libc::c_int ==
                     '\u{0}' as i32 ||
                     *exeName.offset(strlen(test) as isize) as libc::c_int ==
                         '.' as i32)) as libc::c_int;
}
/* ! readU32FromChar() :
 * @return : unsigned integer value read from input in `char` format
 *  allows and interprets K, KB, KiB, M, MB and MiB suffix.
 *  Will also modify `*stringPtr`, advancing it to position where it stopped reading.
 *  Note : function result can overflow if digit string > MAX_UINT */
unsafe extern "C" fn readU32FromChar(mut stringPtr: *mut *const libc::c_char)
 -> libc::c_uint {
    let mut result: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while **stringPtr as libc::c_int >= '0' as i32 &&
              **stringPtr as libc::c_int <= '9' as i32 {
        result = result.wrapping_mul(10 as libc::c_int as libc::c_uint);
        result =
            result.wrapping_add((**stringPtr as libc::c_int - '0' as i32) as
                                    libc::c_uint);
        *stringPtr = (*stringPtr).offset(1)
    }
    if **stringPtr as libc::c_int == 'K' as i32 ||
           **stringPtr as libc::c_int == 'M' as i32 {
        result <<= 10 as libc::c_int;
        if **stringPtr as libc::c_int == 'M' as i32 {
            result <<= 10 as libc::c_int
        }
        *stringPtr = (*stringPtr).offset(1);
        if **stringPtr as libc::c_int == 'i' as i32 {
            *stringPtr = (*stringPtr).offset(1)
        }
        if **stringPtr as libc::c_int == 'B' as i32 {
            *stringPtr = (*stringPtr).offset(1)
        }
    }
    return result;
}
/* * longCommandWArg() :
 *  check if *stringPtr is the same as longCommand.
 *  If yes, @return 1 and advances *stringPtr to the position which immediately follows longCommand.
 * @return 0 and doesn't modify *stringPtr otherwise.
 */
unsafe extern "C" fn longCommandWArg(mut stringPtr: *mut *const libc::c_char,
                                     mut longCommand: *const libc::c_char)
 -> libc::c_int {
    let comSize: size_t = strlen(longCommand);
    let result: libc::c_int =
        (strncmp(*stringPtr, longCommand, comSize) == 0) as libc::c_int;
    if result != 0 { *stringPtr = (*stringPtr).offset(comSize as isize) }
    return result;
}
/* * determineOpMode() :
 *  auto-determine operation mode, based on input filename extension
 *  @return `om_decompress` if input filename has .lz4 extension and `om_compress` otherwise.
 */
unsafe extern "C" fn determineOpMode(mut inputFilename: *const libc::c_char)
 -> operationMode_e {
    let inSize: size_t = strlen(inputFilename);
    let extSize: size_t =
        strlen(b".lz4\x00" as *const u8 as *const libc::c_char);
    let extStart: size_t =
        if inSize > extSize {
            inSize.wrapping_sub(extSize)
        } else { 0 as libc::c_int as libc::c_ulong };
    if strcmp(inputFilename.offset(extStart as isize),
              b".lz4\x00" as *const u8 as *const libc::c_char) == 0 {
        return om_decompress
    } else { return om_compress };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut cLevel: libc::c_int = 1 as libc::c_int;
    let mut cLevelLast: libc::c_int = -(10000 as libc::c_int);
    let mut legacy_format: libc::c_int = 0 as libc::c_int;
    let mut forceStdout: libc::c_int = 0 as libc::c_int;
    let mut main_pause: libc::c_int = 0 as libc::c_int;
    let mut multiple_inputs: libc::c_int = 0 as libc::c_int;
    let mut all_arguments_are_files: libc::c_int = 0 as libc::c_int;
    let mut operationResult: libc::c_int = 0 as libc::c_int;
    let mut mode: operationMode_e = om_auto;
    let mut input_filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut output_filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut dictionary_filename: *const libc::c_char =
        0 as *const libc::c_char;
    let mut dynNameSpace: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inFileNames: *mut *const libc::c_char =
        calloc(argc as size_t,
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *const libc::c_char;
    let mut ifnIdx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let prefs: *mut LZ4IO_prefs_t = LZ4IO_defaultPreferences();
    let nullOutput: [libc::c_char; 5] =
        *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"null\x00");
    let extension: [libc::c_char; 5] =
        *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".lz4\x00");
    let mut blockSize: size_t =
        LZ4IO_setBlockSizeID(prefs, 7 as libc::c_int as libc::c_uint);
    let exeName: *const libc::c_char =
        lastNameFromPath(*argv.offset(0 as libc::c_int as isize));
    let mut extendedFileList: *mut *const libc::c_char =
        0 as *mut *const libc::c_char;
    let mut fileNamesBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileNamesNb: libc::c_uint = 0;
    let mut recursive: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    /* Init */
    if inFileNames.is_null() {
        fprintf(stderr,
                b"Allocation error : not enough memory \n\x00" as *const u8 as
                    *const libc::c_char);
        return 1 as libc::c_int
    }
    let ref mut fresh1 = *inFileNames.offset(0 as libc::c_int as isize);
    *fresh1 = stdinmark.as_ptr();
    LZ4IO_setOverwrite(prefs, 0 as libc::c_int);
    /* predefined behaviors, based on binary/link name */
    if exeNameMatch(exeName,
                    b"lz4cat\x00" as *const u8 as *const libc::c_char) != 0 {
        mode = om_decompress;
        LZ4IO_setOverwrite(prefs, 1 as libc::c_int);
        LZ4IO_setPassThrough(prefs, 1 as libc::c_int);
        LZ4IO_setRemoveSrcFile(prefs, 0 as libc::c_int as libc::c_uint);
        forceStdout = 1 as libc::c_int;
        output_filename = stdoutmark.as_ptr();
        displayLevel = 1 as libc::c_int as libc::c_uint;
        multiple_inputs = 1 as libc::c_int
    }
    if exeNameMatch(exeName, b"unlz4\x00" as *const u8 as *const libc::c_char)
           != 0 {
        mode = om_decompress
    }
    if exeNameMatch(exeName, b"lz4c\x00" as *const u8 as *const libc::c_char)
           != 0 {
        g_lz4c_legacy_commands = 1 as libc::c_int
    }
    /* command switches */
    i = 1 as libc::c_int; /* Protection if argument empty */
    's_145:
        loop  {
            if !(i < argc) { current_block = 14908777651318078790; break ; }
            let mut argument: *const libc::c_char = *argv.offset(i as isize);
            if !argument.is_null() {
                /* Short commands (note : aggregated short commands are allowed) */
                if all_arguments_are_files == 0 &&
                       *argument.offset(0 as libc::c_int as isize) as
                           libc::c_int == '-' as i32 {
                    /* '-' means stdin/stdout */
                    if *argument.offset(1 as libc::c_int as isize) as
                           libc::c_int == 0 as libc::c_int {
                        if input_filename.is_null() {
                            input_filename = stdinmark.as_ptr()
                        } else { output_filename = stdoutmark.as_ptr() }
                    } else {
                        /* long commands (--long-word) */
                        if *argument.offset(1 as libc::c_int as isize) as
                               libc::c_int == '-' as i32 {
                            if strcmp(argument,
                                      b"--\x00" as *const u8 as
                                          *const libc::c_char) == 0 {
                                all_arguments_are_files =
                                    1 as
                                        libc::c_int; /* keep source file (default) */
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--compress\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                mode = om_compress;
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--decompress\x00" as *const u8
                                                 as *const libc::c_char) == 0
                                          ||
                                          strcmp(argument,
                                                 b"--uncompress\x00" as
                                                     *const u8 as
                                                     *const libc::c_char) == 0
                             {
                                mode = om_decompress;
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--multiple\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                multiple_inputs = 1 as libc::c_int;
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--test\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                mode = om_test;
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--force\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                LZ4IO_setOverwrite(prefs, 1 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--no-force\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                LZ4IO_setOverwrite(prefs, 0 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--stdout\x00" as *const u8 as
                                                 *const libc::c_char) == 0 ||
                                          strcmp(argument,
                                                 b"--to-stdout\x00" as
                                                     *const u8 as
                                                     *const libc::c_char) == 0
                             {
                                forceStdout = 1 as libc::c_int;
                                output_filename = stdoutmark.as_ptr();
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--frame-crc\x00" as *const u8
                                                 as *const libc::c_char) == 0
                             {
                                LZ4IO_setStreamChecksumMode(prefs,
                                                            1 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--no-frame-crc\x00" as
                                                 *const u8 as
                                                 *const libc::c_char) == 0 {
                                LZ4IO_setStreamChecksumMode(prefs,
                                                            0 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--content-size\x00" as
                                                 *const u8 as
                                                 *const libc::c_char) == 0 {
                                LZ4IO_setContentSize(prefs, 1 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--no-content-size\x00" as
                                                 *const u8 as
                                                 *const libc::c_char) == 0 {
                                LZ4IO_setContentSize(prefs, 0 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--list\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                mode = om_list;
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--sparse\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                LZ4IO_setSparseFile(prefs, 2 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--no-sparse\x00" as *const u8
                                                 as *const libc::c_char) == 0
                             {
                                LZ4IO_setSparseFile(prefs, 0 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--favor-decSpeed\x00" as
                                                 *const u8 as
                                                 *const libc::c_char) == 0 {
                                LZ4IO_favorDecSpeed(prefs, 1 as libc::c_int);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--verbose\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                displayLevel = displayLevel.wrapping_add(1);
                                current_block = 4775909272756257391;
                            } else if strcmp(argument,
                                             b"--quiet\x00" as *const u8 as
                                                 *const libc::c_char) == 0 {
                                if displayLevel != 0 {
                                    displayLevel =
                                        displayLevel.wrapping_sub(1)
                                }
                                current_block = 4775909272756257391;
                            } else {
                                if strcmp(argument,
                                          b"--version\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                    fprintf(stdout,
                                            b"*** %s %i-bits v%s, by %s ***\n\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            b"LZ4 command line interface\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            (::std::mem::size_of::<*mut libc::c_void>()
                                                 as
                                                 libc::c_ulong).wrapping_mul(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                as libc::c_int,
                                            LZ4_versionString(),
                                            b"Yann Collet\x00" as *const u8 as
                                                *const libc::c_char);
                                    return 0 as libc::c_int
                                }
                                if strcmp(argument,
                                          b"--help\x00" as *const u8 as
                                              *const libc::c_char) == 0 {
                                    usage_advanced(exeName);
                                    current_block = 1870196864736364587;
                                    break ;
                                } else if strcmp(argument,
                                                 b"--keep\x00" as *const u8 as
                                                     *const libc::c_char) == 0
                                 {
                                    LZ4IO_setRemoveSrcFile(prefs,
                                                           0 as libc::c_int as
                                                               libc::c_uint);
                                    current_block = 4775909272756257391;
                                } else if strcmp(argument,
                                                 b"--rm\x00" as *const u8 as
                                                     *const libc::c_char) == 0
                                 {
                                    LZ4IO_setRemoveSrcFile(prefs,
                                                           1 as libc::c_int as
                                                               libc::c_uint);
                                    current_block = 4775909272756257391;
                                } else if longCommandWArg(&mut argument,
                                                          b"--fast\x00" as
                                                              *const u8 as
                                                              *const libc::c_char)
                                              != 0 {
                                    /* Parse optional acceleration factor */
                                    if *argument as libc::c_int == '=' as i32
                                       {
                                        let mut fastLevel: U32 = 0;
                                        argument = argument.offset(1);
                                        fastLevel =
                                            readU32FromChar(&mut argument);
                                        if fastLevel != 0 {
                                            cLevel =
                                                -(fastLevel as libc::c_int)
                                        } else { badusage(exeName); }
                                    } else if *argument as libc::c_int !=
                                                  0 as libc::c_int {
                                        /* Invalid character following --fast */
                                        badusage(exeName);
                                    } else {
                                        cLevel = -(1 as libc::c_int)
                                        /* default for --fast */
                                    }
                                    current_block = 4775909272756257391;
                                } else if strcmp(argument,
                                                 b"--best\x00" as *const u8 as
                                                     *const libc::c_char) == 0
                                 {
                                    cLevel = 12 as libc::c_int;
                                    current_block = 4775909272756257391;
                                } else {
                                    current_block = 2956972668325154207;
                                }
                            }
                        } else { current_block = 2956972668325154207; }
                        match current_block {
                            4775909272756257391 => { }
                            _ => {
                                while *argument.offset(1 as libc::c_int as
                                                           isize) as
                                          libc::c_int != 0 as libc::c_int {
                                    argument = argument.offset(1);
                                    if g_lz4c_legacy_commands != 0 {
                                        /* For gzip(1) compatibility */
                                        /* Legacy commands (-c0, -c1, -hc, -y) */
                                        if strcmp(argument,
                                                  b"c0\x00" as *const u8 as
                                                      *const libc::c_char) ==
                                               0 {
                                            cLevel =
                                                0 as
                                                    libc::c_int; /* -c0 (fast compression) */
                                            argument =
                                                argument.offset(1); /* -c1 (high compression) */
                                            continue
                                                ; /* -c2 (very high compression) */
                                        } else if strcmp(argument,
                                                         b"c1\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                                      == 0 {
                                            cLevel =
                                                9 as
                                                    libc::c_int; /* -hc (very high compression) */
                                            argument = argument.offset(1);
                                            continue ;
                                        } else if strcmp(argument,
                                                         b"c2\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                                      == 0 {
                                            cLevel = 12 as libc::c_int;
                                            argument = argument.offset(1);
                                            continue ;
                                        } else if strcmp(argument,
                                                         b"hc\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                                      == 0 {
                                            cLevel = 12 as libc::c_int;
                                            argument = argument.offset(1);
                                            continue ;
                                        } else if strcmp(argument,
                                                         b"y\x00" as *const u8
                                                             as
                                                             *const libc::c_char)
                                                      == 0 {
                                            LZ4IO_setOverwrite(prefs,
                                                               1 as
                                                                   libc::c_int);
                                            continue ;
                                        }
                                        /* -y (answer 'yes' to overwrite permission) */
                                    }
                                    if *argument as libc::c_int >= '0' as i32
                                           &&
                                           *argument as libc::c_int <=
                                               '9' as i32 {
                                        cLevel =
                                            readU32FromChar(&mut argument) as
                                                libc::c_int;
                                        argument = argument.offset(-1)
                                    } else {
                                        match *argument.offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                                                  as libc::c_int {
                                            86 => {
                                                /* Display help */
                                                fprintf(stdout,
                                                        b"*** %s %i-bits v%s, by %s ***\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        b"LZ4 command line interface\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        (::std::mem::size_of::<*mut libc::c_void>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong)
                                                            as libc::c_int,
                                                        LZ4_versionString(),
                                                        b"Yann Collet\x00" as
                                                            *const u8 as
                                                            *const libc::c_char);
                                                current_block =
                                                    1870196864736364587;
                                                break 's_145 ;
                                            }
                                            104 => {
                                                usage_advanced(exeName);
                                                current_block =
                                                    1870196864736364587;
                                                break 's_145 ;
                                            }
                                            72 => {
                                                usage_longhelp(exeName);
                                                current_block =
                                                    1870196864736364587;
                                                break 's_145 ;
                                            }
                                            101 => {
                                                argument = argument.offset(1);
                                                cLevelLast =
                                                    readU32FromChar(&mut argument)
                                                        as libc::c_int;
                                                argument =
                                                    argument.offset(-1);
                                                continue ;
                                            }
                                            122 => {
                                                /* Compression (default) */
                                                mode = om_compress;
                                                continue ;
                                            }
                                            68 => {
                                                if *argument.offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                                       as libc::c_int ==
                                                       '\u{0}' as i32 {
                                                    /* path is next arg */
                                                    if i + 1 as libc::c_int ==
                                                           argc {
                                                        /* there is no next arg */
                                                        badusage(exeName);
                                                    }
                                                    i += 1;
                                                    dictionary_filename =
                                                        *argv.offset(i as
                                                                         isize)
                                                } else {
                                                    /* path follows immediately */
                                                    dictionary_filename =
                                                        argument.offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                }
                                                /* skip to end of argument so that we jump to parsing next argument */
                                                argument =
                                                    argument.offset(strlen(argument).wrapping_sub(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong)
                                                                        as
                                                                        isize);
                                                continue ;
                                            }
                                            108 => {
                                                /* Use Legacy format (ex : Linux kernel compression) */
                                                legacy_format =
                                                    1 as libc::c_int;
                                                blockSize =
                                                    (8 as libc::c_int as
                                                         libc::c_uint).wrapping_mul((1
                                                                                         as
                                                                                         libc::c_uint)
                                                                                        <<
                                                                                        20
                                                                                            as
                                                                                            libc::c_int)
                                                        as size_t;
                                                continue ;
                                            }
                                            100 => {
                                                /* Decoding */
                                                mode = om_decompress;
                                                continue ;
                                            }
                                            99 => {
                                                /* Force stdout, even if stdout==console */
                                                forceStdout =
                                                    1 as libc::c_int;
                                                output_filename =
                                                    stdoutmark.as_ptr();
                                                LZ4IO_setPassThrough(prefs,
                                                                     1 as
                                                                         libc::c_int);
                                                continue ;
                                            }
                                            116 => {
                                                /* Test integrity */
                                                mode = om_test;
                                                continue ;
                                            }
                                            102 => {
                                                /* Overwrite */
                                                LZ4IO_setOverwrite(prefs,
                                                                   1 as
                                                                       libc::c_int);
                                                continue ;
                                            }
                                            118 => {
                                                /* Verbose mode */
                                                displayLevel =
                                                    displayLevel.wrapping_add(1);
                                                continue ;
                                            }
                                            113 => {
                                                /* Quiet mode */
                                                if displayLevel != 0 {
                                                    displayLevel =
                                                        displayLevel.wrapping_sub(1)
                                                }
                                                continue ;
                                            }
                                            107 => {
                                                /* keep source file (default anyway, so useless) (for xz/lzma compatibility) */
                                                LZ4IO_setRemoveSrcFile(prefs,
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint);
                                                continue ;
                                            }
                                            66 => {
                                                /* Modify Block Properties */
                                                while *argument.offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                                          as libc::c_int !=
                                                          0 as libc::c_int {
                                                    let mut exitBlockProperties:
                                                            libc::c_int =
                                                        0 as libc::c_int;
                                                    match *argument.offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                                              as libc::c_int {
                                                        68 => {
                                                            LZ4IO_setBlockMode(prefs,
                                                                               LZ4IO_blockLinked);
                                                            argument =
                                                                argument.offset(1)
                                                        }
                                                        73 => {
                                                            LZ4IO_setBlockMode(prefs,
                                                                               LZ4IO_blockIndependent);
                                                            argument =
                                                                argument.offset(1)
                                                        }
                                                        88 => {
                                                            LZ4IO_setBlockChecksumMode(prefs,
                                                                                       1
                                                                                           as
                                                                                           libc::c_int);
                                                            argument =
                                                                argument.offset(1)
                                                        }
                                                        _ => {
                                                            if (*argument.offset(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)
                                                                    as
                                                                    libc::c_int)
                                                                   <
                                                                   '0' as i32
                                                                   ||
                                                                   *argument.offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                                                       as
                                                                       libc::c_int
                                                                       >
                                                                       '9' as
                                                                           i32
                                                               {
                                                                exitBlockProperties
                                                                    =
                                                                    1 as
                                                                        libc::c_int
                                                            } else {
                                                                let mut B:
                                                                        libc::c_uint =
                                                                    0;
                                                                argument =
                                                                    argument.offset(1);
                                                                B =
                                                                    readU32FromChar(&mut argument);
                                                                argument =
                                                                    argument.offset(-1);
                                                                if B <
                                                                       4 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint
                                                                   {
                                                                    badusage(exeName);
                                                                }
                                                                if B <=
                                                                       7 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint
                                                                   {
                                                                    blockSize
                                                                        =
                                                                        LZ4IO_setBlockSizeID(prefs,
                                                                                             B);
                                                                    BMK_setBlockSize(blockSize);
                                                                    if displayLevel
                                                                           >=
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                       {
                                                                        fprintf(stderr,
                                                                                b"using blocks of size %u KB \n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                (blockSize
                                                                                     >>
                                                                                     10
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    U32);
                                                                    }
                                                                } else {
                                                                    if B <
                                                                           32
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                       {
                                                                        badusage(exeName);
                                                                    }
                                                                    blockSize
                                                                        =
                                                                        LZ4IO_setBlockSize(prefs,
                                                                                           B
                                                                                               as
                                                                                               size_t);
                                                                    BMK_setBlockSize(blockSize);
                                                                    if blockSize
                                                                           >=
                                                                           1024
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong
                                                                       {
                                                                        if displayLevel
                                                                               >=
                                                                               2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                           {
                                                                            fprintf(stderr,
                                                                                    b"using blocks of size %u KB \n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    (blockSize
                                                                                         >>
                                                                                         10
                                                                                             as
                                                                                             libc::c_int)
                                                                                        as
                                                                                        U32);
                                                                        }
                                                                    } else if displayLevel
                                                                                  >=
                                                                                  2
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint
                                                                     {
                                                                        fprintf(stderr,
                                                                                b"using blocks of size %u bytes \n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char,
                                                                                blockSize
                                                                                    as
                                                                                    U32);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if exitBlockProperties !=
                                                           0 {
                                                        break ;
                                                    }
                                                }
                                                continue ;
                                            }
                                            98 => {
                                                /* Benchmark */
                                                mode = om_bench;
                                                multiple_inputs =
                                                    1 as libc::c_int;
                                                continue ;
                                            }
                                            83 => {
                                                /* hidden command : benchmark files, but do not fuse result */
                                                BMK_setBenchSeparately(1 as
                                                                           libc::c_int);
                                                continue ;
                                            }
                                            114 => {
                                                /* recursive */
                                                recursive =
                                                    1 as libc::c_int as
                                                        libc::c_uint
                                            }
                                            109 => { }
                                            105 => {
                                                /* Modify Nb Seconds (benchmark only) */
                                                let mut iters: libc::c_uint =
                                                    0;
                                                argument = argument.offset(1);
                                                iters =
                                                    readU32FromChar(&mut argument);
                                                argument =
                                                    argument.offset(-1);
                                                BMK_setNotificationLevel(displayLevel);
                                                BMK_setNbSeconds(iters);
                                                continue ;
                                            }
                                            112 => {
                                                /* notification if displayLevel >= 3 */
                                                /* Pause at the end (hidden option) */
                                                main_pause = 1 as libc::c_int;
                                                continue ;
                                            }
                                            _ => {
                                                /* Unrecognised command */
                                                badusage(exeName);
                                                continue ;
                                            }
                                        }
                                        /* fall-through */
                    /* Treat non-option args as input files.  See https://code.google.com/p/lz4/issues/detail?id=151 */
                                        multiple_inputs = 1 as libc::c_int
                                    }
                                }
                            }
                        }
                    }
                } else if multiple_inputs != 0 {
                    let fresh2 = ifnIdx;
                    ifnIdx = ifnIdx.wrapping_add(1);
                    let ref mut fresh3 = *inFileNames.offset(fresh2 as isize);
                    *fresh3 = argument
                } else if input_filename.is_null() {
                    input_filename = argument
                } else if output_filename.is_null() {
                    output_filename = argument;
                    if strcmp(output_filename, nullOutput.as_ptr()) == 0 {
                        output_filename = nulmark.as_ptr()
                    }
                } else if displayLevel >= 1 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"Warning : %s won\'t be used ! Do you want multiple input files (-m) ? \n\x00"
                                as *const u8 as *const libc::c_char,
                            argument);
                }
            }
            i += 1
        }
    match current_block {
        14908777651318078790 => {
            if displayLevel >= 3 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"*** %s %i-bits v%s, by %s ***\n\x00" as *const u8 as
                            *const libc::c_char,
                        b"LZ4 command line interface\x00" as *const u8 as
                            *const libc::c_char,
                        (::std::mem::size_of::<*mut libc::c_void>() as
                             libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                             libc::c_ulong) as
                            libc::c_int, LZ4_versionString(),
                        b"Yann Collet\x00" as *const u8 as
                            *const libc::c_char);
            }
            if displayLevel >= 4 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"_POSIX_C_SOURCE defined: %ldL\n\x00" as *const u8 as
                            *const libc::c_char, 200809 as libc::c_long);
            }
            if displayLevel >= 4 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"_POSIX_VERSION defined: %ldL\n\x00" as *const u8 as
                            *const libc::c_char, 200809 as libc::c_long);
            }
            if displayLevel >= 4 as libc::c_int as libc::c_uint {
                fprintf(stderr,
                        b"PLATFORM_POSIX_VERSION defined: %ldL\n\x00" as
                            *const u8 as *const libc::c_char,
                        200809 as libc::c_long);
            }
            if mode as libc::c_uint ==
                   om_compress as libc::c_int as libc::c_uint ||
                   mode as libc::c_uint ==
                       om_bench as libc::c_int as libc::c_uint {
                if displayLevel >= 4 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            b"Blocks size : %u KB\n\x00" as *const u8 as
                                *const libc::c_char,
                            (blockSize >> 10 as libc::c_int) as U32);
                }
            }
            if multiple_inputs != 0 {
                input_filename =
                    *inFileNames.offset(0 as libc::c_int as isize);
                if recursive != 0 {
                    /* Store in *inFileNames[] if -m is used. */
                    /* Store first non-option arg in input_filename to preserve original cli logic. */
                    /* Second non-option arg in output_filename to preserve original cli logic. */
                    /* 3rd non-option arg should not exist */
                    /* at this stage, filenameTable is a list of paths, which can contain both files and directories */
                    extendedFileList =
                        UTIL_createFileList(inFileNames, ifnIdx,
                                            &mut fileNamesBuf,
                                            &mut fileNamesNb);
                    if !extendedFileList.is_null() {
                        let mut u: libc::c_uint = 0;
                        u = 0 as libc::c_int as libc::c_uint;
                        while u < fileNamesNb {
                            if displayLevel >=
                                   4 as libc::c_int as libc::c_uint {
                                fprintf(stderr,
                                        b"%u %s\n\x00" as *const u8 as
                                            *const libc::c_char, u,
                                        *extendedFileList.offset(u as isize));
                            }
                            u = u.wrapping_add(1)
                        }
                        free(inFileNames as *mut libc::c_void);
                        inFileNames = extendedFileList;
                        ifnIdx = fileNamesNb
                    }
                }
            }
            if !dictionary_filename.is_null() {
                if strcmp(dictionary_filename, stdinmark.as_ptr()) == 0 &&
                       isatty(fileno(stdin)) != 0 {
                    if displayLevel >= 1 as libc::c_int as libc::c_uint {
                        fprintf(stderr,
                                b"refusing to read from a console\n\x00" as
                                    *const u8 as *const libc::c_char);
                    }
                    exit(1 as libc::c_int);
                }
                LZ4IO_setDictionaryFilename(prefs, dictionary_filename);
            }
            /* benchmark and test modes */
            if mode as libc::c_uint == om_bench as libc::c_int as libc::c_uint
               {
                BMK_setNotificationLevel(displayLevel);
                operationResult =
                    BMK_benchFiles(inFileNames, ifnIdx, cLevel, cLevelLast,
                                   dictionary_filename)
            } else {
                if mode as libc::c_uint ==
                       om_test as libc::c_int as libc::c_uint {
                    LZ4IO_setTestMode(prefs, 1 as libc::c_int);
                    output_filename = nulmark.as_ptr();
                    mode = om_decompress
                    /* defer to decompress */
                }
                /* compress or decompress */
                if input_filename.is_null() {
                    input_filename = stdinmark.as_ptr()
                }
                /* Check if input is defined as console; trigger an error in this case */
                if strcmp(input_filename, stdinmark.as_ptr()) == 0 &&
                       isatty(fileno(stdin)) != 0 {
                    if displayLevel >= 1 as libc::c_int as libc::c_uint {
                        fprintf(stderr,
                                b"refusing to read from a console\n\x00" as
                                    *const u8 as *const libc::c_char);
                    }
                    exit(1 as libc::c_int);
                }
                if strcmp(input_filename, stdinmark.as_ptr()) == 0 {
                    /* if input==stdin and no output defined, stdout becomes default output */
                    if output_filename.is_null() {
                        output_filename = stdoutmark.as_ptr()
                    }
                } else if recursive == 0 &&
                              UTIL_isRegFile(input_filename) == 0 {
                    if displayLevel >= 1 as libc::c_int as libc::c_uint {
                        fprintf(stderr,
                                b"%s: is not a regular file \n\x00" as
                                    *const u8 as *const libc::c_char,
                                input_filename);
                    }
                    exit(1 as libc::c_int);
                }
                /* No output filename ==> try to select one automatically (when possible) */
                if output_filename.is_null() &&
                       multiple_inputs == 0 as libc::c_int {
                    if isatty(fileno(stdout)) == 0 &&
                           mode as libc::c_uint !=
                               om_list as libc::c_int as libc::c_uint {
                        /* Default to stdout whenever stdout is not the console.
             * Note : this policy may change in the future, therefore don't rely on it !
             * To ensure `stdout` is explicitly selected, use `-c` command flag.
             * Conversely, to ensure output will not become `stdout`, use `-m` command flag */
                        if displayLevel >= 1 as libc::c_int as libc::c_uint {
                            fprintf(stderr,
                                    b"Warning : using stdout as default output. Do not rely on this behavior: use explicit `-c` instead ! \n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                        output_filename = stdoutmark.as_ptr()
                    } else {
                        if mode as libc::c_uint ==
                               om_auto as libc::c_int as libc::c_uint {
                            /* auto-determine compression or decompression, based on file extension */
                            mode = determineOpMode(input_filename)
                        }
                        if mode as libc::c_uint ==
                               om_compress as libc::c_int as libc::c_uint {
                            /* compression to file */
                            let l: size_t = strlen(input_filename);
                            dynNameSpace =
                                calloc(1 as libc::c_int as libc::c_ulong,
                                       l.wrapping_add(5 as libc::c_int as
                                                          libc::c_ulong)) as
                                    *mut libc::c_char;
                            if dynNameSpace.is_null() {
                                perror(exeName);
                                exit(1 as libc::c_int);
                            }
                            strcpy(dynNameSpace, input_filename);
                            strcat(dynNameSpace,
                                   b".lz4\x00" as *const u8 as
                                       *const libc::c_char);
                            output_filename = dynNameSpace;
                            if displayLevel >=
                                   2 as libc::c_int as libc::c_uint {
                                fprintf(stderr,
                                        b"Compressed filename will be : %s \n\x00"
                                            as *const u8 as
                                            *const libc::c_char,
                                        output_filename);
                            }
                        } else if mode as libc::c_uint ==
                                      om_decompress as libc::c_int as
                                          libc::c_uint {
                            /* decompression to file (automatic name will work only if input filename has correct format extension) */
                            let mut outl: size_t = 0;
                            let inl: size_t = strlen(input_filename);
                            dynNameSpace =
                                calloc(1 as libc::c_int as libc::c_ulong,
                                       inl.wrapping_add(1 as libc::c_int as
                                                            libc::c_ulong)) as
                                    *mut libc::c_char;
                            if dynNameSpace.is_null() {
                                perror(exeName);
                                exit(1 as libc::c_int);
                            }
                            strcpy(dynNameSpace, input_filename);
                            outl = inl;
                            if inl > 4 as libc::c_int as libc::c_ulong {
                                while outl >=
                                          inl.wrapping_sub(4 as libc::c_int as
                                                               libc::c_ulong)
                                          &&
                                          *input_filename.offset(outl as
                                                                     isize) as
                                              libc::c_int ==
                                              extension[outl.wrapping_sub(inl).wrapping_add(4
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong)
                                                            as usize] as
                                                  libc::c_int {
                                    let fresh4 = outl;
                                    outl = outl.wrapping_sub(1);
                                    *dynNameSpace.offset(fresh4 as isize) =
                                        0 as libc::c_int as libc::c_char
                                }
                            }
                            if outl !=
                                   inl.wrapping_sub(5 as libc::c_int as
                                                        libc::c_ulong) {
                                if displayLevel >=
                                       1 as libc::c_int as libc::c_uint {
                                    fprintf(stderr,
                                            b"Cannot determine an output filename\n\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                                }
                                badusage(exeName);
                            }
                            output_filename = dynNameSpace;
                            if displayLevel >=
                                   2 as libc::c_int as libc::c_uint {
                                fprintf(stderr,
                                        b"Decoding file %s \n\x00" as
                                            *const u8 as *const libc::c_char,
                                        output_filename);
                            }
                        }
                    }
                }
                if mode as libc::c_uint ==
                       om_list as libc::c_int as libc::c_uint {
                    /* Exit if trying to read from stdin as this isn't supported in this mode */
                    if strcmp(input_filename, stdinmark.as_ptr()) == 0 {
                        if displayLevel >= 1 as libc::c_int as libc::c_uint {
                            fprintf(stderr,
                                    b"refusing to read from standard input in --list mode\n\x00"
                                        as *const u8 as *const libc::c_char);
                        }
                        exit(1 as libc::c_int);
                    }
                    if multiple_inputs == 0 {
                        let fresh5 = ifnIdx;
                        ifnIdx = ifnIdx.wrapping_add(1);
                        let ref mut fresh6 =
                            *inFileNames.offset(fresh5 as isize);
                        *fresh6 = input_filename
                    }
                } else if multiple_inputs == 0 as libc::c_int {
                    if !output_filename.is_null() {
                    } else {
                        __assert_fail(b"output_filename\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"lz4cli.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      725 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 29],
                                                                &[libc::c_char; 29]>(b"int main(int, const char **)\x00")).as_ptr());
                    }
                }
                /* when multiple_inputs==1, output_filename may simply be useless,
     * however, output_filename must be !NULL for next strcmp() tests */
                if output_filename.is_null() {
                    output_filename =
                        b"*\\dummy^!//\x00" as *const u8 as
                            *const libc::c_char
                }
                /* Check if output is defined as console; trigger an error in this case */
                if strcmp(output_filename, stdoutmark.as_ptr()) == 0 &&
                       isatty(fileno(stdout)) != 0 && forceStdout == 0 {
                    if displayLevel >= 1 as libc::c_int as libc::c_uint {
                        fprintf(stderr,
                                b"refusing to write to console without -c \n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    exit(1 as libc::c_int);
                }
                /* Downgrade notification level in stdout and multiple file mode */
                if strcmp(output_filename, stdoutmark.as_ptr()) == 0 &&
                       displayLevel == 2 as libc::c_int as libc::c_uint {
                    displayLevel = 1 as libc::c_int as libc::c_uint
                }
                if multiple_inputs != 0 &&
                       displayLevel == 2 as libc::c_int as libc::c_uint {
                    displayLevel = 1 as libc::c_int as libc::c_uint
                }
                /* Auto-determine compression or decompression, based on file extension */
                if mode as libc::c_uint ==
                       om_auto as libc::c_int as libc::c_uint {
                    mode = determineOpMode(input_filename)
                }
                /* IO Stream/File */
                LZ4IO_setNotificationLevel(displayLevel as
                                               libc::c_int); /* compression is default action */
                if ifnIdx == 0 as libc::c_int as libc::c_uint {
                    multiple_inputs = 0 as libc::c_int
                } /* Version */
                if mode as libc::c_uint ==
                       om_decompress as libc::c_int as libc::c_uint {
                    if multiple_inputs != 0 {
                        if ifnIdx <= 2147483647 as libc::c_int as libc::c_uint
                           {
                        } else {
                            __assert_fail(b"ifnIdx <= INT_MAX\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          b"lz4cli.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          750 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 29],
                                                                    &[libc::c_char; 29]>(b"int main(int, const char **)\x00")).as_ptr());
                        }
                        operationResult =
                            LZ4IO_decompressMultipleFilenames(prefs,
                                                              inFileNames,
                                                              ifnIdx as
                                                                  libc::c_int,
                                                              if strcmp(output_filename,
                                                                        stdoutmark.as_ptr())
                                                                     == 0 {
                                                                  stdoutmark.as_ptr()
                                                              } else {
                                                                  b".lz4\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char
                                                              })
                    } else {
                        operationResult =
                            LZ4IO_decompressFilename(prefs, input_filename,
                                                     output_filename)
                    }
                } else if mode as libc::c_uint ==
                              om_list as libc::c_int as libc::c_uint {
                    operationResult =
                        LZ4IO_displayCompressedFilesInfo(inFileNames,
                                                         ifnIdx as size_t)
                } else if legacy_format != 0 {
                    if displayLevel >= 3 as libc::c_int as libc::c_uint {
                        fprintf(stderr,
                                b"! Generating LZ4 Legacy format (deprecated) ! \n\x00"
                                    as *const u8 as *const libc::c_char);
                    }
                    if multiple_inputs != 0 {
                        LZ4IO_compressMultipleFilenames_Legacy(prefs,
                                                               inFileNames,
                                                               ifnIdx as
                                                                   libc::c_int,
                                                               if strcmp(output_filename,
                                                                         stdoutmark.as_ptr())
                                                                      == 0 {
                                                                   stdoutmark.as_ptr()
                                                               } else {
                                                                   b".lz4\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                               }, cLevel);
                    } else {
                        LZ4IO_compressFilename_Legacy(prefs, input_filename,
                                                      output_filename,
                                                      cLevel);
                    }
                } else if multiple_inputs != 0 {
                    if ifnIdx <= 2147483647 as libc::c_int as libc::c_uint {
                    } else {
                        __assert_fail(b"ifnIdx <= INT_MAX\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"lz4cli.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      767 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 29],
                                                                &[libc::c_char; 29]>(b"int main(int, const char **)\x00")).as_ptr());
                    }
                    operationResult =
                        LZ4IO_compressMultipleFilenames(prefs, inFileNames,
                                                        ifnIdx as libc::c_int,
                                                        if strcmp(output_filename,
                                                                  stdoutmark.as_ptr())
                                                               == 0 {
                                                            stdoutmark.as_ptr()
                                                        } else {
                                                            b".lz4\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                        }, cLevel)
                } else {
                    operationResult =
                        LZ4IO_compressFilename(prefs, input_filename,
                                               output_filename, cLevel)
                }
            }
        }
        _ => { }
    }
    if main_pause != 0 { waitEnter(); }
    free(dynNameSpace as *mut libc::c_void);
    if !extendedFileList.is_null() {
        UTIL_freeFileList(extendedFileList, fileNamesBuf);
        inFileNames = 0 as *mut *const libc::c_char
    }
    LZ4IO_freePreferences(prefs);
    free(inFileNames as *mut libc::c_void);
    return operationResult;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *const libc::c_char) as i32)
    }
}
