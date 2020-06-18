use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type XXH32_hash_t = libc::c_uint;
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type XXH_alignment = libc::c_uint;
pub const XXH_unaligned: XXH_alignment = 1;
pub const XXH_aligned: XXH_alignment = 0;
pub type XXH_endianess = libc::c_uint;
pub const XXH_littleEndian: XXH_endianess = 1;
pub const XXH_bigEndian: XXH_endianess = 0;
/* assert */
/* *************************************
*  Compiler Specific Options
***************************************/
/* Visual Studio */
/* C99 */
/* __STDC_VERSION__ */
/* *************************************
*  Basic Types
***************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: U32,
    pub c: [BYTE; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_state_s {
    pub total_len_32: uint32_t,
    pub large_len: uint32_t,
    pub v1: uint32_t,
    pub v2: uint32_t,
    pub v3: uint32_t,
    pub v4: uint32_t,
    pub mem32: [uint32_t; 4],
    pub memsize: uint32_t,
    pub reserved: uint32_t,
}
pub type XXH32_state_t = XXH32_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_canonical_t {
    pub digest: [libc::c_uchar; 4],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const XXH_sa: C2RustUnnamed_0 = 1;
pub type XXH64_hash_t = libc::c_ulonglong;
/* *******************************************************************
*  64-bit hash functions
*********************************************************************/
/*======   Memory access   ======*/
/* C99 */
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: uint64_t,
    pub v1: uint64_t,
    pub v2: uint64_t,
    pub v3: uint64_t,
    pub v4: uint64_t,
    pub mem64: [uint64_t; 4],
    pub memsize: uint32_t,
    pub reserved: [uint32_t; 2],
}
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_canonical_t {
    pub digest: [libc::c_uchar; 8],
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const XXH_sa_0: C2RustUnnamed_1 = 1;
/* *************************************
*  Includes & Memory related functions
***************************************/
/* ! Modify the local functions below should you wish to use some other memory routines
*   for malloc(), free() */
unsafe extern "C" fn XXH_malloc(mut s: size_t) -> *mut libc::c_void {
    return malloc(s);
}
unsafe extern "C" fn XXH_free(mut p: *mut libc::c_void) { free(p); }
/* ! and for memcpy() */
unsafe extern "C" fn XXH_memcpy(mut dest: *mut libc::c_void,
                                mut src: *const libc::c_void,
                                mut size: size_t) -> *mut libc::c_void {
    return memcpy(dest, src, size);
}
/* portable and safe solution. Generally efficient.
 * see : http://stackoverflow.com/a/32095106/646947
 */
unsafe extern "C" fn XXH_read32(mut memPtr: *const libc::c_void) -> U32 {
    let mut val: U32 = 0;
    memcpy(&mut val as *mut U32 as *mut libc::c_void, memPtr,
           ::std::mem::size_of::<U32>() as libc::c_ulong);
    return val;
}
/* Visual Studio */
unsafe extern "C" fn XXH_swap32(mut x: U32) -> U32 {
    return x << 24 as libc::c_int & 0xff000000 as libc::c_uint |
               x << 8 as libc::c_int & 0xff0000 as libc::c_int as libc::c_uint
               | x >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_uint
               | x >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
}
/* XXH_CPU_LITTLE_ENDIAN can be defined externally, for example on the compiler command line */
unsafe extern "C" fn XXH_isLittleEndian() -> libc::c_int {
    let one: C2RustUnnamed =
        C2RustUnnamed{u:
                          1 as libc::c_int as
                              U32,}; /* don't use static : performance detrimental  */
    return one.c[0 as libc::c_int as usize] as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE32_align(mut ptr: *const libc::c_void,
                                        mut endian: XXH_endianess,
                                        mut align: XXH_alignment) -> U32 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return if endian as libc::c_uint ==
                      XXH_littleEndian as libc::c_int as libc::c_uint {
                   XXH_read32(ptr)
               } else { XXH_swap32(XXH_read32(ptr)) }
    } else {
        return if endian as libc::c_uint ==
                      XXH_littleEndian as libc::c_int as libc::c_uint {
                   *(ptr as *const U32)
               } else { XXH_swap32(*(ptr as *const U32)) }
    };
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE32(mut ptr: *const libc::c_void,
                                  mut endian: XXH_endianess) -> U32 {
    return XXH_readLE32_align(ptr, endian, XXH_unaligned);
}
unsafe extern "C" fn XXH_readBE32(mut ptr: *const libc::c_void) -> U32 {
    return if XXH_isLittleEndian() != 0 {
               XXH_swap32(XXH_read32(ptr))
           } else { XXH_read32(ptr) };
}
/* *************************************
*  Macros
***************************************/
/* use after variable declarations */
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH_versionNumber() -> libc::c_uint {
    return (0 as libc::c_int * 100 as libc::c_int * 100 as libc::c_int +
                6 as libc::c_int * 100 as libc::c_int + 5 as libc::c_int) as
               libc::c_uint;
}
/* *******************************************************************
*  32-bit hash functions
*********************************************************************/
static mut PRIME32_1: U32 = 2654435761 as libc::c_uint;
static mut PRIME32_2: U32 = 2246822519 as libc::c_uint;
static mut PRIME32_3: U32 = 3266489917 as libc::c_uint;
static mut PRIME32_4: U32 = 668265263 as libc::c_uint;
static mut PRIME32_5: U32 = 374761393 as libc::c_uint;
unsafe extern "C" fn XXH32_round(mut seed: U32, mut input: U32) -> U32 {
    seed =
        (seed as libc::c_uint).wrapping_add(input.wrapping_mul(PRIME32_2)) as
            U32 as U32;
    seed =
        seed << 13 as libc::c_int |
            seed >> 32 as libc::c_int - 13 as libc::c_int;
    seed = (seed as libc::c_uint).wrapping_mul(PRIME32_1) as U32 as U32;
    return seed;
}
/* mix all bits */
unsafe extern "C" fn XXH32_avalanche(mut h32: U32) -> U32 {
    h32 ^= h32 >> 15 as libc::c_int;
    h32 = (h32 as libc::c_uint).wrapping_mul(PRIME32_2) as U32 as U32;
    h32 ^= h32 >> 13 as libc::c_int;
    h32 = (h32 as libc::c_uint).wrapping_mul(PRIME32_3) as U32 as U32;
    h32 ^= h32 >> 16 as libc::c_int;
    return h32;
}
unsafe extern "C" fn XXH32_finalize(mut h32: U32,
                                    mut ptr: *const libc::c_void,
                                    mut len: size_t,
                                    mut endian: XXH_endianess,
                                    mut align: XXH_alignment) -> U32 {
    let mut p: *const BYTE = ptr as *const BYTE;
    's_317:
        {
            let mut current_block_69: u64;
            match len & 15 as libc::c_int as libc::c_ulong {
                12 => {
                    /* or switch(bEnd - p) */
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 15369305621768284676;
                }
                8 => { current_block_69 = 15369305621768284676; }
                4 => { current_block_69 = 12738366957046377633; }
                13 => {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 13595044055729136238;
                }
                9 => { current_block_69 = 13595044055729136238; }
                5 => { current_block_69 = 5325356632163684852; }
                14 => {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 7301598146009646751;
                }
                10 => { current_block_69 = 7301598146009646751; }
                6 => { current_block_69 = 13335607515961111997; }
                15 => {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 7048354559524316116;
                }
                11 => { current_block_69 = 7048354559524316116; }
                7 => { current_block_69 = 10281330391366640094; }
                3 => { current_block_69 = 13075855104922526767; }
                2 => { current_block_69 = 11681054742646472013; }
                1 => { current_block_69 = 9989543335252588407; }
                0 => { current_block_69 = 10618910752773269518; }
                _ => { break 's_317 ; }
            }
            match current_block_69 {
                7048354559524316116 =>
                /* fallthrough */
                {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 10281330391366640094;
                }
                7301598146009646751 =>
                /* fallthrough */
                {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 13335607515961111997;
                }
                13595044055729136238 =>
                /* fallthrough */
                {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 5325356632163684852;
                }
                15369305621768284676 =>
                /* fallthrough */
                {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 12738366957046377633;
                }
                _ => { }
            }
            match current_block_69 {
                10281330391366640094 =>
                /* fallthrough */
                {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    current_block_69 = 13075855104922526767;
                }
                13335607515961111997 =>
                /* fallthrough */
                {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    let fresh1 = p;
                    p = p.offset(1);
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add((*fresh1 as
                                                             libc::c_uint).wrapping_mul(PRIME32_5))
                            as U32 as U32;
                    h32 =
                        (h32 << 11 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME32_1);
                    let fresh2 = p;
                    p = p.offset(1);
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add((*fresh2 as
                                                             libc::c_uint).wrapping_mul(PRIME32_5))
                            as U32 as U32;
                    h32 =
                        (h32 << 11 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME32_1);
                    return XXH32_avalanche(h32)
                }
                5325356632163684852 =>
                /* fallthrough */
                {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    let fresh0 = p;
                    p = p.offset(1);
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add((*fresh0 as
                                                             libc::c_uint).wrapping_mul(PRIME32_5))
                            as U32 as U32;
                    h32 =
                        (h32 << 11 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME32_1);
                    return XXH32_avalanche(h32)
                }
                12738366957046377633 =>
                /* fallthrough */
                {
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add(XXH_readLE32_align(p
                                                                               as
                                                                               *const libc::c_void,
                                                                           endian,
                                                                           align).wrapping_mul(PRIME32_3))
                            as U32 as U32;
                    p = p.offset(4 as libc::c_int as isize);
                    h32 =
                        (h32 << 17 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     17 as
                                         libc::c_int).wrapping_mul(PRIME32_4);
                    return XXH32_avalanche(h32)
                }
                _ => { }
            }
            match current_block_69 {
                13075855104922526767 =>
                /* fallthrough */
                {
                    let fresh3 = p;
                    p = p.offset(1);
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add((*fresh3 as
                                                             libc::c_uint).wrapping_mul(PRIME32_5))
                            as U32 as U32;
                    h32 =
                        (h32 << 11 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME32_1);
                    current_block_69 = 11681054742646472013;
                }
                _ => { }
            }
            match current_block_69 {
                11681054742646472013 =>
                /* fallthrough */
                {
                    let fresh4 = p;
                    p = p.offset(1);
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add((*fresh4 as
                                                             libc::c_uint).wrapping_mul(PRIME32_5))
                            as U32 as U32;
                    h32 =
                        (h32 << 11 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME32_1);
                    current_block_69 = 9989543335252588407;
                }
                _ => { }
            }
            match current_block_69 {
                9989543335252588407 =>
                /* fallthrough */
                {
                    let fresh5 = p;
                    p = p.offset(1);
                    h32 =
                        (h32 as
                             libc::c_uint).wrapping_add((*fresh5 as
                                                             libc::c_uint).wrapping_mul(PRIME32_5))
                            as U32 as U32;
                    h32 =
                        (h32 << 11 as libc::c_int |
                             h32 >>
                                 32 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME32_1)
                }
                _ => { }
            }
            /* fallthrough */
            return XXH32_avalanche(h32)
        }
    __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                  b"xxhash.c\x00" as *const u8 as *const libc::c_char,
                  346 as libc::c_int as libc::c_uint,
                  (*::std::mem::transmute::<&[u8; 76],
                                            &[libc::c_char; 76]>(b"U32 XXH32_finalize(U32, const void *, size_t, XXH_endianess, XXH_alignment)\x00")).as_ptr());
    return h32;
    /* reaching this point is deemed impossible */
}
#[inline(always)]
unsafe extern "C" fn XXH32_endian_align(mut input: *const libc::c_void,
                                        mut len: size_t, mut seed: U32,
                                        mut endian: XXH_endianess,
                                        mut align: XXH_alignment) -> U32 {
    let mut p: *const BYTE = input as *const BYTE;
    let mut bEnd: *const BYTE = p.offset(len as isize);
    let mut h32: U32 = 0;
    if len >= 16 as libc::c_int as libc::c_ulong {
        let limit: *const BYTE = bEnd.offset(-(15 as libc::c_int as isize));
        let mut v1: U32 =
            seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        let mut v2: U32 = seed.wrapping_add(PRIME32_2);
        let mut v3: U32 = seed.wrapping_add(0 as libc::c_int as libc::c_uint);
        let mut v4: U32 = seed.wrapping_sub(PRIME32_1);
        loop  {
            v1 =
                XXH32_round(v1,
                            XXH_readLE32_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(4 as libc::c_int as isize);
            v2 =
                XXH32_round(v2,
                            XXH_readLE32_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(4 as libc::c_int as isize);
            v3 =
                XXH32_round(v3,
                            XXH_readLE32_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(4 as libc::c_int as isize);
            v4 =
                XXH32_round(v4,
                            XXH_readLE32_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(4 as libc::c_int as isize);
            if !(p < limit) { break ; }
        }
        h32 =
            (v1 << 1 as libc::c_int |
                 v1 >>
                     32 as libc::c_int -
                         1 as
                             libc::c_int).wrapping_add(v2 << 7 as libc::c_int
                                                           |
                                                           v2 >>
                                                               32 as
                                                                   libc::c_int
                                                                   -
                                                                   7 as
                                                                       libc::c_int).wrapping_add(v3
                                                                                                     <<
                                                                                                     12
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                     |
                                                                                                     v3
                                                                                                         >>
                                                                                                         32
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             -
                                                                                                             12
                                                                                                                 as
                                                                                                                 libc::c_int).wrapping_add(v4
                                                                                                                                               <<
                                                                                                                                               18
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                               |
                                                                                                                                               v4
                                                                                                                                                   >>
                                                                                                                                                   32
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       -
                                                                                                                                                       18
                                                                                                                                                           as
                                                                                                                                                           libc::c_int)
    } else { h32 = seed.wrapping_add(PRIME32_5) }
    h32 = (h32 as libc::c_uint).wrapping_add(len as U32) as U32 as U32;
    return XXH32_finalize(h32, p as *const libc::c_void,
                          len & 15 as libc::c_int as libc::c_ulong, endian,
                          align);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32(mut input: *const libc::c_void,
                                   mut len: size_t, mut seed: libc::c_uint)
 -> XXH32_hash_t {
    let mut endian_detected: XXH_endianess =
        XXH_isLittleEndian() as XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint ||
           0 as libc::c_int != 0 {
        return XXH32_endian_align(input, len, seed, XXH_littleEndian,
                                  XXH_unaligned)
    } else {
        return XXH32_endian_align(input, len, seed, XXH_bigEndian,
                                  XXH_unaligned)
    };
}
/*======   Hash streaming   ======*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32_createState() -> *mut XXH32_state_t {
    return XXH_malloc(::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong)
               as
               *mut XXH32_state_t; /* using a local state to memcpy() in order to avoid strict-aliasing warnings */
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32_freeState(mut statePtr: *mut XXH32_state_t)
 -> XXH_errorcode {
    XXH_free(statePtr as *mut libc::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32_copyState(mut dstState: *mut XXH32_state_t,
                                             mut srcState:
                                                 *const XXH32_state_t) {
    memcpy(dstState as *mut libc::c_void, srcState as *const libc::c_void,
           ::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32_reset(mut statePtr: *mut XXH32_state_t,
                                         mut seed: libc::c_uint)
 -> XXH_errorcode {
    let mut state: XXH32_state_t =
        XXH32_state_t{total_len_32: 0,
                      large_len: 0,
                      v1: 0,
                      v2: 0,
                      v3: 0,
                      v4: 0,
                      mem32: [0; 4],
                      memsize: 0,
                      reserved: 0,};
    memset(&mut state as *mut XXH32_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<XXH32_state_t>() as libc::c_ulong);
    state.v1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
    state.v2 = seed.wrapping_add(PRIME32_2);
    state.v3 = seed.wrapping_add(0 as libc::c_int as libc::c_uint);
    state.v4 = seed.wrapping_sub(PRIME32_1);
    /* do not write into reserved, planned to be removed in a future version */
    memcpy(statePtr as *mut libc::c_void,
           &mut state as *mut XXH32_state_t as *const libc::c_void,
           (::std::mem::size_of::<XXH32_state_t>() as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<uint32_t>()
                                                as libc::c_ulong));
    return XXH_OK;
}
#[inline(always)]
unsafe extern "C" fn XXH32_update_endian(mut state: *mut XXH32_state_t,
                                         mut input: *const libc::c_void,
                                         mut len: size_t,
                                         mut endian: XXH_endianess)
 -> XXH_errorcode {
    if input.is_null() { return XXH_ERROR }
    let mut p: *const BYTE = input as *const BYTE;
    let bEnd: *const BYTE = p.offset(len as isize);
    (*state).total_len_32 =
        ((*state).total_len_32 as
             libc::c_uint).wrapping_add(len as libc::c_uint) as uint32_t as
            uint32_t;
    (*state).large_len |=
        ((len >= 16 as libc::c_int as libc::c_ulong) as libc::c_int |
             ((*state).total_len_32 >= 16 as libc::c_int as libc::c_uint) as
                 libc::c_int) as libc::c_uint;
    if ((*state).memsize as libc::c_ulong).wrapping_add(len) <
           16 as libc::c_int as libc::c_ulong {
        /* fill in tmp buffer */
        XXH_memcpy(((*state).mem32.as_mut_ptr() as
                        *mut BYTE).offset((*state).memsize as isize) as
                       *mut libc::c_void, input, len);
        (*state).memsize =
            ((*state).memsize as
                 libc::c_uint).wrapping_add(len as libc::c_uint) as uint32_t
                as uint32_t;
        return XXH_OK
    }
    if (*state).memsize != 0 {
        /* some data left from previous update */
        XXH_memcpy(((*state).mem32.as_mut_ptr() as
                        *mut BYTE).offset((*state).memsize as isize) as
                       *mut libc::c_void, input,
                   (16 as libc::c_int as
                        libc::c_uint).wrapping_sub((*state).memsize) as
                       size_t);
        let mut p32: *const U32 = (*state).mem32.as_mut_ptr();
        (*state).v1 =
            XXH32_round((*state).v1,
                        XXH_readLE32(p32 as *const libc::c_void, endian));
        p32 = p32.offset(1);
        (*state).v2 =
            XXH32_round((*state).v2,
                        XXH_readLE32(p32 as *const libc::c_void, endian));
        p32 = p32.offset(1);
        (*state).v3 =
            XXH32_round((*state).v3,
                        XXH_readLE32(p32 as *const libc::c_void, endian));
        p32 = p32.offset(1);
        (*state).v4 =
            XXH32_round((*state).v4,
                        XXH_readLE32(p32 as *const libc::c_void, endian));
        p =
            p.offset((16 as libc::c_int as
                          libc::c_uint).wrapping_sub((*state).memsize) as
                         isize);
        (*state).memsize = 0 as libc::c_int as uint32_t
    }
    if p <= bEnd.offset(-(16 as libc::c_int as isize)) {
        let limit: *const BYTE = bEnd.offset(-(16 as libc::c_int as isize));
        let mut v1: U32 = (*state).v1;
        let mut v2: U32 = (*state).v2;
        let mut v3: U32 = (*state).v3;
        let mut v4: U32 = (*state).v4;
        loop  {
            v1 =
                XXH32_round(v1,
                            XXH_readLE32(p as *const libc::c_void, endian));
            p = p.offset(4 as libc::c_int as isize);
            v2 =
                XXH32_round(v2,
                            XXH_readLE32(p as *const libc::c_void, endian));
            p = p.offset(4 as libc::c_int as isize);
            v3 =
                XXH32_round(v3,
                            XXH_readLE32(p as *const libc::c_void, endian));
            p = p.offset(4 as libc::c_int as isize);
            v4 =
                XXH32_round(v4,
                            XXH_readLE32(p as *const libc::c_void, endian));
            p = p.offset(4 as libc::c_int as isize);
            if !(p <= limit) { break ; }
        }
        (*state).v1 = v1;
        (*state).v2 = v2;
        (*state).v3 = v3;
        (*state).v4 = v4
    }
    if p < bEnd {
        XXH_memcpy((*state).mem32.as_mut_ptr() as *mut libc::c_void,
                   p as *const libc::c_void,
                   bEnd.wrapping_offset_from(p) as libc::c_long as size_t);
        (*state).memsize =
            bEnd.wrapping_offset_from(p) as libc::c_long as libc::c_uint
    }
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32_update(mut state_in: *mut XXH32_state_t,
                                          mut input: *const libc::c_void,
                                          mut len: size_t) -> XXH_errorcode {
    let mut endian_detected: XXH_endianess =
        XXH_isLittleEndian() as XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint ||
           0 as libc::c_int != 0 {
        return XXH32_update_endian(state_in, input, len, XXH_littleEndian)
    } else {
        return XXH32_update_endian(state_in, input, len, XXH_bigEndian)
    };
}
#[inline(always)]
unsafe extern "C" fn XXH32_digest_endian(mut state: *const XXH32_state_t,
                                         mut endian: XXH_endianess) -> U32 {
    let mut h32: U32 = 0;
    if (*state).large_len != 0 {
        h32 =
            ((*state).v1 << 1 as libc::c_int |
                 (*state).v1 >>
                     32 as libc::c_int -
                         1 as
                             libc::c_int).wrapping_add((*state).v2 <<
                                                           7 as libc::c_int |
                                                           (*state).v2 >>
                                                               32 as
                                                                   libc::c_int
                                                                   -
                                                                   7 as
                                                                       libc::c_int).wrapping_add((*state).v3
                                                                                                     <<
                                                                                                     12
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                     |
                                                                                                     (*state).v3
                                                                                                         >>
                                                                                                         32
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             -
                                                                                                             12
                                                                                                                 as
                                                                                                                 libc::c_int).wrapping_add((*state).v4
                                                                                                                                               <<
                                                                                                                                               18
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                               |
                                                                                                                                               (*state).v4
                                                                                                                                                   >>
                                                                                                                                                   32
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       -
                                                                                                                                                       18
                                                                                                                                                           as
                                                                                                                                                           libc::c_int)
    } else { h32 = (*state).v3.wrapping_add(PRIME32_5) }
    h32 =
        (h32 as libc::c_uint).wrapping_add((*state).total_len_32) as U32 as
            U32;
    return XXH32_finalize(h32, (*state).mem32.as_ptr() as *const libc::c_void,
                          (*state).memsize as size_t, endian, XXH_aligned);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32_digest(mut state_in: *const XXH32_state_t)
 -> XXH32_hash_t {
    let mut endian_detected: XXH_endianess =
        XXH_isLittleEndian() as XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint ||
           0 as libc::c_int != 0 {
        return XXH32_digest_endian(state_in, XXH_littleEndian)
    } else { return XXH32_digest_endian(state_in, XXH_bigEndian) };
}
/*======   Canonical representation   ======*/
/* ! Default XXH result types are basic unsigned 32 and 64 bits.
*   The canonical representation follows human-readable write convention, aka big-endian (large digits first).
*   These functions allow transformation of hash result into and from its canonical format.
*   This way, hash values can be written into a file or buffer, remaining comparable across different systems.
*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32_canonicalFromHash(mut dst:
                                                         *mut XXH32_canonical_t,
                                                     mut hash: XXH32_hash_t) {
    if XXH_isLittleEndian() != 0 { hash = XXH_swap32(hash) }
    memcpy(dst as *mut libc::c_void,
           &mut hash as *mut XXH32_hash_t as *const libc::c_void,
           ::std::mem::size_of::<XXH32_canonical_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH32_hashFromCanonical(mut src:
                                                         *const XXH32_canonical_t)
 -> XXH32_hash_t {
    return XXH_readBE32(src as *const libc::c_void);
}
/* portable and safe solution. Generally efficient.
 * see : http://stackoverflow.com/a/32095106/646947
 */
unsafe extern "C" fn XXH_read64(mut memPtr: *const libc::c_void) -> U64 {
    let mut val: U64 = 0;
    memcpy(&mut val as *mut U64 as *mut libc::c_void, memPtr,
           ::std::mem::size_of::<U64>() as libc::c_ulong);
    return val;
}
/* XXH_FORCE_DIRECT_MEMORY_ACCESS */
/* Visual Studio */
unsafe extern "C" fn XXH_swap64(mut x: U64) -> U64 {
    return ((x << 56 as libc::c_int) as libc::c_ulonglong &
                0xff00000000000000 as libc::c_ulonglong |
                (x << 40 as libc::c_int) as libc::c_ulonglong &
                    0xff000000000000 as libc::c_ulonglong |
                (x << 24 as libc::c_int) as libc::c_ulonglong &
                    0xff0000000000 as libc::c_ulonglong |
                (x << 8 as libc::c_int) as libc::c_ulonglong &
                    0xff00000000 as libc::c_ulonglong |
                (x >> 8 as libc::c_int) as libc::c_ulonglong &
                    0xff000000 as libc::c_ulonglong |
                (x >> 24 as libc::c_int) as libc::c_ulonglong &
                    0xff0000 as libc::c_ulonglong |
                (x >> 40 as libc::c_int) as libc::c_ulonglong &
                    0xff00 as libc::c_ulonglong |
                (x >> 56 as libc::c_int) as libc::c_ulonglong &
                    0xff as libc::c_ulonglong) as U64;
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE64_align(mut ptr: *const libc::c_void,
                                        mut endian: XXH_endianess,
                                        mut align: XXH_alignment) -> U64 {
    if align as libc::c_uint == XXH_unaligned as libc::c_int as libc::c_uint {
        return if endian as libc::c_uint ==
                      XXH_littleEndian as libc::c_int as libc::c_uint {
                   XXH_read64(ptr)
               } else { XXH_swap64(XXH_read64(ptr)) }
    } else {
        return if endian as libc::c_uint ==
                      XXH_littleEndian as libc::c_int as libc::c_uint {
                   *(ptr as *const U64)
               } else { XXH_swap64(*(ptr as *const U64)) }
    };
}
#[inline(always)]
unsafe extern "C" fn XXH_readLE64(mut ptr: *const libc::c_void,
                                  mut endian: XXH_endianess) -> U64 {
    return XXH_readLE64_align(ptr, endian, XXH_unaligned);
}
unsafe extern "C" fn XXH_readBE64(mut ptr: *const libc::c_void) -> U64 {
    return if XXH_isLittleEndian() != 0 {
               XXH_swap64(XXH_read64(ptr))
           } else { XXH_read64(ptr) };
}
/*======   xxh64   ======*/
static mut PRIME64_1: U64 = 11400714785074694791 as libc::c_ulonglong as U64;
static mut PRIME64_2: U64 = 14029467366897019727 as libc::c_ulonglong as U64;
static mut PRIME64_3: U64 = 1609587929392839161 as libc::c_ulonglong as U64;
static mut PRIME64_4: U64 = 9650029242287828579 as libc::c_ulonglong as U64;
static mut PRIME64_5: U64 = 2870177450012600261 as libc::c_ulonglong as U64;
unsafe extern "C" fn XXH64_round(mut acc: U64, mut input: U64) -> U64 {
    acc =
        (acc as libc::c_ulong).wrapping_add(input.wrapping_mul(PRIME64_2)) as
            U64 as U64;
    acc =
        acc << 31 as libc::c_int |
            acc >> 64 as libc::c_int - 31 as libc::c_int;
    acc = (acc as libc::c_ulong).wrapping_mul(PRIME64_1) as U64 as U64;
    return acc;
}
unsafe extern "C" fn XXH64_mergeRound(mut acc: U64, mut val: U64) -> U64 {
    val = XXH64_round(0 as libc::c_int as U64, val);
    acc ^= val;
    acc = acc.wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
    return acc;
}
unsafe extern "C" fn XXH64_avalanche(mut h64: U64) -> U64 {
    h64 ^= h64 >> 33 as libc::c_int;
    h64 = (h64 as libc::c_ulong).wrapping_mul(PRIME64_2) as U64 as U64;
    h64 ^= h64 >> 29 as libc::c_int;
    h64 = (h64 as libc::c_ulong).wrapping_mul(PRIME64_3) as U64 as U64;
    h64 ^= h64 >> 32 as libc::c_int;
    return h64;
}
unsafe extern "C" fn XXH64_finalize(mut h64: U64,
                                    mut ptr: *const libc::c_void,
                                    mut len: size_t,
                                    mut endian: XXH_endianess,
                                    mut align: XXH_alignment) -> U64 {
    let mut p: *const BYTE = ptr as *const BYTE;
    's_881:
        {
            let mut current_block_179: u64;
            match len & 31 as libc::c_int as libc::c_ulong {
                24 => {
                    let k1: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 15081740356361240479;
                }
                16 => { current_block_179 = 15081740356361240479; }
                8 => { current_block_179 = 4510563271777997454; }
                28 => {
                    let k1_2: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_2;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 2241680329088345986;
                }
                20 => { current_block_179 = 2241680329088345986; }
                12 => { current_block_179 = 10983261971688379477; }
                4 => { current_block_179 = 17789099964501628722; }
                25 => {
                    let k1_5: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_5;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 10489131089047693169;
                }
                17 => { current_block_179 = 10489131089047693169; }
                9 => { current_block_179 = 17109525036494554974; }
                29 => {
                    let k1_8: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_8;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 15406670416841790907;
                }
                21 => { current_block_179 = 15406670416841790907; }
                13 => { current_block_179 = 13935456465286830489; }
                5 => { current_block_179 = 12275382707489007217; }
                26 => {
                    let k1_11: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_11;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 6963491405217644053;
                }
                18 => { current_block_179 = 6963491405217644053; }
                10 => { current_block_179 = 2185850862157077482; }
                30 => {
                    let k1_14: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_14;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 12204687857703593294;
                }
                22 => { current_block_179 = 12204687857703593294; }
                14 => { current_block_179 = 3423204040662899213; }
                6 => { current_block_179 = 4159032308423528815; }
                27 => {
                    let k1_17: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_17;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 6882647720645449468;
                }
                19 => { current_block_179 = 6882647720645449468; }
                11 => { current_block_179 = 5440806569651843976; }
                31 => {
                    let k1_20: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_20;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 3895720959190088603;
                }
                23 => { current_block_179 = 3895720959190088603; }
                15 => { current_block_179 = 14765789812378875900; }
                7 => { current_block_179 = 16702638104816664719; }
                3 => { current_block_179 = 16308235248633175028; }
                2 => { current_block_179 = 17775571965565245011; }
                1 => { current_block_179 = 17440742207893796336; }
                0 => { current_block_179 = 12147583393054401687; }
                _ => { break 's_881 ; }
            }
            match current_block_179 {
                15081740356361240479 =>
                /* fallthrough */
                {
                    let k1_0: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_0;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 4510563271777997454;
                }
                2241680329088345986 =>
                /* fallthrough */
                {
                    let k1_3: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_3;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 10983261971688379477;
                }
                10489131089047693169 =>
                /* fallthrough */
                {
                    let k1_6: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_6;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 17109525036494554974;
                }
                15406670416841790907 =>
                /* fallthrough */
                {
                    let k1_9: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_9;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 13935456465286830489;
                }
                6963491405217644053 =>
                /* fallthrough */
                {
                    let k1_12: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_12;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 2185850862157077482;
                }
                12204687857703593294 =>
                /* fallthrough */
                {
                    let k1_15: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_15;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 3423204040662899213;
                }
                6882647720645449468 =>
                /* fallthrough */
                {
                    let k1_18: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_18;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 5440806569651843976;
                }
                3895720959190088603 =>
                /* fallthrough */
                {
                    let k1_21: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_21;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 14765789812378875900;
                }
                _ => { }
            }
            match current_block_179 {
                5440806569651843976 =>
                /* fallthrough */
                {
                    let k1_19: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_19;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    let fresh12 = p;
                    p = p.offset(1);
                    h64 ^=
                        (*fresh12 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    let fresh13 = p;
                    p = p.offset(1);
                    h64 ^=
                        (*fresh13 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    let fresh14 = p;
                    p = p.offset(1);
                    h64 ^=
                        (*fresh14 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    return XXH64_avalanche(h64)
                }
                2185850862157077482 =>
                /* fallthrough */
                {
                    let k1_13: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_13;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    let fresh8 = p;
                    p = p.offset(1);
                    h64 ^= (*fresh8 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    let fresh9 = p;
                    p = p.offset(1);
                    h64 ^= (*fresh9 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    return XXH64_avalanche(h64)
                }
                17109525036494554974 =>
                /* fallthrough */
                {
                    let k1_7: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_7;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    let fresh6 = p;
                    p = p.offset(1);
                    h64 ^= (*fresh6 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    return XXH64_avalanche(h64)
                }
                4510563271777997454 =>
                /* fallthrough */
                {
                    let k1_1: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_1;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    return XXH64_avalanche(h64)
                }
                10983261971688379477 =>
                /* fallthrough */
                {
                    let k1_4: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_4;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 17789099964501628722;
                }
                13935456465286830489 =>
                /* fallthrough */
                {
                    let k1_10: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_10;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 12275382707489007217;
                }
                3423204040662899213 =>
                /* fallthrough */
                {
                    let k1_16: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_16;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 4159032308423528815;
                }
                14765789812378875900 =>
                /* fallthrough */
                {
                    let k1_22: U64 =
                        XXH64_round(0 as libc::c_int as U64,
                                    XXH_readLE64_align(p as
                                                           *const libc::c_void,
                                                       endian, align));
                    p = p.offset(8 as libc::c_int as isize);
                    h64 ^= k1_22;
                    h64 =
                        (h64 << 27 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     27 as
                                         libc::c_int).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
                    current_block_179 = 16702638104816664719;
                }
                _ => { }
            }
            match current_block_179 {
                16702638104816664719 =>
                /* fallthrough */
                {
                    h64 ^=
                        (XXH_readLE32_align(p as *const libc::c_void, endian,
                                            align) as
                             U64).wrapping_mul(PRIME64_1);
                    p = p.offset(4 as libc::c_int as isize);
                    h64 =
                        (h64 << 23 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     23 as
                                         libc::c_int).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
                    current_block_179 = 16308235248633175028;
                }
                4159032308423528815 =>
                /* fallthrough */
                {
                    h64 ^=
                        (XXH_readLE32_align(p as *const libc::c_void, endian,
                                            align) as
                             U64).wrapping_mul(PRIME64_1);
                    p = p.offset(4 as libc::c_int as isize);
                    h64 =
                        (h64 << 23 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     23 as
                                         libc::c_int).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
                    let fresh10 = p;
                    p = p.offset(1);
                    h64 ^=
                        (*fresh10 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    let fresh11 = p;
                    p = p.offset(1);
                    h64 ^=
                        (*fresh11 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    return XXH64_avalanche(h64)
                }
                12275382707489007217 =>
                /* fallthrough */
                {
                    h64 ^=
                        (XXH_readLE32_align(p as *const libc::c_void, endian,
                                            align) as
                             U64).wrapping_mul(PRIME64_1);
                    p = p.offset(4 as libc::c_int as isize);
                    h64 =
                        (h64 << 23 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     23 as
                                         libc::c_int).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
                    let fresh7 = p;
                    p = p.offset(1);
                    h64 ^= (*fresh7 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    return XXH64_avalanche(h64)
                }
                17789099964501628722 =>
                /* fallthrough */
                {
                    h64 ^=
                        (XXH_readLE32_align(p as *const libc::c_void, endian,
                                            align) as
                             U64).wrapping_mul(PRIME64_1);
                    p = p.offset(4 as libc::c_int as isize);
                    h64 =
                        (h64 << 23 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     23 as
                                         libc::c_int).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
                    return XXH64_avalanche(h64)
                }
                _ => { }
            }
            match current_block_179 {
                16308235248633175028 =>
                /* fallthrough */
                {
                    let fresh15 = p;
                    p = p.offset(1);
                    h64 ^=
                        (*fresh15 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    current_block_179 = 17775571965565245011;
                }
                _ => { }
            }
            match current_block_179 {
                17775571965565245011 =>
                /* fallthrough */
                {
                    let fresh16 = p;
                    p = p.offset(1);
                    h64 ^=
                        (*fresh16 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1);
                    current_block_179 = 17440742207893796336;
                }
                _ => { }
            }
            match current_block_179 {
                17440742207893796336 =>
                /* fallthrough */
                {
                    let fresh17 = p;
                    p = p.offset(1);
                    h64 ^=
                        (*fresh17 as libc::c_ulong).wrapping_mul(PRIME64_5);
                    h64 =
                        (h64 << 11 as libc::c_int |
                             h64 >>
                                 64 as libc::c_int -
                                     11 as
                                         libc::c_int).wrapping_mul(PRIME64_1)
                }
                _ => { }
            }
            /* fallthrough */
            return XXH64_avalanche(h64)
        }
    /* impossible to reach */
    __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                  b"xxhash.c\x00" as *const u8 as *const libc::c_char,
                  806 as libc::c_int as libc::c_uint,
                  (*::std::mem::transmute::<&[u8; 76],
                                            &[libc::c_char; 76]>(b"U64 XXH64_finalize(U64, const void *, size_t, XXH_endianess, XXH_alignment)\x00")).as_ptr());
    return 0 as libc::c_int as U64;
    /* unreachable, but some compilers complain without it */
}
#[inline(always)]
unsafe extern "C" fn XXH64_endian_align(mut input: *const libc::c_void,
                                        mut len: size_t, mut seed: U64,
                                        mut endian: XXH_endianess,
                                        mut align: XXH_alignment) -> U64 {
    let mut p: *const BYTE = input as *const BYTE;
    let mut bEnd: *const BYTE = p.offset(len as isize);
    let mut h64: U64 = 0;
    if len >= 32 as libc::c_int as libc::c_ulong {
        let limit: *const BYTE = bEnd.offset(-(32 as libc::c_int as isize));
        let mut v1: U64 =
            seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2);
        let mut v2: U64 = seed.wrapping_add(PRIME64_2);
        let mut v3: U64 =
            seed.wrapping_add(0 as libc::c_int as libc::c_ulong);
        let mut v4: U64 = seed.wrapping_sub(PRIME64_1);
        loop  {
            v1 =
                XXH64_round(v1,
                            XXH_readLE64_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(8 as libc::c_int as isize);
            v2 =
                XXH64_round(v2,
                            XXH_readLE64_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(8 as libc::c_int as isize);
            v3 =
                XXH64_round(v3,
                            XXH_readLE64_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(8 as libc::c_int as isize);
            v4 =
                XXH64_round(v4,
                            XXH_readLE64_align(p as *const libc::c_void,
                                               endian, align));
            p = p.offset(8 as libc::c_int as isize);
            if !(p <= limit) { break ; }
        }
        h64 =
            (v1 << 1 as libc::c_int |
                 v1 >>
                     64 as libc::c_int -
                         1 as
                             libc::c_int).wrapping_add(v2 << 7 as libc::c_int
                                                           |
                                                           v2 >>
                                                               64 as
                                                                   libc::c_int
                                                                   -
                                                                   7 as
                                                                       libc::c_int).wrapping_add(v3
                                                                                                     <<
                                                                                                     12
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                     |
                                                                                                     v3
                                                                                                         >>
                                                                                                         64
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             -
                                                                                                             12
                                                                                                                 as
                                                                                                                 libc::c_int).wrapping_add(v4
                                                                                                                                               <<
                                                                                                                                               18
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                               |
                                                                                                                                               v4
                                                                                                                                                   >>
                                                                                                                                                   64
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       -
                                                                                                                                                       18
                                                                                                                                                           as
                                                                                                                                                           libc::c_int);
        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4)
    } else { h64 = seed.wrapping_add(PRIME64_5) }
    h64 = (h64 as libc::c_ulong).wrapping_add(len) as U64 as U64;
    return XXH64_finalize(h64, p as *const libc::c_void, len, endian, align);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64(mut input: *const libc::c_void,
                                   mut len: size_t,
                                   mut seed: libc::c_ulonglong)
 -> XXH64_hash_t {
    let mut endian_detected: XXH_endianess =
        XXH_isLittleEndian() as XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint ||
           0 as libc::c_int != 0 {
        return XXH64_endian_align(input, len, seed as U64, XXH_littleEndian,
                                  XXH_unaligned) as XXH64_hash_t
    } else {
        return XXH64_endian_align(input, len, seed as U64, XXH_bigEndian,
                                  XXH_unaligned) as XXH64_hash_t
    };
}
/*======   Hash Streaming   ======*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64_createState() -> *mut XXH64_state_t {
    return XXH_malloc(::std::mem::size_of::<XXH64_state_t>() as libc::c_ulong)
               as
               *mut XXH64_state_t; /* using a local state to memcpy() in order to avoid strict-aliasing warnings */
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64_freeState(mut statePtr: *mut XXH64_state_t)
 -> XXH_errorcode {
    XXH_free(statePtr as *mut libc::c_void);
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64_copyState(mut dstState: *mut XXH64_state_t,
                                             mut srcState:
                                                 *const XXH64_state_t) {
    memcpy(dstState as *mut libc::c_void, srcState as *const libc::c_void,
           ::std::mem::size_of::<XXH64_state_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64_reset(mut statePtr: *mut XXH64_state_t,
                                         mut seed: libc::c_ulonglong)
 -> XXH_errorcode {
    let mut state: XXH64_state_t =
        XXH64_state_t{total_len: 0,
                      v1: 0,
                      v2: 0,
                      v3: 0,
                      v4: 0,
                      mem64: [0; 4],
                      memsize: 0,
                      reserved: [0; 2],};
    memset(&mut state as *mut XXH64_state_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<XXH64_state_t>() as libc::c_ulong);
    state.v1 =
        seed.wrapping_add(PRIME64_1 as
                              libc::c_ulonglong).wrapping_add(PRIME64_2 as
                                                                  libc::c_ulonglong)
            as uint64_t;
    state.v2 = seed.wrapping_add(PRIME64_2 as libc::c_ulonglong) as uint64_t;
    state.v3 =
        seed.wrapping_add(0 as libc::c_int as libc::c_ulonglong) as uint64_t;
    state.v4 = seed.wrapping_sub(PRIME64_1 as libc::c_ulonglong) as uint64_t;
    /* do not write into reserved, planned to be removed in a future version */
    memcpy(statePtr as *mut libc::c_void,
           &mut state as *mut XXH64_state_t as *const libc::c_void,
           (::std::mem::size_of::<XXH64_state_t>() as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<[uint32_t; 2]>()
                                                as libc::c_ulong));
    return XXH_OK;
}
#[inline(always)]
unsafe extern "C" fn XXH64_update_endian(mut state: *mut XXH64_state_t,
                                         mut input: *const libc::c_void,
                                         mut len: size_t,
                                         mut endian: XXH_endianess)
 -> XXH_errorcode {
    if input.is_null() { return XXH_ERROR }
    let mut p: *const BYTE = input as *const BYTE;
    let bEnd: *const BYTE = p.offset(len as isize);
    (*state).total_len =
        ((*state).total_len as libc::c_ulong).wrapping_add(len) as uint64_t as
            uint64_t;
    if ((*state).memsize as libc::c_ulong).wrapping_add(len) <
           32 as libc::c_int as libc::c_ulong {
        /* fill in tmp buffer */
        XXH_memcpy(((*state).mem64.as_mut_ptr() as
                        *mut BYTE).offset((*state).memsize as isize) as
                       *mut libc::c_void, input, len);
        (*state).memsize =
            ((*state).memsize as libc::c_uint).wrapping_add(len as U32) as
                uint32_t as uint32_t;
        return XXH_OK
    }
    if (*state).memsize != 0 {
        /* tmp buffer is full */
        XXH_memcpy(((*state).mem64.as_mut_ptr() as
                        *mut BYTE).offset((*state).memsize as isize) as
                       *mut libc::c_void, input,
                   (32 as libc::c_int as
                        libc::c_uint).wrapping_sub((*state).memsize) as
                       size_t);
        (*state).v1 =
            XXH64_round((*state).v1,
                        XXH_readLE64((*state).mem64.as_mut_ptr().offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                         as *const libc::c_void, endian));
        (*state).v2 =
            XXH64_round((*state).v2,
                        XXH_readLE64((*state).mem64.as_mut_ptr().offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                         as *const libc::c_void, endian));
        (*state).v3 =
            XXH64_round((*state).v3,
                        XXH_readLE64((*state).mem64.as_mut_ptr().offset(2 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                         as *const libc::c_void, endian));
        (*state).v4 =
            XXH64_round((*state).v4,
                        XXH_readLE64((*state).mem64.as_mut_ptr().offset(3 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                         as *const libc::c_void, endian));
        p =
            p.offset((32 as libc::c_int as
                          libc::c_uint).wrapping_sub((*state).memsize) as
                         isize);
        (*state).memsize = 0 as libc::c_int as uint32_t
    }
    if p.offset(32 as libc::c_int as isize) <= bEnd {
        let limit: *const BYTE = bEnd.offset(-(32 as libc::c_int as isize));
        let mut v1: U64 = (*state).v1;
        let mut v2: U64 = (*state).v2;
        let mut v3: U64 = (*state).v3;
        let mut v4: U64 = (*state).v4;
        loop  {
            v1 =
                XXH64_round(v1,
                            XXH_readLE64(p as *const libc::c_void, endian));
            p = p.offset(8 as libc::c_int as isize);
            v2 =
                XXH64_round(v2,
                            XXH_readLE64(p as *const libc::c_void, endian));
            p = p.offset(8 as libc::c_int as isize);
            v3 =
                XXH64_round(v3,
                            XXH_readLE64(p as *const libc::c_void, endian));
            p = p.offset(8 as libc::c_int as isize);
            v4 =
                XXH64_round(v4,
                            XXH_readLE64(p as *const libc::c_void, endian));
            p = p.offset(8 as libc::c_int as isize);
            if !(p <= limit) { break ; }
        }
        (*state).v1 = v1;
        (*state).v2 = v2;
        (*state).v3 = v3;
        (*state).v4 = v4
    }
    if p < bEnd {
        XXH_memcpy((*state).mem64.as_mut_ptr() as *mut libc::c_void,
                   p as *const libc::c_void,
                   bEnd.wrapping_offset_from(p) as libc::c_long as size_t);
        (*state).memsize =
            bEnd.wrapping_offset_from(p) as libc::c_long as libc::c_uint
    }
    return XXH_OK;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64_update(mut state_in: *mut XXH64_state_t,
                                          mut input: *const libc::c_void,
                                          mut len: size_t) -> XXH_errorcode {
    let mut endian_detected: XXH_endianess =
        XXH_isLittleEndian() as XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint ||
           0 as libc::c_int != 0 {
        return XXH64_update_endian(state_in, input, len, XXH_littleEndian)
    } else {
        return XXH64_update_endian(state_in, input, len, XXH_bigEndian)
    };
}
#[inline(always)]
unsafe extern "C" fn XXH64_digest_endian(mut state: *const XXH64_state_t,
                                         mut endian: XXH_endianess) -> U64 {
    let mut h64: U64 = 0;
    if (*state).total_len >= 32 as libc::c_int as libc::c_ulong {
        let v1: U64 = (*state).v1;
        let v2: U64 = (*state).v2;
        let v3: U64 = (*state).v3;
        let v4: U64 = (*state).v4;
        h64 =
            (v1 << 1 as libc::c_int |
                 v1 >>
                     64 as libc::c_int -
                         1 as
                             libc::c_int).wrapping_add(v2 << 7 as libc::c_int
                                                           |
                                                           v2 >>
                                                               64 as
                                                                   libc::c_int
                                                                   -
                                                                   7 as
                                                                       libc::c_int).wrapping_add(v3
                                                                                                     <<
                                                                                                     12
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                     |
                                                                                                     v3
                                                                                                         >>
                                                                                                         64
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             -
                                                                                                             12
                                                                                                                 as
                                                                                                                 libc::c_int).wrapping_add(v4
                                                                                                                                               <<
                                                                                                                                               18
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                               |
                                                                                                                                               v4
                                                                                                                                                   >>
                                                                                                                                                   64
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       -
                                                                                                                                                       18
                                                                                                                                                           as
                                                                                                                                                           libc::c_int);
        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4)
    } else { h64 = (*state).v3.wrapping_add(PRIME64_5) }
    h64 =
        (h64 as libc::c_ulong).wrapping_add((*state).total_len) as U64 as U64;
    return XXH64_finalize(h64, (*state).mem64.as_ptr() as *const libc::c_void,
                          (*state).total_len, endian, XXH_aligned);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64_digest(mut state_in: *const XXH64_state_t)
 -> XXH64_hash_t {
    let mut endian_detected: XXH_endianess =
        XXH_isLittleEndian() as XXH_endianess;
    if endian_detected as libc::c_uint ==
           XXH_littleEndian as libc::c_int as libc::c_uint ||
           0 as libc::c_int != 0 {
        return XXH64_digest_endian(state_in, XXH_littleEndian) as XXH64_hash_t
    } else {
        return XXH64_digest_endian(state_in, XXH_bigEndian) as XXH64_hash_t
    };
}
/*====== Canonical representation   ======*/
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64_canonicalFromHash(mut dst:
                                                         *mut XXH64_canonical_t,
                                                     mut hash: XXH64_hash_t) {
    if XXH_isLittleEndian() != 0 {
        hash = XXH_swap64(hash as U64) as XXH64_hash_t
    }
    memcpy(dst as *mut libc::c_void,
           &mut hash as *mut XXH64_hash_t as *const libc::c_void,
           ::std::mem::size_of::<XXH64_canonical_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4_XXH64_hashFromCanonical(mut src:
                                                         *const XXH64_canonical_t)
 -> XXH64_hash_t {
    return XXH_readBE64(src as *const libc::c_void) as XXH64_hash_t;
}
/* XXH_NO_LONG_LONG */
