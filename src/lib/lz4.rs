use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub type dictIssue_directive = libc::c_uint;
pub const dictSmall: dictIssue_directive = 1;
pub const noDictIssue: dictIssue_directive = 0;
pub type dict_directive = libc::c_uint;
pub const usingDictCtx: dict_directive = 3;
pub const usingExtDict: dict_directive = 2;
pub const withPrefix64k: dict_directive = 1;
pub const noDict: dict_directive = 0;
pub type tableType_t = libc::c_uint;
pub const byU16: tableType_t = 3;
pub const byU32: tableType_t = 2;
pub const byPtr: tableType_t = 1;
pub const clearedTable: tableType_t = 0;
pub type uptrval = uintptr_t;
pub type uintptr_t = libc::c_ulong;
pub type limitedOutput_directive = libc::c_uint;
pub const fillOutput: limitedOutput_directive = 2;
pub const limitedOutput: limitedOutput_directive = 1;
pub const notLimited: limitedOutput_directive = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub c: libc::c_char,
    pub t: LZ4_stream_t,
}
/*-************************************
*  Error detection
**************************************/
/* use after variable declarations */
/* disabled */
/*-************************************
*  Types
**************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union unalign {
    pub u16_0: U16,
    pub u32_0: U32,
    pub uArch: reg_t,
}
pub type reg_t = U64;
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type U16 = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u: U32,
    pub c: [BYTE; 4],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LZ4_static_assert: C2RustUnnamed_1 = 1;
pub type earlyEnd_directive = libc::c_uint;
pub const partial_decode: earlyEnd_directive = 1;
pub const decode_full_block: earlyEnd_directive = 0;
pub type endCondition_directive = libc::c_uint;
pub const endOnInputSize: endCondition_directive = 1;
pub const endOnOutputSize: endCondition_directive = 0;
pub const ok: variable_length_error = 0;
pub type variable_length_error = libc::c_int;
pub const initial_error: variable_length_error = -1;
pub const loop_error: variable_length_error = -2;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LZ4_static_assert_0: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const LZ4_static_assert_1: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const LZ4_static_assert_2: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LZ4_streamDecode_u {
    pub table: [libc::c_ulonglong; 4],
    pub internal_donotuse: LZ4_streamDecode_t_internal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4_streamDecode_t_internal {
    pub externalDict: *const uint8_t,
    pub extDictSize: size_t,
    pub prefixEnd: *const uint8_t,
    pub prefixSize: size_t,
}
pub type LZ4_streamDecode_t = LZ4_streamDecode_u;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const LZ4_static_assert_3: C2RustUnnamed_5 = 1;
static mut LZ4_minLength: libc::c_int = 12 as libc::c_int + 1 as libc::c_int;
/*-************************************
*  Reading and writing into memory
**************************************/
unsafe extern "C" fn LZ4_isLittleEndian() -> libc::c_uint {
    let one: C2RustUnnamed_0 =
        C2RustUnnamed_0{u:
                            1 as libc::c_int as
                                U32,}; /* don't use static : performance detrimental */
    return one.c[0 as libc::c_int as usize] as libc::c_uint;
}
unsafe extern "C" fn LZ4_read16(mut ptr: *const libc::c_void) -> U16 {
    return (*(ptr as *const unalign)).u16_0;
}
unsafe extern "C" fn LZ4_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign)).u32_0;
}
unsafe extern "C" fn LZ4_read_ARCH(mut ptr: *const libc::c_void) -> reg_t {
    return (*(ptr as *const unalign)).uArch;
}
unsafe extern "C" fn LZ4_write16(mut memPtr: *mut libc::c_void,
                                 mut value: U16) {
    (*(memPtr as *mut unalign)).u16_0 = value;
}
unsafe extern "C" fn LZ4_write32(mut memPtr: *mut libc::c_void,
                                 mut value: U32) {
    (*(memPtr as *mut unalign)).u32_0 = value;
}
/* safe and portable access using memcpy() */
/* LZ4_FORCE_MEMORY_ACCESS */
unsafe extern "C" fn LZ4_readLE16(mut memPtr: *const libc::c_void) -> U16 {
    if LZ4_isLittleEndian() != 0 {
        return LZ4_read16(memPtr)
    } else {
        let mut p: *const BYTE = memPtr as *const BYTE;
        return (*p.offset(0 as libc::c_int as isize) as U16 as libc::c_int +
                    ((*p.offset(1 as libc::c_int as isize) as libc::c_int) <<
                         8 as libc::c_int)) as U16
    };
}
unsafe extern "C" fn LZ4_writeLE16(mut memPtr: *mut libc::c_void,
                                   mut value: U16) {
    if LZ4_isLittleEndian() != 0 {
        LZ4_write16(memPtr, value);
    } else {
        let mut p: *mut BYTE = memPtr as *mut BYTE;
        *p.offset(0 as libc::c_int as isize) = value as BYTE;
        *p.offset(1 as libc::c_int as isize) =
            (value as libc::c_int >> 8 as libc::c_int) as BYTE
    };
}
/* customized variant of memcpy, which can overwrite up to 8 bytes beyond dstEnd */
unsafe extern "C" fn LZ4_wildCopy8(mut dstPtr: *mut libc::c_void,
                                   mut srcPtr: *const libc::c_void,
                                   mut dstEnd: *mut libc::c_void) {
    let mut d: *mut BYTE = dstPtr as *mut BYTE;
    let mut s: *const BYTE = srcPtr as *const BYTE;
    let e: *mut BYTE = dstEnd as *mut BYTE;
    loop  {
        memcpy(d as *mut libc::c_void, s as *const libc::c_void,
               8 as libc::c_int as libc::c_ulong);
        d = d.offset(8 as libc::c_int as isize);
        s = s.offset(8 as libc::c_int as isize);
        if !(d < e) { break ; }
    };
}
static mut inc32table: [libc::c_uint; 8] =
    [0 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     2 as libc::c_int as libc::c_uint, 1 as libc::c_int as libc::c_uint,
     0 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint,
     4 as libc::c_int as libc::c_uint, 4 as libc::c_int as libc::c_uint];
static mut dec64table: [libc::c_int; 8] =
    [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     -(1 as libc::c_int), -(4 as libc::c_int), 1 as libc::c_int,
     2 as libc::c_int, 3 as libc::c_int];
unsafe extern "C" fn LZ4_memcpy_using_offset_base(mut dstPtr: *mut BYTE,
                                                  mut srcPtr: *const BYTE,
                                                  mut dstEnd: *mut BYTE,
                                                  offset: size_t) {
    if offset < 8 as libc::c_int as libc::c_ulong {
        *dstPtr.offset(0 as libc::c_int as isize) =
            *srcPtr.offset(0 as libc::c_int as isize);
        *dstPtr.offset(1 as libc::c_int as isize) =
            *srcPtr.offset(1 as libc::c_int as isize);
        *dstPtr.offset(2 as libc::c_int as isize) =
            *srcPtr.offset(2 as libc::c_int as isize);
        *dstPtr.offset(3 as libc::c_int as isize) =
            *srcPtr.offset(3 as libc::c_int as isize);
        srcPtr = srcPtr.offset(inc32table[offset as usize] as isize);
        memcpy(dstPtr.offset(4 as libc::c_int as isize) as *mut libc::c_void,
               srcPtr as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
        srcPtr = srcPtr.offset(-(dec64table[offset as usize] as isize));
        dstPtr = dstPtr.offset(8 as libc::c_int as isize)
    } else {
        memcpy(dstPtr as *mut libc::c_void, srcPtr as *const libc::c_void,
               8 as libc::c_int as libc::c_ulong);
        dstPtr = dstPtr.offset(8 as libc::c_int as isize);
        srcPtr = srcPtr.offset(8 as libc::c_int as isize)
    }
    LZ4_wildCopy8(dstPtr as *mut libc::c_void, srcPtr as *const libc::c_void,
                  dstEnd as *mut libc::c_void);
}
/* customized variant of memcpy, which can overwrite up to 32 bytes beyond dstEnd
 * this version copies two times 16 bytes (instead of one time 32 bytes)
 * because it must be compatible with offsets >= 16. */
unsafe extern "C" fn LZ4_wildCopy32(mut dstPtr: *mut libc::c_void,
                                    mut srcPtr: *const libc::c_void,
                                    mut dstEnd: *mut libc::c_void) {
    let mut d: *mut BYTE = dstPtr as *mut BYTE;
    let mut s: *const BYTE = srcPtr as *const BYTE;
    let e: *mut BYTE = dstEnd as *mut BYTE;
    loop  {
        memcpy(d as *mut libc::c_void, s as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
        memcpy(d.offset(16 as libc::c_int as isize) as *mut libc::c_void,
               s.offset(16 as libc::c_int as isize) as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
        d = d.offset(32 as libc::c_int as isize);
        s = s.offset(32 as libc::c_int as isize);
        if !(d < e) { break ; }
    };
}
/* LZ4_memcpy_using_offset()  presumes :
 * - dstEnd >= dstPtr + MINMATCH
 * - there is at least 8 bytes available to write after dstEnd */
unsafe extern "C" fn LZ4_memcpy_using_offset(mut dstPtr: *mut BYTE,
                                             mut srcPtr: *const BYTE,
                                             mut dstEnd: *mut BYTE,
                                             offset: size_t) {
    let mut v: [BYTE; 8] =
        [0; 8]; /* silence an msan warning when offset==0 */
    LZ4_write32(dstPtr as *mut libc::c_void, 0 as libc::c_int as U32);
    match offset {
        1 => {
            memset(v.as_mut_ptr() as *mut libc::c_void,
                   *srcPtr as libc::c_int, 8 as libc::c_int as libc::c_ulong);
        }
        2 => {
            memcpy(v.as_mut_ptr() as *mut libc::c_void,
                   srcPtr as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            memcpy(&mut *v.as_mut_ptr().offset(2 as libc::c_int as isize) as
                       *mut BYTE as *mut libc::c_void,
                   srcPtr as *const libc::c_void,
                   2 as libc::c_int as libc::c_ulong);
            memcpy(&mut *v.as_mut_ptr().offset(4 as libc::c_int as isize) as
                       *mut BYTE as *mut libc::c_void,
                   &mut *v.as_mut_ptr().offset(0 as libc::c_int as isize) as
                       *mut BYTE as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
        }
        4 => {
            memcpy(v.as_mut_ptr() as *mut libc::c_void,
                   srcPtr as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            memcpy(&mut *v.as_mut_ptr().offset(4 as libc::c_int as isize) as
                       *mut BYTE as *mut libc::c_void,
                   srcPtr as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
        }
        _ => {
            LZ4_memcpy_using_offset_base(dstPtr, srcPtr, dstEnd, offset);
            return
        }
    }
    memcpy(dstPtr as *mut libc::c_void, v.as_mut_ptr() as *const libc::c_void,
           8 as libc::c_int as libc::c_ulong);
    dstPtr = dstPtr.offset(8 as libc::c_int as isize);
    while dstPtr < dstEnd {
        memcpy(dstPtr as *mut libc::c_void,
               v.as_mut_ptr() as *const libc::c_void,
               8 as libc::c_int as libc::c_ulong);
        dstPtr = dstPtr.offset(8 as libc::c_int as isize)
    };
}
/*-************************************
*  Common functions
**************************************/
unsafe extern "C" fn LZ4_NbCommonBytes(mut val: reg_t) -> libc::c_uint {
    if LZ4_isLittleEndian() != 0 {
        if ::std::mem::size_of::<reg_t>() as libc::c_ulong ==
               8 as libc::c_int as libc::c_ulong {
            return (val as libc::c_ulonglong).trailing_zeros() as i32 as
                       libc::c_uint >> 3 as libc::c_int
        } else {
            /* 32 bits */
            return (val as U32).trailing_zeros() as i32 as libc::c_uint >>
                       3 as libc::c_int
        }
    } else if ::std::mem::size_of::<reg_t>() as libc::c_ulong ==
                  8 as libc::c_int as libc::c_ulong {
        /* Big Endian CPU */
        /* 64-bits */
        return (val as libc::c_ulonglong).leading_zeros() as i32 as
                   libc::c_uint >> 3 as libc::c_int
    } else {
        /* 32 bits */
        return (val as U32).leading_zeros() as i32 as libc::c_uint >>
                   3 as libc::c_int
    };
}
#[inline(always)]
unsafe extern "C" fn LZ4_count(mut pIn: *const BYTE, mut pMatch: *const BYTE,
                               mut pInLimit: *const BYTE) -> libc::c_uint {
    let pStart: *const BYTE = pIn;
    if ((pIn <
             pInLimit.offset(-((::std::mem::size_of::<reg_t>() as
                                    libc::c_ulong).wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                                   as isize))) as libc::c_int !=
            0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        let diff: reg_t =
            LZ4_read_ARCH(pMatch as *const libc::c_void) ^
                LZ4_read_ARCH(pIn as *const libc::c_void);
        if diff == 0 {
            pIn =
                pIn.offset(::std::mem::size_of::<reg_t>() as libc::c_ulong as
                               isize);
            pMatch =
                pMatch.offset(::std::mem::size_of::<reg_t>() as libc::c_ulong
                                  as isize)
        } else { return LZ4_NbCommonBytes(diff) }
    }
    while ((pIn <
                pInLimit.offset(-((::std::mem::size_of::<reg_t>() as
                                       libc::c_ulong).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong)
                                      as isize))) as libc::c_int !=
               0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        let diff_0: reg_t =
            LZ4_read_ARCH(pMatch as *const libc::c_void) ^
                LZ4_read_ARCH(pIn as *const libc::c_void);
        if diff_0 == 0 {
            pIn =
                pIn.offset(::std::mem::size_of::<reg_t>() as libc::c_ulong as
                               isize);
            pMatch =
                pMatch.offset(::std::mem::size_of::<reg_t>() as libc::c_ulong
                                  as isize)
        } else {
            pIn = pIn.offset(LZ4_NbCommonBytes(diff_0) as isize);
            return pIn.wrapping_offset_from(pStart) as libc::c_long as
                       libc::c_uint
        }
    }
    if ::std::mem::size_of::<reg_t>() as libc::c_ulong ==
           8 as libc::c_int as libc::c_ulong &&
           pIn < pInLimit.offset(-(3 as libc::c_int as isize)) &&
           LZ4_read32(pMatch as *const libc::c_void) ==
               LZ4_read32(pIn as *const libc::c_void) {
        pIn = pIn.offset(4 as libc::c_int as isize);
        pMatch = pMatch.offset(4 as libc::c_int as isize)
    }
    if pIn < pInLimit.offset(-(1 as libc::c_int as isize)) &&
           LZ4_read16(pMatch as *const libc::c_void) as libc::c_int ==
               LZ4_read16(pIn as *const libc::c_void) as libc::c_int {
        pIn = pIn.offset(2 as libc::c_int as isize);
        pMatch = pMatch.offset(2 as libc::c_int as isize)
    }
    if pIn < pInLimit && *pMatch as libc::c_int == *pIn as libc::c_int {
        pIn = pIn.offset(1)
    }
    return pIn.wrapping_offset_from(pStart) as libc::c_long as libc::c_uint;
}
/*-************************************
*  Local Constants
**************************************/
static mut LZ4_64Klimit: libc::c_int =
    64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int) +
        (12 as libc::c_int - 1 as libc::c_int);
static mut LZ4_skipTrigger: U32 = 6 as libc::c_int as U32;
/*-************************************
*  Local Utils
**************************************/
#[no_mangle]
pub unsafe extern "C" fn LZ4_versionNumber() -> libc::c_int {
    return 1 as libc::c_int * 100 as libc::c_int * 100 as libc::c_int +
               9 as libc::c_int * 100 as libc::c_int + 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_versionString() -> *const libc::c_char {
    return b"1.9.2\x00" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressBound(mut isize: libc::c_int)
 -> libc::c_int {
    return if isize as libc::c_uint >
                  0x7e000000 as libc::c_int as libc::c_uint {
               0 as libc::c_int
           } else {
               (isize + isize / 255 as libc::c_int) + 16 as libc::c_int
           };
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_sizeofState() -> libc::c_int {
    return ((((1 as libc::c_int) << 14 as libc::c_int - 3 as libc::c_int) +
                 4 as libc::c_int +
                 (if ::std::mem::size_of::<*mut libc::c_void>() as
                         libc::c_ulong == 16 as libc::c_int as libc::c_ulong {
                      4 as libc::c_int
                  } else { 0 as libc::c_int })) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ulonglong>()
                                                as libc::c_ulong) as
               libc::c_int;
}
/*-******************************
*  Compression functions
********************************/
#[inline(always)]
unsafe extern "C" fn LZ4_hash4(mut sequence: U32, tableType: tableType_t)
 -> U32 {
    if tableType as libc::c_uint == byU16 as libc::c_int as libc::c_uint {
        return sequence.wrapping_mul(2654435761 as libc::c_uint) >>
                   4 as libc::c_int * 8 as libc::c_int -
                       (14 as libc::c_int - 2 as libc::c_int +
                            1 as libc::c_int)
    } else {
        return sequence.wrapping_mul(2654435761 as libc::c_uint) >>
                   4 as libc::c_int * 8 as libc::c_int -
                       (14 as libc::c_int - 2 as libc::c_int)
    };
}
#[inline(always)]
unsafe extern "C" fn LZ4_hash5(mut sequence: U64, tableType: tableType_t)
 -> U32 {
    let hashLog: U32 =
        if tableType as libc::c_uint == byU16 as libc::c_int as libc::c_uint {
            (14 as libc::c_int - 2 as libc::c_int) + 1 as libc::c_int
        } else { (14 as libc::c_int) - 2 as libc::c_int } as U32;
    if LZ4_isLittleEndian() != 0 {
        let prime5bytes: U64 = 889523592379 as libc::c_ulonglong as U64;
        return ((sequence << 24 as libc::c_int).wrapping_mul(prime5bytes) >>
                    (64 as libc::c_int as libc::c_uint).wrapping_sub(hashLog))
                   as U32
    } else {
        let prime8bytes: U64 =
            11400714785074694791 as libc::c_ulonglong as U64;
        return ((sequence >> 24 as libc::c_int).wrapping_mul(prime8bytes) >>
                    (64 as libc::c_int as libc::c_uint).wrapping_sub(hashLog))
                   as U32
    };
}
#[inline(always)]
unsafe extern "C" fn LZ4_hashPosition(p: *const libc::c_void,
                                      tableType: tableType_t) -> U32 {
    if ::std::mem::size_of::<reg_t>() as libc::c_ulong ==
           8 as libc::c_int as libc::c_ulong &&
           tableType as libc::c_uint != byU16 as libc::c_int as libc::c_uint {
        return LZ4_hash5(LZ4_read_ARCH(p), tableType)
    }
    return LZ4_hash4(LZ4_read32(p), tableType);
}
#[inline(always)]
unsafe extern "C" fn LZ4_clearHash(mut h: U32,
                                   mut tableBase: *mut libc::c_void,
                                   tableType: tableType_t) {
    match tableType as libc::c_uint {
        1 => {
            let mut hashTable: *mut *const BYTE =
                tableBase as *mut *const BYTE;
            let ref mut fresh0 = *hashTable.offset(h as isize);
            *fresh0 = 0 as *const BYTE;
            return
        }
        2 => {
            let mut hashTable_0: *mut U32 = tableBase as *mut U32;
            *hashTable_0.offset(h as isize) = 0 as libc::c_int as U32;
            return
        }
        3 => {
            let mut hashTable_1: *mut U16 = tableBase as *mut U16;
            *hashTable_1.offset(h as isize) = 0 as libc::c_int as U16;
            return
        }
        0 | _ => {
            /* fallthrough */
            /* illegal! */
            return
        }
    };
}
#[inline(always)]
unsafe extern "C" fn LZ4_putIndexOnHash(mut idx: U32, mut h: U32,
                                        mut tableBase: *mut libc::c_void,
                                        tableType: tableType_t) {
    match tableType as libc::c_uint {
        0 => { }
        2 => {
            let mut hashTable: *mut U32 = tableBase as *mut U32;
            *hashTable.offset(h as isize) = idx;
            return
        }
        3 => {
            let mut hashTable_0: *mut U16 = tableBase as *mut U16;
            *hashTable_0.offset(h as isize) = idx as U16;
            return
        }
        1 | _ => { }
    };
}
#[inline(always)]
unsafe extern "C" fn LZ4_putPositionOnHash(mut p: *const BYTE, mut h: U32,
                                           mut tableBase: *mut libc::c_void,
                                           tableType: tableType_t,
                                           mut srcBase: *const BYTE) {
    match tableType as libc::c_uint {
        0 => {
            /* illegal! */
            return
        }
        1 => {
            let mut hashTable: *mut *const BYTE =
                tableBase as *mut *const BYTE;
            let ref mut fresh1 = *hashTable.offset(h as isize);
            *fresh1 = p;
            return
        }
        2 => {
            let mut hashTable_0: *mut U32 = tableBase as *mut U32;
            *hashTable_0.offset(h as isize) =
                p.wrapping_offset_from(srcBase) as libc::c_long as U32;
            return
        }
        3 => {
            let mut hashTable_1: *mut U16 = tableBase as *mut U16;
            *hashTable_1.offset(h as isize) =
                p.wrapping_offset_from(srcBase) as libc::c_long as U16;
            return
        }
        _ => { }
    };
}
#[inline(always)]
unsafe extern "C" fn LZ4_putPosition(mut p: *const BYTE,
                                     mut tableBase: *mut libc::c_void,
                                     mut tableType: tableType_t,
                                     mut srcBase: *const BYTE) {
    let h: U32 = LZ4_hashPosition(p as *const libc::c_void, tableType);
    LZ4_putPositionOnHash(p, h, tableBase, tableType, srcBase);
}
/* LZ4_getIndexOnHash() :
 * Index of match position registered in hash table.
 * hash position must be calculated by using base+index, or dictBase+index.
 * Assumption 1 : only valid if tableType == byU32 or byU16.
 * Assumption 2 : h is presumed valid (within limits of hash table)
 */
#[inline(always)]
unsafe extern "C" fn LZ4_getIndexOnHash(mut h: U32,
                                        mut tableBase: *const libc::c_void,
                                        mut tableType: tableType_t) -> U32 {
    if tableType as libc::c_uint == byU32 as libc::c_int as libc::c_uint {
        let hashTable: *const U32 = tableBase as *const U32;
        return *hashTable.offset(h as isize)
    }
    if tableType as libc::c_uint == byU16 as libc::c_int as libc::c_uint {
        let hashTable_0: *const U16 = tableBase as *const U16;
        return *hashTable_0.offset(h as isize) as U32
    }
    /* forbidden case */
    return 0 as libc::c_int as U32;
}
unsafe extern "C" fn LZ4_getPositionOnHash(mut h: U32,
                                           mut tableBase: *const libc::c_void,
                                           mut tableType: tableType_t,
                                           mut srcBase: *const BYTE)
 -> *const BYTE {
    if tableType as libc::c_uint == byPtr as libc::c_int as libc::c_uint {
        let mut hashTable: *const *const BYTE =
            tableBase as *const *const BYTE;
        return *hashTable.offset(h as isize)
    }
    if tableType as libc::c_uint == byU32 as libc::c_int as libc::c_uint {
        let hashTable_0: *const U32 = tableBase as *const U32;
        return srcBase.offset(*hashTable_0.offset(h as isize) as isize)
    }
    let hashTable_1: *const U16 = tableBase as *const U16;
    return srcBase.offset(*hashTable_1.offset(h as isize) as libc::c_int as
                              isize);
    /* default, to ensure a return */
}
#[inline(always)]
unsafe extern "C" fn LZ4_getPosition(mut p: *const BYTE,
                                     mut tableBase: *const libc::c_void,
                                     mut tableType: tableType_t,
                                     mut srcBase: *const BYTE)
 -> *const BYTE {
    let h: U32 = LZ4_hashPosition(p as *const libc::c_void, tableType);
    return LZ4_getPositionOnHash(h, tableBase, tableType, srcBase);
}
#[inline(always)]
unsafe extern "C" fn LZ4_prepareTable(cctx: *mut LZ4_stream_t_internal,
                                      inputSize: libc::c_int,
                                      tableType: tableType_t) {
    /* If compression failed during the previous step, then the context
     * is marked as dirty, therefore, it has to be fully reset.
     */
    if (*cctx).dirty != 0 {
        memset(cctx as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<LZ4_stream_t_internal>() as
                   libc::c_ulong);
        return
    }
    /* If the table hasn't been used, it's guaranteed to be zeroed out, and is
     * therefore safe to use no matter what mode we're in. Otherwise, we figure
     * out if it's safe to leave as is or whether it needs to be reset.
     */
    if (*cctx).tableType as libc::c_int != clearedTable as libc::c_int {
        if (*cctx).tableType as libc::c_uint != tableType as libc::c_uint ||
               tableType as libc::c_uint ==
                   byU16 as libc::c_int as libc::c_uint &&
                   (*cctx).currentOffset.wrapping_add(inputSize as
                                                          libc::c_uint) >=
                       0xffff as libc::c_uint ||
               tableType as libc::c_uint ==
                   byU32 as libc::c_int as libc::c_uint &&
                   (*cctx).currentOffset >
                       (1 as libc::c_int as
                            libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                                           30 as libc::c_int)
               ||
               tableType as libc::c_uint ==
                   byPtr as libc::c_int as libc::c_uint ||
               inputSize >=
                   4 as libc::c_int *
                       ((1 as libc::c_int) << 10 as libc::c_int) {
            memset((*cctx).hashTable.as_mut_ptr() as *mut libc::c_void,
                   0 as libc::c_int,
                   ((1 as libc::c_int) << 14 as libc::c_int) as
                       libc::c_ulong);
            (*cctx).currentOffset = 0 as libc::c_int as uint32_t;
            (*cctx).tableType = clearedTable as libc::c_int as uint16_t
        }
    }
    /* Adding a gap, so all previous entries are > LZ4_DISTANCE_MAX back, is faster
     * than compressing without a gap. However, compressing with
     * currentOffset == 0 is faster still, so we preserve that case.
     */
    if (*cctx).currentOffset != 0 as libc::c_int as libc::c_uint &&
           tableType as libc::c_uint == byU32 as libc::c_int as libc::c_uint {
        (*cctx).currentOffset =
            ((*cctx).currentOffset as
                 libc::c_uint).wrapping_add((64 as libc::c_int *
                                                 ((1 as libc::c_int) <<
                                                      10 as libc::c_int)) as
                                                libc::c_uint) as uint32_t as
                uint32_t
    }
    /* Finally, clear history */
    (*cctx).dictCtx = 0 as *const LZ4_stream_t_internal;
    (*cctx).dictionary = 0 as *const uint8_t;
    (*cctx).dictSize = 0 as libc::c_int as uint32_t;
}
/* * LZ4_compress_generic() :
    inlined, to ensure branches are decided at compilation time */
#[inline(always)]
unsafe extern "C" fn LZ4_compress_generic(cctx: *mut LZ4_stream_t_internal,
                                          source: *const libc::c_char,
                                          dest: *mut libc::c_char,
                                          inputSize: libc::c_int,
                                          mut inputConsumed: *mut libc::c_int,
                                          maxOutputSize: libc::c_int,
                                          outputDirective:
                                              limitedOutput_directive,
                                          tableType: tableType_t,
                                          dictDirective: dict_directive,
                                          dictIssue: dictIssue_directive,
                                          acceleration: libc::c_int)
 -> libc::c_int {
    let mut result: libc::c_int =
        0; /* make indexes in dictCtx comparable with index in current context */
    let mut ip: *const BYTE =
        source as *const BYTE; /* used when dictDirective == dictSmall */
    let startIndex: U32 = (*cctx).currentOffset;
    let mut base: *const BYTE =
        (source as *const BYTE).offset(-(startIndex as isize));
    let mut lowLimit: *const BYTE = 0 as *const BYTE;
    let mut dictCtx: *const LZ4_stream_t_internal = (*cctx).dictCtx;
    let dictionary: *const BYTE =
        if dictDirective as libc::c_uint ==
               usingDictCtx as libc::c_int as libc::c_uint {
            (*dictCtx).dictionary
        } else { (*cctx).dictionary };
    let dictSize: U32 =
        if dictDirective as libc::c_uint ==
               usingDictCtx as libc::c_int as libc::c_uint {
            (*dictCtx).dictSize
        } else { (*cctx).dictSize };
    let dictDelta: U32 =
        if dictDirective as libc::c_uint ==
               usingDictCtx as libc::c_int as libc::c_uint {
            startIndex.wrapping_sub((*dictCtx).currentOffset)
        } else { 0 as libc::c_int as libc::c_uint };
    let maybe_extMem: libc::c_int =
        (dictDirective as libc::c_uint ==
             usingExtDict as libc::c_int as libc::c_uint ||
             dictDirective as libc::c_uint ==
                 usingDictCtx as libc::c_int as libc::c_uint) as libc::c_int;
    let prefixIdxLimit: U32 = startIndex.wrapping_sub(dictSize);
    let dictEnd: *const BYTE = dictionary.offset(dictSize as isize);
    let mut anchor: *const BYTE = source as *const BYTE;
    let iend: *const BYTE = ip.offset(inputSize as isize);
    let mflimitPlusOne: *const BYTE =
        iend.offset(-(12 as libc::c_int as
                          isize)).offset(1 as libc::c_int as isize);
    let matchlimit: *const BYTE = iend.offset(-(5 as libc::c_int as isize));
    /* the dictCtx currentOffset is indexed on the start of the dictionary,
     * while a dictionary in the current context precedes the currentOffset */
    let mut dictBase: *const BYTE =
        if dictDirective as libc::c_uint ==
               usingDictCtx as libc::c_int as libc::c_uint {
            dictionary.offset(dictSize as
                                  isize).offset(-((*dictCtx).currentOffset as
                                                      isize))
        } else {
            dictionary.offset(dictSize as
                                  isize).offset(-(startIndex as isize))
        };
    let mut op: *mut BYTE = dest as *mut BYTE;
    let olimit: *mut BYTE = op.offset(maxOutputSize as isize);
    let mut offset: U32 = 0 as libc::c_int as U32;
    let mut forwardH: U32 = 0;
    /* If init conditions are not met, we don't have to mark stream
     * as having dirty context, since no action was taken yet */
    if outputDirective as libc::c_uint ==
           fillOutput as libc::c_int as libc::c_uint &&
           maxOutputSize < 1 as libc::c_int {
        return 0 as libc::c_int
    } /* Impossible to store anything */
    if inputSize as U32 > 0x7e000000 as libc::c_int as U32 {
        return 0 as libc::c_int
    } /* Unsupported inputSize, too large (or negative) */
    if tableType as libc::c_uint == byU16 as libc::c_int as libc::c_uint &&
           inputSize >= LZ4_64Klimit {
        return 0 as libc::c_int
    } /* Size too large (not within 64K limit) */
    (tableType as libc::c_uint) ==
        byPtr as libc::c_int as
            libc::c_uint; /* only supported use case with byPtr */
    lowLimit =
        (source as
             *const BYTE).offset(-((if dictDirective as libc::c_uint ==
                                           withPrefix64k as libc::c_int as
                                               libc::c_uint {
                                        dictSize
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as isize));
    /* Update context state */
    if dictDirective as libc::c_uint ==
           usingDictCtx as libc::c_int as libc::c_uint {
        /* Subsequent linked blocks can't use the dictionary. */
        /* Instead, they use the block we just compressed. */
        (*cctx).dictCtx =
            0 as
                *const LZ4_stream_t_internal; /* Input too small, no compression (all literals) */
        (*cctx).dictSize = inputSize as U32
    } else {
        (*cctx).dictSize =
            ((*cctx).dictSize as libc::c_uint).wrapping_add(inputSize as U32)
                as uint32_t as uint32_t
    }
    (*cctx).currentOffset =
        ((*cctx).currentOffset as libc::c_uint).wrapping_add(inputSize as U32)
            as uint32_t as uint32_t;
    (*cctx).tableType = tableType as U16;
    if !(inputSize < LZ4_minLength) {
        /* First Byte */
        LZ4_putPosition(ip,
                        (*cctx).hashTable.as_mut_ptr() as *mut libc::c_void,
                        tableType, base);
        ip = ip.offset(1);
        forwardH = LZ4_hashPosition(ip as *const libc::c_void, tableType);
        's_185:
            loop 
                 /* Main Loop */
                 {
                let mut match_0: *const BYTE = 0 as *const BYTE;
                let mut token: *mut BYTE = 0 as *mut BYTE;
                let mut filledIp: *const BYTE = 0 as *const BYTE;
                /* Find a match */
                if tableType as libc::c_uint ==
                       byPtr as libc::c_int as libc::c_uint {
                    let mut forwardIp: *const BYTE = ip; /* byU32, byU16 */
                    let mut step: libc::c_int = 1 as libc::c_int;
                    let mut searchMatchNb: libc::c_int =
                        acceleration << LZ4_skipTrigger;
                    loop  {
                        let h: U32 = forwardH;
                        ip = forwardIp;
                        forwardIp = forwardIp.offset(step as isize);
                        let fresh2 = searchMatchNb;
                        searchMatchNb = searchMatchNb + 1;
                        step = fresh2 >> LZ4_skipTrigger;
                        if ((forwardIp > mflimitPlusOne) as libc::c_int !=
                                0 as libc::c_int) as libc::c_int as
                               libc::c_long != 0 {
                            break 's_185 ;
                        }
                        match_0 =
                            LZ4_getPositionOnHash(h,
                                                  (*cctx).hashTable.as_mut_ptr()
                                                      as *const libc::c_void,
                                                  tableType, base);
                        forwardH =
                            LZ4_hashPosition(forwardIp as *const libc::c_void,
                                             tableType);
                        LZ4_putPositionOnHash(ip, h,
                                              (*cctx).hashTable.as_mut_ptr()
                                                  as *mut libc::c_void,
                                              tableType, base);
                        if !(match_0.offset(65535 as libc::c_int as isize) <
                                 ip ||
                                 LZ4_read32(match_0 as *const libc::c_void) !=
                                     LZ4_read32(ip as *const libc::c_void)) {
                            break ;
                        }
                    }
                } else {
                    let mut forwardIp_0: *const BYTE = ip;
                    let mut step_0: libc::c_int = 1 as libc::c_int;
                    let mut searchMatchNb_0: libc::c_int =
                        acceleration << LZ4_skipTrigger;
                    loop  {
                        let h_0: U32 = forwardH;
                        let current: U32 =
                            forwardIp_0.wrapping_offset_from(base) as
                                libc::c_long as U32;
                        let mut matchIndex: U32 =
                            LZ4_getIndexOnHash(h_0,
                                               (*cctx).hashTable.as_mut_ptr()
                                                   as *const libc::c_void,
                                               tableType);
                        ip = forwardIp_0;
                        forwardIp_0 = forwardIp_0.offset(step_0 as isize);
                        let fresh3 = searchMatchNb_0;
                        searchMatchNb_0 = searchMatchNb_0 + 1;
                        step_0 = fresh3 >> LZ4_skipTrigger;
                        if ((forwardIp_0 > mflimitPlusOne) as libc::c_int !=
                                0 as libc::c_int) as libc::c_int as
                               libc::c_long != 0 {
                            break 's_185 ;
                        }
                        if dictDirective as libc::c_uint ==
                               usingDictCtx as libc::c_int as libc::c_uint {
                            if matchIndex < startIndex {
                                /* there was no match, try the dictionary */
                                matchIndex =
                                    LZ4_getIndexOnHash(h_0,
                                                       (*dictCtx).hashTable.as_ptr()
                                                           as
                                                           *const libc::c_void,
                                                       byU32); /* make dictCtx index comparable with current context */
                                match_0 =
                                    dictBase.offset(matchIndex as
                                                        isize); /* single continuous memory segment */
                                matchIndex =
                                    (matchIndex as
                                         libc::c_uint).wrapping_add(dictDelta)
                                        as U32 as
                                        U32; /* match outside of valid area */
                                lowLimit = dictionary
                            } else {
                                match_0 =
                                    base.offset(matchIndex as
                                                    isize); /* too far */
                                lowLimit = source as *const BYTE
                            }
                        } else if dictDirective as libc::c_uint ==
                                      usingExtDict as libc::c_int as
                                          libc::c_uint {
                            if matchIndex < startIndex {
                                match_0 =
                                    dictBase.offset(matchIndex as isize);
                                lowLimit = dictionary
                            } else {
                                match_0 = base.offset(matchIndex as isize);
                                lowLimit = source as *const BYTE
                            }
                        } else { match_0 = base.offset(matchIndex as isize) }
                        forwardH =
                            LZ4_hashPosition(forwardIp_0 as
                                                 *const libc::c_void,
                                             tableType);
                        LZ4_putIndexOnHash(current, h_0,
                                           (*cctx).hashTable.as_mut_ptr() as
                                               *mut libc::c_void, tableType);
                        if dictIssue as libc::c_uint ==
                               dictSmall as libc::c_int as libc::c_uint &&
                               matchIndex < prefixIdxLimit {
                            continue ;
                        }
                        if (tableType as libc::c_uint !=
                                byU16 as libc::c_int as libc::c_uint ||
                                (65535 as libc::c_int) < 65535 as libc::c_int)
                               &&
                               matchIndex.wrapping_add(65535 as libc::c_int as
                                                           libc::c_uint) <
                                   current {
                            continue ;
                        }
                        /* match now expected within distance */
                        if !(LZ4_read32(match_0 as *const libc::c_void) ==
                                 LZ4_read32(ip as *const libc::c_void)) {
                            continue ;
                        }
                        if maybe_extMem != 0 {
                            offset = current.wrapping_sub(matchIndex)
                        }
                        break ;
                        /* match found */
                    }
                }
                /* Catch up */
                filledIp = ip;
                while (ip > anchor) as libc::c_int &
                          (match_0 > lowLimit) as libc::c_int != 0 &&
                          ((*ip.offset(-(1 as libc::c_int) as isize) as
                                libc::c_int ==
                                *match_0.offset(-(1 as libc::c_int) as isize)
                                    as libc::c_int) as libc::c_int !=
                               0 as libc::c_int) as libc::c_int as
                              libc::c_long != 0 {
                    ip = ip.offset(-1);
                    match_0 = match_0.offset(-1)
                }
                /* Encode Literals */
                let litLength: libc::c_uint =
                    ip.wrapping_offset_from(anchor) as libc::c_long as
                        libc::c_uint;
                let fresh4 = op;
                op = op.offset(1);
                token = fresh4;
                if outputDirective as libc::c_uint ==
                       limitedOutput as libc::c_int as libc::c_uint &&
                       ((op.offset(litLength as
                                       isize).offset((2 as libc::c_int +
                                                          1 as libc::c_int +
                                                          5 as libc::c_int) as
                                                         isize).offset(litLength.wrapping_div(255
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint)
                                                                           as
                                                                           isize)
                             > olimit) as libc::c_int != 0 as libc::c_int) as
                           libc::c_int as libc::c_long != 0 {
                    return 0 as libc::c_int
                    /* cannot compress within `dst` budget. Stored indexes in hash table are nonetheless fine */
                }
                if outputDirective as libc::c_uint ==
                       fillOutput as libc::c_int as libc::c_uint &&
                       ((op.offset(litLength.wrapping_add(240 as libc::c_int
                                                              as
                                                              libc::c_uint).wrapping_div(255
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint)
                                       as
                                       isize).offset(litLength as
                                                         isize).offset(2 as
                                                                           libc::c_int
                                                                           as
                                                                           isize).offset(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize).offset(12
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               isize).offset(-(4
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   isize))
                             > olimit) as libc::c_int != 0 as libc::c_int) as
                           libc::c_int as libc::c_long != 0 {
                    op = op.offset(-1);
                    break ;
                } else {
                    if litLength >=
                           ((1 as libc::c_uint) <<
                                8 as libc::c_int -
                                    4 as
                                        libc::c_int).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                       {
                        let mut len: libc::c_int =
                            litLength.wrapping_sub(((1 as libc::c_uint) <<
                                                        8 as libc::c_int -
                                                            4 as
                                                                libc::c_int).wrapping_sub(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
                                as libc::c_int;
                        *token =
                            (((1 as libc::c_uint) <<
                                  8 as libc::c_int -
                                      4 as
                                          libc::c_int).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                 << 4 as libc::c_int) as BYTE;
                        while len >= 255 as libc::c_int {
                            let fresh5 = op;
                            op = op.offset(1);
                            *fresh5 = 255 as libc::c_int as BYTE;
                            len -= 255 as libc::c_int
                        }
                        let fresh6 = op;
                        op = op.offset(1);
                        *fresh6 = len as BYTE
                    } else {
                        *token = (litLength << 4 as libc::c_int) as BYTE
                    }
                    /* Copy Literals */
                    LZ4_wildCopy8(op as *mut libc::c_void,
                                  anchor as *const libc::c_void,
                                  op.offset(litLength as isize) as
                                      *mut libc::c_void);
                    op = op.offset(litLength as isize);
                    loop 
                         /* at this stage, the following variables must be correctly set :
         * - ip : at start of LZ operation
         * - match : at start of previous pattern occurence; can be within current prefix, or within extDict
         * - offset : if maybe_ext_memSegment==1 (constant)
         * - lowLimit : must be == dictionary to mean "match is within extDict"; must be == source otherwise
         * - token and *token : position to write 4-bits for match length; higher 4-bits for literal length supposed already written
         */
                         {
                        if outputDirective as libc::c_uint ==
                               fillOutput as libc::c_int as libc::c_uint &&
                               op.offset(2 as libc::c_int as
                                             isize).offset(1 as libc::c_int as
                                                               isize).offset(12
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize).offset(-(4
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize))
                                   > olimit {
                            /* the match was too close to the end, rewind and go to last literals */
                            op = token;
                            break 's_185 ;
                        } else {
                            /* Encode Offset */
                            if maybe_extMem != 0 {
                                /* static test */
                                LZ4_writeLE16(op as *mut libc::c_void,
                                              offset as U16);
                                op = op.offset(2 as libc::c_int as isize)
                            } else {
                                LZ4_writeLE16(op as *mut libc::c_void,
                                              ip.wrapping_offset_from(match_0)
                                                  as libc::c_long as U16);
                                op = op.offset(2 as libc::c_int as isize)
                            }
                            /* Encode MatchLength */
                            let mut matchCode: libc::c_uint = 0;
                            if (dictDirective as libc::c_uint ==
                                    usingExtDict as libc::c_int as
                                        libc::c_uint ||
                                    dictDirective as libc::c_uint ==
                                        usingDictCtx as libc::c_int as
                                            libc::c_uint) &&
                                   lowLimit == dictionary {
                                /* match within extDict */
                                let mut limit: *const BYTE =
                                    ip.offset(dictEnd.wrapping_offset_from(match_0)
                                                  as libc::c_long as isize);
                                if limit > matchlimit { limit = matchlimit }
                                matchCode =
                                    LZ4_count(ip.offset(4 as libc::c_int as
                                                            isize),
                                              match_0.offset(4 as libc::c_int
                                                                 as isize),
                                              limit);
                                ip =
                                    ip.offset((matchCode as
                                                   size_t).wrapping_add(4 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong)
                                                  as isize);
                                if ip == limit {
                                    let more: libc::c_uint =
                                        LZ4_count(limit,
                                                  source as *const BYTE,
                                                  matchlimit);
                                    matchCode = matchCode.wrapping_add(more);
                                    ip = ip.offset(more as isize)
                                }
                            } else {
                                matchCode =
                                    LZ4_count(ip.offset(4 as libc::c_int as
                                                            isize),
                                              match_0.offset(4 as libc::c_int
                                                                 as isize),
                                              matchlimit);
                                ip =
                                    ip.offset((matchCode as
                                                   size_t).wrapping_add(4 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong)
                                                  as isize)
                            }
                            if outputDirective as libc::c_uint != 0 &&
                                   ((op.offset((1 as libc::c_int +
                                                    5 as libc::c_int) as
                                                   isize).offset(matchCode.wrapping_add(240
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_div(255
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)
                                                                     as isize)
                                         > olimit) as libc::c_int !=
                                        0 as libc::c_int) as libc::c_int as
                                       libc::c_long != 0 {
                                if outputDirective as libc::c_uint ==
                                       fillOutput as libc::c_int as
                                           libc::c_uint {
                                    /* Match description too long : reduce it */
                                    let mut newMatchCode: U32 =
                                        ((15 as libc::c_int -
                                              1 as libc::c_int) as
                                             libc::c_uint).wrapping_add((olimit.wrapping_offset_from(op)
                                                                             as
                                                                             libc::c_long
                                                                             as
                                                                             U32).wrapping_sub(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_sub(5
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint).wrapping_mul(255
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_uint));
                                    ip =
                                        ip.offset(-(matchCode.wrapping_sub(newMatchCode)
                                                        as isize));
                                    matchCode = newMatchCode;
                                    if ((ip <= filledIp) as libc::c_int !=
                                            0 as libc::c_int) as libc::c_int
                                           as libc::c_long != 0 {
                                        /* We have already filled up to filledIp so if ip ends up less than filledIp
                         * we have positions in the hash table beyond the current position. This is
                         * a problem if we reuse the hash table. So we have to remove these positions
                         * from the hash table.
                         */
                                        let mut ptr: *const BYTE =
                                            0 as *const BYTE;
                                        ptr = ip;
                                        while ptr <= filledIp {
                                            let h_1: U32 =
                                                LZ4_hashPosition(ptr as
                                                                     *const libc::c_void,
                                                                 tableType);
                                            LZ4_clearHash(h_1,
                                                          (*cctx).hashTable.as_mut_ptr()
                                                              as
                                                              *mut libc::c_void,
                                                          tableType);
                                            ptr = ptr.offset(1)
                                        }
                                    }
                                } else {
                                    return 0 as libc::c_int
                                    /* cannot compress within `dst` budget. Stored indexes in hash table are nonetheless fine */
                                }
                            }
                            if matchCode >=
                                   ((1 as libc::c_uint) <<
                                        4 as
                                            libc::c_int).wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                               {
                                *token =
                                    (*token as
                                         libc::c_uint).wrapping_add(((1 as
                                                                          libc::c_uint)
                                                                         <<
                                                                         4 as
                                                                             libc::c_int).wrapping_sub(1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint))
                                        as BYTE as BYTE;
                                matchCode =
                                    matchCode.wrapping_sub(((1 as
                                                                 libc::c_uint)
                                                                <<
                                                                4 as
                                                                    libc::c_int).wrapping_sub(1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint));
                                LZ4_write32(op as *mut libc::c_void,
                                            0xffffffff as libc::c_uint);
                                while matchCode >=
                                          (4 as libc::c_int *
                                               255 as libc::c_int) as
                                              libc::c_uint {
                                    op = op.offset(4 as libc::c_int as isize);
                                    LZ4_write32(op as *mut libc::c_void,
                                                0xffffffff as libc::c_uint);
                                    matchCode =
                                        matchCode.wrapping_sub((4 as
                                                                    libc::c_int
                                                                    *
                                                                    255 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint)
                                }
                                op =
                                    op.offset(matchCode.wrapping_div(255 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                  as isize);
                                let fresh7 = op;
                                op = op.offset(1);
                                *fresh7 =
                                    matchCode.wrapping_rem(255 as libc::c_int
                                                               as
                                                               libc::c_uint)
                                        as BYTE
                            } else {
                                *token =
                                    (*token as libc::c_int +
                                         matchCode as BYTE as libc::c_int) as
                                        BYTE
                            }
                            /* Ensure we have enough space for the last literals. */
                            anchor = ip;
                            /* Test end of chunk */
                            if ip >= mflimitPlusOne { break 's_185 ; }
                            /* Fill table */
                            LZ4_putPosition(ip.offset(-(2 as libc::c_int as
                                                            isize)),
                                            (*cctx).hashTable.as_mut_ptr() as
                                                *mut libc::c_void, tableType,
                                            base);
                            /* Test next position */
                            if tableType as libc::c_uint ==
                                   byPtr as libc::c_int as libc::c_uint {
                                match_0 =
                                    LZ4_getPosition(ip,
                                                    (*cctx).hashTable.as_mut_ptr()
                                                        as
                                                        *const libc::c_void,
                                                    tableType,
                                                    base); /* byU32, byU16 */
                                LZ4_putPosition(ip,
                                                (*cctx).hashTable.as_mut_ptr()
                                                    as *mut libc::c_void,
                                                tableType, base);
                                if !(match_0.offset(65535 as libc::c_int as
                                                        isize) >= ip &&
                                         LZ4_read32(match_0 as
                                                        *const libc::c_void)
                                             ==
                                             LZ4_read32(ip as
                                                            *const libc::c_void))
                                   {
                                    break ;
                                }
                                let fresh8 = op;
                                op = op.offset(1);
                                token = fresh8;
                                *token = 0 as libc::c_int as BYTE
                            } else {
                                let h_2: U32 =
                                    LZ4_hashPosition(ip as
                                                         *const libc::c_void,
                                                     tableType);
                                let current_0: U32 =
                                    ip.wrapping_offset_from(base) as
                                        libc::c_long as U32;
                                let mut matchIndex_0: U32 =
                                    LZ4_getIndexOnHash(h_2,
                                                       (*cctx).hashTable.as_mut_ptr()
                                                           as
                                                           *const libc::c_void,
                                                       tableType);
                                if dictDirective as libc::c_uint ==
                                       usingDictCtx as libc::c_int as
                                           libc::c_uint {
                                    if matchIndex_0 < startIndex {
                                        /* there was no match, try the dictionary */
                                        matchIndex_0 =
                                            LZ4_getIndexOnHash(h_2,
                                                               (*dictCtx).hashTable.as_ptr()
                                                                   as
                                                                   *const libc::c_void,
                                                               byU32); /* required for match length counter */
                                        match_0 =
                                            dictBase.offset(matchIndex_0 as
                                                                isize);
                                        lowLimit = dictionary;
                                        matchIndex_0 =
                                            (matchIndex_0 as
                                                 libc::c_uint).wrapping_add(dictDelta)
                                                as U32 as U32
                                    } else {
                                        match_0 =
                                            base.offset(matchIndex_0 as
                                                            isize);
                                        lowLimit = source as *const BYTE
                                        /* required for match length counter */
                                    }
                                } else if dictDirective as libc::c_uint ==
                                              usingExtDict as libc::c_int as
                                                  libc::c_uint {
                                    if matchIndex_0 < startIndex {
                                        match_0 =
                                            dictBase.offset(matchIndex_0 as
                                                                isize);
                                        lowLimit =
                                            dictionary /* single memory segment */
                                        /* required for match length counter */
                                    } else {
                                        match_0 =
                                            base.offset(matchIndex_0 as
                                                            isize);
                                        lowLimit = source as *const BYTE
                                        /* required for match length counter */
                                    }
                                } else {
                                    match_0 =
                                        base.offset(matchIndex_0 as isize)
                                }
                                LZ4_putIndexOnHash(current_0, h_2,
                                                   (*cctx).hashTable.as_mut_ptr()
                                                       as *mut libc::c_void,
                                                   tableType);
                                if !((if dictIssue as libc::c_uint ==
                                             dictSmall as libc::c_int as
                                                 libc::c_uint {
                                          (matchIndex_0 >= prefixIdxLimit) as
                                              libc::c_int
                                      } else { 1 as libc::c_int }) != 0 &&
                                         (if tableType as libc::c_uint ==
                                                 byU16 as libc::c_int as
                                                     libc::c_uint &&
                                                 65535 as libc::c_int ==
                                                     65535 as libc::c_int {
                                              1 as libc::c_int
                                          } else {
                                              (matchIndex_0.wrapping_add(65535
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint)
                                                   >= current_0) as
                                                  libc::c_int
                                          }) != 0 &&
                                         LZ4_read32(match_0 as
                                                        *const libc::c_void)
                                             ==
                                             LZ4_read32(ip as
                                                            *const libc::c_void))
                                   {
                                    break ;
                                }
                                let fresh9 = op;
                                op = op.offset(1);
                                token = fresh9;
                                *token = 0 as libc::c_int as BYTE;
                                if maybe_extMem != 0 {
                                    offset =
                                        current_0.wrapping_sub(matchIndex_0)
                                }
                            }
                        }
                    }
                    /* Prepare next loop */
                    ip = ip.offset(1);
                    forwardH =
                        LZ4_hashPosition(ip as *const libc::c_void, tableType)
                }
            }
    }
    /* Encode Last Literals */
    let mut lastRun: size_t =
        iend.wrapping_offset_from(anchor) as libc::c_long as size_t;
    if outputDirective as libc::c_uint != 0 &&
           op.offset(lastRun as
                         isize).offset(1 as libc::c_int as
                                           isize).offset(lastRun.wrapping_add(255
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong).wrapping_sub(((1
                                                                                                                    as
                                                                                                                    libc::c_uint)
                                                                                                                   <<
                                                                                                                   8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       -
                                                                                                                       4
                                                                                                                           as
                                                                                                                           libc::c_int).wrapping_sub(1
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint)
                                                                                                                  as
                                                                                                                  libc::c_ulong).wrapping_div(255
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_ulong)
                                                             as isize) >
               olimit {
        if outputDirective as libc::c_uint ==
               fillOutput as libc::c_int as libc::c_uint {
            /* adapt lastRun to fill 'dst' */
            lastRun =
                (olimit.wrapping_offset_from(op) as libc::c_long as
                     size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong);
            lastRun =
                (lastRun as
                     libc::c_ulong).wrapping_sub(lastRun.wrapping_add(240 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong).wrapping_div(255
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_ulong))
                    as size_t as size_t
        } else {
            return 0 as libc::c_int
            /* cannot compress within `dst` budget. Stored indexes in hash table are nonetheless fine */
        }
    }
    if lastRun >=
           ((1 as libc::c_uint) <<
                8 as libc::c_int -
                    4 as
                        libc::c_int).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as
               libc::c_ulong {
        let mut accumulator: size_t =
            lastRun.wrapping_sub(((1 as libc::c_uint) <<
                                      8 as libc::c_int -
                                          4 as
                                              libc::c_int).wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                     as libc::c_ulong);
        let fresh10 = op;
        op = op.offset(1);
        *fresh10 =
            (((1 as libc::c_uint) <<
                  8 as libc::c_int -
                      4 as
                          libc::c_int).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) <<
                 4 as libc::c_int) as BYTE;
        while accumulator >= 255 as libc::c_int as libc::c_ulong {
            let fresh11 = op;
            op = op.offset(1);
            *fresh11 = 255 as libc::c_int as BYTE;
            accumulator =
                (accumulator as
                     libc::c_ulong).wrapping_sub(255 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t
        }
        let fresh12 = op;
        op = op.offset(1);
        *fresh12 = accumulator as BYTE
    } else {
        let fresh13 = op;
        op = op.offset(1);
        *fresh13 = (lastRun << 4 as libc::c_int) as BYTE
    }
    memcpy(op as *mut libc::c_void, anchor as *const libc::c_void, lastRun);
    ip = anchor.offset(lastRun as isize);
    op = op.offset(lastRun as isize);
    if outputDirective as libc::c_uint ==
           fillOutput as libc::c_int as libc::c_uint {
        *inputConsumed =
            (ip as *const libc::c_char).wrapping_offset_from(source) as
                libc::c_long as libc::c_int
    }
    result =
        (op as *mut libc::c_char).wrapping_offset_from(dest) as libc::c_long
            as libc::c_int;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_fast_extState(mut state:
                                                        *mut libc::c_void,
                                                    mut source:
                                                        *const libc::c_char,
                                                    mut dest:
                                                        *mut libc::c_char,
                                                    mut inputSize:
                                                        libc::c_int,
                                                    mut maxOutputSize:
                                                        libc::c_int,
                                                    mut acceleration:
                                                        libc::c_int)
 -> libc::c_int {
    let ctx: *mut LZ4_stream_t_internal =
        &mut (*(LZ4_initStream as
                    unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
                        ->
                            *mut LZ4_stream_t)(state,
                                               ::std::mem::size_of::<LZ4_stream_t>()
                                                   as
                                                   libc::c_ulong)).internal_donotuse;
    if acceleration < 1 as libc::c_int { acceleration = 1 as libc::c_int }
    if maxOutputSize >= LZ4_compressBound(inputSize) {
        if inputSize < LZ4_64Klimit {
            return LZ4_compress_generic(ctx, source, dest, inputSize,
                                        0 as *mut libc::c_int,
                                        0 as libc::c_int, notLimited, byU16,
                                        noDict, noDictIssue, acceleration)
        } else {
            let tableType: tableType_t =
                if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                       == 4 as libc::c_int as libc::c_ulong &&
                       source as uptrval >
                           65535 as libc::c_int as libc::c_ulong {
                    byPtr as libc::c_int
                } else { byU32 as libc::c_int } as tableType_t;
            return LZ4_compress_generic(ctx, source, dest, inputSize,
                                        0 as *mut libc::c_int,
                                        0 as libc::c_int, notLimited,
                                        tableType, noDict, noDictIssue,
                                        acceleration)
        }
    } else if inputSize < LZ4_64Klimit {
        return LZ4_compress_generic(ctx, source, dest, inputSize,
                                    0 as *mut libc::c_int, maxOutputSize,
                                    limitedOutput, byU16, noDict, noDictIssue,
                                    acceleration)
    } else {
        let tableType_0: tableType_t =
            if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong ==
                   4 as libc::c_int as libc::c_ulong &&
                   source as uptrval > 65535 as libc::c_int as libc::c_ulong {
                byPtr as libc::c_int
            } else { byU32 as libc::c_int } as tableType_t;
        return LZ4_compress_generic(ctx, source, dest, inputSize,
                                    0 as *mut libc::c_int, maxOutputSize,
                                    limitedOutput, tableType_0, noDict,
                                    noDictIssue, acceleration)
    };
}
/* *
 * LZ4_compress_fast_extState_fastReset() :
 * A variant of LZ4_compress_fast_extState().
 *
 * Using this variant avoids an expensive initialization step. It is only safe
 * to call if the state buffer is known to be correctly initialized already
 * (see comment in lz4.h on LZ4_resetStream_fast() for a definition of
 * "correctly initialized").
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_fast_extState_fastReset(mut state:
                                                                  *mut libc::c_void,
                                                              mut src:
                                                                  *const libc::c_char,
                                                              mut dst:
                                                                  *mut libc::c_char,
                                                              mut srcSize:
                                                                  libc::c_int,
                                                              mut dstCapacity:
                                                                  libc::c_int,
                                                              mut acceleration:
                                                                  libc::c_int)
 -> libc::c_int {
    let mut ctx: *mut LZ4_stream_t_internal =
        &mut (*(state as *mut LZ4_stream_t)).internal_donotuse;
    if acceleration < 1 as libc::c_int { acceleration = 1 as libc::c_int }
    if dstCapacity >= LZ4_compressBound(srcSize) {
        if srcSize < LZ4_64Klimit {
            let tableType: tableType_t = byU16;
            LZ4_prepareTable(ctx, srcSize, tableType);
            if (*ctx).currentOffset != 0 {
                return LZ4_compress_generic(ctx, src, dst, srcSize,
                                            0 as *mut libc::c_int,
                                            0 as libc::c_int, notLimited,
                                            tableType, noDict, dictSmall,
                                            acceleration)
            } else {
                return LZ4_compress_generic(ctx, src, dst, srcSize,
                                            0 as *mut libc::c_int,
                                            0 as libc::c_int, notLimited,
                                            tableType, noDict, noDictIssue,
                                            acceleration)
            }
        } else {
            let tableType_0: tableType_t =
                if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                       == 4 as libc::c_int as libc::c_ulong &&
                       src as uptrval > 65535 as libc::c_int as libc::c_ulong
                   {
                    byPtr as libc::c_int
                } else { byU32 as libc::c_int } as tableType_t;
            LZ4_prepareTable(ctx, srcSize, tableType_0);
            return LZ4_compress_generic(ctx, src, dst, srcSize,
                                        0 as *mut libc::c_int,
                                        0 as libc::c_int, notLimited,
                                        tableType_0, noDict, noDictIssue,
                                        acceleration)
        }
    } else if srcSize < LZ4_64Klimit {
        let tableType_1: tableType_t = byU16;
        LZ4_prepareTable(ctx, srcSize, tableType_1);
        if (*ctx).currentOffset != 0 {
            return LZ4_compress_generic(ctx, src, dst, srcSize,
                                        0 as *mut libc::c_int, dstCapacity,
                                        limitedOutput, tableType_1, noDict,
                                        dictSmall, acceleration)
        } else {
            return LZ4_compress_generic(ctx, src, dst, srcSize,
                                        0 as *mut libc::c_int, dstCapacity,
                                        limitedOutput, tableType_1, noDict,
                                        noDictIssue, acceleration)
        }
    } else {
        let tableType_2: tableType_t =
            if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong ==
                   4 as libc::c_int as libc::c_ulong &&
                   src as uptrval > 65535 as libc::c_int as libc::c_ulong {
                byPtr as libc::c_int
            } else { byU32 as libc::c_int } as tableType_t;
        LZ4_prepareTable(ctx, srcSize, tableType_2);
        return LZ4_compress_generic(ctx, src, dst, srcSize,
                                    0 as *mut libc::c_int, dstCapacity,
                                    limitedOutput, tableType_2, noDict,
                                    noDictIssue, acceleration)
    };
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_fast(mut source: *const libc::c_char,
                                           mut dest: *mut libc::c_char,
                                           mut inputSize: libc::c_int,
                                           mut maxOutputSize: libc::c_int,
                                           mut acceleration: libc::c_int)
 -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut ctx: LZ4_stream_t = LZ4_stream_u{table: [0; 2052],};
    let ctxPtr: *mut LZ4_stream_t = &mut ctx;
    result =
        LZ4_compress_fast_extState(ctxPtr as *mut libc::c_void, source, dest,
                                   inputSize, maxOutputSize, acceleration);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_default(mut src: *const libc::c_char,
                                              mut dst: *mut libc::c_char,
                                              mut srcSize: libc::c_int,
                                              mut maxOutputSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_fast(src, dst, srcSize, maxOutputSize,
                             1 as libc::c_int);
}
/* hidden debug function */
/* strangely enough, gcc generates faster code when this function is uncommented, even if unused */
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_fast_force(mut src: *const libc::c_char,
                                                 mut dst: *mut libc::c_char,
                                                 mut srcSize: libc::c_int,
                                                 mut dstCapacity: libc::c_int,
                                                 mut acceleration:
                                                     libc::c_int)
 -> libc::c_int {
    let mut ctx: LZ4_stream_t = LZ4_stream_u{table: [0; 2052],};
    LZ4_initStream(&mut ctx as *mut LZ4_stream_t as *mut libc::c_void,
                   ::std::mem::size_of::<LZ4_stream_t>() as libc::c_ulong);
    if srcSize < LZ4_64Klimit {
        return LZ4_compress_generic(&mut ctx.internal_donotuse, src, dst,
                                    srcSize, 0 as *mut libc::c_int,
                                    dstCapacity, limitedOutput, byU16, noDict,
                                    noDictIssue, acceleration)
    } else {
        let addrMode: tableType_t =
            if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong >
                   4 as libc::c_int as libc::c_ulong {
                byU32 as libc::c_int
            } else { byPtr as libc::c_int } as tableType_t;
        return LZ4_compress_generic(&mut ctx.internal_donotuse, src, dst,
                                    srcSize, 0 as *mut libc::c_int,
                                    dstCapacity, limitedOutput, addrMode,
                                    noDict, noDictIssue, acceleration)
    };
}
/* Note!: This function leaves the stream in an unclean/broken state!
 * It is not safe to subsequently use the same state with a _fastReset() or
 * _continue() call without resetting it. */
unsafe extern "C" fn LZ4_compress_destSize_extState(mut state:
                                                        *mut LZ4_stream_t,
                                                    mut src:
                                                        *const libc::c_char,
                                                    mut dst:
                                                        *mut libc::c_char,
                                                    mut srcSizePtr:
                                                        *mut libc::c_int,
                                                    mut targetDstSize:
                                                        libc::c_int)
 -> libc::c_int {
    let s: *mut libc::c_void =
        LZ4_initStream(state as *mut libc::c_void,
                       ::std::mem::size_of::<LZ4_stream_t>() as libc::c_ulong)
            as *mut libc::c_void;
    if targetDstSize >= LZ4_compressBound(*srcSizePtr) {
        /* compression success is guaranteed */
        return LZ4_compress_fast_extState(state as *mut libc::c_void, src,
                                          dst, *srcSizePtr, targetDstSize,
                                          1 as libc::c_int)
    } else if *srcSizePtr < LZ4_64Klimit {
        return LZ4_compress_generic(&mut (*state).internal_donotuse, src, dst,
                                    *srcSizePtr, srcSizePtr, targetDstSize,
                                    fillOutput, byU16, noDict, noDictIssue,
                                    1 as libc::c_int)
    } else {
        let addrMode: tableType_t =
            if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong ==
                   4 as libc::c_int as libc::c_ulong &&
                   src as uptrval > 65535 as libc::c_int as libc::c_ulong {
                byPtr as libc::c_int
            } else { byU32 as libc::c_int } as tableType_t;
        return LZ4_compress_generic(&mut (*state).internal_donotuse, src, dst,
                                    *srcSizePtr, srcSizePtr, targetDstSize,
                                    fillOutput, addrMode, noDict, noDictIssue,
                                    1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_destSize(mut src: *const libc::c_char,
                                               mut dst: *mut libc::c_char,
                                               mut srcSizePtr:
                                                   *mut libc::c_int,
                                               mut targetDstSize: libc::c_int)
 -> libc::c_int {
    let mut ctxBody: LZ4_stream_t = LZ4_stream_u{table: [0; 2052],};
    let mut ctx: *mut LZ4_stream_t = &mut ctxBody;
    let mut result: libc::c_int =
        LZ4_compress_destSize_extState(ctx, src, dst, srcSizePtr,
                                       targetDstSize);
    return result;
}
/*-******************************
*  Streaming functions
********************************/
#[no_mangle]
pub unsafe extern "C" fn LZ4_createStream() -> *mut LZ4_stream_t {
    let lz4s: *mut LZ4_stream_t =
        malloc(::std::mem::size_of::<LZ4_stream_t>() as libc::c_ulong) as
            *mut LZ4_stream_t;
    /* A compilation error here means LZ4_STREAMSIZE is not large enough */
    if lz4s.is_null() { return 0 as *mut LZ4_stream_t }
    LZ4_initStream(lz4s as *mut libc::c_void,
                   ::std::mem::size_of::<LZ4_stream_t>() as libc::c_ulong);
    return lz4s;
}
/* for some reason, Visual fails the aligment test on 32-bit x86 :
                     it reports an aligment of 8-bytes,
                     while actually aligning LZ4_stream_t on 4 bytes. */
unsafe extern "C" fn LZ4_stream_t_alignment() -> size_t {
    let mut t_a: C2RustUnnamed =
        C2RustUnnamed{c: 0, t: LZ4_stream_u{table: [0; 2052],},};
    return (::std::mem::size_of::<C2RustUnnamed>() as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<LZ4_stream_t>()
                                                as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_initStream(mut buffer: *mut libc::c_void,
                                        mut size: size_t)
 -> *mut LZ4_stream_t {
    if buffer.is_null() { return 0 as *mut LZ4_stream_t }
    if size < ::std::mem::size_of::<LZ4_stream_t>() as libc::c_ulong {
        return 0 as *mut LZ4_stream_t
    }
    /* for some reason, Visual fails the aligment test on 32-bit x86 :
                     it reports an aligment of 8-bytes,
                     while actually aligning LZ4_stream_t on 4 bytes. */
    if buffer as size_t &
           LZ4_stream_t_alignment().wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) != 0 {
        return 0 as *mut LZ4_stream_t
    } /* alignment check */
    memset(buffer, 0 as libc::c_int,
           ::std::mem::size_of::<LZ4_stream_t>() as libc::c_ulong);
    return buffer as *mut LZ4_stream_t;
}
/* resetStream is now deprecated,
 * prefer initStream() which is more general */
#[no_mangle]
pub unsafe extern "C" fn LZ4_resetStream(mut LZ4_stream: *mut LZ4_stream_t) {
    memset(LZ4_stream as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<LZ4_stream_t>() as
               libc::c_ulong); /* support free on NULL */
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_resetStream_fast(mut ctx: *mut LZ4_stream_t) {
    LZ4_prepareTable(&mut (*ctx).internal_donotuse, 0 as libc::c_int, byU32);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_freeStream(mut LZ4_stream: *mut LZ4_stream_t)
 -> libc::c_int {
    if LZ4_stream.is_null() { return 0 as libc::c_int }
    free(LZ4_stream as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_loadDict(mut LZ4_dict: *mut LZ4_stream_t,
                                      mut dictionary: *const libc::c_char,
                                      mut dictSize: libc::c_int)
 -> libc::c_int {
    let mut dict: *mut LZ4_stream_t_internal =
        &mut (*LZ4_dict).internal_donotuse;
    let tableType: tableType_t = byU32;
    let mut p: *const BYTE = dictionary as *const BYTE;
    let dictEnd: *const BYTE = p.offset(dictSize as isize);
    let mut base: *const BYTE = 0 as *const BYTE;
    /* It's necessary to reset the context,
     * and not just continue it with prepareTable()
     * to avoid any risk of generating overflowing matchIndex
     * when compressing using this dictionary */
    LZ4_resetStream(LZ4_dict);
    /* We always increment the offset by 64 KB, since, if the dict is longer,
     * we truncate it to the last 64k, and if it's shorter, we still want to
     * advance by a whole window length so we can provide the guarantee that
     * there are only valid offsets in the window, which allows an optimization
     * in LZ4_compress_fast_continue() where it uses noDictIssue even when the
     * dictionary isn't a full 64k. */
    (*dict).currentOffset =
        ((*dict).currentOffset as
             libc::c_uint).wrapping_add((64 as libc::c_int *
                                             ((1 as libc::c_int) <<
                                                  10 as libc::c_int)) as
                                            libc::c_uint) as uint32_t as
            uint32_t;
    if dictSize <
           ::std::mem::size_of::<reg_t>() as libc::c_ulong as libc::c_int {
        return 0 as libc::c_int
    }
    if dictEnd.wrapping_offset_from(p) as libc::c_long >
           (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
               libc::c_long {
        p =
            dictEnd.offset(-((64 as libc::c_int *
                                  ((1 as libc::c_int) << 10 as libc::c_int))
                                 as isize))
    }
    base = dictEnd.offset(-((*dict).currentOffset as isize));
    (*dict).dictionary = p;
    (*dict).dictSize = dictEnd.wrapping_offset_from(p) as libc::c_long as U32;
    (*dict).tableType = tableType as uint16_t;
    while p <=
              dictEnd.offset(-(::std::mem::size_of::<reg_t>() as libc::c_ulong
                                   as isize)) {
        LZ4_putPosition(p,
                        (*dict).hashTable.as_mut_ptr() as *mut libc::c_void,
                        tableType, base);
        p = p.offset(3 as libc::c_int as isize)
    }
    return (*dict).dictSize as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_attach_dictionary(mut workingStream:
                                                   *mut LZ4_stream_t,
                                               mut dictionaryStream:
                                                   *const LZ4_stream_t) {
    let mut dictCtx: *const LZ4_stream_t_internal =
        if dictionaryStream.is_null() {
            0 as *const LZ4_stream_t_internal
        } else { &(*dictionaryStream).internal_donotuse };
    /* Calling LZ4_resetStream_fast() here makes sure that changes will not be
     * erased by subsequent calls to LZ4_resetStream_fast() in case stream was
     * marked as having dirty context, e.g. requiring full reset.
     */
    LZ4_resetStream_fast(workingStream);
    if !dictCtx.is_null() {
        /* If the current offset is zero, we will never look in the
         * external dictionary context, since there is no value a table
         * entry can take that indicate a miss. In that case, we need
         * to bump the offset to something non-zero.
         */
        if (*workingStream).internal_donotuse.currentOffset ==
               0 as libc::c_int as libc::c_uint {
            (*workingStream).internal_donotuse.currentOffset =
                (64 as libc::c_int *
                     ((1 as libc::c_int) << 10 as libc::c_int)) as uint32_t
        }
        /* Don't actually attach an empty dictionary.
         */
        if (*dictCtx).dictSize == 0 as libc::c_int as libc::c_uint {
            dictCtx = 0 as *const LZ4_stream_t_internal
        }
    }
    (*workingStream).internal_donotuse.dictCtx = dictCtx;
}
unsafe extern "C" fn LZ4_renormDictT(mut LZ4_dict: *mut LZ4_stream_t_internal,
                                     mut nextSize: libc::c_int) {
    if (*LZ4_dict).currentOffset.wrapping_add(nextSize as libc::c_uint) >
           0x80000000 as libc::c_uint {
        /* potential ptrdiff_t overflow (32-bits mode) */
        /* rescale hash table */
        let delta: U32 =
            (*LZ4_dict).currentOffset.wrapping_sub((64 as libc::c_int *
                                                        ((1 as libc::c_int) <<
                                                             10 as
                                                                 libc::c_int))
                                                       as
                                                       libc::c_uint); /* Uninitialized structure detected */
        let mut dictEnd: *const BYTE =
            (*LZ4_dict).dictionary.offset((*LZ4_dict).dictSize as
                                              isize); /* avoid index overflow */
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << 14 as libc::c_int - 2 as libc::c_int {
            if (*LZ4_dict).hashTable[i as usize] < delta {
                (*LZ4_dict).hashTable[i as usize] =
                    0 as libc::c_int as uint32_t
            } else {
                (*LZ4_dict).hashTable[i as usize] =
                    ((*LZ4_dict).hashTable[i as usize] as
                         libc::c_uint).wrapping_sub(delta) as uint32_t as
                        uint32_t
            }
            i += 1
        }
        (*LZ4_dict).currentOffset =
            (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
                uint32_t;
        if (*LZ4_dict).dictSize >
               (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                   as libc::c_uint {
            (*LZ4_dict).dictSize =
                (64 as libc::c_int *
                     ((1 as libc::c_int) << 10 as libc::c_int)) as uint32_t
        }
        (*LZ4_dict).dictionary =
            dictEnd.offset(-((*LZ4_dict).dictSize as isize))
    };
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_fast_continue(mut LZ4_stream:
                                                        *mut LZ4_stream_t,
                                                    mut source:
                                                        *const libc::c_char,
                                                    mut dest:
                                                        *mut libc::c_char,
                                                    mut inputSize:
                                                        libc::c_int,
                                                    mut maxOutputSize:
                                                        libc::c_int,
                                                    mut acceleration:
                                                        libc::c_int)
 -> libc::c_int {
    let tableType: tableType_t = byU32;
    let mut streamPtr: *mut LZ4_stream_t_internal =
        &mut (*LZ4_stream).internal_donotuse;
    let mut dictEnd: *const BYTE =
        (*streamPtr).dictionary.offset((*streamPtr).dictSize as isize);
    if (*streamPtr).dirty != 0 { return 0 as libc::c_int }
    LZ4_renormDictT(streamPtr, inputSize);
    if acceleration < 1 as libc::c_int { acceleration = 1 as libc::c_int }
    /* invalidate tiny dictionaries */
    if (*streamPtr).dictSize.wrapping_sub(1 as libc::c_int as libc::c_uint) <
           (4 as libc::c_int - 1 as libc::c_int) as libc::c_uint &&
           dictEnd != source as *const BYTE {
        (*streamPtr).dictSize = 0 as libc::c_int as uint32_t;
        (*streamPtr).dictionary = source as *const BYTE;
        dictEnd = source as *const BYTE
    }
    /* Check overlapping input/dictionary space */
    let mut sourceEnd: *const BYTE =
        (source as *const BYTE).offset(inputSize as isize);
    if sourceEnd > (*streamPtr).dictionary && sourceEnd < dictEnd {
        (*streamPtr).dictSize =
            dictEnd.wrapping_offset_from(sourceEnd) as libc::c_long as U32;
        if (*streamPtr).dictSize >
               (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                   as libc::c_uint {
            (*streamPtr).dictSize =
                (64 as libc::c_int *
                     ((1 as libc::c_int) << 10 as libc::c_int)) as uint32_t
        }
        if (*streamPtr).dictSize < 4 as libc::c_int as libc::c_uint {
            (*streamPtr).dictSize = 0 as libc::c_int as uint32_t
        }
        (*streamPtr).dictionary =
            dictEnd.offset(-((*streamPtr).dictSize as isize))
    }
    /* prefix mode : source data follows dictionary */
    if dictEnd == source as *const BYTE {
        if (*streamPtr).dictSize <
               (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                   as libc::c_uint &&
               (*streamPtr).dictSize < (*streamPtr).currentOffset {
            return LZ4_compress_generic(streamPtr, source, dest, inputSize,
                                        0 as *mut libc::c_int, maxOutputSize,
                                        limitedOutput, tableType,
                                        withPrefix64k, dictSmall,
                                        acceleration)
        } else {
            return LZ4_compress_generic(streamPtr, source, dest, inputSize,
                                        0 as *mut libc::c_int, maxOutputSize,
                                        limitedOutput, tableType,
                                        withPrefix64k, noDictIssue,
                                        acceleration)
        }
    }
    /* external dictionary mode */
    let mut result: libc::c_int = 0;
    if !(*streamPtr).dictCtx.is_null() {
        /* We depend here on the fact that dictCtx'es (produced by
             * LZ4_loadDict) guarantee that their tables contain no references
             * to offsets between dictCtx->currentOffset - 64 KB and
             * dictCtx->currentOffset - dictCtx->dictSize. This makes it safe
             * to use noDictIssue even when the dict isn't a full 64 KB.
             */
        if inputSize >
               4 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int) {
            /* For compressing large blobs, it is faster to pay the setup
                 * cost to copy the dictionary's tables into the active context,
                 * so that the compression loop is only looking into one table.
                 */
            memcpy(streamPtr as *mut libc::c_void,
                   (*streamPtr).dictCtx as *const libc::c_void,
                   ::std::mem::size_of::<LZ4_stream_t>() as libc::c_ulong);
            result =
                LZ4_compress_generic(streamPtr, source, dest, inputSize,
                                     0 as *mut libc::c_int, maxOutputSize,
                                     limitedOutput, tableType, usingExtDict,
                                     noDictIssue, acceleration)
        } else {
            result =
                LZ4_compress_generic(streamPtr, source, dest, inputSize,
                                     0 as *mut libc::c_int, maxOutputSize,
                                     limitedOutput, tableType, usingDictCtx,
                                     noDictIssue, acceleration)
        }
    } else if (*streamPtr).dictSize <
                  (64 as libc::c_int *
                       ((1 as libc::c_int) << 10 as libc::c_int)) as
                      libc::c_uint &&
                  (*streamPtr).dictSize < (*streamPtr).currentOffset {
        result =
            LZ4_compress_generic(streamPtr, source, dest, inputSize,
                                 0 as *mut libc::c_int, maxOutputSize,
                                 limitedOutput, tableType, usingExtDict,
                                 dictSmall, acceleration)
    } else {
        result =
            LZ4_compress_generic(streamPtr, source, dest, inputSize,
                                 0 as *mut libc::c_int, maxOutputSize,
                                 limitedOutput, tableType, usingExtDict,
                                 noDictIssue, acceleration)
    }
    (*streamPtr).dictionary = source as *const BYTE;
    (*streamPtr).dictSize = inputSize as U32;
    return result;
}
/*-************************************
*  Internal Definitions used in Tests
**************************************/
/* Hidden debug function, to force-test external dictionary mode */
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_forceExtDict(mut LZ4_dict:
                                                       *mut LZ4_stream_t,
                                                   mut source:
                                                       *const libc::c_char,
                                                   mut dest:
                                                       *mut libc::c_char,
                                                   mut srcSize: libc::c_int)
 -> libc::c_int {
    let mut streamPtr: *mut LZ4_stream_t_internal =
        &mut (*LZ4_dict).internal_donotuse;
    let mut result: libc::c_int = 0;
    LZ4_renormDictT(streamPtr, srcSize);
    if (*streamPtr).dictSize <
           (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
               libc::c_uint &&
           (*streamPtr).dictSize < (*streamPtr).currentOffset {
        result =
            LZ4_compress_generic(streamPtr, source, dest, srcSize,
                                 0 as *mut libc::c_int, 0 as libc::c_int,
                                 notLimited, byU32, usingExtDict, dictSmall,
                                 1 as libc::c_int)
    } else {
        result =
            LZ4_compress_generic(streamPtr, source, dest, srcSize,
                                 0 as *mut libc::c_int, 0 as libc::c_int,
                                 notLimited, byU32, usingExtDict, noDictIssue,
                                 1 as libc::c_int)
    }
    (*streamPtr).dictionary = source as *const BYTE;
    (*streamPtr).dictSize = srcSize as U32;
    return result;
}
/* ! LZ4_saveDict() :
 *  If previously compressed data block is not guaranteed to remain available at its memory location,
 *  save it into a safer place (char* safeBuffer).
 *  Note : you don't need to call LZ4_loadDict() afterwards,
 *         dictionary is immediately usable, you can therefore call LZ4_compress_fast_continue().
 *  Return : saved dictionary size in bytes (necessarily <= dictSize), or 0 if error.
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4_saveDict(mut LZ4_dict: *mut LZ4_stream_t,
                                      mut safeBuffer: *mut libc::c_char,
                                      mut dictSize: libc::c_int)
 -> libc::c_int {
    let dict: *mut LZ4_stream_t_internal =
        &mut (*LZ4_dict).internal_donotuse; /* useless to define a dictionary > 64 KB */
    let previousDictEnd: *const BYTE =
        (*dict).dictionary.offset((*dict).dictSize as isize);
    if dictSize as U32 >
           (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
               libc::c_uint {
        dictSize =
            64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)
    }
    if dictSize as U32 > (*dict).dictSize {
        dictSize = (*dict).dictSize as libc::c_int
    }
    memmove(safeBuffer as *mut libc::c_void,
            previousDictEnd.offset(-(dictSize as isize)) as
                *const libc::c_void, dictSize as libc::c_ulong);
    (*dict).dictionary = safeBuffer as *const BYTE;
    (*dict).dictSize = dictSize as U32;
    return dictSize;
}
#[inline(always)]
unsafe extern "C" fn read_variable_length(mut ip: *mut *const BYTE,
                                          mut lencheck: *const BYTE,
                                          mut loop_check: libc::c_int,
                                          mut initial_check: libc::c_int,
                                          mut error:
                                              *mut variable_length_error)
 -> libc::c_uint {
    let mut length: U32 = 0 as libc::c_int as U32;
    let mut s: U32 = 0;
    if initial_check != 0 &&
           ((*ip >= lencheck) as libc::c_int != 0 as libc::c_int) as
               libc::c_int as libc::c_long != 0 {
        /* overflow detection */
        *error = initial_error;
        return length
    }
    loop  {
        s = **ip as U32;
        *ip = (*ip).offset(1);
        length = (length as libc::c_uint).wrapping_add(s) as U32 as U32;
        if loop_check != 0 &&
               ((*ip >= lencheck) as libc::c_int != 0 as libc::c_int) as
                   libc::c_int as libc::c_long != 0 {
            /* overflow detection */
            *error = loop_error;
            return length
        }
        if !(s == 255 as libc::c_int as libc::c_uint) { break ; }
    }
    return length;
}
/* ! LZ4_decompress_generic() :
 *  This generic decompression function covers all use cases.
 *  It shall be instantiated several times, using different sets of directives.
 *  Note that it is important for performance that this function really get inlined,
 *  in order to remove useless branches during compilation optimization.
 */
#[inline(always)]
unsafe extern "C" fn LZ4_decompress_generic(src: *const libc::c_char,
                                            dst: *mut libc::c_char,
                                            mut srcSize: libc::c_int,
                                            mut outputSize: libc::c_int,
                                            mut endOnInput:
                                                endCondition_directive,
                                            mut partialDecoding:
                                                earlyEnd_directive,
                                            mut dict: dict_directive,
                                            lowPrefix: *const BYTE,
                                            dictStart: *const BYTE,
                                            dictSize: size_t) -> libc::c_int 
 /* note : = 0 if noDict */
 {
    let mut current_block: u64;
    if src.is_null() { return -(1 as libc::c_int) }
    let mut ip: *const BYTE = src as *const BYTE;
    let iend: *const BYTE = ip.offset(srcSize as isize);
    let mut op: *mut BYTE = dst as *mut BYTE;
    let oend: *mut BYTE = op.offset(outputSize as isize);
    let mut cpy: *mut BYTE = 0 as *mut BYTE;
    let dictEnd: *const BYTE =
        if dictStart.is_null() {
            0 as *const BYTE
        } else { dictStart.offset(dictSize as isize) };
    let safeDecode: libc::c_int =
        (endOnInput as libc::c_uint ==
             endOnInputSize as libc::c_int as libc::c_uint) as libc::c_int;
    let checkOffset: libc::c_int =
        (safeDecode != 0 &&
             dictSize <
                 (64 as libc::c_int *
                      ((1 as libc::c_int) << 10 as libc::c_int)) as
                     libc::c_ulong) as libc::c_int;
    /* Set up the "end" pointers for the shortcut. */
    let shortiend: *const BYTE =
        iend.offset(-((if endOnInput as libc::c_uint != 0 {
                           14 as libc::c_int
                       } else { 8 as libc::c_int }) as
                          isize)).offset(-(2 as libc::c_int as isize));
    let shortoend: *const BYTE =
        oend.offset(-((if endOnInput as libc::c_uint != 0 {
                           14 as libc::c_int
                       } else { 8 as libc::c_int }) as
                          isize)).offset(-(18 as libc::c_int as isize));
    let mut match_0: *const BYTE = 0 as *const BYTE;
    let mut offset: size_t = 0;
    let mut token: libc::c_uint = 0;
    let mut length: size_t = 0;
    /*offset*/
    /*maxML*/
    /* Special cases */
    if endOnInput as libc::c_uint != 0 &&
           ((outputSize == 0 as libc::c_int) as libc::c_int !=
                0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        /* Empty output buffer */
        if partialDecoding as u64 != 0 { return 0 as libc::c_int }
        return if srcSize == 1 as libc::c_int &&
                      *ip as libc::c_int == 0 as libc::c_int {
                   0 as libc::c_int
               } else { -(1 as libc::c_int) }
    }
    if endOnInput as u64 == 0 &&
           ((outputSize == 0 as libc::c_int) as libc::c_int !=
                0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return if *ip as libc::c_int == 0 as libc::c_int {
                   1 as libc::c_int
               } else { -(1 as libc::c_int) }
    }
    if endOnInput as libc::c_uint != 0 &&
           ((srcSize == 0 as libc::c_int) as libc::c_int != 0 as libc::c_int)
               as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int)
    }
    /* Currently the fast loop shows a regression on qualcomm arm chips. */
    if (oend.wrapping_offset_from(op) as libc::c_long) <
           64 as libc::c_int as libc::c_long {
        /* Main Loop : decode remaining sequences where output < FASTLOOP_SAFE_DISTANCE */
        current_block = 248631179418912492;
    } else {
        /* Fast loop : decode sequences as long as output < iend-FASTLOOP_SAFE_DISTANCE */
        loop  {
            (endOnInput as u64) != 0;
            let fresh14 = ip;
            ip = ip.offset(1);
            token = *fresh14 as libc::c_uint;
            /* wildcopy correction */
            length =
                (token >> 4 as libc::c_int) as size_t; /* literal length */
            /* ip < iend before the increment */
            /* decode literal length */
            if length ==
                   ((1 as libc::c_uint) <<
                        8 as libc::c_int -
                            4 as
                                libc::c_int).wrapping_sub(1 as libc::c_int as
                                                              libc::c_uint) as
                       libc::c_ulong {
                let mut error: variable_length_error =
                    ok; /* overflow detection */
                length =
                    (length as
                         libc::c_ulong).wrapping_add(read_variable_length(&mut ip,
                                                                          iend.offset(-(((1
                                                                                              as
                                                                                              libc::c_uint)
                                                                                             <<
                                                                                             8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 -
                                                                                                 4
                                                                                                     as
                                                                                                     libc::c_int).wrapping_sub(1
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   libc::c_uint)
                                                                                            as
                                                                                            isize)),
                                                                          endOnInput
                                                                              as
                                                                              libc::c_int,
                                                                          endOnInput
                                                                              as
                                                                              libc::c_int,
                                                                          &mut error)
                                                         as libc::c_ulong) as
                        size_t as size_t; /* overflow detection */
                if error as libc::c_int == initial_error as libc::c_int {
                    current_block = 848226666458327087;
                    break ;
                }
                if safeDecode != 0 &&
                       (((op as uptrval).wrapping_add(length) < op as uptrval)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                           as libc::c_long != 0 {
                    current_block = 848226666458327087;
                    break ;
                }
                if safeDecode != 0 &&
                       (((ip as uptrval).wrapping_add(length) < ip as uptrval)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                           as libc::c_long != 0 {
                    current_block = 848226666458327087;
                    break ;
                }
                /* copy literals */
                cpy = op.offset(length as isize); /* LZ4_decompress_fast() */
                if endOnInput as u64 != 0 {
                    if cpy > oend.offset(-(32 as libc::c_int as isize)) ||
                           ip.offset(length as isize) >
                               iend.offset(-(32 as libc::c_int as isize)) {
                        current_block = 2103801789718498838;
                        break ;
                    }
                    LZ4_wildCopy32(op as *mut libc::c_void,
                                   ip as *const libc::c_void,
                                   cpy as *mut libc::c_void);
                } else {
                    if cpy > oend.offset(-(8 as libc::c_int as isize)) {
                        current_block = 2103801789718498838;
                        break ;
                        /* LZ4_decompress_safe() */
                        /* LZ4_decompress_fast() cannot copy more than 8 bytes at a time :
                                                 * it doesn't know input length, and only relies on end-of-block properties */
                    } /* LZ4_decompress_fast() */
                    LZ4_wildCopy8(op as *mut libc::c_void,
                                  ip as *const libc::c_void,
                                  cpy as *mut libc::c_void);
                }
                ip = ip.offset(length as isize);
                op = cpy
            } else {
                cpy = op.offset(length as isize);
                if endOnInput as u64 != 0 {
                    if ip >
                           iend.offset(-((16 as libc::c_int +
                                              1 as libc::c_int) as isize)) {
                        current_block = 2103801789718498838;
                        break ;
                    }
                    /* LZ4_decompress_safe() */
                    /* We don't need to check oend, since we check it once for each loop below */
                    /* Literals can only be 14, but hope compilers optimize if we copy by a register size */
                    memcpy(op as *mut libc::c_void, ip as *const libc::c_void,
                           16 as libc::c_int as libc::c_ulong);
                } else {
                    /* LZ4_decompress_fast() cannot copy more than 8 bytes at a time :
                     * it doesn't know input length, and relies on end-of-block properties */
                    memcpy(op as *mut libc::c_void, ip as *const libc::c_void,
                           8 as libc::c_int as libc::c_ulong);
                    if length > 8 as libc::c_int as libc::c_ulong {
                        memcpy(op.offset(8 as libc::c_int as isize) as
                                   *mut libc::c_void,
                               ip.offset(8 as libc::c_int as isize) as
                                   *const libc::c_void,
                               8 as libc::c_int as libc::c_ulong);
                    }
                }
                ip = ip.offset(length as isize);
                op = cpy
            }
            /* get offset */
            offset = LZ4_readLE16(ip as *const libc::c_void) as size_t;
            ip = ip.offset(2 as libc::c_int as isize);
            match_0 = op.offset(-(offset as isize));
            /* get matchlength */
            length =
                (token &
                     ((1 as libc::c_uint) <<
                          4 as
                              libc::c_int).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint)) as
                    size_t; /* Error : offset outside buffers */
            if length ==
                   ((1 as libc::c_uint) <<
                        4 as
                            libc::c_int).wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint) as
                       libc::c_ulong {
                let mut error_0: variable_length_error =
                    ok; /* overflow detection */
                if checkOffset != 0 &&
                       ((match_0.offset(dictSize as isize) < lowPrefix) as
                            libc::c_int != 0 as libc::c_int) as libc::c_int as
                           libc::c_long != 0 {
                    current_block = 848226666458327087;
                    break ;
                }
                length =
                    (length as
                         libc::c_ulong).wrapping_add(read_variable_length(&mut ip,
                                                                          iend.offset(-(5
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)).offset(1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               isize),
                                                                          endOnInput
                                                                              as
                                                                              libc::c_int,
                                                                          0 as
                                                                              libc::c_int,
                                                                          &mut error_0)
                                                         as libc::c_ulong) as
                        size_t as size_t;
                if error_0 as libc::c_int != ok as libc::c_int {
                    current_block = 848226666458327087;
                    break ;
                }
                if safeDecode != 0 &&
                       (((op as uptrval).wrapping_add(length) < op as uptrval)
                            as libc::c_int != 0 as libc::c_int) as libc::c_int
                           as libc::c_long != 0 {
                    current_block = 848226666458327087;
                    break ;
                }
                length =
                    (length as
                         libc::c_ulong).wrapping_add(4 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                if op.offset(length as isize) >=
                       oend.offset(-(64 as libc::c_int as isize)) {
                    current_block = 8094010457889587865;
                    break ;
                }
            } else {
                length =
                    (length as
                         libc::c_ulong).wrapping_add(4 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                if op.offset(length as isize) >=
                       oend.offset(-(64 as libc::c_int as isize)) {
                    current_block = 8094010457889587865;
                    break ;
                }
                /* Fastpath check: Avoids a branch in LZ4_wildCopy32 if true */
                if dict as libc::c_uint ==
                       withPrefix64k as libc::c_int as libc::c_uint ||
                       match_0 >= lowPrefix {
                    if offset >= 8 as libc::c_int as libc::c_ulong {
                        memcpy(op as *mut libc::c_void,
                               match_0 as *const libc::c_void,
                               8 as libc::c_int as
                                   libc::c_ulong); /* Error : offset outside buffers */
                        memcpy(op.offset(8 as libc::c_int as isize) as
                                   *mut libc::c_void,
                               match_0.offset(8 as libc::c_int as isize) as
                                   *const libc::c_void,
                               8 as libc::c_int as libc::c_ulong);
                        memcpy(op.offset(16 as libc::c_int as isize) as
                                   *mut libc::c_void,
                               match_0.offset(16 as libc::c_int as isize) as
                                   *const libc::c_void,
                               2 as libc::c_int as libc::c_ulong);
                        op = op.offset(length as isize);
                        continue ;
                    }
                }
            }
            if checkOffset != 0 &&
                   ((match_0.offset(dictSize as isize) < lowPrefix) as
                        libc::c_int != 0 as libc::c_int) as libc::c_int as
                       libc::c_long != 0 {
                current_block = 848226666458327087;
                break ;
            }
            if dict as libc::c_uint ==
                   usingExtDict as libc::c_int as libc::c_uint &&
                   match_0 < lowPrefix {
                if ((op.offset(length as isize) >
                         oend.offset(-(5 as libc::c_int as isize))) as
                        libc::c_int != 0 as libc::c_int) as libc::c_int as
                       libc::c_long != 0 {
                    if partialDecoding as u64 != 0 {
                        length =
                            if length <
                                   oend.wrapping_offset_from(op) as
                                       libc::c_long as size_t {
                                length
                            } else {
                                oend.wrapping_offset_from(op) as libc::c_long
                                    as size_t
                            }
                        /* match starting within external dictionary */
                        /* reach end of buffer */
                    } else {
                        current_block = 848226666458327087;
                        break ;
                        /* end-of-block condition violated */
                    }
                }
                if length <=
                       lowPrefix.wrapping_offset_from(match_0) as libc::c_long
                           as size_t {
                    /* match fits entirely within external dictionary : just copy */
                    memmove(op as *mut libc::c_void,
                            dictEnd.offset(-(lowPrefix.wrapping_offset_from(match_0)
                                                 as libc::c_long as isize)) as
                                *const libc::c_void, length);
                    op = op.offset(length as isize)
                } else {
                    /* match stretches into both external dictionary and current block */
                    let copySize: size_t =
                        lowPrefix.wrapping_offset_from(match_0) as
                            libc::c_long as size_t;
                    let restSize: size_t = length.wrapping_sub(copySize);
                    memcpy(op as *mut libc::c_void,
                           dictEnd.offset(-(copySize as isize)) as
                               *const libc::c_void, copySize);
                    op = op.offset(copySize as isize);
                    if restSize >
                           op.wrapping_offset_from(lowPrefix) as libc::c_long
                               as size_t {
                        /* overlap copy */
                        let endOfMatch: *mut BYTE =
                            op.offset(restSize as isize);
                        let mut copyFrom: *const BYTE = lowPrefix;
                        while op < endOfMatch {
                            let fresh15 = copyFrom;
                            copyFrom = copyFrom.offset(1);
                            let fresh16 = op;
                            op = op.offset(1);
                            *fresh16 = *fresh15
                        }
                    } else {
                        memcpy(op as *mut libc::c_void,
                               lowPrefix as *const libc::c_void, restSize);
                        op = op.offset(restSize as isize)
                    }
                }
            } else {
                /* copy match within block */
                cpy = op.offset(length as isize);
                if ((offset < 16 as libc::c_int as libc::c_ulong) as
                        libc::c_int != 0 as libc::c_int) as libc::c_int as
                       libc::c_long != 0 {
                    LZ4_memcpy_using_offset(op, match_0, cpy, offset);
                } else {
                    LZ4_wildCopy32(op as *mut libc::c_void,
                                   match_0 as *const libc::c_void,
                                   cpy as *mut libc::c_void);
                }
                op = cpy
            }
        }
    }
    loop  {
        match current_block {
            848226666458327087 =>
            /* Overflow error detected */
            {
                return -((ip as *const libc::c_char).wrapping_offset_from(src)
                             as libc::c_long) as libc::c_int -
                           1 as libc::c_int
            }
            2103801789718498838 => {
                if endOnInput as libc::c_uint != 0 &&
                       (cpy > oend.offset(-(12 as libc::c_int as isize)) ||
                            ip.offset(length as isize) >
                                iend.offset(-((2 as libc::c_int +
                                                   1 as libc::c_int +
                                                   5 as libc::c_int) as
                                                  isize))) ||
                       endOnInput as u64 == 0 &&
                           cpy > oend.offset(-(8 as libc::c_int as isize)) {
                    /* We've either hit the input parsing restriction or the output parsing restriction.
                 * If we've hit the input parsing condition then this must be the last sequence.
                 * If we've hit the output parsing condition then we are either using partialDecoding
                 * or we've hit the output parsing condition.
                 */
                    if partialDecoding as u64 != 0 {
                        /* Since we are partial decoding we may be in this block because of the output parsing
                     * restriction, which is not valid since the output buffer is allowed to be undersized.
                     */
                        /* If we're in this block because of the input parsing condition, then we must be on the
                     * last sequence (or invalid), so we must check that we exactly consume the input.
                     */
                        if ip.offset(length as isize) >
                               iend.offset(-((2 as libc::c_int +
                                                  1 as libc::c_int +
                                                  5 as libc::c_int) as isize))
                               && ip.offset(length as isize) != iend {
                            current_block = 848226666458327087;
                            continue ;
                        }
                        /* We are finishing in the middle of a literals segment.
                     * Break after the copy.
                     */
                        if cpy > oend {
                            cpy = oend;
                            length =
                                oend.wrapping_offset_from(op) as libc::c_long
                                    as size_t
                        }
                    } else {
                        /* We must be on the last sequence because of the parsing limitations so check
                     * that we exactly regenerate the original size (must be exact when !endOnInput).
                     */
                        if endOnInput as u64 == 0 && cpy != oend {
                            current_block = 848226666458327087;
                            continue ;
                        }
                        /* We must be on the last sequence (or invalid) because of the parsing limitations
                      * so check that we exactly consume the input and don't overrun the output buffer.
                      */
                        if endOnInput as libc::c_uint != 0 &&
                               (ip.offset(length as isize) != iend ||
                                    cpy > oend) {
                            current_block =
                                848226666458327087; /* supports overlapping memory regions, which only matters for in-place decompression scenarios */
                            continue ;
                        }
                    }
                    memmove(op as *mut libc::c_void,
                            ip as *const libc::c_void, length);
                    ip = ip.offset(length as isize);
                    op = op.offset(length as isize);
                    /* Necessarily EOF when !partialDecoding. When partialDecoding
                 * it is EOF if we've either filled the output buffer or hit
                 * the input parsing restriction.
                 */
                    if partialDecoding as u64 == 0 || cpy == oend ||
                           ip == iend {
                        break
                            ; /* may overwrite up to WILDCOPYLENGTH beyond cpy */
                    }
                } else {
                    LZ4_wildCopy8(op as *mut libc::c_void,
                                  ip as *const libc::c_void,
                                  cpy as *mut libc::c_void);
                    ip = ip.offset(length as isize);
                    op = cpy
                }
                /* get offset */
                offset = LZ4_readLE16(ip as *const libc::c_void) as size_t;
                ip = ip.offset(2 as libc::c_int as isize);
                match_0 = op.offset(-(offset as isize));
                /* get matchlength */
                length =
                    (token &
                         ((1 as libc::c_uint) <<
                              4 as
                                  libc::c_int).wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_uint))
                        as size_t
            }
            248631179418912492 => {
                let fresh17 = ip;
                ip = ip.offset(1);
                token = *fresh17 as libc::c_uint;
                /* wildcopy correction */
                length =
                    (token >> 4 as libc::c_int) as
                        size_t; /* literal length */
                /* ip < iend before the increment */
                /* A two-stage shortcut for the most common case:
             * 1) If the literal length is 0..14, and there is enough space,
             * enter the shortcut and copy 16 bytes on behalf of the literals
             * (in the fast mode, only 8 bytes can be safely copied this way).
             * 2) Further if the match length is 4..18, copy 18 bytes in a similar
             * manner; but we ensure that there's enough space in the output for
             * those 18 bytes earlier, upon entering the shortcut (in other words,
             * there is a combined check for both stages).
             */
                if (if endOnInput as libc::c_uint != 0 {
                        (length !=
                             ((1 as libc::c_uint) <<
                                  8 as libc::c_int -
                                      4 as
                                          libc::c_int).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                 as libc::c_ulong) as libc::c_int
                    } else {
                        (length <= 8 as libc::c_int as libc::c_ulong) as
                            libc::c_int
                    }) != 0 &&
                       ((if endOnInput as libc::c_uint != 0 {
                             (ip < shortiend) as libc::c_int
                         } else { 1 as libc::c_int }) &
                            (op <= shortoend as *mut BYTE) as libc::c_int !=
                            0 as libc::c_int) as libc::c_int as libc::c_long
                           != 0 {
                    /* Copy the literals */
                    memcpy(op as *mut libc::c_void, ip as *const libc::c_void,
                           if endOnInput as libc::c_uint != 0 {
                               16 as libc::c_int
                           } else { 8 as libc::c_int } as libc::c_ulong);
                    op = op.offset(length as isize);
                    ip = ip.offset(length as isize);
                    /* The second stage: prepare for match copying, decode full info.
                 * If it doesn't work out, the info won't be wasted. */
                    length =
                        (token &
                             ((1 as libc::c_uint) <<
                                  4 as
                                      libc::c_int).wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                            as size_t; /* match length */
                    offset =
                        LZ4_readLE16(ip as *const libc::c_void) as size_t;
                    ip = ip.offset(2 as libc::c_int as isize);
                    match_0 = op.offset(-(offset as isize));
                    /* check overflow */
                    /* Do not deal with overlapping matches. */
                    if length !=
                           ((1 as libc::c_uint) <<
                                4 as
                                    libc::c_int).wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                               as libc::c_ulong &&
                           offset >= 8 as libc::c_int as libc::c_ulong &&
                           (dict as libc::c_uint ==
                                withPrefix64k as libc::c_int as libc::c_uint
                                || match_0 >= lowPrefix) {
                        /* Copy the match. */
                        memcpy(op.offset(0 as libc::c_int as isize) as
                                   *mut libc::c_void,
                               match_0.offset(0 as libc::c_int as isize) as
                                   *const libc::c_void,
                               8 as libc::c_int as libc::c_ulong);
                        memcpy(op.offset(8 as libc::c_int as isize) as
                                   *mut libc::c_void,
                               match_0.offset(8 as libc::c_int as isize) as
                                   *const libc::c_void,
                               8 as libc::c_int as libc::c_ulong);
                        memcpy(op.offset(16 as libc::c_int as isize) as
                                   *mut libc::c_void,
                               match_0.offset(16 as libc::c_int as isize) as
                                   *const libc::c_void,
                               2 as libc::c_int as libc::c_ulong);
                        op =
                            op.offset(length.wrapping_add(4 as libc::c_int as
                                                              libc::c_ulong)
                                          as isize);
                        /* Both stages worked, load the next token. */
                        current_block = 248631179418912492;
                        continue ;
                    }
                } else {
                    /* decode literal length */
                    if length ==
                           ((1 as libc::c_uint) <<
                                8 as libc::c_int -
                                    4 as
                                        libc::c_int).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                               as libc::c_ulong {
                        let mut error_1: variable_length_error = ok;
                        length =
                            (length as
                                 libc::c_ulong).wrapping_add(read_variable_length(&mut ip,
                                                                                  iend.offset(-(((1
                                                                                                      as
                                                                                                      libc::c_uint)
                                                                                                     <<
                                                                                                     8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         -
                                                                                                         4
                                                                                                             as
                                                                                                             libc::c_int).wrapping_sub(1
                                                                                                                                           as
                                                                                                                                           libc::c_int
                                                                                                                                           as
                                                                                                                                           libc::c_uint)
                                                                                                    as
                                                                                                    isize)),
                                                                                  endOnInput
                                                                                      as
                                                                                      libc::c_int,
                                                                                  endOnInput
                                                                                      as
                                                                                      libc::c_int,
                                                                                  &mut error_1)
                                                                 as
                                                                 libc::c_ulong)
                                as size_t as size_t;
                        if error_1 as libc::c_int ==
                               initial_error as libc::c_int {
                            current_block = 848226666458327087;
                            continue ;
                        }
                        /* overflow detection */
                        if safeDecode != 0 &&
                               (((op as uptrval).wrapping_add(length) <
                                     op as uptrval) as libc::c_int !=
                                    0 as libc::c_int) as libc::c_int as
                                   libc::c_long != 0 {
                            current_block =
                                848226666458327087; /* overflow detection */
                            continue ;
                        }
                        if safeDecode != 0 &&
                               (((ip as uptrval).wrapping_add(length) <
                                     ip as uptrval) as libc::c_int !=
                                    0 as libc::c_int) as libc::c_int as
                                   libc::c_long != 0 {
                            current_block = 848226666458327087;
                            continue ;
                        }
                    }
                    /* copy literals */
                    cpy =
                        op.offset(length as
                                      isize); /* Error : offset outside buffers */
                    current_block = 2103801789718498838;
                    continue ;
                }
            }
            _ => {
                if checkOffset != 0 &&
                       ((match_0.offset(dictSize as isize) < lowPrefix) as
                            libc::c_int != 0 as libc::c_int) as libc::c_int as
                           libc::c_long != 0 {
                    current_block = 848226666458327087;
                    continue ;
                }
                /* match starting within external dictionary */
                if dict as libc::c_uint ==
                       usingExtDict as libc::c_int as libc::c_uint &&
                       match_0 < lowPrefix {
                    if ((op.offset(length as isize) >
                             oend.offset(-(5 as libc::c_int as isize))) as
                            libc::c_int != 0 as libc::c_int) as libc::c_int as
                           libc::c_long != 0 {
                        if !(partialDecoding as u64 != 0) {
                            current_block = 848226666458327087;
                            continue ;
                            /* doesn't respect parsing restriction */
                        }
                        length =
                            if length <
                                   oend.wrapping_offset_from(op) as
                                       libc::c_long as size_t {
                                length
                            } else {
                                oend.wrapping_offset_from(op) as libc::c_long
                                    as size_t
                            }
                    }
                    if length <=
                           lowPrefix.wrapping_offset_from(match_0) as
                               libc::c_long as size_t {
                        /* match fits entirely within external dictionary : just copy */
                        memmove(op as *mut libc::c_void,
                                dictEnd.offset(-(lowPrefix.wrapping_offset_from(match_0)
                                                     as libc::c_long as
                                                     isize)) as
                                    *const libc::c_void, length);
                        op = op.offset(length as isize)
                    } else {
                        /* match stretches into both external dictionary and current block */
                        let copySize_0: size_t =
                            lowPrefix.wrapping_offset_from(match_0) as
                                libc::c_long as size_t;
                        let restSize_0: size_t =
                            length.wrapping_sub(copySize_0);
                        memcpy(op as *mut libc::c_void,
                               dictEnd.offset(-(copySize_0 as isize)) as
                                   *const libc::c_void, copySize_0);
                        op = op.offset(copySize_0 as isize);
                        if restSize_0 >
                               op.wrapping_offset_from(lowPrefix) as
                                   libc::c_long as size_t {
                            /* overlap copy */
                            let endOfMatch_0: *mut BYTE =
                                op.offset(restSize_0 as isize);
                            let mut copyFrom_0: *const BYTE = lowPrefix;
                            while op < endOfMatch_0 {
                                let fresh18 = copyFrom_0;
                                copyFrom_0 = copyFrom_0.offset(1);
                                let fresh19 = op;
                                op = op.offset(1);
                                *fresh19 = *fresh18
                            }
                        } else {
                            memcpy(op as *mut libc::c_void,
                                   lowPrefix as *const libc::c_void,
                                   restSize_0);
                            op = op.offset(restSize_0 as isize)
                        }
                    }
                    current_block = 248631179418912492;
                    continue ;
                } else {
                    /* copy match within block */
                    cpy = op.offset(length as isize);
                    /* partialDecoding : may end anywhere within the block */
                    if partialDecoding as libc::c_uint != 0 &&
                           cpy >
                               oend.offset(-((2 as libc::c_int *
                                                  8 as libc::c_int -
                                                  4 as libc::c_int) as isize))
                       {
                        let mlen: size_t =
                            if length <
                                   oend.wrapping_offset_from(op) as
                                       libc::c_long as size_t {
                                length
                            } else {
                                oend.wrapping_offset_from(op) as libc::c_long
                                    as size_t
                            };
                        let matchEnd: *const BYTE =
                            match_0.offset(mlen as isize);
                        let copyEnd: *mut BYTE = op.offset(mlen as isize);
                        if matchEnd > op as *const BYTE {
                            /* overlap copy */
                            while op < copyEnd {
                                let fresh20 =
                                    match_0; /* silence msan warning when offset==0 */
                                match_0 =
                                    match_0.offset(1); /* Error : last LASTLITERALS bytes must be literals (uncompressed) */
                                let fresh21 = op;
                                op = op.offset(1);
                                *fresh21 = *fresh20
                            }
                        } else {
                            memcpy(op as *mut libc::c_void,
                                   match_0 as *const libc::c_void, mlen);
                        }
                        op = copyEnd;
                        if op == oend {
                            break ;
                        } else {
                            current_block = 248631179418912492;
                            continue ;
                        }
                    } else {
                        if ((offset < 8 as libc::c_int as libc::c_ulong) as
                                libc::c_int != 0 as libc::c_int) as
                               libc::c_int as libc::c_long != 0 {
                            LZ4_write32(op as *mut libc::c_void,
                                        0 as libc::c_int as U32);
                            *op.offset(0 as libc::c_int as isize) =
                                *match_0.offset(0 as libc::c_int as isize);
                            *op.offset(1 as libc::c_int as isize) =
                                *match_0.offset(1 as libc::c_int as isize);
                            *op.offset(2 as libc::c_int as isize) =
                                *match_0.offset(2 as libc::c_int as isize);
                            *op.offset(3 as libc::c_int as isize) =
                                *match_0.offset(3 as libc::c_int as isize);
                            match_0 =
                                match_0.offset(inc32table[offset as usize] as
                                                   isize);
                            memcpy(op.offset(4 as libc::c_int as isize) as
                                       *mut libc::c_void,
                                   match_0 as *const libc::c_void,
                                   4 as libc::c_int as libc::c_ulong);
                            match_0 =
                                match_0.offset(-(dec64table[offset as usize]
                                                     as isize))
                        } else {
                            memcpy(op as *mut libc::c_void,
                                   match_0 as *const libc::c_void,
                                   8 as libc::c_int as libc::c_ulong);
                            match_0 =
                                match_0.offset(8 as libc::c_int as isize)
                        }
                        op = op.offset(8 as libc::c_int as isize);
                        if ((cpy >
                                 oend.offset(-((2 as libc::c_int *
                                                    8 as libc::c_int -
                                                    4 as libc::c_int) as
                                                   isize))) as libc::c_int !=
                                0 as libc::c_int) as libc::c_int as
                               libc::c_long != 0 {
                            let oCopyLimit: *mut BYTE =
                                oend.offset(-((8 as libc::c_int -
                                                   1 as libc::c_int) as
                                                  isize));
                            if cpy > oend.offset(-(5 as libc::c_int as isize))
                               {
                                current_block = 848226666458327087;
                                continue ;
                            }
                            if op < oCopyLimit {
                                LZ4_wildCopy8(op as *mut libc::c_void,
                                              match_0 as *const libc::c_void,
                                              oCopyLimit as
                                                  *mut libc::c_void);
                                match_0 =
                                    match_0.offset(oCopyLimit.wrapping_offset_from(op)
                                                       as libc::c_long as
                                                       isize);
                                op = oCopyLimit
                            }
                            while op < cpy {
                                let fresh22 = match_0;
                                match_0 = match_0.offset(1);
                                let fresh23 = op;
                                op = op.offset(1);
                                *fresh23 = *fresh22
                            }
                        } else {
                            memcpy(op as *mut libc::c_void,
                                   match_0 as *const libc::c_void,
                                   8 as libc::c_int as libc::c_ulong);
                            if length > 16 as libc::c_int as libc::c_ulong {
                                LZ4_wildCopy8(op.offset(8 as libc::c_int as
                                                            isize) as
                                                  *mut libc::c_void,
                                              match_0.offset(8 as libc::c_int
                                                                 as isize) as
                                                  *const libc::c_void,
                                              cpy as *mut libc::c_void);
                            }
                        }
                        op = cpy;
                        current_block = 248631179418912492;
                        continue ;
                    }
                }
            }
        }
        /* The second stage didn't work out, but the info is ready.
                 * Propel it right to the point of match copying. */
        if length ==
               ((1 as libc::c_uint) <<
                    4 as
                        libc::c_int).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as
                   libc::c_ulong {
            let mut error_2: variable_length_error = ok;
            length =
                (length as
                     libc::c_ulong).wrapping_add(read_variable_length(&mut ip,
                                                                      iend.offset(-(5
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).offset(1
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           isize),
                                                                      endOnInput
                                                                          as
                                                                          libc::c_int,
                                                                      0 as
                                                                          libc::c_int,
                                                                      &mut error_2)
                                                     as libc::c_ulong) as
                    size_t as size_t;
            if error_2 as libc::c_int != ok as libc::c_int {
                current_block = 848226666458327087;
                continue ;
            }
            if safeDecode != 0 &&
                   (((op as uptrval).wrapping_add(length) < op as uptrval) as
                        libc::c_int != 0 as libc::c_int) as libc::c_int as
                       libc::c_long != 0 {
                current_block = 848226666458327087;
                continue ;
            }
            /* overflow detection */
        }
        length =
            (length as
                 libc::c_ulong).wrapping_add(4 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        current_block = 8094010457889587865;
    }
    if endOnInput as u64 != 0 {
        return (op as *mut libc::c_char).wrapping_offset_from(dst) as
                   libc::c_long as libc::c_int
        /* Nb of output bytes decoded */
    } else {
        return (ip as *const libc::c_char).wrapping_offset_from(src) as
                   libc::c_long as libc::c_int
        /* Nb of input bytes read */
    };
}
/* end of decoding */
/*===== Instantiate the API decoding functions. =====*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_safe(mut source: *const libc::c_char,
                                             mut dest: *mut libc::c_char,
                                             mut compressedSize: libc::c_int,
                                             mut maxDecompressedSize:
                                                 libc::c_int) -> libc::c_int {
    return LZ4_decompress_generic(source, dest, compressedSize,
                                  maxDecompressedSize, endOnInputSize,
                                  decode_full_block, noDict,
                                  dest as *mut BYTE, 0 as *const BYTE,
                                  0 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_safe_partial(mut src:
                                                         *const libc::c_char,
                                                     mut dst:
                                                         *mut libc::c_char,
                                                     mut compressedSize:
                                                         libc::c_int,
                                                     mut targetOutputSize:
                                                         libc::c_int,
                                                     mut dstCapacity:
                                                         libc::c_int)
 -> libc::c_int {
    dstCapacity =
        if targetOutputSize < dstCapacity {
            targetOutputSize
        } else { dstCapacity };
    return LZ4_decompress_generic(src, dst, compressedSize, dstCapacity,
                                  endOnInputSize, partial_decode, noDict,
                                  dst as *mut BYTE, 0 as *const BYTE,
                                  0 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_fast(mut source: *const libc::c_char,
                                             mut dest: *mut libc::c_char,
                                             mut originalSize: libc::c_int)
 -> libc::c_int {
    return LZ4_decompress_generic(source, dest, 0 as libc::c_int,
                                  originalSize, endOnOutputSize,
                                  decode_full_block, withPrefix64k,
                                  (dest as
                                       *mut BYTE).offset(-((64 as libc::c_int
                                                                *
                                                                ((1 as
                                                                      libc::c_int)
                                                                     <<
                                                                     10 as
                                                                         libc::c_int))
                                                               as isize)),
                                  0 as *const BYTE,
                                  0 as libc::c_int as size_t);
}
/*===== Instantiate a few more decoding cases, used more than once. =====*/
/* Exported, an obsolete API function. */
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_safe_withPrefix64k(mut source:
                                                               *const libc::c_char,
                                                           mut dest:
                                                               *mut libc::c_char,
                                                           mut compressedSize:
                                                               libc::c_int,
                                                           mut maxOutputSize:
                                                               libc::c_int)
 -> libc::c_int {
    return LZ4_decompress_generic(source, dest, compressedSize, maxOutputSize,
                                  endOnInputSize, decode_full_block,
                                  withPrefix64k,
                                  (dest as
                                       *mut BYTE).offset(-((64 as libc::c_int
                                                                *
                                                                ((1 as
                                                                      libc::c_int)
                                                                     <<
                                                                     10 as
                                                                         libc::c_int))
                                                               as isize)),
                                  0 as *const BYTE,
                                  0 as libc::c_int as size_t);
}
/* Another obsolete API function, paired with the previous one. */
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_fast_withPrefix64k(mut source:
                                                               *const libc::c_char,
                                                           mut dest:
                                                               *mut libc::c_char,
                                                           mut originalSize:
                                                               libc::c_int)
 -> libc::c_int {
    /* LZ4_decompress_fast doesn't validate match offsets,
     * and thus serves well with any prefixed dictionary. */
    return LZ4_decompress_fast(source, dest, originalSize);
}
unsafe extern "C" fn LZ4_decompress_safe_withSmallPrefix(mut source:
                                                             *const libc::c_char,
                                                         mut dest:
                                                             *mut libc::c_char,
                                                         mut compressedSize:
                                                             libc::c_int,
                                                         mut maxOutputSize:
                                                             libc::c_int,
                                                         mut prefixSize:
                                                             size_t)
 -> libc::c_int {
    return LZ4_decompress_generic(source, dest, compressedSize, maxOutputSize,
                                  endOnInputSize, decode_full_block, noDict,
                                  (dest as
                                       *mut BYTE).offset(-(prefixSize as
                                                               isize)),
                                  0 as *const BYTE,
                                  0 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_safe_forceExtDict(mut source:
                                                              *const libc::c_char,
                                                          mut dest:
                                                              *mut libc::c_char,
                                                          mut compressedSize:
                                                              libc::c_int,
                                                          mut maxOutputSize:
                                                              libc::c_int,
                                                          mut dictStart:
                                                              *const libc::c_void,
                                                          mut dictSize:
                                                              size_t)
 -> libc::c_int {
    return LZ4_decompress_generic(source, dest, compressedSize, maxOutputSize,
                                  endOnInputSize, decode_full_block,
                                  usingExtDict, dest as *mut BYTE,
                                  dictStart as *const BYTE, dictSize);
}
unsafe extern "C" fn LZ4_decompress_fast_extDict(mut source:
                                                     *const libc::c_char,
                                                 mut dest: *mut libc::c_char,
                                                 mut originalSize:
                                                     libc::c_int,
                                                 mut dictStart:
                                                     *const libc::c_void,
                                                 mut dictSize: size_t)
 -> libc::c_int {
    return LZ4_decompress_generic(source, dest, 0 as libc::c_int,
                                  originalSize, endOnOutputSize,
                                  decode_full_block, usingExtDict,
                                  dest as *mut BYTE, dictStart as *const BYTE,
                                  dictSize);
}
/* The "double dictionary" mode, for use with e.g. ring buffers: the first part
 * of the dictionary is passed as prefix, and the second via dictStart + dictSize.
 * These routines are used only once, in LZ4_decompress_*_continue().
 */
#[inline(always)]
unsafe extern "C" fn LZ4_decompress_safe_doubleDict(mut source:
                                                        *const libc::c_char,
                                                    mut dest:
                                                        *mut libc::c_char,
                                                    mut compressedSize:
                                                        libc::c_int,
                                                    mut maxOutputSize:
                                                        libc::c_int,
                                                    mut prefixSize: size_t,
                                                    mut dictStart:
                                                        *const libc::c_void,
                                                    mut dictSize: size_t)
 -> libc::c_int {
    return LZ4_decompress_generic(source, dest, compressedSize, maxOutputSize,
                                  endOnInputSize, decode_full_block,
                                  usingExtDict,
                                  (dest as
                                       *mut BYTE).offset(-(prefixSize as
                                                               isize)),
                                  dictStart as *const BYTE, dictSize);
}
#[inline(always)]
unsafe extern "C" fn LZ4_decompress_fast_doubleDict(mut source:
                                                        *const libc::c_char,
                                                    mut dest:
                                                        *mut libc::c_char,
                                                    mut originalSize:
                                                        libc::c_int,
                                                    mut prefixSize: size_t,
                                                    mut dictStart:
                                                        *const libc::c_void,
                                                    mut dictSize: size_t)
 -> libc::c_int {
    return LZ4_decompress_generic(source, dest, 0 as libc::c_int,
                                  originalSize, endOnOutputSize,
                                  decode_full_block, usingExtDict,
                                  (dest as
                                       *mut BYTE).offset(-(prefixSize as
                                                               isize)),
                                  dictStart as *const BYTE, dictSize);
}
/*===== streaming decompression functions =====*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_createStreamDecode() -> *mut LZ4_streamDecode_t {
    let mut lz4s: *mut LZ4_streamDecode_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<LZ4_streamDecode_t>() as libc::c_ulong)
            as *mut LZ4_streamDecode_t;
    /* A compilation error here means LZ4_STREAMDECODESIZE is not large enough */
    return lz4s; /* support free on NULL */
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_freeStreamDecode(mut LZ4_stream:
                                                  *mut LZ4_streamDecode_t)
 -> libc::c_int {
    if LZ4_stream.is_null() { return 0 as libc::c_int }
    free(LZ4_stream as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* ! LZ4_setStreamDecode() :
 *  Use this function to instruct where to find the dictionary.
 *  This function is not necessary if previous data is still available where it was decoded.
 *  Loading a size of 0 is allowed (same effect as no dictionary).
 * @return : 1 if OK, 0 if error
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4_setStreamDecode(mut LZ4_streamDecode:
                                                 *mut LZ4_streamDecode_t,
                                             mut dictionary:
                                                 *const libc::c_char,
                                             mut dictSize: libc::c_int)
 -> libc::c_int {
    let mut lz4sd: *mut LZ4_streamDecode_t_internal =
        &mut (*LZ4_streamDecode).internal_donotuse;
    (*lz4sd).prefixSize = dictSize as size_t;
    (*lz4sd).prefixEnd =
        (dictionary as *const BYTE).offset(dictSize as isize);
    (*lz4sd).externalDict = 0 as *const uint8_t;
    (*lz4sd).extDictSize = 0 as libc::c_int as size_t;
    return 1 as libc::c_int;
}
/* ! LZ4_decoderRingBufferSize() :
 *  when setting a ring buffer for streaming decompression (optional scenario),
 *  provides the minimum size of this ring buffer
 *  to be compatible with any source respecting maxBlockSize condition.
 *  Note : in a ring buffer scenario,
 *  blocks are presumed decompressed next to each other.
 *  When not enough space remains for next block (remainingSize < maxBlockSize),
 *  decoding resumes from beginning of ring buffer.
 * @return : minimum ring buffer size,
 *           or 0 if there is an error (invalid maxBlockSize).
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4_decoderRingBufferSize(mut maxBlockSize:
                                                       libc::c_int)
 -> libc::c_int {
    if maxBlockSize < 0 as libc::c_int { return 0 as libc::c_int }
    if maxBlockSize > 0x7e000000 as libc::c_int { return 0 as libc::c_int }
    if maxBlockSize < 16 as libc::c_int { maxBlockSize = 16 as libc::c_int }
    return 65536 as libc::c_int + 14 as libc::c_int + maxBlockSize;
}
/*
*_continue() :
    These decoding functions allow decompression of multiple blocks in "streaming" mode.
    Previously decoded blocks must still be available at the memory position where they were decoded.
    If it's not possible, save the relevant part of decoded data into a safe buffer,
    and indicate where it stands using LZ4_setStreamDecode()
*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_safe_continue(mut LZ4_streamDecode:
                                                          *mut LZ4_streamDecode_t,
                                                      mut source:
                                                          *const libc::c_char,
                                                      mut dest:
                                                          *mut libc::c_char,
                                                      mut compressedSize:
                                                          libc::c_int,
                                                      mut maxOutputSize:
                                                          libc::c_int)
 -> libc::c_int {
    let mut lz4sd: *mut LZ4_streamDecode_t_internal =
        &mut (*LZ4_streamDecode).internal_donotuse;
    let mut result: libc::c_int = 0;
    if (*lz4sd).prefixSize == 0 as libc::c_int as libc::c_ulong {
        /* The first call, no dictionary yet. */
        result =
            LZ4_decompress_safe(source, dest, compressedSize, maxOutputSize);
        if result <= 0 as libc::c_int { return result }
        (*lz4sd).prefixSize = result as size_t;
        (*lz4sd).prefixEnd = (dest as *mut BYTE).offset(result as isize)
    } else if (*lz4sd).prefixEnd == dest as *mut BYTE as *const uint8_t {
        /* They're rolling the current segment. */
        if (*lz4sd).prefixSize >=
               (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)
                    - 1 as libc::c_int) as libc::c_ulong {
            result =
                LZ4_decompress_safe_withPrefix64k(source, dest,
                                                  compressedSize,
                                                  maxOutputSize)
        } else if (*lz4sd).extDictSize == 0 as libc::c_int as libc::c_ulong {
            result =
                LZ4_decompress_safe_withSmallPrefix(source, dest,
                                                    compressedSize,
                                                    maxOutputSize,
                                                    (*lz4sd).prefixSize)
        } else {
            result =
                LZ4_decompress_safe_doubleDict(source, dest, compressedSize,
                                               maxOutputSize,
                                               (*lz4sd).prefixSize,
                                               (*lz4sd).externalDict as
                                                   *const libc::c_void,
                                               (*lz4sd).extDictSize)
        }
        if result <= 0 as libc::c_int { return result }
        (*lz4sd).prefixSize =
            ((*lz4sd).prefixSize as
                 libc::c_ulong).wrapping_add(result as size_t) as size_t as
                size_t;
        (*lz4sd).prefixEnd = (*lz4sd).prefixEnd.offset(result as isize)
    } else {
        /* The buffer wraps around, or they're switching to another buffer. */
        (*lz4sd).extDictSize = (*lz4sd).prefixSize;
        (*lz4sd).externalDict =
            (*lz4sd).prefixEnd.offset(-((*lz4sd).extDictSize as isize));
        result =
            LZ4_decompress_safe_forceExtDict(source, dest, compressedSize,
                                             maxOutputSize,
                                             (*lz4sd).externalDict as
                                                 *const libc::c_void,
                                             (*lz4sd).extDictSize);
        if result <= 0 as libc::c_int { return result }
        (*lz4sd).prefixSize = result as size_t;
        (*lz4sd).prefixEnd = (dest as *mut BYTE).offset(result as isize)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_fast_continue(mut LZ4_streamDecode:
                                                          *mut LZ4_streamDecode_t,
                                                      mut source:
                                                          *const libc::c_char,
                                                      mut dest:
                                                          *mut libc::c_char,
                                                      mut originalSize:
                                                          libc::c_int)
 -> libc::c_int {
    let mut lz4sd: *mut LZ4_streamDecode_t_internal =
        &mut (*LZ4_streamDecode).internal_donotuse;
    let mut result: libc::c_int = 0;
    if (*lz4sd).prefixSize == 0 as libc::c_int as libc::c_ulong {
        result = LZ4_decompress_fast(source, dest, originalSize);
        if result <= 0 as libc::c_int { return result }
        (*lz4sd).prefixSize = originalSize as size_t;
        (*lz4sd).prefixEnd = (dest as *mut BYTE).offset(originalSize as isize)
    } else if (*lz4sd).prefixEnd == dest as *mut BYTE as *const uint8_t {
        if (*lz4sd).prefixSize >=
               (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)
                    - 1 as libc::c_int) as libc::c_ulong ||
               (*lz4sd).extDictSize == 0 as libc::c_int as libc::c_ulong {
            result = LZ4_decompress_fast(source, dest, originalSize)
        } else {
            result =
                LZ4_decompress_fast_doubleDict(source, dest, originalSize,
                                               (*lz4sd).prefixSize,
                                               (*lz4sd).externalDict as
                                                   *const libc::c_void,
                                               (*lz4sd).extDictSize)
        }
        if result <= 0 as libc::c_int { return result }
        (*lz4sd).prefixSize =
            ((*lz4sd).prefixSize as
                 libc::c_ulong).wrapping_add(originalSize as size_t) as size_t
                as size_t;
        (*lz4sd).prefixEnd = (*lz4sd).prefixEnd.offset(originalSize as isize)
    } else {
        (*lz4sd).extDictSize = (*lz4sd).prefixSize;
        (*lz4sd).externalDict =
            (*lz4sd).prefixEnd.offset(-((*lz4sd).extDictSize as isize));
        result =
            LZ4_decompress_fast_extDict(source, dest, originalSize,
                                        (*lz4sd).externalDict as
                                            *const libc::c_void,
                                        (*lz4sd).extDictSize);
        if result <= 0 as libc::c_int { return result }
        (*lz4sd).prefixSize = originalSize as size_t;
        (*lz4sd).prefixEnd = (dest as *mut BYTE).offset(originalSize as isize)
    }
    return result;
}
/*
Advanced decoding functions :
*_usingDict() :
    These decoding functions work the same as "_continue" ones,
    the dictionary must be explicitly provided within parameters
*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_safe_usingDict(mut source:
                                                           *const libc::c_char,
                                                       mut dest:
                                                           *mut libc::c_char,
                                                       mut compressedSize:
                                                           libc::c_int,
                                                       mut maxOutputSize:
                                                           libc::c_int,
                                                       mut dictStart:
                                                           *const libc::c_char,
                                                       mut dictSize:
                                                           libc::c_int)
 -> libc::c_int {
    if dictSize == 0 as libc::c_int {
        return LZ4_decompress_safe(source, dest, compressedSize,
                                   maxOutputSize)
    }
    if dictStart.offset(dictSize as isize) == dest as *const libc::c_char {
        if dictSize >=
               64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int) -
                   1 as libc::c_int {
            return LZ4_decompress_safe_withPrefix64k(source, dest,
                                                     compressedSize,
                                                     maxOutputSize)
        }
        return LZ4_decompress_safe_withSmallPrefix(source, dest,
                                                   compressedSize,
                                                   maxOutputSize,
                                                   dictSize as size_t)
    }
    return LZ4_decompress_safe_forceExtDict(source, dest, compressedSize,
                                            maxOutputSize,
                                            dictStart as *const libc::c_void,
                                            dictSize as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_decompress_fast_usingDict(mut source:
                                                           *const libc::c_char,
                                                       mut dest:
                                                           *mut libc::c_char,
                                                       mut originalSize:
                                                           libc::c_int,
                                                       mut dictStart:
                                                           *const libc::c_char,
                                                       mut dictSize:
                                                           libc::c_int)
 -> libc::c_int {
    if dictSize == 0 as libc::c_int ||
           dictStart.offset(dictSize as isize) == dest as *const libc::c_char
       {
        return LZ4_decompress_fast(source, dest, originalSize)
    }
    return LZ4_decompress_fast_extDict(source, dest, originalSize,
                                       dictStart as *const libc::c_void,
                                       dictSize as size_t);
}
/*=*************************************************
*  Obsolete Functions
***************************************************/
/* obsolete compression functions */
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_limitedOutput(mut source:
                                                        *const libc::c_char,
                                                    mut dest:
                                                        *mut libc::c_char,
                                                    mut inputSize:
                                                        libc::c_int,
                                                    mut maxOutputSize:
                                                        libc::c_int)
 -> libc::c_int {
    return LZ4_compress_default(source, dest, inputSize, maxOutputSize);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress(mut src: *const libc::c_char,
                                      mut dest: *mut libc::c_char,
                                      mut srcSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_default(src, dest, srcSize,
                                LZ4_compressBound(srcSize));
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_limitedOutput_withState(mut state:
                                                                  *mut libc::c_void,
                                                              mut src:
                                                                  *const libc::c_char,
                                                              mut dst:
                                                                  *mut libc::c_char,
                                                              mut srcSize:
                                                                  libc::c_int,
                                                              mut dstSize:
                                                                  libc::c_int)
 -> libc::c_int {
    return LZ4_compress_fast_extState(state, src, dst, srcSize, dstSize,
                                      1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_withState(mut state: *mut libc::c_void,
                                                mut src: *const libc::c_char,
                                                mut dst: *mut libc::c_char,
                                                mut srcSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_fast_extState(state, src, dst, srcSize,
                                      LZ4_compressBound(srcSize),
                                      1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_limitedOutput_continue(mut LZ4_stream:
                                                                 *mut LZ4_stream_t,
                                                             mut src:
                                                                 *const libc::c_char,
                                                             mut dst:
                                                                 *mut libc::c_char,
                                                             mut srcSize:
                                                                 libc::c_int,
                                                             mut dstCapacity:
                                                                 libc::c_int)
 -> libc::c_int {
    return LZ4_compress_fast_continue(LZ4_stream, src, dst, srcSize,
                                      dstCapacity, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_continue(mut LZ4_stream:
                                                   *mut LZ4_stream_t,
                                               mut source:
                                                   *const libc::c_char,
                                               mut dest: *mut libc::c_char,
                                               mut inputSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_fast_continue(LZ4_stream, source, dest, inputSize,
                                      LZ4_compressBound(inputSize),
                                      1 as libc::c_int);
}
/*
These decompression functions are deprecated and should no longer be used.
They are only provided here for compatibility with older user programs.
- LZ4_uncompress is totally equivalent to LZ4_decompress_fast
- LZ4_uncompress_unknownOutputSize is totally equivalent to LZ4_decompress_safe
*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_uncompress(mut source: *const libc::c_char,
                                        mut dest: *mut libc::c_char,
                                        mut outputSize: libc::c_int)
 -> libc::c_int {
    return LZ4_decompress_fast(source, dest, outputSize);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_uncompress_unknownOutputSize(mut source:
                                                              *const libc::c_char,
                                                          mut dest:
                                                              *mut libc::c_char,
                                                          mut isize:
                                                              libc::c_int,
                                                          mut maxOutputSize:
                                                              libc::c_int)
 -> libc::c_int {
    return LZ4_decompress_safe(source, dest, isize, maxOutputSize);
}
/* Obsolete Streaming functions */
#[no_mangle]
pub unsafe extern "C" fn LZ4_sizeofStreamState() -> libc::c_int {
    return ((((1 as libc::c_int) << 14 as libc::c_int - 3 as libc::c_int) +
                 4 as libc::c_int +
                 (if ::std::mem::size_of::<*mut libc::c_void>() as
                         libc::c_ulong == 16 as libc::c_int as libc::c_ulong {
                      4 as libc::c_int
                  } else { 0 as libc::c_int })) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_ulonglong>()
                                                as libc::c_ulong) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_resetStreamState(mut state: *mut libc::c_void,
                                              mut inputBuffer:
                                                  *mut libc::c_char)
 -> libc::c_int {
    LZ4_resetStream(state as *mut LZ4_stream_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_create(mut inputBuffer: *mut libc::c_char)
 -> *mut libc::c_void {
    return LZ4_createStream() as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_slideInputBuffer(mut state: *mut libc::c_void)
 -> *mut libc::c_char {
    /* avoid const char * -> char * conversion warning */
    return (*(state as *mut LZ4_stream_t)).internal_donotuse.dictionary as
               uptrval as *mut libc::c_char;
}
/* LZ4_COMMONDEFS_ONLY */
