use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
              _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type litDistribTable = [BYTE; 8192];
unsafe extern "C" fn RDG_rand(mut src: *mut U32) -> libc::c_uint {
    let mut rand32: U32 =
        *src; /* try to ease static analyzer. u < end <= LTSIZE */
    rand32 =
        (rand32 as libc::c_uint).wrapping_mul(2654435761 as libc::c_uint) as
            U32 as U32;
    rand32 ^= 2246822519 as libc::c_uint;
    rand32 =
        rand32 << 13 as libc::c_int |
            rand32 >> 32 as libc::c_int - 13 as libc::c_int;
    *src = rand32;
    return rand32;
}
unsafe extern "C" fn RDG_fillLiteralDistrib(mut lt: *mut BYTE,
                                            mut ld: libc::c_double) {
    let firstChar: BYTE =
        if ld <= 0.0f64 { 0 as libc::c_int } else { '(' as i32 } as BYTE;
    let lastChar: BYTE =
        if ld <= 0.0f64 { 255 as libc::c_int } else { '}' as i32 } as BYTE;
    let mut character: BYTE =
        if ld <= 0.0f64 { 0 as libc::c_int } else { '0' as i32 } as BYTE;
    let mut u: U32 = 0 as libc::c_int as U32;
    while u < ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint {
        let weight: U32 =
            (((((1 as libc::c_int) << 13 as libc::c_int) as
                   libc::c_uint).wrapping_sub(u) as libc::c_double * ld) as
                 U32).wrapping_add(1 as libc::c_int as libc::c_uint);
        let end: U32 =
            if u.wrapping_add(weight) <
                   ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint {
                u.wrapping_add(weight)
            } else {
                ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint
            };
        while u < end {
            if u < ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint {
            } else {
                __assert_fail(b"u<LTSIZE\x00" as *const u8 as
                                  *const libc::c_char,
                              b"datagen.c\x00" as *const u8 as
                                  *const libc::c_char,
                              83 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 44],
                                                        &[libc::c_char; 44]>(b"void RDG_fillLiteralDistrib(BYTE *, double)\x00")).as_ptr());
            }
            let fresh0 = u;
            u = u.wrapping_add(1);
            *lt.offset(fresh0 as isize) = character
        }
        character = character.wrapping_add(1);
        if character as libc::c_int > lastChar as libc::c_int {
            character = firstChar
        }
    };
}
unsafe extern "C" fn RDG_genChar(mut seed: *mut U32, mut lt: *const BYTE)
 -> BYTE {
    let mut id: U32 =
        RDG_rand(seed) &
            (((1 as libc::c_int) << 13 as libc::c_int) - 1 as libc::c_int) as
                libc::c_uint;
    return *lt.offset(id as isize);
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genBlock(mut buffer: *mut libc::c_void,
                                      mut buffSize: size_t,
                                      mut prefixSize: size_t,
                                      mut matchProba: libc::c_double,
                                      mut lt: *mut BYTE,
                                      mut seedPtr: *mut libc::c_uint) {
    let mut buffPtr: *mut BYTE = buffer as *mut BYTE;
    let matchProba32: U32 =
        (32768 as libc::c_int as libc::c_double * matchProba) as U32;
    let mut pos: size_t = prefixSize;
    let mut seed: *mut U32 = seedPtr;
    /* special case */
    while matchProba >= 1.0f64 {
        let mut size0: size_t =
            (RDG_rand(seed) & 3 as libc::c_int as libc::c_uint) as
                size_t; /* because size0 is power of 2*/
        size0 =
            (1 as libc::c_int as size_t) <<
                (16 as libc::c_int as
                     libc::c_ulong).wrapping_add(size0.wrapping_mul(2 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong));
        size0 =
            (size0 as
                 libc::c_ulong).wrapping_add(RDG_rand(seed) as libc::c_ulong &
                                                 size0.wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong))
                as size_t as size_t;
        if buffSize < pos.wrapping_add(size0) {
            memset(buffPtr.offset(pos as isize) as *mut libc::c_void,
                   0 as libc::c_int, buffSize.wrapping_sub(pos));
            return
        }
        memset(buffPtr.offset(pos as isize) as *mut libc::c_void,
               0 as libc::c_int, size0);
        pos = (pos as libc::c_ulong).wrapping_add(size0) as size_t as size_t;
        *buffPtr.offset(pos.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                            isize) = RDG_genChar(seed, lt as *const BYTE)
    }
    /* init */
    if pos == 0 as libc::c_int as libc::c_ulong {
        *buffPtr.offset(0 as libc::c_int as isize) =
            RDG_genChar(seed, lt as *const BYTE);
        pos = 1 as libc::c_int as size_t
    }
    /* Generate compressible data */
    while pos < buffSize {
        /* Select : Literal (char) or Match (within 32K) */
        if (RDG_rand(seed) >> 3 as libc::c_int &
                32767 as libc::c_int as libc::c_uint) < matchProba32 {
            /* Copy (within 32K) */
            let mut match_0: size_t = 0;
            let mut d: size_t = 0;
            let mut length: libc::c_int =
                (if RDG_rand(seed) >> 7 as libc::c_int &
                        7 as libc::c_int as libc::c_uint != 0 {
                     (RDG_rand(seed)) & 15 as libc::c_int as libc::c_uint
                 } else {
                     (RDG_rand(seed) &
                          511 as libc::c_int as
                              libc::c_uint).wrapping_add(15 as libc::c_int as
                                                             libc::c_uint)
                 }).wrapping_add(4 as libc::c_int as libc::c_uint) as
                    libc::c_int;
            let mut offset: U32 =
                (RDG_rand(seed) >> 3 as libc::c_int &
                     32767 as libc::c_int as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint);
            if offset as libc::c_ulong > pos { offset = pos as U32 }
            match_0 = pos.wrapping_sub(offset as libc::c_ulong);
            d = pos.wrapping_add(length as libc::c_ulong);
            if d > buffSize { d = buffSize }
            while pos < d {
                let fresh1 = match_0;
                match_0 = match_0.wrapping_add(1);
                let fresh2 = pos;
                pos = pos.wrapping_add(1);
                *buffPtr.offset(fresh2 as isize) =
                    *buffPtr.offset(fresh1 as isize)
            }
        } else {
            /* Literal (noise) */
            let mut d_0: size_t = 0;
            let mut length_0: size_t =
                if RDG_rand(seed) >> 7 as libc::c_int &
                       7 as libc::c_int as libc::c_uint != 0 {
                    (RDG_rand(seed)) & 15 as libc::c_int as libc::c_uint
                } else {
                    (RDG_rand(seed) &
                         511 as libc::c_int as
                             libc::c_uint).wrapping_add(15 as libc::c_int as
                                                            libc::c_uint)
                } as size_t;
            d_0 = pos.wrapping_add(length_0);
            if d_0 > buffSize { d_0 = buffSize }
            while pos < d_0 {
                let fresh3 = pos;
                pos = pos.wrapping_add(1);
                *buffPtr.offset(fresh3 as isize) =
                    RDG_genChar(seed, lt as *const BYTE)
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genBuffer(mut buffer: *mut libc::c_void,
                                       mut size: size_t,
                                       mut matchProba: libc::c_double,
                                       mut litProba: libc::c_double,
                                       mut seed: libc::c_uint) {
    let mut lt: litDistribTable = [0; 8192];
    if litProba == 0.0f64 { litProba = matchProba / 4.5f64 }
    RDG_fillLiteralDistrib(lt.as_mut_ptr(), litProba);
    RDG_genBlock(buffer, size, 0 as libc::c_int as size_t, matchProba,
                 lt.as_mut_ptr(), &mut seed);
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genOut(mut size: libc::c_ulonglong,
                                    mut matchProba: libc::c_double,
                                    mut litProba: libc::c_double,
                                    mut seed: libc::c_uint) {
    let mut buff: [BYTE; 163840] = [0; 163840];
    let mut total: U64 = 0 as libc::c_int as U64;
    let mut genBlockSize: size_t =
        (128 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
            size_t;
    let mut lt: litDistribTable = [0; 8192];
    /* init */
    if litProba == 0.0f64 { litProba = matchProba / 4.5f64 }
    RDG_fillLiteralDistrib(lt.as_mut_ptr(), litProba);
    /* Generate dict */
    RDG_genBlock(buff.as_mut_ptr() as *mut libc::c_void,
                 (32 as libc::c_int *
                      ((1 as libc::c_int) << 10 as libc::c_int)) as size_t,
                 0 as libc::c_int as size_t, matchProba, lt.as_mut_ptr(),
                 &mut seed);
    /* Generate compressible data */
    while (total as libc::c_ulonglong) < size {
        RDG_genBlock(buff.as_mut_ptr() as *mut libc::c_void,
                     (32 as libc::c_int *
                          ((1 as libc::c_int) << 10 as libc::c_int) +
                          128 as libc::c_int *
                              ((1 as libc::c_int) << 10 as libc::c_int)) as
                         size_t,
                     (32 as libc::c_int *
                          ((1 as libc::c_int) << 10 as libc::c_int)) as
                         size_t, matchProba, lt.as_mut_ptr(),
                     &mut seed); /* should check potential write error */
        if size.wrapping_sub(total as libc::c_ulonglong) <
               (128 as libc::c_int *
                    ((1 as libc::c_int) << 10 as libc::c_int)) as
                   libc::c_ulonglong {
            genBlockSize =
                size.wrapping_sub(total as libc::c_ulonglong) as size_t
        }
        total =
            (total as libc::c_ulong).wrapping_add(genBlockSize) as U64 as U64;
        fwrite(buff.as_mut_ptr() as *const libc::c_void,
               1 as libc::c_int as libc::c_ulong, genBlockSize, stdout);
        /* update dict */
        memcpy(buff.as_mut_ptr() as *mut libc::c_void,
               buff.as_mut_ptr().offset((128 as libc::c_int *
                                             ((1 as libc::c_int) <<
                                                  10 as libc::c_int)) as
                                            isize) as *const libc::c_void,
               (32 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                   as libc::c_ulong);
    };
}
