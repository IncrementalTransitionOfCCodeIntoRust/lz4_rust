use ::libc;
extern "C" {
    #[no_mangle]
    fn LZ4_compressBound(inputSize: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
pub type __int8_t = libc::c_schar;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
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
pub type limitedOutput_directive = libc::c_uint;
pub const fillOutput: limitedOutput_directive = 2;
pub const limitedOutput: limitedOutput_directive = 1;
pub const notLimited: limitedOutput_directive = 0;
pub type dictCtx_directive = libc::c_uint;
pub const usingDictCtxHc: dictCtx_directive = 1;
pub const noDictCtx: dictCtx_directive = 0;
pub type HCfavor_e = libc::c_uint;
pub const favorDecompressionSpeed: HCfavor_e = 1;
pub const favorCompressionRatio: HCfavor_e = 0;
pub type U32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cParams_t {
    pub strat: lz4hc_strat_e,
    pub nbSearches: U32,
    pub targetLength: U32,
}
pub type lz4hc_strat_e = libc::c_uint;
pub const lz4opt: lz4hc_strat_e = 1;
pub const lz4hc: lz4hc_strat_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4HC_optimal_t {
    pub price: libc::c_int,
    pub off: libc::c_int,
    pub mlen: libc::c_int,
    pub litlen: libc::c_int,
}
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union unalign {
    pub u16_0: U16,
    pub u32_0: U32,
    pub uArch: reg_t,
}
pub type reg_t = U64;
pub type U64 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: U32,
    pub c: [BYTE; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4HC_match_t {
    pub off: libc::c_int,
    pub len: libc::c_int,
}
pub const rep_confirmed: repeat_state_e = 2;
pub type repeat_state_e = libc::c_uint;
pub const rep_not: repeat_state_e = 1;
pub const rep_untested: repeat_state_e = 0;
pub type uptrval = uintptr_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LZ4_static_assert: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub c: libc::c_char,
    pub t: LZ4_streamHC_t,
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
unsafe extern "C" fn LZ4_write16(mut memPtr: *mut libc::c_void,
                                 mut value: U16) {
    (*(memPtr as *mut unalign)).u16_0 = value;
}
unsafe extern "C" fn LZ4_isLittleEndian() -> libc::c_uint {
    let one: C2RustUnnamed = C2RustUnnamed{u: 1 as libc::c_int as U32,};
    return one.c[0 as libc::c_int as usize] as libc::c_uint;
}
static mut LZ4_minLength: libc::c_int = 12 as libc::c_int + 1 as libc::c_int;
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
unsafe extern "C" fn LZ4_NbCommonBytes(mut val: reg_t) -> libc::c_uint {
    if LZ4_isLittleEndian() != 0 {
        if ::std::mem::size_of::<reg_t>() as libc::c_ulong ==
               8 as libc::c_int as libc::c_ulong {
            return (val as libc::c_ulonglong).trailing_zeros() as i32 as
                       libc::c_uint >> 3 as libc::c_int
        } else {
            return (val as U32).trailing_zeros() as i32 as libc::c_uint >>
                       3 as libc::c_int
        }
    } else if ::std::mem::size_of::<reg_t>() as libc::c_ulong ==
                  8 as libc::c_int as libc::c_ulong {
        return (val as libc::c_ulonglong).leading_zeros() as i32 as
                   libc::c_uint >> 3 as libc::c_int
    } else {
        return (val as U32).leading_zeros() as i32 as libc::c_uint >>
                   3 as libc::c_int
    };
}
unsafe extern "C" fn LZ4_read_ARCH(mut ptr: *const libc::c_void) -> reg_t {
    return (*(ptr as *const unalign)).uArch;
}
unsafe extern "C" fn LZ4_read16(mut ptr: *const libc::c_void) -> U16 {
    return (*(ptr as *const unalign)).u16_0;
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
unsafe extern "C" fn LZ4_read32(mut ptr: *const libc::c_void) -> U32 {
    return (*(ptr as *const unalign)).u32_0;
}
/* flexible, LZ4HC_MAXD dependent */
/* faster */
/* Make fields passed to, and updated by LZ4HC_encodeSequence explicit */
unsafe extern "C" fn LZ4HC_hashPtr(mut ptr: *const libc::c_void) -> U32 {
    return LZ4_read32(ptr).wrapping_mul(2654435761 as libc::c_uint) >>
               4 as libc::c_int * 8 as libc::c_int - 15 as libc::c_int;
}
/* *************************************
*  HC Compression
**************************************/
unsafe extern "C" fn LZ4HC_clearTables(mut hc4: *mut LZ4HC_CCtx_internal) {
    memset((*hc4).hashTable.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[uint32_t; 32768]>() as libc::c_ulong);
    memset((*hc4).chainTable.as_mut_ptr() as *mut libc::c_void,
           0xff as libc::c_int,
           ::std::mem::size_of::<[uint16_t; 65536]>() as libc::c_ulong);
}
unsafe extern "C" fn LZ4HC_init_internal(mut hc4: *mut LZ4HC_CCtx_internal,
                                         mut start: *const BYTE) {
    let mut startingOffset: uptrval =
        (*hc4).end.wrapping_offset_from((*hc4).base) as libc::c_long as
            uptrval;
    if startingOffset >
           (1 as libc::c_int as
                libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                               30 as libc::c_int) as
               libc::c_ulong {
        LZ4HC_clearTables(hc4);
        startingOffset = 0 as libc::c_int as uptrval
    }
    startingOffset =
        (startingOffset as
             libc::c_ulong).wrapping_add((64 as libc::c_int *
                                              ((1 as libc::c_int) <<
                                                   10 as libc::c_int)) as
                                             libc::c_ulong) as uptrval as
            uptrval;
    (*hc4).nextToUpdate = startingOffset as U32;
    (*hc4).base = start.offset(-(startingOffset as isize));
    (*hc4).end = start;
    (*hc4).dictBase = start.offset(-(startingOffset as isize));
    (*hc4).dictLimit = startingOffset as U32;
    (*hc4).lowLimit = startingOffset as U32;
}
/* Update chains up to ip (excluded) */
#[inline(always)]
unsafe extern "C" fn LZ4HC_Insert(mut hc4: *mut LZ4HC_CCtx_internal,
                                  mut ip: *const BYTE) {
    let chainTable: *mut U16 = (*hc4).chainTable.as_mut_ptr();
    let hashTable: *mut U32 = (*hc4).hashTable.as_mut_ptr();
    let base: *const BYTE = (*hc4).base;
    let target: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let mut idx: U32 = (*hc4).nextToUpdate;
    while idx < target {
        let h: U32 =
            LZ4HC_hashPtr(base.offset(idx as isize) as *const libc::c_void);
        let mut delta: size_t =
            idx.wrapping_sub(*hashTable.offset(h as isize)) as size_t;
        if delta > 65535 as libc::c_int as libc::c_ulong {
            delta = 65535 as libc::c_int as size_t
        }
        *chainTable.offset(idx as U16 as isize) = delta as U16;
        *hashTable.offset(h as isize) = idx;
        idx = idx.wrapping_add(1)
    }
    (*hc4).nextToUpdate = target;
}
/* * LZ4HC_countBack() :
 * @return : negative value, nb of common bytes before ip/match */
#[inline(always)]
unsafe extern "C" fn LZ4HC_countBack(ip: *const BYTE, match_0: *const BYTE,
                                     iMin: *const BYTE, mMin: *const BYTE)
 -> libc::c_int {
    let mut back: libc::c_int = 0 as libc::c_int;
    let min: libc::c_int =
        if iMin.wrapping_offset_from(ip) as libc::c_long >
               mMin.wrapping_offset_from(match_0) as libc::c_long {
            iMin.wrapping_offset_from(ip) as libc::c_long
        } else { mMin.wrapping_offset_from(match_0) as libc::c_long } as
            libc::c_int;
    while back > min &&
              *ip.offset((back - 1 as libc::c_int) as isize) as libc::c_int ==
                  *match_0.offset((back - 1 as libc::c_int) as isize) as
                      libc::c_int {
        back -= 1
    }
    return back;
}
unsafe extern "C" fn LZ4HC_rotatePattern(rotate: size_t, pattern: U32)
 -> U32 {
    let bitsToRotate: size_t =
        (rotate &
             (::std::mem::size_of::<U32>() as
                  libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong)) <<
            3 as libc::c_int;
    if bitsToRotate == 0 as libc::c_int as libc::c_ulong { return pattern }
    return pattern << bitsToRotate as libc::c_int |
               pattern >> 32 as libc::c_int - bitsToRotate as libc::c_int;
}
/* LZ4HC_countPattern() :
 * pattern32 must be a sample of repetitive pattern of length 1, 2 or 4 (but not 3!) */
unsafe extern "C" fn LZ4HC_countPattern(mut ip: *const BYTE,
                                        iEnd: *const BYTE, pattern32: U32)
 -> libc::c_uint {
    let iStart: *const BYTE = ip; /* big endian */
    let pattern: reg_t =
        if ::std::mem::size_of::<reg_t>() as libc::c_ulong ==
               8 as libc::c_int as libc::c_ulong {
            (pattern32 as
                 reg_t).wrapping_add((pattern32 as reg_t) <<
                                         32 as libc::c_int)
        } else { pattern32 as libc::c_ulong };
    while ((ip <
                iEnd.offset(-((::std::mem::size_of::<reg_t>() as
                                   libc::c_ulong).wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong)
                                  as isize))) as libc::c_int !=
               0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        let diff: reg_t = LZ4_read_ARCH(ip as *const libc::c_void) ^ pattern;
        if diff == 0 {
            ip =
                ip.offset(::std::mem::size_of::<reg_t>() as libc::c_ulong as
                              isize)
        } else {
            ip = ip.offset(LZ4_NbCommonBytes(diff) as isize);
            return ip.wrapping_offset_from(iStart) as libc::c_long as
                       libc::c_uint
        }
    }
    if LZ4_isLittleEndian() != 0 {
        let mut patternByte: reg_t = pattern;
        while ip < iEnd &&
                  *ip as libc::c_int == patternByte as BYTE as libc::c_int {
            ip = ip.offset(1);
            patternByte >>= 8 as libc::c_int
        }
    } else {
        let mut bitOffset: U32 =
            (::std::mem::size_of::<reg_t>() as
                 libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                 libc::c_ulong).wrapping_sub(8
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                as U32;
        while ip < iEnd {
            let byte: BYTE = (pattern >> bitOffset) as BYTE;
            if *ip as libc::c_int != byte as libc::c_int { break ; }
            ip = ip.offset(1);
            bitOffset =
                (bitOffset as
                     libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                    libc::c_uint) as U32 as
                    U32
        }
    }
    return ip.wrapping_offset_from(iStart) as libc::c_long as libc::c_uint;
}
/* LZ4HC_reverseCountPattern() :
 * pattern must be a sample of repetitive pattern of length 1, 2 or 4 (but not 3!)
 * read using natural platform endianess */
unsafe extern "C" fn LZ4HC_reverseCountPattern(mut ip: *const BYTE,
                                               iLow: *const BYTE,
                                               mut pattern: U32)
 -> libc::c_uint {
    let iStart: *const BYTE = ip; /* works for any endianess */
    while ((ip >= iLow.offset(4 as libc::c_int as isize)) as libc::c_int !=
               0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if LZ4_read32(ip.offset(-(4 as libc::c_int as isize)) as
                          *const libc::c_void) != pattern {
            break ;
        }
        ip = ip.offset(-(4 as libc::c_int as isize))
    }
    let mut bytePtr: *const BYTE =
        (&mut pattern as *mut U32 as
             *const BYTE).offset(3 as libc::c_int as isize);
    while ((ip > iLow) as libc::c_int != 0 as libc::c_int) as libc::c_int as
              libc::c_long != 0 {
        if *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int !=
               *bytePtr as libc::c_int {
            break ;
        }
        ip = ip.offset(-1);
        bytePtr = bytePtr.offset(-1)
    }
    return iStart.wrapping_offset_from(ip) as libc::c_long as libc::c_uint;
}
/* LZ4HC_protectDictEnd() :
 * Checks if the match is in the last 3 bytes of the dictionary, so reading the
 * 4 byte MINMATCH would overflow.
 * @returns true if the match index is okay.
 */
unsafe extern "C" fn LZ4HC_protectDictEnd(dictLimit: U32, matchIndex: U32)
 -> libc::c_int {
    return (dictLimit.wrapping_sub(1 as libc::c_int as
                                       libc::c_uint).wrapping_sub(matchIndex)
                >= 3 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn LZ4HC_InsertAndGetWiderMatch(mut hc4:
                                                      *mut LZ4HC_CCtx_internal,
                                                  ip: *const BYTE,
                                                  iLowLimit: *const BYTE,
                                                  iHighLimit: *const BYTE,
                                                  mut longest: libc::c_int,
                                                  mut matchpos:
                                                      *mut *const BYTE,
                                                  mut startpos:
                                                      *mut *const BYTE,
                                                  maxNbAttempts: libc::c_int,
                                                  patternAnalysis:
                                                      libc::c_int,
                                                  chainSwap: libc::c_int,
                                                  dict: dictCtx_directive,
                                                  favorDecSpeed: HCfavor_e)
 -> libc::c_int {
    let chainTable: *mut U16 = (*hc4).chainTable.as_mut_ptr();
    let HashTable: *mut U32 = (*hc4).hashTable.as_mut_ptr();
    let dictCtx: *const LZ4HC_CCtx_internal = (*hc4).dictCtx;
    let base: *const BYTE = (*hc4).base;
    let dictLimit: U32 = (*hc4).dictLimit;
    let lowPrefixPtr: *const BYTE = base.offset(dictLimit as isize);
    let ipIndex: U32 = ip.wrapping_offset_from(base) as libc::c_long as U32;
    let lowestMatchIndex: U32 =
        if (*hc4).lowLimit.wrapping_add((65535 as libc::c_int +
                                             1 as libc::c_int) as
                                            libc::c_uint) > ipIndex {
            (*hc4).lowLimit
        } else { ipIndex.wrapping_sub(65535 as libc::c_int as libc::c_uint) };
    let dictBase: *const BYTE = (*hc4).dictBase;
    let lookBackLength: libc::c_int =
        ip.wrapping_offset_from(iLowLimit) as libc::c_long as libc::c_int;
    let mut nbAttempts: libc::c_int = maxNbAttempts;
    let mut matchChainPos: U32 = 0 as libc::c_int as U32;
    let pattern: U32 = LZ4_read32(ip as *const libc::c_void);
    let mut matchIndex: U32 = 0;
    let mut repeat: repeat_state_e = rep_untested;
    let mut srcPatternLength: size_t = 0 as libc::c_int as size_t;
    /* First Match */
    LZ4HC_Insert(hc4,
                 ip); /* while ((matchIndex>=lowestMatchIndex) && (nbAttempts)) */
    matchIndex =
        *HashTable.offset(LZ4HC_hashPtr(ip as *const libc::c_void) as
                              isize); /* lowestMatchIndex <= matchIndex < dictLimit */
    while matchIndex >= lowestMatchIndex && nbAttempts != 0 {
        let mut matchLength: libc::c_int =
            0 as
                libc::c_int; /* virtual pos, relative to ip, to retrieve offset */
        nbAttempts -= 1;
        if !(favorDecSpeed as libc::c_uint != 0 &&
                 ipIndex.wrapping_sub(matchIndex) <
                     8 as libc::c_int as libc::c_uint) {
            if matchIndex >= dictLimit {
                let matchPtr: *const BYTE = base.offset(matchIndex as isize);
                if LZ4_read16(iLowLimit.offset(longest as
                                                   isize).offset(-(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize))
                                  as *const libc::c_void) as libc::c_int ==
                       LZ4_read16(matchPtr.offset(-(lookBackLength as
                                                        isize)).offset(longest
                                                                           as
                                                                           isize).offset(-(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize))
                                      as *const libc::c_void) as libc::c_int {
                    if LZ4_read32(matchPtr as *const libc::c_void) == pattern
                       {
                        let back: libc::c_int =
                            if lookBackLength != 0 {
                                LZ4HC_countBack(ip, matchPtr, iLowLimit,
                                                lowPrefixPtr)
                            } else { 0 as libc::c_int };
                        matchLength =
                            4 as libc::c_int +
                                LZ4_count(ip.offset(4 as libc::c_int as
                                                        isize),
                                          matchPtr.offset(4 as libc::c_int as
                                                              isize),
                                          iHighLimit) as libc::c_int;
                        matchLength -= back;
                        if matchLength > longest {
                            longest = matchLength;
                            *matchpos = matchPtr.offset(back as isize);
                            *startpos = ip.offset(back as isize)
                        }
                    }
                }
            } else {
                let matchPtr_0: *const BYTE =
                    dictBase.offset(matchIndex as isize);
                if LZ4_read32(matchPtr_0 as *const libc::c_void) == pattern {
                    let dictStart: *const BYTE =
                        dictBase.offset((*hc4).lowLimit as isize);
                    let mut back_0: libc::c_int = 0 as libc::c_int;
                    let mut vLimit: *const BYTE =
                        ip.offset(dictLimit.wrapping_sub(matchIndex) as
                                      isize);
                    if vLimit > iHighLimit { vLimit = iHighLimit }
                    matchLength =
                        LZ4_count(ip.offset(4 as libc::c_int as isize),
                                  matchPtr_0.offset(4 as libc::c_int as
                                                        isize), vLimit) as
                            libc::c_int + 4 as libc::c_int;
                    if ip.offset(matchLength as isize) == vLimit &&
                           vLimit < iHighLimit {
                        matchLength =
                            (matchLength as
                                 libc::c_uint).wrapping_add(LZ4_count(ip.offset(matchLength
                                                                                    as
                                                                                    isize),
                                                                      lowPrefixPtr,
                                                                      iHighLimit))
                                as libc::c_int as libc::c_int
                    }
                    back_0 =
                        if lookBackLength != 0 {
                            LZ4HC_countBack(ip, matchPtr_0, iLowLimit,
                                            dictStart)
                        } else { 0 as libc::c_int };
                    matchLength -= back_0;
                    if matchLength > longest {
                        longest = matchLength;
                        *matchpos =
                            base.offset(matchIndex as
                                            isize).offset(back_0 as isize);
                        *startpos = ip.offset(back_0 as isize)
                    }
                }
            }
        }
        if chainSwap != 0 && matchLength == longest {
            /* better match => select a better chain */
            /* search forward only */
            if matchIndex.wrapping_add(longest as U32) <= ipIndex {
                let kTrigger: libc::c_int =
                    4 as libc::c_int; /* avoid overflow */
                let mut distanceToNextMatch: U32 =
                    1 as libc::c_int as U32; /* PA optimization */
                let end: libc::c_int =
                    longest - 4 as libc::c_int + 1 as libc::c_int;
                let mut step: libc::c_int = 1 as libc::c_int;
                let mut accel: libc::c_int = (1 as libc::c_int) << kTrigger;
                let mut pos: libc::c_int = 0;
                pos = 0 as libc::c_int;
                while pos < end {
                    let candidateDist: U32 =
                        *chainTable.offset(matchIndex.wrapping_add(pos as U32)
                                               as U16 as isize) as U32;
                    let fresh0 = accel;
                    accel = accel + 1;
                    step = fresh0 >> kTrigger;
                    if candidateDist > distanceToNextMatch {
                        distanceToNextMatch = candidateDist;
                        matchChainPos = pos as U32;
                        accel = (1 as libc::c_int) << kTrigger
                    }
                    pos += step
                }
                if distanceToNextMatch > 1 as libc::c_int as libc::c_uint {
                    if distanceToNextMatch > matchIndex { break ; }
                    matchIndex =
                        (matchIndex as
                             libc::c_uint).wrapping_sub(distanceToNextMatch)
                            as U32 as U32;
                    continue ;
                }
            }
        }
        let distNextMatch: U32 =
            *chainTable.offset(matchIndex as U16 as isize) as U32;
        if patternAnalysis != 0 &&
               distNextMatch == 1 as libc::c_int as libc::c_uint &&
               matchChainPos == 0 as libc::c_int as libc::c_uint {
            let matchCandidateIdx: U32 =
                matchIndex.wrapping_sub(1 as libc::c_int as libc::c_uint);
            /* may be a repeated pattern */
            if repeat as libc::c_uint ==
                   rep_untested as libc::c_int as libc::c_uint {
                if (pattern & 0xffff as libc::c_int as libc::c_uint ==
                        pattern >> 16 as libc::c_int) as libc::c_int &
                       (pattern & 0xff as libc::c_int as libc::c_uint ==
                            pattern >> 24 as libc::c_int) as libc::c_int != 0
                   {
                    repeat = rep_confirmed;
                    srcPatternLength =
                        (LZ4HC_countPattern(ip.offset(::std::mem::size_of::<U32>()
                                                          as libc::c_ulong as
                                                          isize), iHighLimit,
                                            pattern) as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<U32>()
                                                             as libc::c_ulong)
                } else { repeat = rep_not }
            }
            if repeat as libc::c_uint ==
                   rep_confirmed as libc::c_int as libc::c_uint &&
                   matchCandidateIdx >= lowestMatchIndex &&
                   LZ4HC_protectDictEnd(dictLimit, matchCandidateIdx) != 0 {
                let extDict: libc::c_int =
                    (matchCandidateIdx < dictLimit) as libc::c_int;
                let matchPtr_1: *const BYTE =
                    (if extDict != 0 {
                         dictBase
                     } else { base }).offset(matchCandidateIdx as isize);
                if LZ4_read32(matchPtr_1 as *const libc::c_void) == pattern {
                    /* good candidate */
                    let dictStart_0: *const BYTE =
                        dictBase.offset((*hc4).lowLimit as isize);
                    let iLimit: *const BYTE =
                        if extDict != 0 {
                            dictBase.offset(dictLimit as isize)
                        } else { iHighLimit };
                    let mut forwardPatternLength: size_t =
                        (LZ4HC_countPattern(matchPtr_1.offset(::std::mem::size_of::<U32>()
                                                                  as
                                                                  libc::c_ulong
                                                                  as isize),
                                            iLimit, pattern) as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<U32>()
                                                             as
                                                             libc::c_ulong);
                    if extDict != 0 &&
                           matchPtr_1.offset(forwardPatternLength as isize) ==
                               iLimit {
                        let rotatedPattern: U32 =
                            LZ4HC_rotatePattern(forwardPatternLength,
                                                pattern);
                        forwardPatternLength =
                            (forwardPatternLength as
                                 libc::c_ulong).wrapping_add(LZ4HC_countPattern(lowPrefixPtr,
                                                                                iHighLimit,
                                                                                rotatedPattern)
                                                                 as
                                                                 libc::c_ulong)
                                as size_t as size_t
                    }
                    let lowestMatchPtr: *const BYTE =
                        if extDict != 0 { dictStart_0 } else { lowPrefixPtr };
                    let mut backLength: size_t =
                        LZ4HC_reverseCountPattern(matchPtr_1, lowestMatchPtr,
                                                  pattern) as size_t;
                    let mut currentSegmentLength: size_t = 0;
                    if extDict == 0 &&
                           matchPtr_1.offset(-(backLength as isize)) ==
                               lowPrefixPtr && (*hc4).lowLimit < dictLimit {
                        let rotatedPattern_0: U32 =
                            LZ4HC_rotatePattern(-(backLength as libc::c_int)
                                                    as U32 as size_t,
                                                pattern);
                        backLength =
                            (backLength as
                                 libc::c_ulong).wrapping_add(LZ4HC_reverseCountPattern(dictBase.offset(dictLimit
                                                                                                           as
                                                                                                           isize),
                                                                                       dictStart_0,
                                                                                       rotatedPattern_0)
                                                                 as
                                                                 libc::c_ulong)
                                as size_t as size_t
                    }
                    /* Limit backLength not go further than lowestMatchIndex */
                    backLength =
                        matchCandidateIdx.wrapping_sub(if matchCandidateIdx.wrapping_sub(backLength
                                                                                              as
                                                                                              U32)
                                                               >
                                                               lowestMatchIndex
                                                           {
                                                            matchCandidateIdx.wrapping_sub(backLength
                                                                                               as
                                                                                               U32)
                                                        } else {
                                                            lowestMatchIndex
                                                        }) as size_t;
                    currentSegmentLength =
                        backLength.wrapping_add(forwardPatternLength);
                    /* Adjust to end of pattern if the source pattern fits, otherwise the beginning of the pattern */
                    if currentSegmentLength >= srcPatternLength &&
                           forwardPatternLength <= srcPatternLength {
                        /* haven't reached this position yet */
                        let newMatchIndex: U32 =
                            matchCandidateIdx.wrapping_add(forwardPatternLength
                                                               as
                                                               U32).wrapping_sub(srcPatternLength
                                                                                     as
                                                                                     U32); /* best position, full pattern, might be followed by more match */
                        if LZ4HC_protectDictEnd(dictLimit, newMatchIndex) != 0
                           {
                            matchIndex = newMatchIndex
                        } else {
                            /* Can only happen if started in the prefix */
                            matchIndex = dictLimit
                        } /* farthest position in current segment, will find a match of length currentSegmentLength + maybe some back */
                        continue ;
                    } else {
                        let newMatchIndex_0: U32 =
                            matchCandidateIdx.wrapping_sub(backLength as U32);
                        if LZ4HC_protectDictEnd(dictLimit, newMatchIndex_0) ==
                               0 {
                            matchIndex = dictLimit;
                            continue ;
                        } else {
                            matchIndex = newMatchIndex_0;
                            if !(lookBackLength == 0 as libc::c_int) {
                                continue ;
                            }
                            /* no back possible */
                            let maxML: size_t =
                                if currentSegmentLength < srcPatternLength {
                                    currentSegmentLength
                                } else {
                                    srcPatternLength
                                }; /* virtual pos, relative to ip, to retrieve offset */
                            if (longest as size_t) < maxML {
                                if (ip.wrapping_offset_from(base) as
                                        libc::c_long as
                                        size_t).wrapping_sub(matchIndex as
                                                                 libc::c_ulong)
                                       > 65535 as libc::c_int as libc::c_ulong
                                   {
                                    break ; /* avoid overflow */
                                }
                                longest = maxML as libc::c_int;
                                *matchpos = base.offset(matchIndex as isize);
                                *startpos = ip
                            }
                            let distToNextPattern: U32 =
                                *chainTable.offset(matchIndex as U16 as isize)
                                    as U32;
                            if distToNextPattern > matchIndex { break ; }
                            matchIndex =
                                (matchIndex as
                                     libc::c_uint).wrapping_sub(distToNextPattern)
                                    as U32 as U32;
                            continue ;
                        }
                    }
                }
            }
        }
        /* follow current chain */
        matchIndex =
            (matchIndex as
                 libc::c_uint).wrapping_sub(*chainTable.offset(matchIndex.wrapping_add(matchChainPos)
                                                                   as U16 as
                                                                   isize) as
                                                libc::c_uint) as U32 as U32
    }
    if dict as libc::c_uint == usingDictCtxHc as libc::c_int as libc::c_uint
           && nbAttempts != 0 &&
           ipIndex.wrapping_sub(lowestMatchIndex) <
               65535 as libc::c_int as libc::c_uint {
        let dictEndOffset: size_t =
            (*dictCtx).end.wrapping_offset_from((*dictCtx).base) as
                libc::c_long as size_t;
        let mut dictMatchIndex: U32 =
            (*dictCtx).hashTable[LZ4HC_hashPtr(ip as *const libc::c_void) as
                                     usize];
        matchIndex =
            dictMatchIndex.wrapping_add(lowestMatchIndex).wrapping_sub(dictEndOffset
                                                                           as
                                                                           U32);
        while ipIndex.wrapping_sub(matchIndex) <=
                  65535 as libc::c_int as libc::c_uint &&
                  {
                      let fresh1 = nbAttempts;
                      nbAttempts = nbAttempts - 1;
                      (fresh1) != 0
                  } {
            let matchPtr_2: *const BYTE =
                (*dictCtx).base.offset(dictMatchIndex as isize);
            if LZ4_read32(matchPtr_2 as *const libc::c_void) == pattern {
                let mut mlt: libc::c_int = 0;
                let mut back_1: libc::c_int = 0 as libc::c_int;
                let mut vLimit_0: *const BYTE =
                    ip.offset(dictEndOffset.wrapping_sub(dictMatchIndex as
                                                             libc::c_ulong) as
                                  isize);
                if vLimit_0 > iHighLimit { vLimit_0 = iHighLimit }
                mlt =
                    LZ4_count(ip.offset(4 as libc::c_int as isize),
                              matchPtr_2.offset(4 as libc::c_int as isize),
                              vLimit_0) as libc::c_int + 4 as libc::c_int;
                back_1 =
                    if lookBackLength != 0 {
                        LZ4HC_countBack(ip, matchPtr_2, iLowLimit,
                                        (*dictCtx).base.offset((*dictCtx).dictLimit
                                                                   as isize))
                    } else { 0 as libc::c_int };
                mlt -= back_1;
                if mlt > longest {
                    longest = mlt;
                    *matchpos =
                        base.offset(matchIndex as
                                        isize).offset(back_1 as isize);
                    *startpos = ip.offset(back_1 as isize)
                }
            }
            let nextOffset: U32 =
                (*dictCtx).chainTable[dictMatchIndex as U16 as usize] as U32;
            dictMatchIndex =
                (dictMatchIndex as libc::c_uint).wrapping_sub(nextOffset) as
                    U32 as U32;
            matchIndex =
                (matchIndex as libc::c_uint).wrapping_sub(nextOffset) as U32
                    as U32
        }
    }
    return longest;
}
#[inline(always)]
unsafe extern "C" fn LZ4HC_InsertAndFindBestMatch(hc4:
                                                      *mut LZ4HC_CCtx_internal,
                                                  ip: *const BYTE,
                                                  iLimit: *const BYTE,
                                                  mut matchpos:
                                                      *mut *const BYTE,
                                                  maxNbAttempts: libc::c_int,
                                                  patternAnalysis:
                                                      libc::c_int,
                                                  dict: dictCtx_directive)
 -> libc::c_int {
    let mut uselessPtr: *const BYTE = ip;
    /* note : LZ4HC_InsertAndGetWiderMatch() is able to modify the starting position of a match (*startpos),
     * but this won't be the case here, as we define iLowLimit==ip,
     * so LZ4HC_InsertAndGetWiderMatch() won't be allowed to search past ip */
    return LZ4HC_InsertAndGetWiderMatch(hc4, ip, ip, iLimit,
                                        4 as libc::c_int - 1 as libc::c_int,
                                        matchpos, &mut uselessPtr,
                                        maxNbAttempts, patternAnalysis,
                                        0 as libc::c_int, dict,
                                        favorCompressionRatio);
}
/* LZ4HC_encodeSequence() :
 * @return : 0 if ok,
 *           1 if buffer issue detected */
#[inline(always)]
unsafe extern "C" fn LZ4HC_encodeSequence(mut ip: *mut *const BYTE,
                                          mut op: *mut *mut BYTE,
                                          mut anchor: *mut *const BYTE,
                                          mut matchLength: libc::c_int,
                                          match_0: *const BYTE,
                                          mut limit: limitedOutput_directive,
                                          mut oend: *mut BYTE)
 -> libc::c_int {
    let mut length: size_t = 0;
    let fresh2 = *op;
    *op = (*op).offset(1);
    let token: *mut BYTE = fresh2;
    /* Encode Literal length */
    length =
        (*ip).wrapping_offset_from(*anchor) as libc::c_long as
            size_t; /* Check output limit */
    if limit as libc::c_uint != 0 &&
           (*op).offset(length.wrapping_div(255 as libc::c_int as
                                                libc::c_ulong) as
                            isize).offset(length as
                                              isize).offset((2 as libc::c_int
                                                                 +
                                                                 1 as
                                                                     libc::c_int
                                                                 +
                                                                 5 as
                                                                     libc::c_int)
                                                                as isize) >
               oend {
        return 1 as libc::c_int
    }
    if length >=
           ((1 as libc::c_uint) <<
                8 as libc::c_int -
                    4 as
                        libc::c_int).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as
               libc::c_ulong {
        let mut len: size_t =
            length.wrapping_sub(((1 as libc::c_uint) <<
                                     8 as libc::c_int -
                                         4 as
                                             libc::c_int).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                                    as libc::c_ulong);
        *token =
            (((1 as libc::c_uint) <<
                  8 as libc::c_int -
                      4 as
                          libc::c_int).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) <<
                 4 as libc::c_int) as BYTE;
        while len >= 255 as libc::c_int as libc::c_ulong {
            let fresh3 = *op;
            *op = (*op).offset(1);
            *fresh3 = 255 as libc::c_int as BYTE;
            len =
                (len as
                     libc::c_ulong).wrapping_sub(255 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t
        }
        let fresh4 = *op;
        *op = (*op).offset(1);
        *fresh4 = len as BYTE
    } else { *token = (length << 4 as libc::c_int) as BYTE }
    /* Copy Literals */
    LZ4_wildCopy8(*op as *mut libc::c_void, *anchor as *const libc::c_void,
                  (*op).offset(length as isize) as *mut libc::c_void);
    *op = (*op).offset(length as isize);
    /* Encode Offset */
    /* note : consider providing offset as a value, rather than as a pointer difference */
    LZ4_writeLE16(*op as *mut libc::c_void,
                  (*ip).wrapping_offset_from(match_0) as libc::c_long as U16);
    *op = (*op).offset(2 as libc::c_int as isize);
    /* Encode MatchLength */
    length =
        (matchLength as
             size_t).wrapping_sub(4 as libc::c_int as
                                      libc::c_ulong); /* Check output limit */
    if limit as libc::c_uint != 0 &&
           (*op).offset(length.wrapping_div(255 as libc::c_int as
                                                libc::c_ulong) as
                            isize).offset((1 as libc::c_int +
                                               5 as libc::c_int) as isize) >
               oend {
        return 1 as libc::c_int
    }
    if length >=
           ((1 as libc::c_uint) <<
                4 as
                    libc::c_int).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) as
               libc::c_ulong {
        *token =
            (*token as
                 libc::c_uint).wrapping_add(((1 as libc::c_uint) <<
                                                 4 as
                                                     libc::c_int).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint))
                as BYTE as BYTE;
        length =
            (length as
                 libc::c_ulong).wrapping_sub(((1 as libc::c_uint) <<
                                                  4 as
                                                      libc::c_int).wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                 as libc::c_ulong) as size_t
                as size_t;
        while length >= 510 as libc::c_int as libc::c_ulong {
            let fresh5 = *op;
            *op = (*op).offset(1);
            *fresh5 = 255 as libc::c_int as BYTE;
            let fresh6 = *op;
            *op = (*op).offset(1);
            *fresh6 = 255 as libc::c_int as BYTE;
            length =
                (length as
                     libc::c_ulong).wrapping_sub(510 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t
        }
        if length >= 255 as libc::c_int as libc::c_ulong {
            length =
                (length as
                     libc::c_ulong).wrapping_sub(255 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t;
            let fresh7 = *op;
            *op = (*op).offset(1);
            *fresh7 = 255 as libc::c_int as BYTE
        }
        let fresh8 = *op;
        *op = (*op).offset(1);
        *fresh8 = length as BYTE
    } else {
        *token =
            (*token as libc::c_int + length as BYTE as libc::c_int) as BYTE
    }
    /* Prepare next loop */
    *ip = (*ip).offset(matchLength as isize); /* levels 9+ */
    *anchor = *ip;
    return 0 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn LZ4HC_compress_hashChain(ctx: *mut LZ4HC_CCtx_internal,
                                              source: *const libc::c_char,
                                              dest: *mut libc::c_char,
                                              mut srcSizePtr:
                                                  *mut libc::c_int,
                                              maxOutputSize: libc::c_int,
                                              mut maxNbAttempts: libc::c_uint,
                                              limit: limitedOutput_directive,
                                              dict: dictCtx_directive)
 -> libc::c_int {
    let mut current_block: u64;
    let inputSize: libc::c_int = *srcSizePtr;
    let patternAnalysis: libc::c_int =
        (maxNbAttempts > 128 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut ip: *const BYTE = source as *const BYTE;
    let mut anchor: *const BYTE = ip;
    let iend: *const BYTE = ip.offset(inputSize as isize);
    let mflimit: *const BYTE = iend.offset(-(12 as libc::c_int as isize));
    let matchlimit: *const BYTE = iend.offset(-(5 as libc::c_int as isize));
    let mut optr: *mut BYTE = dest as *mut BYTE;
    let mut op: *mut BYTE = dest as *mut BYTE;
    let mut oend: *mut BYTE = op.offset(maxOutputSize as isize);
    let mut ml0: libc::c_int = 0;
    let mut ml: libc::c_int = 0;
    let mut ml2: libc::c_int = 0;
    let mut ml3: libc::c_int = 0;
    let mut start0: *const BYTE = 0 as *const BYTE;
    let mut ref0: *const BYTE = 0 as *const BYTE;
    let mut ref_0: *const BYTE = 0 as *const BYTE;
    let mut start2: *const BYTE = 0 as *const BYTE;
    let mut ref2: *const BYTE = 0 as *const BYTE;
    let mut start3: *const BYTE = 0 as *const BYTE;
    let mut ref3: *const BYTE = 0 as *const BYTE;
    /* init */
    *srcSizePtr =
        0 as libc::c_int; /* Hack for support LZ4 format restriction */
    if limit as libc::c_uint == fillOutput as libc::c_int as libc::c_uint {
        oend = oend.offset(-(5 as libc::c_int as isize))
    } /* Input too small, no compression (all literals) */
    if inputSize < LZ4_minLength {
        current_block = 10090817204669828264;
    } else { current_block = 8236137900636309791; }
    'c_5703:
        loop  {
            match current_block {
                8236137900636309791 =>
                /* Main Loop */
                {
                    if !(ip <= mflimit) {
                        current_block = 10090817204669828264;
                        continue ;
                    }
                    ml =
                        LZ4HC_InsertAndFindBestMatch(ctx, ip, matchlimit,
                                                     &mut ref_0,
                                                     maxNbAttempts as
                                                         libc::c_int,
                                                     patternAnalysis, dict);
                    if ml < 4 as libc::c_int {
                        ip = ip.offset(1);
                        current_block = 8236137900636309791;
                    } else {
                        /* saved, in case we would skip too much */
                        start0 = ip;
                        ref0 = ref_0;
                        ml0 = ml;
                        'c_6022:
                            loop  {
                                if ip.offset(ml as isize) <= mflimit {
                                    ml2 =
                                        LZ4HC_InsertAndGetWiderMatch(ctx,
                                                                     ip.offset(ml
                                                                                   as
                                                                                   isize).offset(-(2
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       isize)),
                                                                     ip.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                                                     matchlimit,
                                                                     ml,
                                                                     &mut ref2,
                                                                     &mut start2,
                                                                     maxNbAttempts
                                                                         as
                                                                         libc::c_int,
                                                                     patternAnalysis,
                                                                     0 as
                                                                         libc::c_int,
                                                                     dict,
                                                                     favorCompressionRatio)
                                } else { ml2 = ml }
                                if ml2 == ml {
                                    /* No better match => encode ML1 */
                                    optr = op;
                                    if LZ4HC_encodeSequence(&mut ip, &mut op,
                                                            &mut anchor, ml,
                                                            ref_0, limit,
                                                            oend) != 0 {
                                        current_block = 4429472873095296980;
                                        break ;
                                    } else {
                                        current_block = 8236137900636309791;
                                        continue 'c_5703 ;
                                    }
                                } else {
                                    if start0 < ip {
                                        /* first match was skipped at least once */
                                        if start2 < ip.offset(ml0 as isize) {
                                            /* squeezing ML1 between ML0(original ML1) and ML2 */
                                            ip = start0;
                                            ref_0 = ref0;
                                            ml = ml0
                                            /* restore initial ML1 */
                                        }
                                    }
                                    /* Here, start0==ip */
                                    if (start2.wrapping_offset_from(ip) as
                                            libc::c_long) <
                                           3 as libc::c_int as libc::c_long {
                                        /* First Match too small : removed */
                                        ml = ml2;
                                        ip = start2;
                                        ref_0 = ref2
                                    } else {
                                        loop 
                                             /* let's find a new ML3 */
                                             /* At this stage, we have :
        *  ml2 > ml1, and
        *  ip1+3 <= ip2 (usually < ip1+ml1) */
                                             {
                                            if (start2.wrapping_offset_from(ip)
                                                    as libc::c_long) <
                                                   ((1 as libc::c_uint) <<
                                                        4 as
                                                            libc::c_int).wrapping_sub(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint).wrapping_sub(1
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_uint).wrapping_add(4
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint)
                                                       as libc::c_int as
                                                       libc::c_long {
                                                let mut correction:
                                                        libc::c_int = 0;
                                                let mut new_ml: libc::c_int =
                                                    ml;
                                                if new_ml >
                                                       ((1 as libc::c_uint) <<
                                                            4 as
                                                                libc::c_int).wrapping_sub(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint).wrapping_sub(1
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             libc::c_uint).wrapping_add(4
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            as
                                                                                                                                                            libc::c_uint)
                                                           as libc::c_int {
                                                    new_ml =
                                                        ((1 as libc::c_uint)
                                                             <<
                                                             4 as
                                                                 libc::c_int).wrapping_sub(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint).wrapping_sub(1
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint).wrapping_add(4
                                                                                                                                                             as
                                                                                                                                                             libc::c_int
                                                                                                                                                             as
                                                                                                                                                             libc::c_uint)
                                                            as libc::c_int
                                                }
                                                if ip.offset(new_ml as isize)
                                                       >
                                                       start2.offset(ml2 as
                                                                         isize).offset(-(4
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize))
                                                   {
                                                    new_ml =
                                                        start2.wrapping_offset_from(ip)
                                                            as libc::c_long as
                                                            libc::c_int + ml2
                                                            - 4 as libc::c_int
                                                }
                                                correction =
                                                    new_ml -
                                                        start2.wrapping_offset_from(ip)
                                                            as libc::c_long as
                                                            libc::c_int;
                                                if correction >
                                                       0 as libc::c_int {
                                                    start2 =
                                                        start2.offset(correction
                                                                          as
                                                                          isize);
                                                    ref2 =
                                                        ref2.offset(correction
                                                                        as
                                                                        isize);
                                                    ml2 -= correction
                                                }
                                            }
                                            /* Now, we have start2 = ip+new_ml, with new_ml = min(ml, OPTIMAL_ML=18) */
                                            if start2.offset(ml2 as isize) <=
                                                   mflimit {
                                                ml3 =
                                                    LZ4HC_InsertAndGetWiderMatch(ctx,
                                                                                 start2.offset(ml2
                                                                                                   as
                                                                                                   isize).offset(-(3
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       isize)),
                                                                                 start2,
                                                                                 matchlimit,
                                                                                 ml2,
                                                                                 &mut ref3,
                                                                                 &mut start3,
                                                                                 maxNbAttempts
                                                                                     as
                                                                                     libc::c_int,
                                                                                 patternAnalysis,
                                                                                 0
                                                                                     as
                                                                                     libc::c_int,
                                                                                 dict,
                                                                                 favorCompressionRatio)
                                            } else { ml3 = ml2 }
                                            if ml3 == ml2 {
                                                /* No better match => encode ML1 and ML2 */
                                                /* ip & ref are known; Now for ml */
                                                if start2 <
                                                       ip.offset(ml as isize)
                                                   {
                                                    ml =
                                                        start2.wrapping_offset_from(ip)
                                                            as libc::c_long as
                                                            libc::c_int
                                                }
                                                /* Now, encode 2 sequences */
                                                optr = op;
                                                if LZ4HC_encodeSequence(&mut ip,
                                                                        &mut op,
                                                                        &mut anchor,
                                                                        ml,
                                                                        ref_0,
                                                                        limit,
                                                                        oend)
                                                       != 0 {
                                                    current_block =
                                                        4429472873095296980;
                                                    break 'c_6022 ;
                                                } else {
                                                    current_block =
                                                        3392087639489470149;
                                                    break 'c_6022 ;
                                                }
                                            } else if start3 <
                                                          ip.offset(ml as
                                                                        isize).offset(3
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize)
                                             {
                                                /* Not enough space for match 2 : remove it */
                                                if start3 >=
                                                       ip.offset(ml as isize)
                                                   {
                                                    /* can write Seq1 immediately ==> Seq2 is removed, so Seq3 becomes Seq1 */
                                                    if start2 <
                                                           ip.offset(ml as
                                                                         isize)
                                                       {
                                                        let mut correction_0:
                                                                libc::c_int =
                                                            ip.offset(ml as
                                                                          isize).wrapping_offset_from(start2)
                                                                as
                                                                libc::c_long
                                                                as
                                                                libc::c_int;
                                                        start2 =
                                                            start2.offset(correction_0
                                                                              as
                                                                              isize);
                                                        ref2 =
                                                            ref2.offset(correction_0
                                                                            as
                                                                            isize);
                                                        ml2 -= correction_0;
                                                        if ml2 <
                                                               4 as
                                                                   libc::c_int
                                                           {
                                                            start2 = start3;
                                                            ref2 = ref3;
                                                            ml2 = ml3
                                                        }
                                                    }
                                                    optr = op;
                                                    if LZ4HC_encodeSequence(&mut ip,
                                                                            &mut op,
                                                                            &mut anchor,
                                                                            ml,
                                                                            ref_0,
                                                                            limit,
                                                                            oend)
                                                           != 0 {
                                                        current_block =
                                                            4429472873095296980;
                                                        break 'c_6022 ;
                                                    }
                                                    ip = start3;
                                                    ref_0 = ref3;
                                                    ml = ml3;
                                                    start0 = start2;
                                                    ref0 = ref2;
                                                    ml0 = ml2;
                                                    break ;
                                                } else {
                                                    start2 = start3;
                                                    ref2 = ref3;
                                                    ml2 = ml3
                                                }
                                            } else {
                                                /*
        * OK, now we have 3 ascending matches;
        * let's write the first one ML1.
        * ip & ref are known; Now decide ml.
        */
                                                if start2 <
                                                       ip.offset(ml as isize)
                                                   {
                                                    if (start2.wrapping_offset_from(ip)
                                                            as libc::c_long) <
                                                           ((1 as
                                                                 libc::c_uint)
                                                                <<
                                                                4 as
                                                                    libc::c_int).wrapping_sub(1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint).wrapping_sub(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint).wrapping_add(4
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                                as
                                                                                                                                                                libc::c_uint)
                                                               as libc::c_int
                                                               as libc::c_long
                                                       {
                                                        let mut correction_1:
                                                                libc::c_int =
                                                            0;
                                                        if ml >
                                                               ((1 as
                                                                     libc::c_uint)
                                                                    <<
                                                                    4 as
                                                                        libc::c_int).wrapping_sub(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint).wrapping_sub(1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_uint).wrapping_add(4
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_int
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_uint)
                                                                   as
                                                                   libc::c_int
                                                           {
                                                            ml =
                                                                ((1 as
                                                                      libc::c_uint)
                                                                     <<
                                                                     4 as
                                                                         libc::c_int).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint).wrapping_sub(1
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_uint).wrapping_add(4
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_uint)
                                                                    as
                                                                    libc::c_int
                                                        }
                                                        if ip.offset(ml as
                                                                         isize)
                                                               >
                                                               start2.offset(ml2
                                                                                 as
                                                                                 isize).offset(-(4
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize))
                                                           {
                                                            ml =
                                                                start2.wrapping_offset_from(ip)
                                                                    as
                                                                    libc::c_long
                                                                    as
                                                                    libc::c_int
                                                                    + ml2 -
                                                                    4 as
                                                                        libc::c_int
                                                        }
                                                        correction_1 =
                                                            ml -
                                                                start2.wrapping_offset_from(ip)
                                                                    as
                                                                    libc::c_long
                                                                    as
                                                                    libc::c_int;
                                                        if correction_1 >
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            start2 =
                                                                start2.offset(correction_1
                                                                                  as
                                                                                  isize);
                                                            ref2 =
                                                                ref2.offset(correction_1
                                                                                as
                                                                                isize);
                                                            ml2 -=
                                                                correction_1
                                                        }
                                                    } else {
                                                        ml =
                                                            start2.wrapping_offset_from(ip)
                                                                as
                                                                libc::c_long
                                                                as libc::c_int
                                                    }
                                                }
                                                optr = op;
                                                if LZ4HC_encodeSequence(&mut ip,
                                                                        &mut op,
                                                                        &mut anchor,
                                                                        ml,
                                                                        ref_0,
                                                                        limit,
                                                                        oend)
                                                       != 0 {
                                                    current_block =
                                                        4429472873095296980;
                                                    break 'c_6022 ;
                                                }
                                                /* ML2 becomes ML1 */
                                                ip = start2;
                                                ref_0 = ref2;
                                                ml = ml2;
                                                /* ML3 becomes ML2 */
                                                start2 =
                                                    start3; /* restore correct out pointer */
                                                ref2 = ref3;
                                                ml2 = ml3
                                            }
                                        }
                                    }
                                }
                            }
                        match current_block {
                            3392087639489470149 => {
                                ip = start2;
                                optr = op;
                                if !(LZ4HC_encodeSequence(&mut ip, &mut op,
                                                          &mut anchor, ml2,
                                                          ref2, limit, oend)
                                         != 0) {
                                    current_block = 8236137900636309791;
                                    continue ;
                                }
                            }
                            _ => { }
                        }
                        if limit as libc::c_uint ==
                               fillOutput as libc::c_int as libc::c_uint {
                            op = optr;
                            current_block = 10090817204669828264;
                        } else { return 0 as libc::c_int }
                    }
                }
                _ => {
                    /* Encode Last Literals */
                    let mut lastRunSize: size_t =
                        iend.wrapping_offset_from(anchor) as libc::c_long as
                            size_t; /* literals */
                    let mut litLength: size_t =
                        lastRunSize.wrapping_add(255 as libc::c_int as
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
                                                                                                                     libc::c_ulong); /* restore correct value */
                    let totalSize: size_t =
                        (1 as libc::c_int as
                             libc::c_ulong).wrapping_add(litLength).wrapping_add(lastRunSize); /* Check output limit */
                    if limit as libc::c_uint ==
                           fillOutput as libc::c_int as libc::c_uint {
                        oend = oend.offset(5 as libc::c_int as isize)
                    }
                    if limit as libc::c_uint != 0 &&
                           op.offset(totalSize as isize) > oend {
                        if limit as libc::c_uint ==
                               limitedOutput as libc::c_int as libc::c_uint {
                            return 0 as libc::c_int
                        }
                        /* adapt lastRunSize to fill 'dest' */
                        lastRunSize =
                            (oend.wrapping_offset_from(op) as libc::c_long as
                                 size_t).wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong);
                        litLength =
                            lastRunSize.wrapping_add(255 as libc::c_int as
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
                                                                                                                         libc::c_ulong);
                        lastRunSize =
                            (lastRunSize as
                                 libc::c_ulong).wrapping_sub(litLength) as
                                size_t as size_t
                    }
                    ip = anchor.offset(lastRunSize as isize);
                    if lastRunSize >=
                           ((1 as libc::c_uint) <<
                                8 as libc::c_int -
                                    4 as
                                        libc::c_int).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                               as libc::c_ulong {
                        let mut accumulator: size_t =
                            lastRunSize.wrapping_sub(((1 as libc::c_uint) <<
                                                          8 as libc::c_int -
                                                              4 as
                                                                  libc::c_int).wrapping_sub(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)
                                                         as libc::c_ulong);
                        let fresh9 = op;
                        op = op.offset(1);
                        *fresh9 =
                            (((1 as libc::c_uint) <<
                                  8 as libc::c_int -
                                      4 as
                                          libc::c_int).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                 << 4 as libc::c_int) as BYTE;
                        while accumulator >=
                                  255 as libc::c_int as libc::c_ulong {
                            let fresh10 = op;
                            op = op.offset(1);
                            *fresh10 = 255 as libc::c_int as BYTE;
                            accumulator =
                                (accumulator as
                                     libc::c_ulong).wrapping_sub(255 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as size_t as size_t
                        }
                        let fresh11 = op;
                        op = op.offset(1);
                        *fresh11 = accumulator as BYTE
                    } else {
                        let fresh12 = op;
                        op = op.offset(1);
                        *fresh12 = (lastRunSize << 4 as libc::c_int) as BYTE
                    }
                    memcpy(op as *mut libc::c_void,
                           anchor as *const libc::c_void, lastRunSize);
                    op = op.offset(lastRunSize as isize);
                    /* End */
                    *srcSizePtr =
                        (ip as
                             *const libc::c_char).wrapping_offset_from(source)
                            as libc::c_long as
                            libc::c_int; /* Impossible to store anything */
                    return (op as
                                *mut libc::c_char).wrapping_offset_from(dest)
                               as libc::c_long as libc::c_int
                }
            }
        }; /* Unsupported input size (too large or negative) */
}
#[inline(always)]
unsafe extern "C" fn LZ4HC_compress_generic_internal(ctx:
                                                         *mut LZ4HC_CCtx_internal,
                                                     src: *const libc::c_char,
                                                     dst: *mut libc::c_char,
                                                     srcSizePtr:
                                                         *mut libc::c_int,
                                                     dstCapacity: libc::c_int,
                                                     mut cLevel: libc::c_int,
                                                     limit:
                                                         limitedOutput_directive,
                                                     dict: dictCtx_directive)
 -> libc::c_int {
    static mut clTable: [cParams_t; 13] =
        [{
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 2 as libc::c_int as U32,
                           targetLength:
                               16 as libc::c_int as
                                   U32,}; /* note : convention is different from lz4frame, maybe something to review */
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 2 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 2 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 4 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 8 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 16 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 32 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 64 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 128 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4hc,
                           nbSearches: 256 as libc::c_int as U32,
                           targetLength: 16 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4opt,
                           nbSearches: 96 as libc::c_int as U32,
                           targetLength: 64 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4opt,
                           nbSearches: 512 as libc::c_int as U32,
                           targetLength: 128 as libc::c_int as U32,};
             init
         },
         {
             let mut init =
                 cParams_t{strat: lz4opt,
                           nbSearches: 16384 as libc::c_int as U32,
                           targetLength:
                               ((1 as libc::c_int) << 12 as libc::c_int) as
                                   U32,};
             init
         }];
    if limit as libc::c_uint == fillOutput as libc::c_int as libc::c_uint &&
           dstCapacity < 1 as libc::c_int {
        return 0 as libc::c_int
    }
    if *srcSizePtr as U32 > 0x7e000000 as libc::c_int as U32 {
        return 0 as libc::c_int
    }
    (*ctx).end = (*ctx).end.offset(*srcSizePtr as isize);
    if cLevel < 1 as libc::c_int { cLevel = 9 as libc::c_int }
    cLevel =
        if (12 as libc::c_int) < cLevel { 12 as libc::c_int } else { cLevel };
    let cParam: cParams_t = clTable[cLevel as usize];
    let favor: HCfavor_e =
        if (*ctx).favorDecSpeed as libc::c_int != 0 {
            favorDecompressionSpeed as libc::c_int
        } else { favorCompressionRatio as libc::c_int } as HCfavor_e;
    let mut result: libc::c_int = 0;
    if cParam.strat as libc::c_uint == lz4hc as libc::c_int as libc::c_uint {
        result =
            LZ4HC_compress_hashChain(ctx, src, dst, srcSizePtr, dstCapacity,
                                     cParam.nbSearches, limit, dict)
    } else {
        result =
            LZ4HC_compress_optimal(ctx, src, dst, srcSizePtr, dstCapacity,
                                   cParam.nbSearches as libc::c_int,
                                   cParam.targetLength as size_t, limit,
                                   (cLevel == 12 as libc::c_int) as
                                       libc::c_int, dict, favor)
    }
    if result <= 0 as libc::c_int {
        (*ctx).dirty = 1 as libc::c_int as int8_t
    }
    return result;
}
unsafe extern "C" fn LZ4HC_compress_generic_noDictCtx(ctx:
                                                          *mut LZ4HC_CCtx_internal,
                                                      src:
                                                          *const libc::c_char,
                                                      dst: *mut libc::c_char,
                                                      srcSizePtr:
                                                          *mut libc::c_int,
                                                      dstCapacity:
                                                          libc::c_int,
                                                      mut cLevel: libc::c_int,
                                                      mut limit:
                                                          limitedOutput_directive)
 -> libc::c_int {
    return LZ4HC_compress_generic_internal(ctx, src, dst, srcSizePtr,
                                           dstCapacity, cLevel, limit,
                                           noDictCtx);
}
unsafe extern "C" fn LZ4HC_compress_generic_dictCtx(ctx:
                                                        *mut LZ4HC_CCtx_internal,
                                                    src: *const libc::c_char,
                                                    dst: *mut libc::c_char,
                                                    srcSizePtr:
                                                        *mut libc::c_int,
                                                    dstCapacity: libc::c_int,
                                                    mut cLevel: libc::c_int,
                                                    mut limit:
                                                        limitedOutput_directive)
 -> libc::c_int {
    let position: size_t =
        ((*ctx).end.wrapping_offset_from((*ctx).base) as libc::c_long as
             size_t).wrapping_sub((*ctx).lowLimit as libc::c_ulong);
    if position >=
           (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
               libc::c_ulong {
        (*ctx).dictCtx = 0 as *const LZ4HC_CCtx_internal;
        return LZ4HC_compress_generic_noDictCtx(ctx, src, dst, srcSizePtr,
                                                dstCapacity, cLevel, limit)
    } else if position == 0 as libc::c_int as libc::c_ulong &&
                  *srcSizePtr >
                      4 as libc::c_int *
                          ((1 as libc::c_int) << 10 as libc::c_int) {
        memcpy(ctx as *mut libc::c_void,
               (*ctx).dictCtx as *const libc::c_void,
               ::std::mem::size_of::<LZ4HC_CCtx_internal>() as libc::c_ulong);
        LZ4HC_setExternalDict(ctx, src as *const BYTE);
        (*ctx).compressionLevel = cLevel as libc::c_short;
        return LZ4HC_compress_generic_noDictCtx(ctx, src, dst, srcSizePtr,
                                                dstCapacity, cLevel, limit)
    } else {
        return LZ4HC_compress_generic_internal(ctx, src, dst, srcSizePtr,
                                               dstCapacity, cLevel, limit,
                                               usingDictCtxHc)
    };
}
unsafe extern "C" fn LZ4HC_compress_generic(ctx: *mut LZ4HC_CCtx_internal,
                                            src: *const libc::c_char,
                                            dst: *mut libc::c_char,
                                            srcSizePtr: *mut libc::c_int,
                                            dstCapacity: libc::c_int,
                                            mut cLevel: libc::c_int,
                                            mut limit:
                                                limitedOutput_directive)
 -> libc::c_int {
    if (*ctx).dictCtx.is_null() {
        return LZ4HC_compress_generic_noDictCtx(ctx, src, dst, srcSizePtr,
                                                dstCapacity, cLevel, limit)
    } else {
        return LZ4HC_compress_generic_dictCtx(ctx, src, dst, srcSizePtr,
                                              dstCapacity, cLevel, limit)
    };
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_sizeofStateHC() -> libc::c_int {
    return ::std::mem::size_of::<LZ4_streamHC_t>() as libc::c_ulong as
               libc::c_int;
}
/* for some reason, Visual fails the aligment test on 32-bit x86 :
                   * it reports an aligment of 8-bytes,
                   * while actually aligning LZ4_streamHC_t on 4 bytes. */
unsafe extern "C" fn LZ4_streamHC_t_alignment() -> size_t {
    let mut _t_a: C2RustUnnamed_1 =
        C2RustUnnamed_1{c: 0, t: LZ4_streamHC_u{table: [0; 32775],},};
    return (::std::mem::size_of::<C2RustUnnamed_1>() as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<LZ4_streamHC_t>()
                                                as libc::c_ulong);
}
/* state is presumed correctly initialized,
 * in which case its size and alignment have already been validate */
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_HC_extStateHC_fastReset(mut state:
                                                                  *mut libc::c_void,
                                                              mut src:
                                                                  *const libc::c_char,
                                                              mut dst:
                                                                  *mut libc::c_char,
                                                              mut srcSize:
                                                                  libc::c_int,
                                                              mut dstCapacity:
                                                                  libc::c_int,
                                                              mut compressionLevel:
                                                                  libc::c_int)
 -> libc::c_int {
    let ctx: *mut LZ4HC_CCtx_internal =
        &mut (*(state as *mut LZ4_streamHC_t)).internal_donotuse;
    /* for some reason, Visual fails the aligment test on 32-bit x86 :
                   * it reports an aligment of 8-bytes,
                   * while actually aligning LZ4_streamHC_t on 4 bytes. */
    /* check alignment */
    if state as size_t &
           (::std::mem::size_of::<*mut libc::c_void>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
           != 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    } /* Error : state is not aligned for pointers (32 or 64 bits) */
    LZ4_resetStreamHC_fast(state as *mut LZ4_streamHC_t,
                           compressionLevel); /* init failure */
    LZ4HC_init_internal(ctx, src as *const BYTE);
    if dstCapacity < LZ4_compressBound(srcSize) {
        return LZ4HC_compress_generic(ctx, src, dst, &mut srcSize,
                                      dstCapacity, compressionLevel,
                                      limitedOutput)
    } else {
        return LZ4HC_compress_generic(ctx, src, dst, &mut srcSize,
                                      dstCapacity, compressionLevel,
                                      notLimited)
    };
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_HC_extStateHC(mut state:
                                                        *mut libc::c_void,
                                                    mut src:
                                                        *const libc::c_char,
                                                    mut dst:
                                                        *mut libc::c_char,
                                                    mut srcSize: libc::c_int,
                                                    mut dstCapacity:
                                                        libc::c_int,
                                                    mut compressionLevel:
                                                        libc::c_int)
 -> libc::c_int {
    let ctx: *mut LZ4_streamHC_t =
        LZ4_initStreamHC(state,
                         ::std::mem::size_of::<LZ4_streamHC_t>() as
                             libc::c_ulong);
    if ctx.is_null() { return 0 as libc::c_int }
    return LZ4_compress_HC_extStateHC_fastReset(state, src, dst, srcSize,
                                                dstCapacity,
                                                compressionLevel);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_HC(mut src: *const libc::c_char,
                                         mut dst: *mut libc::c_char,
                                         mut srcSize: libc::c_int,
                                         mut dstCapacity: libc::c_int,
                                         mut compressionLevel: libc::c_int)
 -> libc::c_int {
    let statePtr: *mut LZ4_streamHC_t =
        malloc(::std::mem::size_of::<LZ4_streamHC_t>() as libc::c_ulong) as
            *mut LZ4_streamHC_t;
    let cSize: libc::c_int =
        LZ4_compress_HC_extStateHC(statePtr as *mut libc::c_void, src, dst,
                                   srcSize, dstCapacity, compressionLevel);
    free(statePtr as *mut libc::c_void);
    return cSize;
}
/* state is presumed sized correctly (>= sizeof(LZ4_streamHC_t)) */
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_HC_destSize(mut state:
                                                      *mut libc::c_void,
                                                  mut source:
                                                      *const libc::c_char,
                                                  mut dest: *mut libc::c_char,
                                                  mut sourceSizePtr:
                                                      *mut libc::c_int,
                                                  mut targetDestSize:
                                                      libc::c_int,
                                                  mut cLevel: libc::c_int)
 -> libc::c_int {
    let ctx: *mut LZ4_streamHC_t =
        LZ4_initStreamHC(state,
                         ::std::mem::size_of::<LZ4_streamHC_t>() as
                             libc::c_ulong); /* init failure */
    if ctx.is_null() { return 0 as libc::c_int }
    LZ4HC_init_internal(&mut (*ctx).internal_donotuse, source as *const BYTE);
    LZ4_setCompressionLevel(ctx, cLevel);
    return LZ4HC_compress_generic(&mut (*ctx).internal_donotuse, source, dest,
                                  sourceSizePtr, targetDestSize, cLevel,
                                  fillOutput);
}
/* *************************************
*  Streaming Functions
**************************************/
/* allocation */
#[no_mangle]
pub unsafe extern "C" fn LZ4_createStreamHC() -> *mut LZ4_streamHC_t {
    let LZ4_streamHCPtr: *mut LZ4_streamHC_t =
        malloc(::std::mem::size_of::<LZ4_streamHC_t>() as libc::c_ulong) as
            *mut LZ4_streamHC_t; /* full initialization, malloc'ed buffer can be full of garbage */
    if LZ4_streamHCPtr.is_null() {
        return 0 as *mut LZ4_streamHC_t
    } /* support free on NULL */
    LZ4_initStreamHC(LZ4_streamHCPtr as *mut libc::c_void,
                     ::std::mem::size_of::<LZ4_streamHC_t>() as
                         libc::c_ulong);
    return LZ4_streamHCPtr;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_freeStreamHC(mut LZ4_streamHCPtr:
                                              *mut LZ4_streamHC_t)
 -> libc::c_int {
    if LZ4_streamHCPtr.is_null() { return 0 as libc::c_int }
    free(LZ4_streamHCPtr as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_initStreamHC(mut buffer: *mut libc::c_void,
                                          mut size: size_t)
 -> *mut LZ4_streamHC_t {
    let LZ4_streamHCPtr: *mut LZ4_streamHC_t = buffer as *mut LZ4_streamHC_t;
    if buffer.is_null() { return 0 as *mut LZ4_streamHC_t }
    if size < ::std::mem::size_of::<LZ4_streamHC_t>() as libc::c_ulong {
        return 0 as *mut LZ4_streamHC_t
    }
    /* for some reason, Visual fails the aligment test on 32-bit x86 :
                   * it reports an aligment of 8-bytes,
                   * while actually aligning LZ4_streamHC_t on 4 bytes. */
    if buffer as size_t &
           LZ4_streamHC_t_alignment().wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) != 0 {
        return 0 as *mut LZ4_streamHC_t
    } /* alignment check */
    /* if compilation fails here, LZ4_STREAMHCSIZE must be increased */
    /* end-base will trigger a clearTable on starting compression */
    (*LZ4_streamHCPtr).internal_donotuse.end =
        -(1 as libc::c_int) as ptrdiff_t as *const BYTE;
    (*LZ4_streamHCPtr).internal_donotuse.base = 0 as *const uint8_t;
    (*LZ4_streamHCPtr).internal_donotuse.dictCtx =
        0 as *const LZ4HC_CCtx_internal;
    (*LZ4_streamHCPtr).internal_donotuse.favorDecSpeed =
        0 as libc::c_int as int8_t;
    (*LZ4_streamHCPtr).internal_donotuse.dirty = 0 as libc::c_int as int8_t;
    LZ4_setCompressionLevel(LZ4_streamHCPtr, 9 as libc::c_int);
    return LZ4_streamHCPtr;
}
/* just a stub */
#[no_mangle]
pub unsafe extern "C" fn LZ4_resetStreamHC(mut LZ4_streamHCPtr:
                                               *mut LZ4_streamHC_t,
                                           mut compressionLevel:
                                               libc::c_int) {
    LZ4_initStreamHC(LZ4_streamHCPtr as *mut libc::c_void,
                     ::std::mem::size_of::<LZ4_streamHC_t>() as
                         libc::c_ulong);
    LZ4_setCompressionLevel(LZ4_streamHCPtr, compressionLevel);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_resetStreamHC_fast(mut LZ4_streamHCPtr:
                                                    *mut LZ4_streamHC_t,
                                                mut compressionLevel:
                                                    libc::c_int) {
    if (*LZ4_streamHCPtr).internal_donotuse.dirty != 0 {
        LZ4_initStreamHC(LZ4_streamHCPtr as *mut libc::c_void,
                         ::std::mem::size_of::<LZ4_streamHC_t>() as
                             libc::c_ulong);
    } else {
        /* preserve end - base : can trigger clearTable's threshold */
        (*LZ4_streamHCPtr).internal_donotuse.end =
            (*LZ4_streamHCPtr).internal_donotuse.end.offset(-((*LZ4_streamHCPtr).internal_donotuse.base
                                                                  as uptrval
                                                                  as isize));
        (*LZ4_streamHCPtr).internal_donotuse.base = 0 as *const uint8_t;
        (*LZ4_streamHCPtr).internal_donotuse.dictCtx =
            0 as *const LZ4HC_CCtx_internal
    }
    LZ4_setCompressionLevel(LZ4_streamHCPtr, compressionLevel);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_setCompressionLevel(mut LZ4_streamHCPtr:
                                                     *mut LZ4_streamHC_t,
                                                 mut compressionLevel:
                                                     libc::c_int) {
    if compressionLevel < 1 as libc::c_int {
        compressionLevel = 9 as libc::c_int
    }
    if compressionLevel > 12 as libc::c_int {
        compressionLevel = 12 as libc::c_int
    }
    (*LZ4_streamHCPtr).internal_donotuse.compressionLevel =
        compressionLevel as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_favorDecompressionSpeed(mut LZ4_streamHCPtr:
                                                         *mut LZ4_streamHC_t,
                                                     mut favor: libc::c_int) {
    (*LZ4_streamHCPtr).internal_donotuse.favorDecSpeed =
        (favor != 0 as libc::c_int) as libc::c_int as int8_t;
}
/* LZ4_loadDictHC() :
 * LZ4_streamHCPtr is presumed properly initialized */
#[no_mangle]
pub unsafe extern "C" fn LZ4_loadDictHC(mut LZ4_streamHCPtr:
                                            *mut LZ4_streamHC_t,
                                        mut dictionary: *const libc::c_char,
                                        mut dictSize: libc::c_int)
 -> libc::c_int {
    let ctxPtr: *mut LZ4HC_CCtx_internal =
        &mut (*LZ4_streamHCPtr).internal_donotuse;
    if dictSize >
           64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int) {
        dictionary =
            dictionary.offset((dictSize as
                                   size_t).wrapping_sub((64 as libc::c_int *
                                                             ((1 as
                                                                   libc::c_int)
                                                                  <<
                                                                  10 as
                                                                      libc::c_int))
                                                            as libc::c_ulong)
                                  as isize);
        dictSize =
            64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)
    }
    /* need a full initialization, there are bad side-effects when using resetFast() */
    let cLevel: libc::c_int = (*ctxPtr).compressionLevel as libc::c_int;
    LZ4_initStreamHC(LZ4_streamHCPtr as *mut libc::c_void,
                     ::std::mem::size_of::<LZ4_streamHC_t>() as
                         libc::c_ulong);
    LZ4_setCompressionLevel(LZ4_streamHCPtr, cLevel);
    LZ4HC_init_internal(ctxPtr, dictionary as *const BYTE);
    (*ctxPtr).end = (dictionary as *const BYTE).offset(dictSize as isize);
    if dictSize >= 4 as libc::c_int {
        LZ4HC_Insert(ctxPtr,
                     (*ctxPtr).end.offset(-(3 as libc::c_int as isize)));
    }
    return dictSize;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_attach_HC_dictionary(mut working_stream:
                                                      *mut LZ4_streamHC_t,
                                                  mut dictionary_stream:
                                                      *const LZ4_streamHC_t) {
    (*working_stream).internal_donotuse.dictCtx =
        if !dictionary_stream.is_null() {
            &(*dictionary_stream).internal_donotuse
        } else { 0 as *const LZ4HC_CCtx_internal };
}
/* compression */
unsafe extern "C" fn LZ4HC_setExternalDict(mut ctxPtr:
                                               *mut LZ4HC_CCtx_internal,
                                           mut newBlock: *const BYTE) {
    if (*ctxPtr).end >=
           (*ctxPtr).base.offset((*ctxPtr).dictLimit as
                                     isize).offset(4 as libc::c_int as isize)
       {
        LZ4HC_Insert(ctxPtr,
                     (*ctxPtr).end.offset(-(3 as libc::c_int as
                                                isize))); /* Referencing remaining dictionary content */
    }
    /* Only one memory segment for extDict, so any previous extDict is lost at this stage */
    (*ctxPtr).lowLimit =
        (*ctxPtr).dictLimit; /* match referencing will resume from there */
    (*ctxPtr).dictLimit =
        (*ctxPtr).end.wrapping_offset_from((*ctxPtr).base) as libc::c_long as
            U32;
    (*ctxPtr).dictBase = (*ctxPtr).base;
    (*ctxPtr).base = newBlock.offset(-((*ctxPtr).dictLimit as isize));
    (*ctxPtr).end = newBlock;
    (*ctxPtr).nextToUpdate = (*ctxPtr).dictLimit;
    /* cannot reference an extDict and a dictCtx at the same time */
    (*ctxPtr).dictCtx = 0 as *const LZ4HC_CCtx_internal;
}
unsafe extern "C" fn LZ4_compressHC_continue_generic(mut LZ4_streamHCPtr:
                                                         *mut LZ4_streamHC_t,
                                                     mut src:
                                                         *const libc::c_char,
                                                     mut dst:
                                                         *mut libc::c_char,
                                                     mut srcSizePtr:
                                                         *mut libc::c_int,
                                                     mut dstCapacity:
                                                         libc::c_int,
                                                     mut limit:
                                                         limitedOutput_directive)
 -> libc::c_int {
    let ctxPtr: *mut LZ4HC_CCtx_internal =
        &mut (*LZ4_streamHCPtr).internal_donotuse;
    /* auto-init if forgotten */
    if (*ctxPtr).base.is_null() {
        LZ4HC_init_internal(ctxPtr, src as *const BYTE);
    }
    /* Check overflow */
    if (*ctxPtr).end.wrapping_offset_from((*ctxPtr).base) as libc::c_long as
           size_t >
           (2 as libc::c_int as
                libc::c_uint).wrapping_mul((1 as libc::c_uint) <<
                                               30 as libc::c_int) as
               libc::c_ulong {
        let mut dictSize: size_t =
            ((*ctxPtr).end.wrapping_offset_from((*ctxPtr).base) as
                 libc::c_long as
                 size_t).wrapping_sub((*ctxPtr).dictLimit as libc::c_ulong);
        if dictSize >
               (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                   as libc::c_ulong {
            dictSize =
                (64 as libc::c_int *
                     ((1 as libc::c_int) << 10 as libc::c_int)) as size_t
        }
        LZ4_loadDictHC(LZ4_streamHCPtr,
                       ((*ctxPtr).end as
                            *const libc::c_char).offset(-(dictSize as isize)),
                       dictSize as libc::c_int);
    }
    /* Check if blocks follow each other */
    if src as *const BYTE != (*ctxPtr).end {
        LZ4HC_setExternalDict(ctxPtr, src as *const BYTE);
    }
    /* Check overlapping input/dictionary space */
    let mut sourceEnd: *const BYTE =
        (src as *const BYTE).offset(*srcSizePtr as isize);
    let dictBegin: *const BYTE =
        (*ctxPtr).dictBase.offset((*ctxPtr).lowLimit as isize);
    let dictEnd: *const BYTE =
        (*ctxPtr).dictBase.offset((*ctxPtr).dictLimit as isize);
    if sourceEnd > dictBegin && (src as *const BYTE) < dictEnd {
        if sourceEnd > dictEnd { sourceEnd = dictEnd }
        (*ctxPtr).lowLimit =
            sourceEnd.wrapping_offset_from((*ctxPtr).dictBase) as libc::c_long
                as U32;
        if (*ctxPtr).dictLimit.wrapping_sub((*ctxPtr).lowLimit) <
               4 as libc::c_int as libc::c_uint {
            (*ctxPtr).lowLimit = (*ctxPtr).dictLimit
        }
    }
    return LZ4HC_compress_generic(ctxPtr, src, dst, srcSizePtr, dstCapacity,
                                  (*ctxPtr).compressionLevel as libc::c_int,
                                  limit);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_HC_continue(mut LZ4_streamHCPtr:
                                                      *mut LZ4_streamHC_t,
                                                  mut src:
                                                      *const libc::c_char,
                                                  mut dst: *mut libc::c_char,
                                                  mut srcSize: libc::c_int,
                                                  mut dstCapacity:
                                                      libc::c_int)
 -> libc::c_int {
    if dstCapacity < LZ4_compressBound(srcSize) {
        return LZ4_compressHC_continue_generic(LZ4_streamHCPtr, src, dst,
                                               &mut srcSize, dstCapacity,
                                               limitedOutput)
    } else {
        return LZ4_compressHC_continue_generic(LZ4_streamHCPtr, src, dst,
                                               &mut srcSize, dstCapacity,
                                               notLimited)
    };
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compress_HC_continue_destSize(mut LZ4_streamHCPtr:
                                                               *mut LZ4_streamHC_t,
                                                           mut src:
                                                               *const libc::c_char,
                                                           mut dst:
                                                               *mut libc::c_char,
                                                           mut srcSizePtr:
                                                               *mut libc::c_int,
                                                           mut targetDestSize:
                                                               libc::c_int)
 -> libc::c_int {
    return LZ4_compressHC_continue_generic(LZ4_streamHCPtr, src, dst,
                                           srcSizePtr, targetDestSize,
                                           fillOutput);
}
/* dictionary saving */
#[no_mangle]
pub unsafe extern "C" fn LZ4_saveDictHC(mut LZ4_streamHCPtr:
                                            *mut LZ4_streamHC_t,
                                        mut safeBuffer: *mut libc::c_char,
                                        mut dictSize: libc::c_int)
 -> libc::c_int {
    let streamPtr: *mut LZ4HC_CCtx_internal =
        &mut (*LZ4_streamHCPtr).internal_donotuse;
    let prefixSize: libc::c_int =
        (*streamPtr).end.wrapping_offset_from((*streamPtr).base.offset((*streamPtr).dictLimit
                                                                           as
                                                                           isize))
            as libc::c_long as libc::c_int;
    if dictSize >
           64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int) {
        dictSize =
            64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)
    }
    if dictSize < 4 as libc::c_int { dictSize = 0 as libc::c_int }
    if dictSize > prefixSize { dictSize = prefixSize }
    memmove(safeBuffer as *mut libc::c_void,
            (*streamPtr).end.offset(-(dictSize as isize)) as
                *const libc::c_void, dictSize as libc::c_ulong);
    let endIndex: U32 =
        (*streamPtr).end.wrapping_offset_from((*streamPtr).base) as
            libc::c_long as U32;
    (*streamPtr).end = (safeBuffer as *const BYTE).offset(dictSize as isize);
    (*streamPtr).base = (*streamPtr).end.offset(-(endIndex as isize));
    (*streamPtr).dictLimit = endIndex.wrapping_sub(dictSize as U32);
    (*streamPtr).lowLimit = endIndex.wrapping_sub(dictSize as U32);
    if (*streamPtr).nextToUpdate < (*streamPtr).dictLimit {
        (*streamPtr).nextToUpdate = (*streamPtr).dictLimit
    }
    return dictSize;
}
/* **************************************************
*  Deprecated Functions
***************************************************/
/* These functions currently generate deprecation warnings */
/* Wrappers for deprecated compression functions */
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC(mut src: *const libc::c_char,
                                        mut dst: *mut libc::c_char,
                                        mut srcSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC(src, dst, srcSize, LZ4_compressBound(srcSize),
                           0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC_limitedOutput(mut src:
                                                          *const libc::c_char,
                                                      mut dst:
                                                          *mut libc::c_char,
                                                      mut srcSize:
                                                          libc::c_int,
                                                      mut maxDstSize:
                                                          libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC(src, dst, srcSize, maxDstSize, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC2(mut src: *const libc::c_char,
                                         mut dst: *mut libc::c_char,
                                         mut srcSize: libc::c_int,
                                         mut cLevel: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC(src, dst, srcSize, LZ4_compressBound(srcSize),
                           cLevel);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC2_limitedOutput(mut src:
                                                           *const libc::c_char,
                                                       mut dst:
                                                           *mut libc::c_char,
                                                       mut srcSize:
                                                           libc::c_int,
                                                       mut maxDstSize:
                                                           libc::c_int,
                                                       mut cLevel:
                                                           libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC(src, dst, srcSize, maxDstSize, cLevel);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC_withStateHC(mut state:
                                                        *mut libc::c_void,
                                                    mut src:
                                                        *const libc::c_char,
                                                    mut dst:
                                                        *mut libc::c_char,
                                                    mut srcSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC_extStateHC(state, src, dst, srcSize,
                                      LZ4_compressBound(srcSize),
                                      0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC_limitedOutput_withStateHC(mut state:
                                                                      *mut libc::c_void,
                                                                  mut src:
                                                                      *const libc::c_char,
                                                                  mut dst:
                                                                      *mut libc::c_char,
                                                                  mut srcSize:
                                                                      libc::c_int,
                                                                  mut maxDstSize:
                                                                      libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC_extStateHC(state, src, dst, srcSize, maxDstSize,
                                      0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC2_withStateHC(mut state:
                                                         *mut libc::c_void,
                                                     mut src:
                                                         *const libc::c_char,
                                                     mut dst:
                                                         *mut libc::c_char,
                                                     mut srcSize: libc::c_int,
                                                     mut cLevel: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC_extStateHC(state, src, dst, srcSize,
                                      LZ4_compressBound(srcSize), cLevel);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC2_limitedOutput_withStateHC(mut state:
                                                                       *mut libc::c_void,
                                                                   mut src:
                                                                       *const libc::c_char,
                                                                   mut dst:
                                                                       *mut libc::c_char,
                                                                   mut srcSize:
                                                                       libc::c_int,
                                                                   mut maxDstSize:
                                                                       libc::c_int,
                                                                   mut cLevel:
                                                                       libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC_extStateHC(state, src, dst, srcSize, maxDstSize,
                                      cLevel);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC_continue(mut ctx: *mut LZ4_streamHC_t,
                                                 mut src: *const libc::c_char,
                                                 mut dst: *mut libc::c_char,
                                                 mut srcSize: libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC_continue(ctx, src, dst, srcSize,
                                    LZ4_compressBound(srcSize));
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC_limitedOutput_continue(mut ctx:
                                                                   *mut LZ4_streamHC_t,
                                                               mut src:
                                                                   *const libc::c_char,
                                                               mut dst:
                                                                   *mut libc::c_char,
                                                               mut srcSize:
                                                                   libc::c_int,
                                                               mut maxDstSize:
                                                                   libc::c_int)
 -> libc::c_int {
    return LZ4_compress_HC_continue(ctx, src, dst, srcSize, maxDstSize);
}
/* Deprecated streaming functions */
#[no_mangle]
pub unsafe extern "C" fn LZ4_sizeofStreamStateHC() -> libc::c_int {
    return 4 as libc::c_int * ((1 as libc::c_int) << 15 as libc::c_int) +
               2 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int) +
               56 as libc::c_int +
               (if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                       == 16 as libc::c_int as libc::c_ulong {
                    56 as libc::c_int
                } else { 0 as libc::c_int });
}
/* state is presumed correctly sized, aka >= sizeof(LZ4_streamHC_t)
 * @return : 0 on success, !=0 if error */
#[no_mangle]
pub unsafe extern "C" fn LZ4_resetStreamStateHC(mut state: *mut libc::c_void,
                                                mut inputBuffer:
                                                    *mut libc::c_char)
 -> libc::c_int {
    let hc4: *mut LZ4_streamHC_t =
        LZ4_initStreamHC(state,
                         ::std::mem::size_of::<LZ4_streamHC_t>() as
                             libc::c_ulong); /* init failed */
    if hc4.is_null() { return 1 as libc::c_int } /* not enough memory */
    LZ4HC_init_internal(&mut (*hc4).internal_donotuse,
                        inputBuffer as
                            *const BYTE); /* support free on NULL */
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_createHC(mut inputBuffer: *const libc::c_char)
 -> *mut libc::c_void {
    let hc4: *mut LZ4_streamHC_t = LZ4_createStreamHC();
    if hc4.is_null() { return 0 as *mut libc::c_void }
    LZ4HC_init_internal(&mut (*hc4).internal_donotuse,
                        inputBuffer as *const BYTE);
    return hc4 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_freeHC(mut LZ4HC_Data: *mut libc::c_void)
 -> libc::c_int {
    if LZ4HC_Data.is_null() { return 0 as libc::c_int }
    free(LZ4HC_Data);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC2_continue(mut LZ4HC_Data:
                                                      *mut libc::c_void,
                                                  mut src:
                                                      *const libc::c_char,
                                                  mut dst: *mut libc::c_char,
                                                  mut srcSize: libc::c_int,
                                                  mut cLevel: libc::c_int)
 -> libc::c_int {
    return LZ4HC_compress_generic(&mut (*(LZ4HC_Data as
                                              *mut LZ4_streamHC_t)).internal_donotuse,
                                  src, dst, &mut srcSize, 0 as libc::c_int,
                                  cLevel, notLimited);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_compressHC2_limitedOutput_continue(mut LZ4HC_Data:
                                                                    *mut libc::c_void,
                                                                mut src:
                                                                    *const libc::c_char,
                                                                mut dst:
                                                                    *mut libc::c_char,
                                                                mut srcSize:
                                                                    libc::c_int,
                                                                mut dstCapacity:
                                                                    libc::c_int,
                                                                mut cLevel:
                                                                    libc::c_int)
 -> libc::c_int {
    return LZ4HC_compress_generic(&mut (*(LZ4HC_Data as
                                              *mut LZ4_streamHC_t)).internal_donotuse,
                                  src, dst, &mut srcSize, dstCapacity, cLevel,
                                  limitedOutput);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_slideInputBufferHC(mut LZ4HC_Data:
                                                    *mut libc::c_void)
 -> *mut libc::c_char {
    let mut ctx: *mut LZ4_streamHC_t = LZ4HC_Data as *mut LZ4_streamHC_t;
    let mut bufferStart: *const BYTE =
        (*ctx).internal_donotuse.base.offset((*ctx).internal_donotuse.lowLimit
                                                 as isize);
    LZ4_resetStreamHC_fast(ctx,
                           (*ctx).internal_donotuse.compressionLevel as
                               libc::c_int);
    /* avoid const char * -> char * conversion warning :( */
    return bufferStart as uptrval as *mut libc::c_char;
}
/* price in bytes */
#[inline(always)]
unsafe extern "C" fn LZ4HC_literalsPrice(litlen: libc::c_int) -> libc::c_int {
    let mut price: libc::c_int = litlen;
    if litlen >=
           ((1 as libc::c_uint) <<
                8 as libc::c_int -
                    4 as
                        libc::c_int).wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as
               libc::c_int {
        price +=
            1 as libc::c_int +
                (litlen -
                     ((1 as libc::c_uint) <<
                          8 as libc::c_int -
                              4 as
                                  libc::c_int).wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                         as libc::c_int) / 255 as libc::c_int
    }
    return price;
}
/* requires mlen >= MINMATCH */
#[inline(always)]
unsafe extern "C" fn LZ4HC_sequencePrice(mut litlen: libc::c_int,
                                         mut mlen: libc::c_int)
 -> libc::c_int {
    let mut price: libc::c_int =
        1 as libc::c_int + 2 as libc::c_int; /* token + 16-bit offset */
    price += LZ4HC_literalsPrice(litlen);
    if mlen >=
           ((1 as libc::c_uint) <<
                4 as
                    libc::c_int).wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint).wrapping_add(4
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
               as libc::c_int {
        price +=
            1 as libc::c_int +
                (mlen -
                     ((1 as libc::c_uint) <<
                          4 as
                              libc::c_int).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint).wrapping_add(4
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint)
                         as libc::c_int) / 255 as libc::c_int
    }
    return price;
}
#[inline(always)]
unsafe extern "C" fn LZ4HC_FindLongerMatch(ctx: *mut LZ4HC_CCtx_internal,
                                           mut ip: *const BYTE,
                                           iHighLimit: *const BYTE,
                                           mut minLen: libc::c_int,
                                           mut nbSearches: libc::c_int,
                                           dict: dictCtx_directive,
                                           favorDecSpeed: HCfavor_e)
 -> LZ4HC_match_t {
    let mut match_0: LZ4HC_match_t =
        {
            let mut init =
                LZ4HC_match_t{off: 0 as libc::c_int, len: 0 as libc::c_int,};
            init
        };
    let mut matchPtr: *const BYTE = 0 as *const BYTE;
    /* note : LZ4HC_InsertAndGetWiderMatch() is able to modify the starting position of a match (*startpos),
     * but this won't be the case here, as we define iLowLimit==ip,
     * so LZ4HC_InsertAndGetWiderMatch() won't be allowed to search past ip */
    let mut matchLength: libc::c_int =
        LZ4HC_InsertAndGetWiderMatch(ctx, ip, ip, iHighLimit, minLen,
                                     &mut matchPtr, &mut ip, nbSearches,
                                     1 as libc::c_int, 1 as libc::c_int, dict,
                                     favorDecSpeed);
    if matchLength <= minLen { return match_0 }
    if favorDecSpeed as u64 != 0 {
        if (matchLength > 18 as libc::c_int) as libc::c_int &
               (matchLength <= 36 as libc::c_int) as libc::c_int != 0 {
            matchLength = 18 as libc::c_int
        }
        /* favor shortcut */
    }
    match_0.len = matchLength;
    match_0.off =
        ip.wrapping_offset_from(matchPtr) as libc::c_long as libc::c_int;
    return match_0;
}
unsafe extern "C" fn LZ4HC_compress_optimal(mut ctx: *mut LZ4HC_CCtx_internal,
                                            source: *const libc::c_char,
                                            mut dst: *mut libc::c_char,
                                            mut srcSizePtr: *mut libc::c_int,
                                            mut dstCapacity: libc::c_int,
                                            nbSearches: libc::c_int,
                                            mut sufficient_len: size_t,
                                            limit: limitedOutput_directive,
                                            fullUpdate: libc::c_int,
                                            dict: dictCtx_directive,
                                            favorDecSpeed: HCfavor_e)
 -> libc::c_int {
    let mut current_block: u64;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let opt: *mut LZ4HC_optimal_t =
        malloc((::std::mem::size_of::<LZ4HC_optimal_t>() as
                    libc::c_ulong).wrapping_mul((((1 as libc::c_int) <<
                                                      12 as libc::c_int) +
                                                     3 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut LZ4HC_optimal_t;
    let mut ip: *const BYTE = source as *const BYTE;
    let mut anchor: *const BYTE = ip;
    let iend: *const BYTE = ip.offset(*srcSizePtr as isize);
    let mflimit: *const BYTE = iend.offset(-(12 as libc::c_int as isize));
    let matchlimit: *const BYTE = iend.offset(-(5 as libc::c_int as isize));
    let mut op: *mut BYTE = dst as *mut BYTE;
    let mut opSaved: *mut BYTE = dst as *mut BYTE;
    let mut oend: *mut BYTE = op.offset(dstCapacity as isize);
    /* init */
    if !opt.is_null() {
        *srcSizePtr =
            0 as libc::c_int; /* Hack for support LZ4 format restriction */
        if limit as libc::c_uint == fillOutput as libc::c_int as libc::c_uint
           {
            oend = oend.offset(-(5 as libc::c_int as isize))
        }
        if sufficient_len >=
               ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_ulong {
            sufficient_len =
                (((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int)
                    as size_t
        }
        /* Main Loop */
        's_73:
            loop  {
                if !(ip <= mflimit) {
                    current_block =
                        6535448574339961634; /* while (ip <= mflimit) */
                    break ;
                }
                let llen: libc::c_int =
                    ip.wrapping_offset_from(anchor) as libc::c_long as
                        libc::c_int;
                let mut best_mlen: libc::c_int = 0;
                let mut best_off: libc::c_int = 0;
                let mut cur: libc::c_int = 0;
                let mut last_match_pos: libc::c_int = 0 as libc::c_int;
                let firstMatch: LZ4HC_match_t =
                    LZ4HC_FindLongerMatch(ctx, ip, matchlimit,
                                          4 as libc::c_int - 1 as libc::c_int,
                                          nbSearches, dict, favorDecSpeed);
                if firstMatch.len == 0 as libc::c_int {
                    ip = ip.offset(1)
                } else if firstMatch.len as size_t > sufficient_len {
                    /* good enough solution : immediate encoding */
                    let firstML: libc::c_int = firstMatch.len;
                    let matchPos: *const BYTE =
                        ip.offset(-(firstMatch.off as isize));
                    opSaved = op;
                    if !(LZ4HC_encodeSequence(&mut ip, &mut op, &mut anchor,
                                              firstML, matchPos, limit, oend)
                             != 0) {
                        continue ;
                    }
                    /* updates ip, op and anchor */
                    current_block = 12983150111686922718;
                    break ;
                } else {
                    /* set prices for first positions (literals) */
                    let mut rPos: libc::c_int = 0;
                    rPos = 0 as libc::c_int;
                    while rPos < 4 as libc::c_int {
                        let cost: libc::c_int =
                            LZ4HC_literalsPrice(llen + rPos);
                        (*opt.offset(rPos as isize)).mlen = 1 as libc::c_int;
                        (*opt.offset(rPos as isize)).off = 0 as libc::c_int;
                        (*opt.offset(rPos as isize)).litlen = llen + rPos;
                        (*opt.offset(rPos as isize)).price = cost;
                        rPos += 1
                    }
                    /* set prices using initial match */
                    let mut mlen: libc::c_int =
                        4 as
                            libc::c_int; /* necessarily < sufficient_len < LZ4_OPT_NUM */
                    let matchML: libc::c_int = firstMatch.len; /* literal */
                    let offset: libc::c_int = firstMatch.off;
                    while mlen <= matchML {
                        let cost_0: libc::c_int =
                            LZ4HC_sequencePrice(llen, mlen);
                        (*opt.offset(mlen as isize)).mlen = mlen;
                        (*opt.offset(mlen as isize)).off = offset;
                        (*opt.offset(mlen as isize)).litlen = llen;
                        (*opt.offset(mlen as isize)).price = cost_0;
                        mlen += 1
                    }
                    last_match_pos = firstMatch.len;
                    let mut addLit: libc::c_int = 0;
                    addLit = 1 as libc::c_int;
                    while addLit <= 3 as libc::c_int {
                        (*opt.offset((last_match_pos + addLit) as isize)).mlen
                            = 1 as libc::c_int;
                        (*opt.offset((last_match_pos + addLit) as isize)).off
                            = 0 as libc::c_int;
                        (*opt.offset((last_match_pos + addLit) as
                                         isize)).litlen = addLit;
                        (*opt.offset((last_match_pos + addLit) as
                                         isize)).price =
                            (*opt.offset(last_match_pos as isize)).price +
                                LZ4HC_literalsPrice(addLit);
                        addLit += 1
                    }
                    /* check further positions */
                    cur =
                        1 as
                            libc::c_int; /* for (cur = 1; cur <= last_match_pos; cur++) */
                    loop  {
                        if !(cur < last_match_pos) {
                            current_block = 5590933039760577279;
                            break ;
                        }
                        let curPtr: *const BYTE = ip.offset(cur as isize);
                        let mut newMatch: LZ4HC_match_t =
                            LZ4HC_match_t{off: 0, len: 0,};
                        if curPtr > mflimit {
                            current_block = 5590933039760577279;
                            break ;
                        }
                        if fullUpdate != 0 {
                            /* not useful to search here if next position has same (or lower) cost */
                            if (*opt.offset((cur + 1 as libc::c_int) as
                                                isize)).price <=
                                   (*opt.offset(cur as isize)).price &&
                                   (*opt.offset((cur + 4 as libc::c_int) as
                                                    isize)).price <
                                       (*opt.offset(cur as isize)).price +
                                           3 as libc::c_int {
                                current_block = 6721012065216013753;
                            } else { current_block = 13484060386966298149; }
                        } else if (*opt.offset((cur + 1 as libc::c_int) as
                                                   isize)).price <=
                                      (*opt.offset(cur as isize)).price {
                            current_block = 6721012065216013753;
                        } else { current_block = 13484060386966298149; }
                        match current_block {
                            13484060386966298149 => {
                                if fullUpdate != 0 {
                                    newMatch =
                                        LZ4HC_FindLongerMatch(ctx, curPtr,
                                                              matchlimit,
                                                              4 as libc::c_int
                                                                  -
                                                                  1 as
                                                                      libc::c_int,
                                                              nbSearches,
                                                              dict,
                                                              favorDecSpeed)
                                } else {
                                    /* not useful to search here if next position has same (or lower) cost */
                                    /* only test matches of minimum length; slightly faster, but misses a few bytes */
                                    newMatch =
                                        LZ4HC_FindLongerMatch(ctx, curPtr,
                                                              matchlimit,
                                                              last_match_pos -
                                                                  cur,
                                                              nbSearches,
                                                              dict,
                                                              favorDecSpeed)
                                }
                                if !(newMatch.len == 0) {
                                    if newMatch.len as size_t > sufficient_len
                                           ||
                                           newMatch.len + cur >=
                                               (1 as libc::c_int) <<
                                                   12 as libc::c_int {
                                        /* immediate encoding */
                                        best_mlen = newMatch.len;
                                        best_off = newMatch.off;
                                        last_match_pos =
                                            cur + 1 as libc::c_int;
                                        current_block = 11611820308269048410;
                                        break ;
                                    } else {
                                        /* before match : set price with literals at beginning */
                                        let baseLitlen: libc::c_int =
                                            (*opt.offset(cur as
                                                             isize)).litlen; /* literal */
                                        let mut litlen: libc::c_int = 0;
                                        litlen = 1 as libc::c_int;
                                        while litlen < 4 as libc::c_int {
                                            let price: libc::c_int =
                                                (*opt.offset(cur as
                                                                 isize)).price
                                                    -
                                                    LZ4HC_literalsPrice(baseLitlen)
                                                    +
                                                    LZ4HC_literalsPrice(baseLitlen
                                                                            +
                                                                            litlen);
                                            let pos: libc::c_int =
                                                cur + litlen;
                                            if price <
                                                   (*opt.offset(pos as
                                                                    isize)).price
                                               {
                                                (*opt.offset(pos as
                                                                 isize)).mlen
                                                    = 1 as libc::c_int;
                                                (*opt.offset(pos as
                                                                 isize)).off =
                                                    0 as libc::c_int;
                                                (*opt.offset(pos as
                                                                 isize)).litlen
                                                    = baseLitlen + litlen;
                                                (*opt.offset(pos as
                                                                 isize)).price
                                                    = price
                                            }
                                            litlen += 1
                                        }
                                        /* set prices using match at position = cur */
                                        let matchML_0: libc::c_int =
                                            newMatch.len;
                                        let mut ml: libc::c_int =
                                            4 as libc::c_int;
                                        while ml <= matchML_0 {
                                            let pos_0: libc::c_int = cur + ml;
                                            let offset_0: libc::c_int =
                                                newMatch.off;
                                            let mut price_0: libc::c_int = 0;
                                            let mut ll: libc::c_int = 0;
                                            if (*opt.offset(cur as
                                                                isize)).mlen
                                                   == 1 as libc::c_int {
                                                ll =
                                                    (*opt.offset(cur as
                                                                     isize)).litlen;
                                                price_0 =
                                                    (if cur > ll {
                                                         (*opt.offset((cur -
                                                                           ll)
                                                                          as
                                                                          isize)).price
                                                     } else {
                                                         0 as libc::c_int
                                                     }) +
                                                        LZ4HC_sequencePrice(ll,
                                                                            ml)
                                            } else {
                                                ll = 0 as libc::c_int;
                                                price_0 =
                                                    (*opt.offset(cur as
                                                                     isize)).price
                                                        +
                                                        LZ4HC_sequencePrice(0
                                                                                as
                                                                                libc::c_int,
                                                                            ml)
                                            }
                                            if pos_0 >
                                                   last_match_pos +
                                                       3 as libc::c_int ||
                                                   price_0 <=
                                                       (*opt.offset(pos_0 as
                                                                        isize)).price
                                                           -
                                                           favorDecSpeed as
                                                               libc::c_int {
                                                if ml == matchML_0 &&
                                                       last_match_pos < pos_0
                                                   {
                                                    last_match_pos = pos_0
                                                }
                                                (*opt.offset(pos_0 as
                                                                 isize)).mlen
                                                    = ml;
                                                (*opt.offset(pos_0 as
                                                                 isize)).off =
                                                    offset_0;
                                                (*opt.offset(pos_0 as
                                                                 isize)).litlen
                                                    = ll;
                                                (*opt.offset(pos_0 as
                                                                 isize)).price
                                                    = price_0
                                            }
                                            ml += 1
                                        }
                                        /* complete following positions with literals */
                                        let mut addLit_0: libc::c_int =
                                            0; /* literal */
                                        addLit_0 = 1 as libc::c_int;
                                        while addLit_0 <= 3 as libc::c_int {
                                            (*opt.offset((last_match_pos +
                                                              addLit_0) as
                                                             isize)).mlen =
                                                1 as libc::c_int;
                                            (*opt.offset((last_match_pos +
                                                              addLit_0) as
                                                             isize)).off =
                                                0 as libc::c_int;
                                            (*opt.offset((last_match_pos +
                                                              addLit_0) as
                                                             isize)).litlen =
                                                addLit_0;
                                            (*opt.offset((last_match_pos +
                                                              addLit_0) as
                                                             isize)).price =
                                                (*opt.offset(last_match_pos as
                                                                 isize)).price
                                                    +
                                                    LZ4HC_literalsPrice(addLit_0);
                                            addLit_0 += 1
                                        }
                                    }
                                }
                            }
                            _ => { }
                        }
                        cur += 1
                    }
                    match current_block {
                        5590933039760577279 => {
                            best_mlen =
                                (*opt.offset(last_match_pos as isize)).mlen;
                            best_off =
                                (*opt.offset(last_match_pos as isize)).off;
                            cur = last_match_pos - best_mlen
                        }
                        _ => { }
                    }
                    /* cur, last_match_pos, best_mlen, best_off must be set */
                    /* == 1 when only one candidate */
                    let mut candidate_pos: libc::c_int = cur;
                    let mut selected_matchLength: libc::c_int = best_mlen;
                    let mut selected_offset: libc::c_int = best_off;
                    loop 
                         /* from end to beginning */
                         {
                        let next_matchLength: libc::c_int =
                            (*opt.offset(candidate_pos as
                                             isize)).mlen; /* can be 1, means literal */
                        let next_offset: libc::c_int =
                            (*opt.offset(candidate_pos as
                                             isize)).off; /* last match elected, first match to encode */
                        (*opt.offset(candidate_pos as isize)).mlen =
                            selected_matchLength;
                        (*opt.offset(candidate_pos as isize)).off =
                            selected_offset;
                        selected_matchLength = next_matchLength;
                        selected_offset = next_offset;
                        if next_matchLength > candidate_pos { break ; }
                        /* can be 1, means literal */
                        candidate_pos -= next_matchLength
                    }
                    /* encode all recorded sequences in order */
                    let mut rPos_0: libc::c_int =
                        0 as libc::c_int; /* relative position (to ip) */
                    while rPos_0 < last_match_pos {
                        let ml_0: libc::c_int =
                            (*opt.offset(rPos_0 as
                                             isize)).mlen; /* literal; note: can end up with several literals, in which case, skip them */
                        let offset_1: libc::c_int =
                            (*opt.offset(rPos_0 as isize)).off;
                        if ml_0 == 1 as libc::c_int {
                            ip = ip.offset(1);
                            rPos_0 += 1
                        } else {
                            rPos_0 += ml_0;
                            opSaved = op;
                            if LZ4HC_encodeSequence(&mut ip, &mut op,
                                                    &mut anchor, ml_0,
                                                    ip.offset(-(offset_1 as
                                                                    isize)),
                                                    limit, oend) != 0 {
                                current_block = 12983150111686922718;
                                break 's_73 ;
                            }
                        }
                    }
                }
            }
        match current_block {
            6535448574339961634 =>
            /* Encode Last Literals */
            {
                current_block = 12969817083969514432;
            }
            _ =>
            /* updates ip, op and anchor */
            {
                if limit as libc::c_uint ==
                       fillOutput as libc::c_int as libc::c_uint {
                    op = opSaved; /* restore correct out pointer */
                    current_block = 12969817083969514432; /* literals */
                } else {
                    current_block =
                        4283923591314305871; /* restore correct value */
                }
            }
        }
        match current_block {
            4283923591314305871 => { }
            _ => {
                let mut lastRunSize: size_t =
                    iend.wrapping_offset_from(anchor) as libc::c_long as
                        size_t;
                let mut litLength: size_t =
                    lastRunSize.wrapping_add(255 as libc::c_int as
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
                                                                                                                 libc::c_ulong);
                let totalSize: size_t =
                    (1 as libc::c_int as
                         libc::c_ulong).wrapping_add(litLength).wrapping_add(lastRunSize);
                if limit as libc::c_uint ==
                       fillOutput as libc::c_int as libc::c_uint {
                    oend = oend.offset(5 as libc::c_int as isize)
                }
                if limit as libc::c_uint != 0 &&
                       op.offset(totalSize as isize) > oend {
                    if limit as libc::c_uint ==
                           limitedOutput as libc::c_int as libc::c_uint {
                        /* Check output limit */
                        retval = 0 as libc::c_int;
                        current_block = 4283923591314305871;
                    } else {
                        /* adapt lastRunSize to fill 'dst' */
                        lastRunSize =
                            (oend.wrapping_offset_from(op) as libc::c_long as
                                 size_t).wrapping_sub(1 as libc::c_int as
                                                          libc::c_ulong);
                        litLength =
                            lastRunSize.wrapping_add(255 as libc::c_int as
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
                                                                                                                         libc::c_ulong);
                        lastRunSize =
                            (lastRunSize as
                                 libc::c_ulong).wrapping_sub(litLength) as
                                size_t as size_t;
                        current_block = 2408932541243239002;
                    }
                } else { current_block = 2408932541243239002; }
                match current_block {
                    4283923591314305871 => { }
                    _ => {
                        ip = anchor.offset(lastRunSize as isize);
                        if lastRunSize >=
                               ((1 as libc::c_uint) <<
                                    8 as libc::c_int -
                                        4 as
                                            libc::c_int).wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                   as libc::c_ulong {
                            let mut accumulator: size_t =
                                lastRunSize.wrapping_sub(((1 as libc::c_uint)
                                                              <<
                                                              8 as libc::c_int
                                                                  -
                                                                  4 as
                                                                      libc::c_int).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                             as
                                                             libc::c_ulong);
                            let fresh13 = op;
                            op = op.offset(1);
                            *fresh13 =
                                (((1 as libc::c_uint) <<
                                      8 as libc::c_int -
                                          4 as
                                              libc::c_int).wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                     << 4 as libc::c_int) as BYTE;
                            while accumulator >=
                                      255 as libc::c_int as libc::c_ulong {
                                let fresh14 = op;
                                op = op.offset(1);
                                *fresh14 = 255 as libc::c_int as BYTE;
                                accumulator =
                                    (accumulator as
                                         libc::c_ulong).wrapping_sub(255 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as size_t as size_t
                            }
                            let fresh15 = op;
                            op = op.offset(1);
                            *fresh15 = accumulator as BYTE
                        } else {
                            let fresh16 = op;
                            op = op.offset(1);
                            *fresh16 =
                                (lastRunSize << 4 as libc::c_int) as BYTE
                        }
                        memcpy(op as *mut libc::c_void,
                               anchor as *const libc::c_void, lastRunSize);
                        op = op.offset(lastRunSize as isize);
                        /* End */
                        *srcSizePtr =
                            (ip as
                                 *const libc::c_char).wrapping_offset_from(source)
                                as libc::c_long as libc::c_int;
                        retval =
                            (op as
                                 *mut libc::c_char).wrapping_offset_from(dst)
                                as libc::c_long as libc::c_int
                    }
                }
            }
        }
    }
    free(opt as *mut libc::c_void);
    return retval;
}
