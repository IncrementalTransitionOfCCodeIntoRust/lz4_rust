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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
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
    fn LZ4_saveDict(streamPtr: *mut LZ4_stream_t,
                    safeBuffer: *mut libc::c_char, maxDictSize: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4_decompress_safe_usingDict(src: *const libc::c_char,
                                     dst: *mut libc::c_char,
                                     srcSize: libc::c_int,
                                     dstCapcity: libc::c_int,
                                     dictStart: *const libc::c_char,
                                     dictSize: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn LZ4_compress_fast_extState_fastReset(state: *mut libc::c_void,
                                            src: *const libc::c_char,
                                            dst: *mut libc::c_char,
                                            srcSize: libc::c_int,
                                            dstCapacity: libc::c_int,
                                            acceleration: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4_attach_dictionary(workingStream: *mut LZ4_stream_t,
                             dictionaryStream: *const LZ4_stream_t);
    #[no_mangle]
    fn LZ4_initStream(buffer: *mut libc::c_void, size: size_t)
     -> *mut LZ4_stream_t;
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
    fn LZ4_saveDictHC(streamHCPtr: *mut LZ4_streamHC_t,
                      safeBuffer: *mut libc::c_char, maxDictSize: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4_initStreamHC(buffer: *mut libc::c_void, size: size_t)
     -> *mut LZ4_streamHC_t;
    #[no_mangle]
    fn LZ4_setCompressionLevel(LZ4_streamHCPtr: *mut LZ4_streamHC_t,
                               compressionLevel: libc::c_int);
    #[no_mangle]
    fn LZ4_favorDecompressionSpeed(LZ4_streamHCPtr: *mut LZ4_streamHC_t,
                                   favor: libc::c_int);
    #[no_mangle]
    fn LZ4_compress_HC_extStateHC_fastReset(state: *mut libc::c_void,
                                            src: *const libc::c_char,
                                            dst: *mut libc::c_char,
                                            srcSize: libc::c_int,
                                            dstCapacity: libc::c_int,
                                            compressionLevel: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LZ4_attach_HC_dictionary(working_stream: *mut LZ4_streamHC_t,
                                dictionary_stream: *const LZ4_streamHC_t);
    #[no_mangle]
    fn LZ4_XXH32(input: *const libc::c_void, length: size_t,
                 seed: libc::c_uint) -> XXH32_hash_t;
    #[no_mangle]
    fn LZ4_XXH32_reset(statePtr: *mut XXH32_state_t, seed: libc::c_uint)
     -> XXH_errorcode;
    #[no_mangle]
    fn LZ4_XXH32_update(statePtr: *mut XXH32_state_t,
                        input: *const libc::c_void, length: size_t)
     -> XXH_errorcode;
    #[no_mangle]
    fn LZ4_XXH32_digest(statePtr: *const XXH32_state_t) -> XXH32_hash_t;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type ptrdiff_t = libc::c_long;
pub type LZ4F_errorCode_t = size_t;
pub const LZ4F_ERROR_maxCode: LZ4F_errorCodes = 20;
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
pub type LZ4F_errorCodes = libc::c_uint;
pub const _LZ4F_dummy_error_enum_for_c89_never_used: LZ4F_errorCodes = 21;
pub const LZ4F_ERROR_frameDecoding_alreadyStarted: LZ4F_errorCodes = 19;
pub const LZ4F_ERROR_contentChecksum_invalid: LZ4F_errorCodes = 18;
pub const LZ4F_ERROR_headerChecksum_invalid: LZ4F_errorCodes = 17;
pub const LZ4F_ERROR_decompressionFailed: LZ4F_errorCodes = 16;
pub const LZ4F_ERROR_srcPtr_wrong: LZ4F_errorCodes = 15;
pub const LZ4F_ERROR_frameSize_wrong: LZ4F_errorCodes = 14;
pub const LZ4F_ERROR_frameType_unknown: LZ4F_errorCodes = 13;
pub const LZ4F_ERROR_frameHeader_incomplete: LZ4F_errorCodes = 12;
pub const LZ4F_ERROR_dstMaxSize_tooSmall: LZ4F_errorCodes = 11;
pub const LZ4F_ERROR_srcSize_tooLarge: LZ4F_errorCodes = 10;
pub const LZ4F_ERROR_allocation_failed: LZ4F_errorCodes = 9;
pub const LZ4F_ERROR_reservedFlag_set: LZ4F_errorCodes = 8;
pub const LZ4F_ERROR_blockChecksum_invalid: LZ4F_errorCodes = 7;
pub const LZ4F_ERROR_headerVersion_wrong: LZ4F_errorCodes = 6;
pub const LZ4F_ERROR_compressionLevel_invalid: LZ4F_errorCodes = 5;
pub const LZ4F_ERROR_contentChecksumFlag_invalid: LZ4F_errorCodes = 4;
pub const LZ4F_ERROR_blockMode_invalid: LZ4F_errorCodes = 3;
pub const LZ4F_ERROR_maxBlockSize_invalid: LZ4F_errorCodes = 2;
pub const LZ4F_ERROR_GENERIC: LZ4F_errorCodes = 1;
pub const LZ4F_OK_NoError: LZ4F_errorCodes = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const LZ4F_static_assert: C2RustUnnamed = 1;
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
/* block footer : checksum (optional) */
/*-************************************
*  Structures and local types
**************************************/
pub type LZ4F_cctx_t = LZ4F_cctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_cctx_s {
    pub prefs: LZ4F_preferences_t,
    pub version: U32,
    pub cStage: U32,
    pub cdict: *const LZ4F_CDict,
    pub maxBlockSize: size_t,
    pub maxBufferSize: size_t,
    pub tmpBuff: *mut BYTE,
    pub tmpIn: *mut BYTE,
    pub tmpInSize: size_t,
    pub totalInSize: U64,
    pub xxh: XXH32_state_t,
    pub lz4CtxPtr: *mut libc::c_void,
    pub lz4CtxAlloc: U16,
    pub lz4CtxState: U16,
}
pub type U16 = uint16_t;
pub type uint16_t = __uint16_t;
pub type XXH32_state_t = XXH32_state_s;
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
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
/* sized for: 0 = none, 1 = lz4 ctx, 2 = lz4hc ctx */
/* in use as: 0 = none, 1 = lz4 ctx, 2 = lz4hc ctx */
/* memset, memcpy, memmove */
/* avoid redefinition when sources are coalesced */
/*-************************************
*  Library declarations
**************************************/
/*-************************************
*  Debug
**************************************/
/* use only *after* variable declarations */
/* disabled */
/*-************************************
*  Basic Types
**************************************/
/* C99 */
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type LZ4F_CDict = LZ4F_CDict_s;
/*-***************************************************
*   Dictionary compression
*****************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_CDict_s {
    pub dictContent: *mut libc::c_void,
    pub fastCtx: *mut LZ4_stream_t,
    pub HCCtx: *mut LZ4_streamHC_t,
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
pub type LZ4F_cctx = LZ4F_cctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_compressOptions_t {
    pub stableSrc: libc::c_uint,
    pub reserved: [libc::c_uint; 3],
}
pub type XXH32_hash_t = libc::c_uint;
pub type compressFunc_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: *mut libc::c_char, _: libc::c_int,
                                _: libc::c_int, _: libc::c_int,
                                _: *const LZ4F_CDict) -> libc::c_int>;
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub const fromSrcBuffer: LZ4F_lastBlockStatus = 2;
pub type LZ4F_lastBlockStatus = libc::c_uint;
pub const fromTmpBuffer: LZ4F_lastBlockStatus = 1;
pub const notDone: LZ4F_lastBlockStatus = 0;
pub type LZ4F_compressionContext_t = *mut LZ4F_cctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_dctx_s {
    pub frameInfo: LZ4F_frameInfo_t,
    pub version: U32,
    pub dStage: dStage_t,
    pub frameRemainingSize: U64,
    pub maxBlockSize: size_t,
    pub maxBufferSize: size_t,
    pub tmpIn: *mut BYTE,
    pub tmpInSize: size_t,
    pub tmpInTarget: size_t,
    pub tmpOutBuffer: *mut BYTE,
    pub dict: *const BYTE,
    pub dictSize: size_t,
    pub tmpOut: *mut BYTE,
    pub tmpOutSize: size_t,
    pub tmpOutStart: size_t,
    pub xxh: XXH32_state_t,
    pub blockChecksum: XXH32_state_t,
    pub header: [BYTE; 19],
}
pub type dStage_t = libc::c_uint;
pub const dstage_skipSkippable: dStage_t = 14;
pub const dstage_storeSFrameSize: dStage_t = 13;
pub const dstage_getSFrameSize: dStage_t = 12;
pub const dstage_storeSuffix: dStage_t = 11;
pub const dstage_getSuffix: dStage_t = 10;
pub const dstage_flushOut: dStage_t = 9;
pub const dstage_storeCBlock: dStage_t = 8;
pub const dstage_getCBlock: dStage_t = 7;
pub const dstage_getBlockChecksum: dStage_t = 6;
pub const dstage_copyDirect: dStage_t = 5;
pub const dstage_storeBlockHeader: dStage_t = 4;
pub const dstage_getBlockHeader: dStage_t = 3;
pub const dstage_init: dStage_t = 2;
pub const dstage_storeFrameHeader: dStage_t = 1;
pub const dstage_getFrameHeader: dStage_t = 0;
pub type LZ4F_dctx = LZ4F_dctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_decompressOptions_t {
    pub stableDst: libc::c_uint,
    pub reserved: [libc::c_uint; 3],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LZ4F_static_assert_0: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const LZ4F_static_assert_1: C2RustUnnamed_1 = 1;
/* unoptimized version; solves endianess & alignment issues */
unsafe extern "C" fn LZ4F_readLE32(mut src: *const libc::c_void) -> U32 {
    let srcPtr: *const BYTE = src as *const BYTE;
    let mut value32: U32 = *srcPtr.offset(0 as libc::c_int as isize) as U32;
    value32 =
        (value32 as
             libc::c_uint).wrapping_add((*srcPtr.offset(1 as libc::c_int as
                                                            isize) as U32) <<
                                            8 as libc::c_int) as U32 as U32;
    value32 =
        (value32 as
             libc::c_uint).wrapping_add((*srcPtr.offset(2 as libc::c_int as
                                                            isize) as U32) <<
                                            16 as libc::c_int) as U32 as U32;
    value32 =
        (value32 as
             libc::c_uint).wrapping_add((*srcPtr.offset(3 as libc::c_int as
                                                            isize) as U32) <<
                                            24 as libc::c_int) as U32 as U32;
    return value32;
}
unsafe extern "C" fn LZ4F_writeLE32(mut dst: *mut libc::c_void,
                                    mut value32: U32) {
    let dstPtr: *mut BYTE = dst as *mut BYTE;
    *dstPtr.offset(0 as libc::c_int as isize) = value32 as BYTE;
    *dstPtr.offset(1 as libc::c_int as isize) =
        (value32 >> 8 as libc::c_int) as BYTE;
    *dstPtr.offset(2 as libc::c_int as isize) =
        (value32 >> 16 as libc::c_int) as BYTE;
    *dstPtr.offset(3 as libc::c_int as isize) =
        (value32 >> 24 as libc::c_int) as BYTE;
}
unsafe extern "C" fn LZ4F_readLE64(mut src: *const libc::c_void) -> U64 {
    let srcPtr: *const BYTE = src as *const BYTE;
    let mut value64: U64 = *srcPtr.offset(0 as libc::c_int as isize) as U64;
    value64 =
        (value64 as
             libc::c_ulong).wrapping_add((*srcPtr.offset(1 as libc::c_int as
                                                             isize) as U64) <<
                                             8 as libc::c_int) as U64 as U64;
    value64 =
        (value64 as
             libc::c_ulong).wrapping_add((*srcPtr.offset(2 as libc::c_int as
                                                             isize) as U64) <<
                                             16 as libc::c_int) as U64 as U64;
    value64 =
        (value64 as
             libc::c_ulong).wrapping_add((*srcPtr.offset(3 as libc::c_int as
                                                             isize) as U64) <<
                                             24 as libc::c_int) as U64 as U64;
    value64 =
        (value64 as
             libc::c_ulong).wrapping_add((*srcPtr.offset(4 as libc::c_int as
                                                             isize) as U64) <<
                                             32 as libc::c_int) as U64 as U64;
    value64 =
        (value64 as
             libc::c_ulong).wrapping_add((*srcPtr.offset(5 as libc::c_int as
                                                             isize) as U64) <<
                                             40 as libc::c_int) as U64 as U64;
    value64 =
        (value64 as
             libc::c_ulong).wrapping_add((*srcPtr.offset(6 as libc::c_int as
                                                             isize) as U64) <<
                                             48 as libc::c_int) as U64 as U64;
    value64 =
        (value64 as
             libc::c_ulong).wrapping_add((*srcPtr.offset(7 as libc::c_int as
                                                             isize) as U64) <<
                                             56 as libc::c_int) as U64 as U64;
    return value64;
}
unsafe extern "C" fn LZ4F_writeLE64(mut dst: *mut libc::c_void,
                                    mut value64: U64) {
    let dstPtr: *mut BYTE = dst as *mut BYTE;
    *dstPtr.offset(0 as libc::c_int as isize) = value64 as BYTE;
    *dstPtr.offset(1 as libc::c_int as isize) =
        (value64 >> 8 as libc::c_int) as BYTE;
    *dstPtr.offset(2 as libc::c_int as isize) =
        (value64 >> 16 as libc::c_int) as BYTE;
    *dstPtr.offset(3 as libc::c_int as isize) =
        (value64 >> 24 as libc::c_int) as BYTE;
    *dstPtr.offset(4 as libc::c_int as isize) =
        (value64 >> 32 as libc::c_int) as BYTE;
    *dstPtr.offset(5 as libc::c_int as isize) =
        (value64 >> 40 as libc::c_int) as BYTE;
    *dstPtr.offset(6 as libc::c_int as isize) =
        (value64 >> 48 as libc::c_int) as BYTE;
    *dstPtr.offset(7 as libc::c_int as isize) =
        (value64 >> 56 as libc::c_int) as BYTE;
}
static mut minFHSize: size_t = 7 as libc::c_int as size_t;
/*  7 */
static mut maxFHSize: size_t = 19 as libc::c_int as size_t;
/* 19 */
static mut BHSize: size_t = 4 as libc::c_int as size_t;
/* block header : size, and compress flag */
static mut BFSize: size_t = 4 as libc::c_int as size_t;
/*-************************************
*  Error management
**************************************/
static mut LZ4F_errorStrings: [*const libc::c_char; 21] =
    [b"OK_NoError\x00" as *const u8 as *const libc::c_char,
     b"ERROR_GENERIC\x00" as *const u8 as *const libc::c_char,
     b"ERROR_maxBlockSize_invalid\x00" as *const u8 as *const libc::c_char,
     b"ERROR_blockMode_invalid\x00" as *const u8 as *const libc::c_char,
     b"ERROR_contentChecksumFlag_invalid\x00" as *const u8 as
         *const libc::c_char,
     b"ERROR_compressionLevel_invalid\x00" as *const u8 as
         *const libc::c_char,
     b"ERROR_headerVersion_wrong\x00" as *const u8 as *const libc::c_char,
     b"ERROR_blockChecksum_invalid\x00" as *const u8 as *const libc::c_char,
     b"ERROR_reservedFlag_set\x00" as *const u8 as *const libc::c_char,
     b"ERROR_allocation_failed\x00" as *const u8 as *const libc::c_char,
     b"ERROR_srcSize_tooLarge\x00" as *const u8 as *const libc::c_char,
     b"ERROR_dstMaxSize_tooSmall\x00" as *const u8 as *const libc::c_char,
     b"ERROR_frameHeader_incomplete\x00" as *const u8 as *const libc::c_char,
     b"ERROR_frameType_unknown\x00" as *const u8 as *const libc::c_char,
     b"ERROR_frameSize_wrong\x00" as *const u8 as *const libc::c_char,
     b"ERROR_srcPtr_wrong\x00" as *const u8 as *const libc::c_char,
     b"ERROR_decompressionFailed\x00" as *const u8 as *const libc::c_char,
     b"ERROR_headerChecksum_invalid\x00" as *const u8 as *const libc::c_char,
     b"ERROR_contentChecksum_invalid\x00" as *const u8 as *const libc::c_char,
     b"ERROR_frameDecoding_alreadyStarted\x00" as *const u8 as
         *const libc::c_char,
     b"ERROR_maxCode\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn LZ4F_isError(mut code: LZ4F_errorCode_t)
 -> libc::c_uint {
    return (code > -(LZ4F_ERROR_maxCode as libc::c_int) as LZ4F_errorCode_t)
               as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_getErrorName(mut code: LZ4F_errorCode_t)
 -> *const libc::c_char {
    static mut codeError: *const libc::c_char =
        b"Unspecified error code\x00" as *const u8 as *const libc::c_char;
    if LZ4F_isError(code) != 0 {
        return LZ4F_errorStrings[-(code as libc::c_int) as usize]
    }
    return codeError;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_getErrorCode(mut functionResult: size_t)
 -> LZ4F_errorCodes {
    if LZ4F_isError(functionResult) == 0 { return LZ4F_OK_NoError }
    return -(functionResult as ptrdiff_t) as LZ4F_errorCodes;
}
unsafe extern "C" fn err0r(mut code: LZ4F_errorCodes) -> LZ4F_errorCode_t {
    /* A compilation error here means sizeof(ptrdiff_t) is not large enough */
    return -(code as ptrdiff_t) as LZ4F_errorCode_t;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_getVersion() -> libc::c_uint {
    return 100 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressionLevel_max() -> libc::c_int {
    return 12 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_getBlockSize(mut blockSizeID: libc::c_uint)
 -> size_t {
    static mut blockSizes: [size_t; 4] =
        [(64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
             size_t,
         (256 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
             size_t,
         (1 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
             size_t,
         (4 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
             size_t];
    if blockSizeID == 0 as libc::c_int as libc::c_uint {
        blockSizeID = LZ4F_max64KB as libc::c_int as libc::c_uint
    }
    if blockSizeID < LZ4F_max64KB as libc::c_int as libc::c_uint ||
           blockSizeID > LZ4F_max4MB as libc::c_int as libc::c_uint {
        return err0r(LZ4F_ERROR_maxBlockSize_invalid)
    }
    blockSizeID =
        blockSizeID.wrapping_sub(LZ4F_max64KB as libc::c_int as libc::c_uint);
    return blockSizes[blockSizeID as usize];
}
unsafe extern "C" fn LZ4F_headerChecksum(mut header: *const libc::c_void,
                                         mut length: size_t) -> BYTE {
    let xxh: U32 =
        LZ4_XXH32(header, length, 0 as libc::c_int as libc::c_uint);
    return (xxh >> 8 as libc::c_int) as BYTE;
}
/*-************************************
*  Simple-pass compression functions
**************************************/
unsafe extern "C" fn LZ4F_optimalBSID(requestedBSID: LZ4F_blockSizeID_t,
                                      srcSize: size_t) -> LZ4F_blockSizeID_t {
    let mut proposedBSID: LZ4F_blockSizeID_t = LZ4F_max64KB;
    let mut maxBlockSize: size_t =
        (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
            size_t;
    while requestedBSID as libc::c_uint > proposedBSID as libc::c_uint {
        if srcSize <= maxBlockSize { return proposedBSID }
        proposedBSID =
            (proposedBSID as libc::c_int + 1 as libc::c_int) as
                LZ4F_blockSizeID_t;
        maxBlockSize <<= 2 as libc::c_int
    }
    return requestedBSID;
}
/* ! LZ4F_compressBound_internal() :
 *  Provides dstCapacity given a srcSize to guarantee operation success in worst case situations.
 *  prefsPtr is optional : if NULL is provided, preferences will be set to cover worst case scenario.
 * @return is always the same for a srcSize and prefsPtr, so it can be relied upon to size reusable buffers.
 *  When srcSize==0, LZ4F_compressBound() provides an upper bound for LZ4F_flush() and LZ4F_compressEnd() operations.
 */
unsafe extern "C" fn LZ4F_compressBound_internal(mut srcSize: size_t,
                                                 mut preferencesPtr:
                                                     *const LZ4F_preferences_t,
                                                 mut alreadyBuffered: size_t)
 -> size_t {
    let mut prefsNull: LZ4F_preferences_t =
        {
            let mut init =
                LZ4F_preferences_t{frameInfo:
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
                                                                    LZ4F_noBlockChecksum,}; /* worst case */
                                           init
                                       },
                                   compressionLevel: 0 as libc::c_int,
                                   autoFlush: 0 as libc::c_uint,
                                   favorDecSpeed: 0 as libc::c_uint,
                                   reserved:
                                       [0 as libc::c_uint, 0 as libc::c_uint,
                                        0 as libc::c_uint],}; /* worst case */
            init
        }; /* max header size, including optional fields */
    prefsNull.frameInfo.contentChecksumFlag = LZ4F_contentChecksumEnabled;
    prefsNull.frameInfo.blockChecksumFlag = LZ4F_blockChecksumEnabled;
    let prefsPtr: *const LZ4F_preferences_t =
        if preferencesPtr.is_null() {
            &mut prefsNull as *mut LZ4F_preferences_t as
                *const LZ4F_preferences_t
        } else { preferencesPtr };
    let flush: U32 =
        (*prefsPtr).autoFlush |
            (srcSize == 0 as libc::c_int as libc::c_ulong) as libc::c_int as
                libc::c_uint;
    let blockID: LZ4F_blockSizeID_t = (*prefsPtr).frameInfo.blockSizeID;
    let blockSize: size_t = LZ4F_getBlockSize(blockID as libc::c_uint);
    let maxBuffered: size_t =
        blockSize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let bufferedSize: size_t =
        if alreadyBuffered < maxBuffered {
            alreadyBuffered
        } else { maxBuffered };
    let maxSrcSize: size_t = srcSize.wrapping_add(bufferedSize);
    let nbFullBlocks: libc::c_uint =
        maxSrcSize.wrapping_div(blockSize) as libc::c_uint;
    let partialBlockSize: size_t =
        maxSrcSize &
            blockSize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let lastBlockSize: size_t =
        if flush != 0 {
            partialBlockSize
        } else { 0 as libc::c_int as libc::c_ulong };
    let nbBlocks: libc::c_uint =
        nbFullBlocks.wrapping_add((lastBlockSize >
                                       0 as libc::c_int as libc::c_ulong) as
                                      libc::c_int as libc::c_uint);
    let blockCRCSize: size_t =
        BFSize.wrapping_mul((*prefsPtr).frameInfo.blockChecksumFlag as
                                libc::c_ulong);
    let frameEnd: size_t =
        BHSize.wrapping_add(((*prefsPtr).frameInfo.contentChecksumFlag as
                                 libc::c_ulong).wrapping_mul(BFSize));
    return BHSize.wrapping_add(blockCRCSize).wrapping_mul(nbBlocks as
                                                              libc::c_ulong).wrapping_add(blockSize.wrapping_mul(nbFullBlocks
                                                                                                                     as
                                                                                                                     libc::c_ulong)).wrapping_add(lastBlockSize).wrapping_add(frameEnd);
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressFrameBound(mut srcSize: size_t,
                                                 mut preferencesPtr:
                                                     *const LZ4F_preferences_t)
 -> size_t {
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
    let headerSize: size_t = maxFHSize;
    if !preferencesPtr.is_null() {
        prefs = *preferencesPtr
    } else {
        memset(&mut prefs as *mut LZ4F_preferences_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<LZ4F_preferences_t>() as libc::c_ulong);
    }
    prefs.autoFlush = 1 as libc::c_int as libc::c_uint;
    return headerSize.wrapping_add(LZ4F_compressBound_internal(srcSize,
                                                               &mut prefs,
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   size_t));
}
/* ! LZ4F_compressFrame_usingCDict() :
 *  Compress srcBuffer using a dictionary, in a single step.
 *  cdict can be NULL, in which case, no dictionary is used.
 *  dstBuffer MUST be >= LZ4F_compressFrameBound(srcSize, preferencesPtr).
 *  The LZ4F_preferences_t structure is optional : you may provide NULL as argument,
 *  however, it's the only way to provide a dictID, so it's not recommended.
 * @return : number of bytes written into dstBuffer,
 *           or an error code if it fails (can be tested using LZ4F_isError())
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressFrame_usingCDict(mut cctx:
                                                           *mut LZ4F_cctx,
                                                       mut dstBuffer:
                                                           *mut libc::c_void,
                                                       mut dstCapacity:
                                                           size_t,
                                                       mut srcBuffer:
                                                           *const libc::c_void,
                                                       mut srcSize: size_t,
                                                       mut cdict:
                                                           *const LZ4F_CDict,
                                                       mut preferencesPtr:
                                                           *const LZ4F_preferences_t)
 -> size_t {
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
                           reserved:
                               [0;
                                   3],}; /* auto-correct content size if selected (!=0) */
    let mut options: LZ4F_compressOptions_t =
        LZ4F_compressOptions_t{stableSrc: 0,
                               reserved:
                                   [0;
                                       3],}; /* only one block => no need for inter-block link */
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let mut dstPtr: *mut BYTE = dstStart;
    let dstEnd: *mut BYTE = dstStart.offset(dstCapacity as isize);
    if !preferencesPtr.is_null() {
        prefs = *preferencesPtr
    } else {
        memset(&mut prefs as *mut LZ4F_preferences_t as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<LZ4F_preferences_t>() as libc::c_ulong);
    }
    if prefs.frameInfo.contentSize != 0 as libc::c_int as libc::c_ulonglong {
        prefs.frameInfo.contentSize = srcSize as libc::c_ulonglong
    }
    prefs.frameInfo.blockSizeID =
        LZ4F_optimalBSID(prefs.frameInfo.blockSizeID, srcSize);
    prefs.autoFlush = 1 as libc::c_int as libc::c_uint;
    if srcSize <=
           LZ4F_getBlockSize(prefs.frameInfo.blockSizeID as libc::c_uint) {
        prefs.frameInfo.blockMode = LZ4F_blockIndependent
    }
    memset(&mut options as *mut LZ4F_compressOptions_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<LZ4F_compressOptions_t>() as libc::c_ulong);
    options.stableSrc = 1 as libc::c_int as libc::c_uint;
    if dstCapacity < LZ4F_compressFrameBound(srcSize, &mut prefs) {
        /* condition to guarantee success */
        return err0r(LZ4F_ERROR_dstMaxSize_tooSmall)
    } /* write header */
    let headerSize: size_t =
        LZ4F_compressBegin_usingCDict(cctx, dstBuffer, dstCapacity, cdict,
                                      &mut prefs); /* flush last block, and generate suffix */
    if LZ4F_isError(headerSize) != 0 { return headerSize }
    dstPtr = dstPtr.offset(headerSize as isize);
    let cSize: size_t =
        LZ4F_compressUpdate(cctx, dstPtr as *mut libc::c_void,
                            dstEnd.wrapping_offset_from(dstPtr) as
                                libc::c_long as size_t, srcBuffer, srcSize,
                            &mut options);
    if LZ4F_isError(cSize) != 0 { return cSize }
    dstPtr = dstPtr.offset(cSize as isize);
    let tailSize: size_t =
        LZ4F_compressEnd(cctx, dstPtr as *mut libc::c_void,
                         dstEnd.wrapping_offset_from(dstPtr) as libc::c_long
                             as size_t, &mut options);
    if LZ4F_isError(tailSize) != 0 { return tailSize }
    dstPtr = dstPtr.offset(tailSize as isize);
    return dstPtr.wrapping_offset_from(dstStart) as libc::c_long as size_t;
}
/* ! LZ4F_compressFrame() :
 *  Compress an entire srcBuffer into a valid LZ4 frame, in a single step.
 *  dstBuffer MUST be >= LZ4F_compressFrameBound(srcSize, preferencesPtr).
 *  The LZ4F_preferences_t structure is optional : you can provide NULL as argument. All preferences will be set to default.
 * @return : number of bytes written into dstBuffer.
 *           or an error code if it fails (can be tested using LZ4F_isError())
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressFrame(mut dstBuffer: *mut libc::c_void,
                                            mut dstCapacity: size_t,
                                            mut srcBuffer:
                                                *const libc::c_void,
                                            mut srcSize: size_t,
                                            mut preferencesPtr:
                                                *const LZ4F_preferences_t)
 -> size_t {
    let mut result: size_t =
        0; /* mess with real buffer size to prevent dynamic allocation; works only because autoflush==1 & stableSrc==1 */
    let mut cctx: LZ4F_cctx_t =
        LZ4F_cctx_t{prefs:
                        LZ4F_preferences_t{frameInfo:
                                               LZ4F_frameInfo_t{blockSizeID:
                                                                    LZ4F_default,
                                                                blockMode:
                                                                    LZ4F_blockLinked,
                                                                contentChecksumFlag:
                                                                    LZ4F_noContentChecksum,
                                                                frameType:
                                                                    LZ4F_frame,
                                                                contentSize:
                                                                    0,
                                                                dictID: 0,
                                                                blockChecksumFlag:
                                                                    LZ4F_noBlockChecksum,},
                                           compressionLevel: 0,
                                           autoFlush: 0,
                                           favorDecSpeed: 0,
                                           reserved: [0; 3],},
                    version: 0,
                    cStage: 0,
                    cdict: 0 as *const LZ4F_CDict,
                    maxBlockSize: 0,
                    maxBufferSize: 0,
                    tmpBuff: 0 as *mut BYTE,
                    tmpIn: 0 as *mut BYTE,
                    tmpInSize: 0,
                    totalInSize: 0,
                    xxh:
                        XXH32_state_t{total_len_32: 0,
                                      large_len: 0,
                                      v1: 0,
                                      v2: 0,
                                      v3: 0,
                                      v4: 0,
                                      mem32: [0; 4],
                                      memsize: 0,
                                      reserved: 0,},
                    lz4CtxPtr: 0 as *mut libc::c_void,
                    lz4CtxAlloc: 0,
                    lz4CtxState: 0,};
    let mut lz4ctx: LZ4_stream_t = LZ4_stream_u{table: [0; 2052],};
    let mut cctxPtr: *mut LZ4F_cctx_t = &mut cctx;
    memset(&mut cctx as *mut LZ4F_cctx_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<LZ4F_cctx_t>() as libc::c_ulong);
    cctx.version = 100 as libc::c_int as U32;
    cctx.maxBufferSize =
        (5 as libc::c_int * ((1 as libc::c_int) << 20 as libc::c_int)) as
            size_t;
    if preferencesPtr.is_null() ||
           (*preferencesPtr).compressionLevel < 3 as libc::c_int {
        LZ4_initStream(&mut lz4ctx as *mut LZ4_stream_t as *mut libc::c_void,
                       ::std::mem::size_of::<LZ4_stream_t>() as
                           libc::c_ulong);
        (*cctxPtr).lz4CtxPtr =
            &mut lz4ctx as *mut LZ4_stream_t as *mut libc::c_void;
        (*cctxPtr).lz4CtxAlloc = 1 as libc::c_int as U16;
        (*cctxPtr).lz4CtxState = 1 as libc::c_int as U16
    }
    result =
        LZ4F_compressFrame_usingCDict(cctxPtr, dstBuffer, dstCapacity,
                                      srcBuffer, srcSize,
                                      0 as *const LZ4F_CDict, preferencesPtr);
    if !preferencesPtr.is_null() &&
           (*preferencesPtr).compressionLevel >= 3 as libc::c_int {
        free((*cctxPtr).lz4CtxPtr);
    }
    return result;
}
/* typedef'd to LZ4F_CDict within lz4frame_static.h */
/* ! LZ4F_createCDict() :
 *  When compressing multiple messages / blocks with the same dictionary, it's recommended to load it just once.
 *  LZ4F_createCDict() will create a digested dictionary, ready to start future compression operations without startup delay.
 *  LZ4F_CDict can be created once and shared by multiple threads concurrently, since its usage is read-only.
 * `dictBuffer` can be released after LZ4F_CDict creation, since its content is copied within CDict
 * @return : digested dictionary for compression, or NULL if failed */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_createCDict(mut dictBuffer: *const libc::c_void,
                                          mut dictSize: size_t)
 -> *mut LZ4F_CDict {
    let mut dictStart: *const libc::c_char =
        dictBuffer as *const libc::c_char; /* support free on NULL */
    let mut cdict: *mut LZ4F_CDict =
        malloc(::std::mem::size_of::<LZ4F_CDict>() as libc::c_ulong) as
            *mut LZ4F_CDict;
    if cdict.is_null() { return 0 as *mut LZ4F_CDict }
    if dictSize >
           (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
               libc::c_ulong {
        dictStart =
            dictStart.offset(dictSize.wrapping_sub((64 as libc::c_int *
                                                        ((1 as libc::c_int) <<
                                                             10 as
                                                                 libc::c_int))
                                                       as libc::c_ulong) as
                                 isize);
        dictSize =
            (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
                size_t
    }
    (*cdict).dictContent = malloc(dictSize);
    (*cdict).fastCtx = LZ4_createStream();
    (*cdict).HCCtx = LZ4_createStreamHC();
    if (*cdict).dictContent.is_null() || (*cdict).fastCtx.is_null() ||
           (*cdict).HCCtx.is_null() {
        LZ4F_freeCDict(cdict);
        return 0 as *mut LZ4F_CDict
    }
    memcpy((*cdict).dictContent, dictStart as *const libc::c_void, dictSize);
    LZ4_loadDict((*cdict).fastCtx,
                 (*cdict).dictContent as *const libc::c_char,
                 dictSize as libc::c_int);
    LZ4_setCompressionLevel((*cdict).HCCtx, 9 as libc::c_int);
    LZ4_loadDictHC((*cdict).HCCtx,
                   (*cdict).dictContent as *const libc::c_char,
                   dictSize as libc::c_int);
    return cdict;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_freeCDict(mut cdict: *mut LZ4F_CDict) {
    if cdict.is_null() { return }
    free((*cdict).dictContent);
    LZ4_freeStream((*cdict).fastCtx);
    LZ4_freeStreamHC((*cdict).HCCtx);
    free(cdict as *mut libc::c_void);
}
/*-*********************************
*  Advanced compression functions
***********************************/
/* ! LZ4F_createCompressionContext() :
 *  The first thing to do is to create a compressionContext object, which will be used in all compression operations.
 *  This is achieved using LZ4F_createCompressionContext(), which takes as argument a version and an LZ4F_preferences_t structure.
 *  The version provided MUST be LZ4F_VERSION. It is intended to track potential incompatible differences between different binaries.
 *  The function will provide a pointer to an allocated LZ4F_compressionContext_t object.
 *  If the result LZ4F_errorCode_t is not OK_NoError, there was an error during context creation.
 *  Object can release its memory using LZ4F_freeCompressionContext();
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_createCompressionContext(mut LZ4F_compressionContextPtr:
                                                           *mut LZ4F_compressionContext_t,
                                                       mut version:
                                                           libc::c_uint)
 -> LZ4F_errorCode_t {
    let cctxPtr: *mut LZ4F_cctx_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<LZ4F_cctx_t>() as libc::c_ulong) as
            *mut LZ4F_cctx_t; /* Next stage : init stream */
    if cctxPtr.is_null() { return err0r(LZ4F_ERROR_allocation_failed) }
    (*cctxPtr).version = version;
    (*cctxPtr).cStage = 0 as libc::c_int as U32;
    *LZ4F_compressionContextPtr = cctxPtr as LZ4F_compressionContext_t;
    return LZ4F_OK_NoError as libc::c_int as LZ4F_errorCode_t;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_freeCompressionContext(mut LZ4F_compressionContext:
                                                         LZ4F_compressionContext_t)
 -> LZ4F_errorCode_t {
    let cctxPtr: *mut LZ4F_cctx_t =
        LZ4F_compressionContext as *mut LZ4F_cctx_t;
    if !cctxPtr.is_null() {
        /* support free on NULL */
        free((*cctxPtr).lz4CtxPtr); /* works because LZ4_streamHC_t and LZ4_stream_t are simple POD types */
        free((*cctxPtr).tmpBuff as *mut libc::c_void);
        free(LZ4F_compressionContext as *mut libc::c_void);
    }
    return LZ4F_OK_NoError as libc::c_int as LZ4F_errorCode_t;
}
/* *
 * This function prepares the internal LZ4(HC) stream for a new compression,
 * resetting the context and attaching the dictionary, if there is one.
 *
 * It needs to be called at the beginning of each independent compression
 * stream (i.e., at the beginning of a frame in blockLinked mode, or at the
 * beginning of each block in blockIndependent mode).
 */
unsafe extern "C" fn LZ4F_initStream(mut ctx: *mut libc::c_void,
                                     mut cdict: *const LZ4F_CDict,
                                     mut level: libc::c_int,
                                     mut blockMode: LZ4F_blockMode_t) {
    if level < 3 as libc::c_int {
        if !cdict.is_null() ||
               blockMode as libc::c_uint ==
                   LZ4F_blockLinked as libc::c_int as libc::c_uint {
            /* In these cases, we will call LZ4_compress_fast_continue(),
             * which needs an already reset context. Otherwise, we'll call a
             * one-shot API. The non-continued APIs internally perform their own
             * resets at the beginning of their calls, where they know what
             * tableType they need the context to be in. So in that case this
             * would be misguided / wasted work. */
            LZ4_resetStream_fast(ctx as *mut LZ4_stream_t);
        }
        LZ4_attach_dictionary(ctx as *mut LZ4_stream_t,
                              if !cdict.is_null() {
                                  (*cdict).fastCtx
                              } else { 0 as *mut LZ4_stream_t });
    } else {
        LZ4_resetStreamHC_fast(ctx as *mut LZ4_streamHC_t, level);
        LZ4_attach_HC_dictionary(ctx as *mut LZ4_streamHC_t,
                                 if !cdict.is_null() {
                                     (*cdict).HCCtx
                                 } else { 0 as *mut LZ4_streamHC_t });
    };
}
/* ! LZ4F_compressBegin_usingCDict() :
 *  init streaming compression and writes frame header into dstBuffer.
 *  dstBuffer must be >= LZ4F_HEADER_SIZE_MAX bytes.
 * @return : number of bytes written into dstBuffer for the header
 *           or an error code (can be tested using LZ4F_isError())
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressBegin_usingCDict(mut cctxPtr:
                                                           *mut LZ4F_cctx,
                                                       mut dstBuffer:
                                                           *mut libc::c_void,
                                                       mut dstCapacity:
                                                           size_t,
                                                       mut cdict:
                                                           *const LZ4F_CDict,
                                                       mut preferencesPtr:
                                                           *const LZ4F_preferences_t)
 -> size_t {
    let mut prefNull: LZ4F_preferences_t =
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
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let mut dstPtr: *mut BYTE = dstStart;
    let mut headerStart: *mut BYTE = 0 as *mut BYTE;
    if dstCapacity < maxFHSize {
        return err0r(LZ4F_ERROR_dstMaxSize_tooSmall)
    }
    memset(&mut prefNull as *mut LZ4F_preferences_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<LZ4F_preferences_t>() as libc::c_ulong);
    if preferencesPtr.is_null() { preferencesPtr = &mut prefNull }
    (*cctxPtr).prefs = *preferencesPtr;
    /* Ctx Management */
    let ctxTypeID: U16 =
        if (*cctxPtr).prefs.compressionLevel < 3 as libc::c_int {
            1 as libc::c_int
        } else { 2 as libc::c_int } as U16;
    if ((*cctxPtr).lz4CtxAlloc as libc::c_int) < ctxTypeID as libc::c_int {
        free((*cctxPtr).lz4CtxPtr);
        if (*cctxPtr).prefs.compressionLevel < 3 as libc::c_int {
            (*cctxPtr).lz4CtxPtr = LZ4_createStream() as *mut libc::c_void
        } else {
            (*cctxPtr).lz4CtxPtr = LZ4_createStreamHC() as *mut libc::c_void
        }
        if (*cctxPtr).lz4CtxPtr.is_null() {
            return err0r(LZ4F_ERROR_allocation_failed)
        }
        (*cctxPtr).lz4CtxAlloc = ctxTypeID;
        (*cctxPtr).lz4CtxState = ctxTypeID
    } else if (*cctxPtr).lz4CtxState as libc::c_int !=
                  ctxTypeID as libc::c_int {
        /* otherwise, a sufficient buffer is allocated, but we need to
             * reset it to the correct context type */
        if (*cctxPtr).prefs.compressionLevel < 3 as libc::c_int {
            LZ4_initStream((*cctxPtr).lz4CtxPtr as *mut LZ4_stream_t as
                               *mut libc::c_void,
                           ::std::mem::size_of::<LZ4_stream_t>() as
                               libc::c_ulong);
        } else {
            LZ4_initStreamHC((*cctxPtr).lz4CtxPtr as *mut LZ4_streamHC_t as
                                 *mut libc::c_void,
                             ::std::mem::size_of::<LZ4_streamHC_t>() as
                                 libc::c_ulong);
            LZ4_setCompressionLevel((*cctxPtr).lz4CtxPtr as
                                        *mut LZ4_streamHC_t,
                                    (*cctxPtr).prefs.compressionLevel);
        }
        (*cctxPtr).lz4CtxState = ctxTypeID
    }
    /* Buffer Management */
    if (*cctxPtr).prefs.frameInfo.blockSizeID as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        (*cctxPtr).prefs.frameInfo.blockSizeID = LZ4F_max64KB
    }
    (*cctxPtr).maxBlockSize =
        LZ4F_getBlockSize((*cctxPtr).prefs.frameInfo.blockSizeID as
                              libc::c_uint);
    let requiredBuffSize: size_t =
        if (*preferencesPtr).autoFlush != 0 {
            (if (*cctxPtr).prefs.frameInfo.blockMode as libc::c_uint ==
                    LZ4F_blockLinked as libc::c_int as libc::c_uint {
                 (64 as libc::c_int) *
                     ((1 as libc::c_int) << 10 as libc::c_int)
             } else { 0 as libc::c_int }) as libc::c_ulong
        } else {
            (*cctxPtr).maxBlockSize.wrapping_add((if (*cctxPtr).prefs.frameInfo.blockMode
                                                         as libc::c_uint ==
                                                         LZ4F_blockLinked as
                                                             libc::c_int as
                                                             libc::c_uint {
                                                      (128 as libc::c_int) *
                                                          ((1 as libc::c_int)
                                                               <<
                                                               10 as
                                                                   libc::c_int)
                                                  } else { 0 as libc::c_int })
                                                     as libc::c_ulong)
        };
    if (*cctxPtr).maxBufferSize < requiredBuffSize {
        (*cctxPtr).maxBufferSize = 0 as libc::c_int as size_t;
        free((*cctxPtr).tmpBuff as *mut libc::c_void);
        (*cctxPtr).tmpBuff =
            calloc(1 as libc::c_int as libc::c_ulong, requiredBuffSize) as
                *mut BYTE;
        if (*cctxPtr).tmpBuff.is_null() {
            return err0r(LZ4F_ERROR_allocation_failed)
        }
        (*cctxPtr).maxBufferSize = requiredBuffSize
    }
    (*cctxPtr).tmpIn = (*cctxPtr).tmpBuff;
    (*cctxPtr).tmpInSize = 0 as libc::c_int as size_t;
    LZ4_XXH32_reset(&mut (*cctxPtr).xxh, 0 as libc::c_int as libc::c_uint);
    /* context init */
    (*cctxPtr).cdict = cdict;
    if (*cctxPtr).prefs.frameInfo.blockMode as libc::c_uint ==
           LZ4F_blockLinked as libc::c_int as libc::c_uint {
        /* frame init only for blockLinked : blockIndependent will be init at each block */
        LZ4F_initStream((*cctxPtr).lz4CtxPtr, cdict,
                        (*cctxPtr).prefs.compressionLevel, LZ4F_blockLinked);
    }
    if (*preferencesPtr).compressionLevel >= 3 as libc::c_int {
        LZ4_favorDecompressionSpeed((*cctxPtr).lz4CtxPtr as
                                        *mut LZ4_streamHC_t,
                                    (*preferencesPtr).favorDecSpeed as
                                        libc::c_int);
    }
    /* Magic Number */
    LZ4F_writeLE32(dstPtr as *mut libc::c_void, 0x184d2204 as libc::c_uint);
    dstPtr = dstPtr.offset(4 as libc::c_int as isize);
    headerStart = dstPtr;
    /* FLG Byte */
    let fresh0 = dstPtr;
    dstPtr = dstPtr.offset(1);
    *fresh0 =
        (((1 as libc::c_int & 0x3 as libc::c_int) << 6 as libc::c_int) as
             libc::c_uint).wrapping_add(((*cctxPtr).prefs.frameInfo.blockMode
                                             as libc::c_uint &
                                             0x1 as libc::c_int as
                                                 libc::c_uint) <<
                                            5 as
                                                libc::c_int).wrapping_add(((*cctxPtr).prefs.frameInfo.blockChecksumFlag
                                                                               as
                                                                               libc::c_uint
                                                                               &
                                                                               0x1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                                                                              <<
                                                                              4
                                                                                  as
                                                                                  libc::c_int).wrapping_add((((*cctxPtr).prefs.frameInfo.contentSize
                                                                                                                  >
                                                                                                                  0
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulonglong)
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint)
                                                                                                                <<
                                                                                                                3
                                                                                                                    as
                                                                                                                    libc::c_int).wrapping_add(((*cctxPtr).prefs.frameInfo.contentChecksumFlag
                                                                                                                                                   as
                                                                                                                                                   libc::c_uint
                                                                                                                                                   &
                                                                                                                                                   0x1
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint)
                                                                                                                                                  <<
                                                                                                                                                  2
                                                                                                                                                      as
                                                                                                                                                      libc::c_int).wrapping_add(((*cctxPtr).prefs.frameInfo.dictID
                                                                                                                                                                                     >
                                                                                                                                                                                     0
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_uint)
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_uint)
            as BYTE;
    /* BD Byte */
    let fresh1 = dstPtr;
    dstPtr = dstPtr.offset(1);
    *fresh1 =
        (((*cctxPtr).prefs.frameInfo.blockSizeID as libc::c_uint &
              0x7 as libc::c_int as libc::c_uint) << 4 as libc::c_int) as
            BYTE;
    /* Optional Frame content size field */
    if (*cctxPtr).prefs.frameInfo.contentSize != 0 {
        LZ4F_writeLE64(dstPtr as *mut libc::c_void,
                       (*cctxPtr).prefs.frameInfo.contentSize as U64);
        dstPtr = dstPtr.offset(8 as libc::c_int as isize);
        (*cctxPtr).totalInSize = 0 as libc::c_int as U64
    }
    /* Optional dictionary ID field */
    if (*cctxPtr).prefs.frameInfo.dictID != 0 {
        LZ4F_writeLE32(dstPtr as *mut libc::c_void,
                       (*cctxPtr).prefs.frameInfo.dictID);
        dstPtr = dstPtr.offset(4 as libc::c_int as isize)
    }
    /* Header CRC Byte */
    *dstPtr =
        LZ4F_headerChecksum(headerStart as *const libc::c_void,
                            dstPtr.wrapping_offset_from(headerStart) as
                                libc::c_long as
                                size_t); /* header written, now request input data block */
    dstPtr = dstPtr.offset(1);
    (*cctxPtr).cStage = 1 as libc::c_int as U32;
    return dstPtr.wrapping_offset_from(dstStart) as libc::c_long as size_t;
}
/* ! LZ4F_compressBegin() :
 *  init streaming compression and writes frame header into dstBuffer.
 *  dstBuffer must be >= LZ4F_HEADER_SIZE_MAX bytes.
 *  preferencesPtr can be NULL, in which case default parameters are selected.
 * @return : number of bytes written into dstBuffer for the header
 *        or an error code (can be tested using LZ4F_isError())
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressBegin(mut cctxPtr: *mut LZ4F_cctx,
                                            mut dstBuffer: *mut libc::c_void,
                                            mut dstCapacity: size_t,
                                            mut preferencesPtr:
                                                *const LZ4F_preferences_t)
 -> size_t {
    return LZ4F_compressBegin_usingCDict(cctxPtr, dstBuffer, dstCapacity,
                                         0 as *const LZ4F_CDict,
                                         preferencesPtr);
}
/*  LZ4F_compressBound() :
 * @return minimum capacity of dstBuffer for a given srcSize to handle worst case scenario.
 *  LZ4F_preferences_t structure is optional : if NULL, preferences will be set to cover worst case scenario.
 *  This function cannot fail.
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressBound(mut srcSize: size_t,
                                            mut preferencesPtr:
                                                *const LZ4F_preferences_t)
 -> size_t {
    return LZ4F_compressBound_internal(srcSize, preferencesPtr,
                                       -(1 as libc::c_int) as size_t);
}
/* ! LZ4F_makeBlock():
 *  compress a single block, add header and optional checksum.
 *  assumption : dst buffer capacity is >= BHSize + srcSize + crcSize
 */
unsafe extern "C" fn LZ4F_makeBlock(mut dst: *mut libc::c_void,
                                    mut src: *const libc::c_void,
                                    mut srcSize: size_t,
                                    mut compress: compressFunc_t,
                                    mut lz4ctx: *mut libc::c_void,
                                    mut level: libc::c_int,
                                    mut cdict: *const LZ4F_CDict,
                                    mut crcFlag: LZ4F_blockChecksum_t)
 -> size_t {
    let cSizePtr: *mut BYTE = dst as *mut BYTE;
    let mut cSize: U32 =
        compress.expect("non-null function pointer")(lz4ctx,
                                                     src as
                                                         *const libc::c_char,
                                                     cSizePtr.offset(BHSize as
                                                                         isize)
                                                         as *mut libc::c_char,
                                                     srcSize as libc::c_int,
                                                     srcSize.wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                                                         as libc::c_int,
                                                     level, cdict) as U32;
    if cSize == 0 as libc::c_int as libc::c_uint {
        /* compression failed */
        cSize = srcSize as U32; /* checksum of compressed data */
        LZ4F_writeLE32(cSizePtr as *mut libc::c_void,
                       cSize | 0x80000000 as libc::c_uint);
        memcpy(cSizePtr.offset(BHSize as isize) as *mut libc::c_void, src,
               srcSize);
    } else { LZ4F_writeLE32(cSizePtr as *mut libc::c_void, cSize); }
    if crcFlag as u64 != 0 {
        let crc32: U32 =
            LZ4_XXH32(cSizePtr.offset(BHSize as isize) as *const libc::c_void,
                      cSize as size_t, 0 as libc::c_int as libc::c_uint);
        LZ4F_writeLE32(cSizePtr.offset(BHSize as isize).offset(cSize as isize)
                           as *mut libc::c_void, crc32);
    }
    return BHSize.wrapping_add(cSize as
                                   libc::c_ulong).wrapping_add((crcFlag as U32
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(BFSize));
}
unsafe extern "C" fn LZ4F_compressBlock(mut ctx: *mut libc::c_void,
                                        mut src: *const libc::c_char,
                                        mut dst: *mut libc::c_char,
                                        mut srcSize: libc::c_int,
                                        mut dstCapacity: libc::c_int,
                                        mut level: libc::c_int,
                                        mut cdict: *const LZ4F_CDict)
 -> libc::c_int {
    let acceleration: libc::c_int =
        if level < 0 as libc::c_int {
            (-level) + 1 as libc::c_int
        } else { 1 as libc::c_int };
    LZ4F_initStream(ctx, cdict, level, LZ4F_blockIndependent);
    if !cdict.is_null() {
        return LZ4_compress_fast_continue(ctx as *mut LZ4_stream_t, src, dst,
                                          srcSize, dstCapacity, acceleration)
    } else {
        return LZ4_compress_fast_extState_fastReset(ctx, src, dst, srcSize,
                                                    dstCapacity, acceleration)
    };
}
unsafe extern "C" fn LZ4F_compressBlock_continue(mut ctx: *mut libc::c_void,
                                                 mut src: *const libc::c_char,
                                                 mut dst: *mut libc::c_char,
                                                 mut srcSize: libc::c_int,
                                                 mut dstCapacity: libc::c_int,
                                                 mut level: libc::c_int,
                                                 mut cdict: *const LZ4F_CDict)
 -> libc::c_int {
    let acceleration: libc::c_int =
        if level < 0 as libc::c_int {
            (-level) + 1 as libc::c_int
        } else { 1 as libc::c_int };
    /* init once at beginning of frame */
    return LZ4_compress_fast_continue(ctx as *mut LZ4_stream_t, src, dst,
                                      srcSize, dstCapacity, acceleration);
}
unsafe extern "C" fn LZ4F_compressBlockHC(mut ctx: *mut libc::c_void,
                                          mut src: *const libc::c_char,
                                          mut dst: *mut libc::c_char,
                                          mut srcSize: libc::c_int,
                                          mut dstCapacity: libc::c_int,
                                          mut level: libc::c_int,
                                          mut cdict: *const LZ4F_CDict)
 -> libc::c_int {
    LZ4F_initStream(ctx, cdict, level, LZ4F_blockIndependent);
    if !cdict.is_null() {
        return LZ4_compress_HC_continue(ctx as *mut LZ4_streamHC_t, src, dst,
                                        srcSize, dstCapacity)
    }
    return LZ4_compress_HC_extStateHC_fastReset(ctx, src, dst, srcSize,
                                                dstCapacity, level);
}
unsafe extern "C" fn LZ4F_compressBlockHC_continue(mut ctx: *mut libc::c_void,
                                                   mut src:
                                                       *const libc::c_char,
                                                   mut dst: *mut libc::c_char,
                                                   mut srcSize: libc::c_int,
                                                   mut dstCapacity:
                                                       libc::c_int,
                                                   mut level: libc::c_int,
                                                   mut cdict:
                                                       *const LZ4F_CDict)
 -> libc::c_int {
    /* init once at beginning of frame */
    return LZ4_compress_HC_continue(ctx as *mut LZ4_streamHC_t, src, dst,
                                    srcSize, dstCapacity);
}
unsafe extern "C" fn LZ4F_selectCompression(mut blockMode: LZ4F_blockMode_t,
                                            mut level: libc::c_int)
 -> compressFunc_t {
    if level < 3 as libc::c_int {
        if blockMode as libc::c_uint ==
               LZ4F_blockIndependent as libc::c_int as libc::c_uint {
            return Some(LZ4F_compressBlock as
                            unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: *const libc::c_char,
                                                 _: *mut libc::c_char,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: libc::c_int,
                                                 _: *const LZ4F_CDict)
                                -> libc::c_int)
        }
        return Some(LZ4F_compressBlock_continue as
                        unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: *const libc::c_char,
                                             _: *mut libc::c_char,
                                             _: libc::c_int, _: libc::c_int,
                                             _: libc::c_int,
                                             _: *const LZ4F_CDict)
                            -> libc::c_int)
    }
    if blockMode as libc::c_uint ==
           LZ4F_blockIndependent as libc::c_int as libc::c_uint {
        return Some(LZ4F_compressBlockHC as
                        unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: *const libc::c_char,
                                             _: *mut libc::c_char,
                                             _: libc::c_int, _: libc::c_int,
                                             _: libc::c_int,
                                             _: *const LZ4F_CDict)
                            -> libc::c_int)
    }
    return Some(LZ4F_compressBlockHC_continue as
                    unsafe extern "C" fn(_: *mut libc::c_void,
                                         _: *const libc::c_char,
                                         _: *mut libc::c_char, _: libc::c_int,
                                         _: libc::c_int, _: libc::c_int,
                                         _: *const LZ4F_CDict)
                        -> libc::c_int);
}
unsafe extern "C" fn LZ4F_localSaveDict(mut cctxPtr: *mut LZ4F_cctx_t)
 -> libc::c_int {
    if (*cctxPtr).prefs.compressionLevel < 3 as libc::c_int {
        return LZ4_saveDict((*cctxPtr).lz4CtxPtr as *mut LZ4_stream_t,
                            (*cctxPtr).tmpBuff as *mut libc::c_char,
                            64 as libc::c_int *
                                ((1 as libc::c_int) << 10 as libc::c_int))
    }
    return LZ4_saveDictHC((*cctxPtr).lz4CtxPtr as *mut LZ4_streamHC_t,
                          (*cctxPtr).tmpBuff as *mut libc::c_char,
                          64 as libc::c_int *
                              ((1 as libc::c_int) << 10 as libc::c_int));
}
/* ! LZ4F_compressUpdate() :
 *  LZ4F_compressUpdate() can be called repetitively to compress as much data as necessary.
 *  dstBuffer MUST be >= LZ4F_compressBound(srcSize, preferencesPtr).
 *  LZ4F_compressOptions_t structure is optional : you can provide NULL as argument.
 * @return : the number of bytes written into dstBuffer. It can be zero, meaning input data was just buffered.
 *           or an error code if it fails (which can be tested using LZ4F_isError())
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressUpdate(mut cctxPtr: *mut LZ4F_cctx,
                                             mut dstBuffer: *mut libc::c_void,
                                             mut dstCapacity: size_t,
                                             mut srcBuffer:
                                                 *const libc::c_void,
                                             mut srcSize: size_t,
                                             mut compressOptionsPtr:
                                                 *const LZ4F_compressOptions_t)
 -> size_t {
    let mut cOptionsNull: LZ4F_compressOptions_t =
        LZ4F_compressOptions_t{stableSrc: 0, reserved: [0; 3],};
    let blockSize: size_t = (*cctxPtr).maxBlockSize;
    let mut srcPtr: *const BYTE = srcBuffer as *const BYTE;
    let srcEnd: *const BYTE = srcPtr.offset(srcSize as isize);
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let mut dstPtr: *mut BYTE = dstStart;
    let mut lastBlockCompressed: LZ4F_lastBlockStatus = notDone;
    let compress: compressFunc_t =
        LZ4F_selectCompression((*cctxPtr).prefs.frameInfo.blockMode,
                               (*cctxPtr).prefs.compressionLevel);
    if (*cctxPtr).cStage != 1 as libc::c_int as libc::c_uint {
        return err0r(LZ4F_ERROR_GENERIC)
    }
    if dstCapacity <
           LZ4F_compressBound_internal(srcSize, &mut (*cctxPtr).prefs,
                                       (*cctxPtr).tmpInSize) {
        return err0r(LZ4F_ERROR_dstMaxSize_tooSmall)
    }
    memset(&mut cOptionsNull as *mut LZ4F_compressOptions_t as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<LZ4F_compressOptions_t>() as libc::c_ulong);
    if compressOptionsPtr.is_null() { compressOptionsPtr = &mut cOptionsNull }
    /* complete tmp buffer */
    if (*cctxPtr).tmpInSize > 0 as libc::c_int as libc::c_ulong {
        /* some data already within tmp buffer */
        let sizeToCopy: size_t = blockSize.wrapping_sub((*cctxPtr).tmpInSize);
        if sizeToCopy > srcSize {
            /* add src to tmpIn buffer */
            memcpy((*cctxPtr).tmpIn.offset((*cctxPtr).tmpInSize as isize) as
                       *mut libc::c_void, srcBuffer, srcSize);
            srcPtr = srcEnd;
            (*cctxPtr).tmpInSize =
                ((*cctxPtr).tmpInSize as libc::c_ulong).wrapping_add(srcSize)
                    as size_t as size_t
            /* still needs some CRC */
        } else {
            /* complete tmpIn block and then compress it */
            lastBlockCompressed = fromTmpBuffer;
            memcpy((*cctxPtr).tmpIn.offset((*cctxPtr).tmpInSize as isize) as
                       *mut libc::c_void, srcBuffer, sizeToCopy);
            srcPtr = srcPtr.offset(sizeToCopy as isize);
            dstPtr =
                dstPtr.offset(LZ4F_makeBlock(dstPtr as *mut libc::c_void,
                                             (*cctxPtr).tmpIn as
                                                 *const libc::c_void,
                                             blockSize, compress,
                                             (*cctxPtr).lz4CtxPtr,
                                             (*cctxPtr).prefs.compressionLevel,
                                             (*cctxPtr).cdict,
                                             (*cctxPtr).prefs.frameInfo.blockChecksumFlag)
                                  as isize);
            if (*cctxPtr).prefs.frameInfo.blockMode as libc::c_uint ==
                   LZ4F_blockLinked as libc::c_int as libc::c_uint {
                (*cctxPtr).tmpIn = (*cctxPtr).tmpIn.offset(blockSize as isize)
            }
            (*cctxPtr).tmpInSize = 0 as libc::c_int as size_t
        }
    }
    while srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as size_t >=
              blockSize {
        /* compress full blocks */
        lastBlockCompressed = fromSrcBuffer;
        dstPtr =
            dstPtr.offset(LZ4F_makeBlock(dstPtr as *mut libc::c_void,
                                         srcPtr as *const libc::c_void,
                                         blockSize, compress,
                                         (*cctxPtr).lz4CtxPtr,
                                         (*cctxPtr).prefs.compressionLevel,
                                         (*cctxPtr).cdict,
                                         (*cctxPtr).prefs.frameInfo.blockChecksumFlag)
                              as isize);
        srcPtr = srcPtr.offset(blockSize as isize)
    }
    if (*cctxPtr).prefs.autoFlush != 0 && srcPtr < srcEnd {
        /* compress remaining input < blockSize */
        lastBlockCompressed = fromSrcBuffer;
        dstPtr =
            dstPtr.offset(LZ4F_makeBlock(dstPtr as *mut libc::c_void,
                                         srcPtr as *const libc::c_void,
                                         srcEnd.wrapping_offset_from(srcPtr)
                                             as libc::c_long as size_t,
                                         compress, (*cctxPtr).lz4CtxPtr,
                                         (*cctxPtr).prefs.compressionLevel,
                                         (*cctxPtr).cdict,
                                         (*cctxPtr).prefs.frameInfo.blockChecksumFlag)
                              as isize);
        srcPtr = srcEnd
    }
    /* preserve dictionary if necessary */
    if (*cctxPtr).prefs.frameInfo.blockMode as libc::c_uint ==
           LZ4F_blockLinked as libc::c_int as libc::c_uint &&
           lastBlockCompressed as libc::c_uint ==
               fromSrcBuffer as libc::c_int as libc::c_uint {
        if (*compressOptionsPtr).stableSrc != 0 {
            (*cctxPtr).tmpIn = (*cctxPtr).tmpBuff
        } else {
            let realDictSize: libc::c_int = LZ4F_localSaveDict(cctxPtr);
            if realDictSize == 0 as libc::c_int {
                return err0r(LZ4F_ERROR_GENERIC)
            }
            (*cctxPtr).tmpIn =
                (*cctxPtr).tmpBuff.offset(realDictSize as isize)
        }
    }
    /* keep tmpIn within limits */
    if (*cctxPtr).tmpIn.offset(blockSize as isize) >
           (*cctxPtr).tmpBuff.offset((*cctxPtr).maxBufferSize as isize) &&
           (*cctxPtr).prefs.autoFlush == 0 {
        let realDictSize_0: libc::c_int = LZ4F_localSaveDict(cctxPtr);
        (*cctxPtr).tmpIn = (*cctxPtr).tmpBuff.offset(realDictSize_0 as isize)
    }
    /* some input data left, necessarily < blockSize */
    if srcPtr < srcEnd {
        /* fill tmp buffer */
        let sizeToCopy_0: size_t =
            srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as size_t;
        memcpy((*cctxPtr).tmpIn as *mut libc::c_void,
               srcPtr as *const libc::c_void, sizeToCopy_0);
        (*cctxPtr).tmpInSize = sizeToCopy_0
    }
    if (*cctxPtr).prefs.frameInfo.contentChecksumFlag as libc::c_uint ==
           LZ4F_contentChecksumEnabled as libc::c_int as libc::c_uint {
        LZ4_XXH32_update(&mut (*cctxPtr).xxh, srcBuffer, srcSize);
    }
    (*cctxPtr).totalInSize =
        ((*cctxPtr).totalInSize as libc::c_ulong).wrapping_add(srcSize) as U64
            as U64;
    return dstPtr.wrapping_offset_from(dstStart) as libc::c_long as size_t;
}
/* ! LZ4F_flush() :
 *  When compressed data must be sent immediately, without waiting for a block to be filled,
 *  invoke LZ4_flush(), which will immediately compress any remaining data stored within LZ4F_cctx.
 *  The result of the function is the number of bytes written into dstBuffer.
 *  It can be zero, this means there was no data left within LZ4F_cctx.
 *  The function outputs an error code if it fails (can be tested using LZ4F_isError())
 *  LZ4F_compressOptions_t* is optional. NULL is a valid argument.
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_flush(mut cctxPtr: *mut LZ4F_cctx,
                                    mut dstBuffer: *mut libc::c_void,
                                    mut dstCapacity: size_t,
                                    mut compressOptionsPtr:
                                        *const LZ4F_compressOptions_t)
 -> size_t {
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE; /* nothing to flush */
    let mut dstPtr: *mut BYTE = dstStart;
    let mut compress: compressFunc_t = None;
    if (*cctxPtr).tmpInSize == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t
    }
    if (*cctxPtr).cStage != 1 as libc::c_int as libc::c_uint {
        return err0r(LZ4F_ERROR_GENERIC)
    }
    if dstCapacity <
           (*cctxPtr).tmpInSize.wrapping_add(BHSize).wrapping_add(BFSize) {
        return err0r(LZ4F_ERROR_dstMaxSize_tooSmall)
    }
    /* not yet useful */
    /* select compression function */
    compress =
        LZ4F_selectCompression((*cctxPtr).prefs.frameInfo.blockMode,
                               (*cctxPtr).prefs.compressionLevel);
    /* compress tmp buffer */
    dstPtr =
        dstPtr.offset(LZ4F_makeBlock(dstPtr as *mut libc::c_void,
                                     (*cctxPtr).tmpIn as *const libc::c_void,
                                     (*cctxPtr).tmpInSize, compress,
                                     (*cctxPtr).lz4CtxPtr,
                                     (*cctxPtr).prefs.compressionLevel,
                                     (*cctxPtr).cdict,
                                     (*cctxPtr).prefs.frameInfo.blockChecksumFlag)
                          as isize);
    if (*cctxPtr).prefs.frameInfo.blockMode as libc::c_uint ==
           LZ4F_blockLinked as libc::c_int as libc::c_uint {
        (*cctxPtr).tmpIn =
            (*cctxPtr).tmpIn.offset((*cctxPtr).tmpInSize as isize)
    }
    (*cctxPtr).tmpInSize = 0 as libc::c_int as size_t;
    /* keep tmpIn within limits */
    if (*cctxPtr).tmpIn.offset((*cctxPtr).maxBlockSize as isize) >
           (*cctxPtr).tmpBuff.offset((*cctxPtr).maxBufferSize as isize) {
        /* necessarily LZ4F_blockLinked */
        let realDictSize: libc::c_int = LZ4F_localSaveDict(cctxPtr);
        (*cctxPtr).tmpIn = (*cctxPtr).tmpBuff.offset(realDictSize as isize)
    }
    return dstPtr.wrapping_offset_from(dstStart) as libc::c_long as size_t;
}
/* ! LZ4F_compressEnd() :
 *  When you want to properly finish the compressed frame, just call LZ4F_compressEnd().
 *  It will flush whatever data remained within compressionContext (like LZ4_flush())
 *  but also properly finalize the frame, with an endMark and an (optional) checksum.
 *  LZ4F_compressOptions_t structure is optional : you can provide NULL as argument.
 * @return: the number of bytes written into dstBuffer (necessarily >= 4 (endMark size))
 *       or an error code if it fails (can be tested using LZ4F_isError())
 *  The context can then be used again to compress a new frame, starting with LZ4F_compressBegin().
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_compressEnd(mut cctxPtr: *mut LZ4F_cctx,
                                          mut dstBuffer: *mut libc::c_void,
                                          mut dstCapacity: size_t,
                                          mut compressOptionsPtr:
                                              *const LZ4F_compressOptions_t)
 -> size_t {
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE; /* endMark */
    let mut dstPtr: *mut BYTE = dstStart;
    let flushSize: size_t =
        LZ4F_flush(cctxPtr, dstBuffer, dstCapacity, compressOptionsPtr);
    if LZ4F_isError(flushSize) != 0 { return flushSize }
    dstPtr = dstPtr.offset(flushSize as isize);
    dstCapacity =
        (dstCapacity as libc::c_ulong).wrapping_sub(flushSize) as size_t as
            size_t;
    if dstCapacity < 4 as libc::c_int as libc::c_ulong {
        return err0r(LZ4F_ERROR_dstMaxSize_tooSmall)
    }
    LZ4F_writeLE32(dstPtr as *mut libc::c_void, 0 as libc::c_int as U32);
    dstPtr = dstPtr.offset(4 as libc::c_int as isize);
    if (*cctxPtr).prefs.frameInfo.contentChecksumFlag as libc::c_uint ==
           LZ4F_contentChecksumEnabled as libc::c_int as libc::c_uint {
        let xxh: U32 = LZ4_XXH32_digest(&mut (*cctxPtr).xxh);
        if dstCapacity < 8 as libc::c_int as libc::c_ulong {
            return err0r(LZ4F_ERROR_dstMaxSize_tooSmall)
        }
        LZ4F_writeLE32(dstPtr as *mut libc::c_void, xxh);
        dstPtr = dstPtr.offset(4 as libc::c_int as isize)
        /* content Checksum */
    } /* state is now re-usable (with identical preferences) */
    (*cctxPtr).cStage = 0 as libc::c_int as U32; /* reuse HC context */
    (*cctxPtr).maxBufferSize = 0 as libc::c_int as size_t;
    if (*cctxPtr).prefs.frameInfo.contentSize != 0 {
        if (*cctxPtr).prefs.frameInfo.contentSize !=
               (*cctxPtr).totalInSize as libc::c_ulonglong {
            return err0r(LZ4F_ERROR_frameSize_wrong)
        }
    }
    return dstPtr.wrapping_offset_from(dstStart) as libc::c_long as size_t;
}
/* typedef'd to LZ4F_dctx in lz4frame.h */
/* ! LZ4F_createDecompressionContext() :
 *  Create a decompressionContext object, which will track all decompression operations.
 *  Provides a pointer to a fully allocated and initialized LZ4F_decompressionContext object.
 *  Object can later be released using LZ4F_freeDecompressionContext().
 * @return : if != 0, there was an error during context creation.
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_createDecompressionContext(mut LZ4F_decompressionContextPtr:
                                                             *mut *mut LZ4F_dctx,
                                                         mut versionNumber:
                                                             libc::c_uint)
 -> LZ4F_errorCode_t {
    let dctx: *mut LZ4F_dctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<LZ4F_dctx>() as libc::c_ulong) as
            *mut LZ4F_dctx;
    if dctx.is_null() {
        /* failed allocation */
        *LZ4F_decompressionContextPtr = 0 as *mut LZ4F_dctx;
        return err0r(LZ4F_ERROR_allocation_failed)
    }
    (*dctx).version = versionNumber;
    *LZ4F_decompressionContextPtr = dctx;
    return LZ4F_OK_NoError as libc::c_int as LZ4F_errorCode_t;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_freeDecompressionContext(mut dctx:
                                                           *mut LZ4F_dctx)
 -> LZ4F_errorCode_t {
    let mut result: LZ4F_errorCode_t =
        LZ4F_OK_NoError as libc::c_int as LZ4F_errorCode_t;
    if !dctx.is_null() {
        /* can accept NULL input, like free() */
        result = (*dctx).dStage as LZ4F_errorCode_t;
        free((*dctx).tmpIn as *mut libc::c_void);
        free((*dctx).tmpOutBuffer as *mut libc::c_void);
        free(dctx as *mut libc::c_void);
    }
    return result;
}
/*==---   Streaming Decompression operations   ---==*/
#[no_mangle]
pub unsafe extern "C" fn LZ4F_resetDecompressionContext(mut dctx:
                                                            *mut LZ4F_dctx) {
    (*dctx).dStage = dstage_getFrameHeader;
    (*dctx).dict = 0 as *const BYTE;
    (*dctx).dictSize = 0 as libc::c_int as size_t;
}
/* ! LZ4F_decodeHeader() :
 *  input   : `src` points at the **beginning of the frame**
 *  output  : set internal values of dctx, such as
 *            dctx->frameInfo and dctx->dStage.
 *            Also allocates internal buffers.
 *  @return : nb Bytes read from src (necessarily <= srcSize)
 *            or an error code (testable with LZ4F_isError())
 */
unsafe extern "C" fn LZ4F_decodeHeader(mut dctx: *mut LZ4F_dctx,
                                       mut src: *const libc::c_void,
                                       mut srcSize: size_t) -> size_t {
    let mut blockMode: libc::c_uint = 0;
    let mut blockChecksumFlag: libc::c_uint = 0;
    let mut contentSizeFlag: libc::c_uint = 0;
    let mut contentChecksumFlag: libc::c_uint = 0;
    let mut dictIDFlag: libc::c_uint = 0;
    let mut blockSizeID: libc::c_uint = 0;
    let mut frameHeaderSize: size_t = 0;
    let mut srcPtr: *const BYTE = src as *const BYTE;
    /* need to decode header to get frameInfo */
    if srcSize < minFHSize {
        return err0r(LZ4F_ERROR_frameHeader_incomplete)
    } /* minimal frame header size */
    memset(&mut (*dctx).frameInfo as *mut LZ4F_frameInfo_t as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<LZ4F_frameInfo_t>() as libc::c_ulong);
    /* special case : skippable frames */
    if LZ4F_readLE32(srcPtr as *const libc::c_void) &
           0xfffffff0 as libc::c_uint == 0x184d2a50 as libc::c_uint {
        (*dctx).frameInfo.frameType = LZ4F_skippableFrame;
        if src ==
               (*dctx).header.as_mut_ptr() as *mut libc::c_void as
                   *const libc::c_void {
            (*dctx).tmpInSize = srcSize;
            (*dctx).tmpInTarget = 8 as libc::c_int as size_t;
            (*dctx).dStage = dstage_storeSFrameSize;
            return srcSize
        } else {
            (*dctx).dStage = dstage_getSFrameSize;
            return 4 as libc::c_int as size_t
        }
    }
    /* control magic number */
    if LZ4F_readLE32(srcPtr as *const libc::c_void) !=
           0x184d2204 as libc::c_uint {
        return err0r(LZ4F_ERROR_frameType_unknown)
    }
    (*dctx).frameInfo.frameType = LZ4F_frame;
    /* Flags */
    let FLG: U32 = *srcPtr.offset(4 as libc::c_int as isize) as U32;
    let version: U32 =
        FLG >> 6 as libc::c_int & 0x3 as libc::c_int as libc::c_uint;
    blockChecksumFlag =
        FLG >> 4 as libc::c_int & 0x1 as libc::c_int as libc::c_uint;
    blockMode = FLG >> 5 as libc::c_int & 0x1 as libc::c_int as libc::c_uint;
    contentSizeFlag =
        FLG >> 3 as libc::c_int & 0x1 as libc::c_int as libc::c_uint;
    contentChecksumFlag =
        FLG >> 2 as libc::c_int & 0x1 as libc::c_int as libc::c_uint;
    dictIDFlag = FLG & 0x1 as libc::c_int as libc::c_uint;
    /* Version Number, only supported value */
    if FLG >> 1 as libc::c_int & 0x1 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        return err0r(LZ4F_ERROR_reservedFlag_set)
    }
    if version != 1 as libc::c_int as libc::c_uint {
        return err0r(LZ4F_ERROR_headerVersion_wrong)
    }
    /* validate */
    /* Reserved bit */
    /* Frame Header Size */
    frameHeaderSize =
        minFHSize.wrapping_add((if contentSizeFlag != 0 {
                                    8 as libc::c_int
                                } else { 0 as libc::c_int }) as
                                   libc::c_ulong).wrapping_add((if dictIDFlag
                                                                       != 0 {
                                                                    4 as
                                                                        libc::c_int
                                                                } else {
                                                                    0 as
                                                                        libc::c_int
                                                                }) as
                                                                   libc::c_ulong);
    if srcSize < frameHeaderSize {
        /* not enough input to fully decode frame header */
        if srcPtr != (*dctx).header.as_mut_ptr() as *const BYTE {
            memcpy((*dctx).header.as_mut_ptr() as *mut libc::c_void,
                   srcPtr as *const libc::c_void, srcSize);
        }
        (*dctx).tmpInSize = srcSize;
        (*dctx).tmpInTarget = frameHeaderSize;
        (*dctx).dStage = dstage_storeFrameHeader;
        return srcSize
    }
    let BD: U32 = *srcPtr.offset(5 as libc::c_int as isize) as U32;
    blockSizeID = BD >> 4 as libc::c_int & 0x7 as libc::c_int as libc::c_uint;
    /* Reserved bits */
    if BD >> 7 as libc::c_int & 0x1 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        return err0r(LZ4F_ERROR_reservedFlag_set)
    }
    if blockSizeID < 4 as libc::c_int as libc::c_uint {
        return err0r(LZ4F_ERROR_maxBlockSize_invalid)
    }
    if BD >> 0 as libc::c_int & 0xf as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        return err0r(LZ4F_ERROR_reservedFlag_set)
    }
    /* validate */
    /* Reserved bit */
    /* 4-7 only supported values for the time being */
    /* check header */
    let HC: BYTE =
        LZ4F_headerChecksum(srcPtr.offset(4 as libc::c_int as isize) as
                                *const libc::c_void,
                            frameHeaderSize.wrapping_sub(5 as libc::c_int as
                                                             libc::c_ulong));
    if HC as libc::c_int !=
           *srcPtr.offset(frameHeaderSize.wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong) as
                              isize) as libc::c_int {
        return err0r(LZ4F_ERROR_headerChecksum_invalid)
    }
    /* save */
    (*dctx).frameInfo.blockMode = blockMode as LZ4F_blockMode_t;
    (*dctx).frameInfo.blockChecksumFlag =
        blockChecksumFlag as LZ4F_blockChecksum_t;
    (*dctx).frameInfo.contentChecksumFlag =
        contentChecksumFlag as LZ4F_contentChecksum_t;
    (*dctx).frameInfo.blockSizeID = blockSizeID as LZ4F_blockSizeID_t;
    (*dctx).maxBlockSize = LZ4F_getBlockSize(blockSizeID);
    if contentSizeFlag != 0 {
        (*dctx).frameInfo.contentSize =
            LZ4F_readLE64(srcPtr.offset(6 as libc::c_int as isize) as
                              *const libc::c_void) as libc::c_ulonglong;
        (*dctx).frameRemainingSize = (*dctx).frameInfo.contentSize as U64
    }
    if dictIDFlag != 0 {
        (*dctx).frameInfo.dictID =
            LZ4F_readLE32(srcPtr.offset(frameHeaderSize as
                                            isize).offset(-(5 as libc::c_int
                                                                as isize)) as
                              *const libc::c_void)
    }
    (*dctx).dStage = dstage_init;
    return frameHeaderSize;
}
/* ! LZ4F_headerSize() :
 * @return : size of frame header
 *           or an error code, which can be tested using LZ4F_isError()
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_headerSize(mut src: *const libc::c_void,
                                         mut srcSize: size_t) -> size_t {
    if src.is_null() { return err0r(LZ4F_ERROR_srcPtr_wrong) }
    /* minimal srcSize to determine header size */
    if srcSize < 5 as libc::c_int as libc::c_ulong {
        return err0r(LZ4F_ERROR_frameHeader_incomplete)
    }
    /* special case : skippable frames */
    if LZ4F_readLE32(src) & 0xfffffff0 as libc::c_uint ==
           0x184d2a50 as libc::c_uint {
        return 8 as libc::c_int as size_t
    }
    /* control magic number */
    if LZ4F_readLE32(src) != 0x184d2204 as libc::c_uint {
        return err0r(LZ4F_ERROR_frameType_unknown)
    }
    /* Frame Header Size */
    let FLG: BYTE = *(src as *const BYTE).offset(4 as libc::c_int as isize);
    let contentSizeFlag: U32 =
        (FLG as libc::c_int >> 3 as libc::c_int & 0x1 as libc::c_int) as U32;
    let dictIDFlag: U32 = (FLG as libc::c_int & 0x1 as libc::c_int) as U32;
    return minFHSize.wrapping_add((if contentSizeFlag != 0 {
                                       8 as libc::c_int
                                   } else { 0 as libc::c_int }) as
                                      libc::c_ulong).wrapping_add((if dictIDFlag
                                                                          != 0
                                                                      {
                                                                       4 as
                                                                           libc::c_int
                                                                   } else {
                                                                       0 as
                                                                           libc::c_int
                                                                   }) as
                                                                      libc::c_ulong);
}
/* ! LZ4F_getFrameInfo() :
 *  This function extracts frame parameters (max blockSize, frame checksum, etc.).
 *  Usage is optional. Objective is to provide relevant information for allocation purposes.
 *  This function works in 2 situations :
 *   - At the beginning of a new frame, in which case it will decode this information from `srcBuffer`, and start the decoding process.
 *     Amount of input data provided must be large enough to successfully decode the frame header.
 *     A header size is variable, but is guaranteed to be <= LZ4F_HEADER_SIZE_MAX bytes. It's possible to provide more input data than this minimum.
 *   - After decoding has been started. In which case, no input is read, frame parameters are extracted from dctx.
 *  The number of bytes consumed from srcBuffer will be updated within *srcSizePtr (necessarily <= original value).
 *  Decompression must resume from (srcBuffer + *srcSizePtr).
 * @return : an hint about how many srcSize bytes LZ4F_decompress() expects for next call,
 *           or an error code which can be tested using LZ4F_isError()
 *  note 1 : in case of error, dctx is not modified. Decoding operations can resume from where they stopped.
 *  note 2 : frame parameters are *copied into* an already allocated LZ4F_frameInfo_t structure.
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_getFrameInfo(mut dctx: *mut LZ4F_dctx,
                                           mut frameInfoPtr:
                                               *mut LZ4F_frameInfo_t,
                                           mut srcBuffer: *const libc::c_void,
                                           mut srcSizePtr: *mut size_t)
 -> size_t {
    if (*dctx).dStage as libc::c_uint >
           dstage_storeFrameHeader as libc::c_int as libc::c_uint {
        /* frameInfo already decoded */
        let mut o: size_t = 0 as libc::c_int as size_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        *srcSizePtr = 0 as libc::c_int as size_t;
        *frameInfoPtr = (*dctx).frameInfo;
        /* returns : recommended nb of bytes for LZ4F_decompress() */
        return LZ4F_decompress(dctx, 0 as *mut libc::c_void, &mut o,
                               0 as *const libc::c_void, &mut i,
                               0 as *const LZ4F_decompressOptions_t)
    } else if (*dctx).dStage as libc::c_uint ==
                  dstage_storeFrameHeader as libc::c_int as libc::c_uint {
        /* frame decoding already started, in the middle of header => automatic fail */
        *srcSizePtr = 0 as libc::c_int as size_t;
        return err0r(LZ4F_ERROR_frameDecoding_alreadyStarted)
    } else {
        let hSize: size_t = LZ4F_headerSize(srcBuffer, *srcSizePtr);
        if LZ4F_isError(hSize) != 0 {
            *srcSizePtr = 0 as libc::c_int as size_t;
            return hSize
        }
        if *srcSizePtr < hSize {
            *srcSizePtr = 0 as libc::c_int as size_t;
            return err0r(LZ4F_ERROR_frameHeader_incomplete)
        }
        let mut decodeResult: size_t =
            LZ4F_decodeHeader(dctx, srcBuffer, hSize);
        if LZ4F_isError(decodeResult) != 0 {
            *srcSizePtr = 0 as libc::c_int as size_t
        } else {
            *srcSizePtr = decodeResult;
            decodeResult = BHSize
            /* block header size */
        }
        *frameInfoPtr = (*dctx).frameInfo;
        return decodeResult
    };
}
/* LZ4F_updateDict() :
 * only used for LZ4F_blockLinked mode */
unsafe extern "C" fn LZ4F_updateDict(mut dctx: *mut LZ4F_dctx,
                                     mut dstPtr: *const BYTE,
                                     mut dstSize: size_t,
                                     mut dstBufferStart: *const BYTE,
                                     mut withinTmp: libc::c_uint) {
    if (*dctx).dictSize == 0 as libc::c_int as libc::c_ulong {
        (*dctx).dict = dstPtr
    } /* priority to dictionary continuity */
    if (*dctx).dict.offset((*dctx).dictSize as isize) == dstPtr {
        /* dictionary continuity, directly within dstBuffer */
        (*dctx).dictSize =
            ((*dctx).dictSize as libc::c_ulong).wrapping_add(dstSize) as
                size_t as size_t;
        return
    }
    if (dstPtr.wrapping_offset_from(dstBufferStart) as libc::c_long as
            size_t).wrapping_add(dstSize) >=
           (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
               libc::c_ulong {
        /* history in dstBuffer becomes large enough to become dictionary */
        (*dctx).dict = dstBufferStart;
        (*dctx).dictSize =
            (dstPtr.wrapping_offset_from(dstBufferStart) as libc::c_long as
                 size_t).wrapping_add(dstSize);
        return
    }
    /* if dstSize >= 64 KB, dictionary would be set into dstBuffer directly */
    /* dstBuffer does not contain whole useful history (64 KB), so it must be saved within tmpOut */
    if withinTmp != 0 && (*dctx).dict == (*dctx).tmpOutBuffer as *const BYTE {
        /* continue history within tmpOutBuffer */
        /* withinTmp expectation : content of [dstPtr,dstSize] is same as [dict+dictSize,dstSize], so we just extend it */
        (*dctx).dictSize =
            ((*dctx).dictSize as libc::c_ulong).wrapping_add(dstSize) as
                size_t as size_t;
        return
    }
    if withinTmp != 0 {
        /* copy relevant dict portion in front of tmpOut within tmpOutBuffer */
        let preserveSize: size_t =
            (*dctx).tmpOut.wrapping_offset_from((*dctx).tmpOutBuffer) as
                libc::c_long as size_t;
        let mut copySize: size_t =
            ((64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                 as libc::c_ulong).wrapping_sub((*dctx).tmpOutSize);
        let oldDictEnd: *const BYTE =
            (*dctx).dict.offset((*dctx).dictSize as
                                    isize).offset(-((*dctx).tmpOutStart as
                                                        isize));
        if (*dctx).tmpOutSize >
               (64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int))
                   as libc::c_ulong {
            copySize = 0 as libc::c_int as size_t
        }
        if copySize > preserveSize { copySize = preserveSize }
        memcpy((*dctx).tmpOutBuffer.offset(preserveSize as
                                               isize).offset(-(copySize as
                                                                   isize)) as
                   *mut libc::c_void,
               oldDictEnd.offset(-(copySize as isize)) as *const libc::c_void,
               copySize);
        (*dctx).dict = (*dctx).tmpOutBuffer;
        (*dctx).dictSize =
            preserveSize.wrapping_add((*dctx).tmpOutStart).wrapping_add(dstSize);
        return
    }
    if (*dctx).dict == (*dctx).tmpOutBuffer as *const BYTE {
        /* copy dst into tmp to complete dict */
        if (*dctx).dictSize.wrapping_add(dstSize) > (*dctx).maxBufferSize {
            /* tmp buffer not large enough */
            let preserveSize_0: size_t =
                ((64 as libc::c_int *
                      ((1 as libc::c_int) << 10 as libc::c_int)) as
                     libc::c_ulong).wrapping_sub(dstSize);
            memcpy((*dctx).tmpOutBuffer as *mut libc::c_void,
                   (*dctx).dict.offset((*dctx).dictSize as
                                           isize).offset(-(preserveSize_0 as
                                                               isize)) as
                       *const libc::c_void, preserveSize_0);
            (*dctx).dictSize = preserveSize_0
        }
        memcpy((*dctx).tmpOutBuffer.offset((*dctx).dictSize as isize) as
                   *mut libc::c_void, dstPtr as *const libc::c_void, dstSize);
        (*dctx).dictSize =
            ((*dctx).dictSize as libc::c_ulong).wrapping_add(dstSize) as
                size_t as size_t;
        return
    }
    /* join dict & dest into tmp */
    let mut preserveSize_1: size_t =
        ((64 as libc::c_int * ((1 as libc::c_int) << 10 as libc::c_int)) as
             libc::c_ulong).wrapping_sub(dstSize);
    if preserveSize_1 > (*dctx).dictSize { preserveSize_1 = (*dctx).dictSize }
    memcpy((*dctx).tmpOutBuffer as *mut libc::c_void,
           (*dctx).dict.offset((*dctx).dictSize as
                                   isize).offset(-(preserveSize_1 as isize))
               as *const libc::c_void, preserveSize_1);
    memcpy((*dctx).tmpOutBuffer.offset(preserveSize_1 as isize) as
               *mut libc::c_void, dstPtr as *const libc::c_void, dstSize);
    (*dctx).dict = (*dctx).tmpOutBuffer;
    (*dctx).dictSize = preserveSize_1.wrapping_add(dstSize);
}
/* ! LZ4F_decompress() :
 *  Call this function repetitively to regenerate compressed data in srcBuffer.
 *  The function will attempt to decode up to *srcSizePtr bytes from srcBuffer
 *  into dstBuffer of capacity *dstSizePtr.
 *
 *  The number of bytes regenerated into dstBuffer will be provided within *dstSizePtr (necessarily <= original value).
 *
 *  The number of bytes effectively read from srcBuffer will be provided within *srcSizePtr (necessarily <= original value).
 *  If number of bytes read is < number of bytes provided, then decompression operation is not complete.
 *  Remaining data will have to be presented again in a subsequent invocation.
 *
 *  The function result is an hint of the better srcSize to use for next call to LZ4F_decompress.
 *  Schematically, it's the size of the current (or remaining) compressed block + header of next block.
 *  Respecting the hint provides a small boost to performance, since it allows less buffer shuffling.
 *  Note that this is just a hint, and it's always possible to any srcSize value.
 *  When a frame is fully decoded, @return will be 0.
 *  If decompression failed, @return is an error code which can be tested using LZ4F_isError().
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_decompress(mut dctx: *mut LZ4F_dctx,
                                         mut dstBuffer: *mut libc::c_void,
                                         mut dstSizePtr: *mut size_t,
                                         mut srcBuffer: *const libc::c_void,
                                         mut srcSizePtr: *mut size_t,
                                         mut decompressOptionsPtr:
                                             *const LZ4F_decompressOptions_t)
 -> size_t {
    let mut optionsNull: LZ4F_decompressOptions_t =
        LZ4F_decompressOptions_t{stableDst: 0, reserved: [0; 3],};
    let srcStart: *const BYTE = srcBuffer as *const BYTE;
    let srcEnd: *const BYTE = srcStart.offset(*srcSizePtr as isize);
    let mut srcPtr: *const BYTE = srcStart;
    let dstStart: *mut BYTE = dstBuffer as *mut BYTE;
    let dstEnd: *mut BYTE = dstStart.offset(*dstSizePtr as isize);
    let mut dstPtr: *mut BYTE = dstStart;
    let mut selectedIn: *const BYTE = 0 as *const BYTE;
    let mut doAnotherStage: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut nextSrcSizeHint: size_t = 1 as libc::c_int as size_t;
    memset(&mut optionsNull as *mut LZ4F_decompressOptions_t as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<LZ4F_decompressOptions_t>() as
               libc::c_ulong);
    if decompressOptionsPtr.is_null() {
        decompressOptionsPtr = &mut optionsNull
    }
    *srcSizePtr = 0 as libc::c_int as size_t;
    *dstSizePtr = 0 as libc::c_int as size_t;
    /* behaves as a state machine */
    while doAnotherStage != 0 {
        let mut current_block_212: u64; /* while (doAnotherStage) */
        match (*dctx).dStage as libc::c_uint {
            0 => {
                if srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                       size_t >= maxFHSize {
                    /* enough to decode - shortcut */
                    let hSize: size_t =
                        LZ4F_decodeHeader(dctx, srcPtr as *const libc::c_void,
                                          srcEnd.wrapping_offset_from(srcPtr)
                                              as libc::c_long as
                                              size_t); /* will update dStage appropriately */
                    if LZ4F_isError(hSize) != 0 {
                        return hSize
                    } /* 0-size input */
                    srcPtr =
                        srcPtr.offset(hSize as
                                          isize); /* minimum size to decode header */
                    current_block_212 = 4309244811846205759;
                } else {
                    (*dctx).tmpInSize = 0 as libc::c_int as size_t;
                    if srcEnd.wrapping_offset_from(srcPtr) as libc::c_long ==
                           0 as libc::c_int as libc::c_long {
                        return minFHSize
                    }
                    (*dctx).tmpInTarget = minFHSize;
                    (*dctx).dStage = dstage_storeFrameHeader;
                    current_block_212 = 9725544835206080425;
                }
            }
            1 => { current_block_212 = 9725544835206080425; }
            2 => {
                if (*dctx).frameInfo.contentChecksumFlag as u64 != 0 {
                    LZ4_XXH32_reset(&mut (*dctx).xxh,
                                    0 as libc::c_int as libc::c_uint);
                }
                /* internal buffers allocation */
                let bufferNeeded: size_t =
                    (*dctx).maxBlockSize.wrapping_add((if (*dctx).frameInfo.blockMode
                                                              as libc::c_uint
                                                              ==
                                                              LZ4F_blockLinked
                                                                  as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint
                                                          {
                                                           (128 as
                                                                libc::c_int) *
                                                               ((1 as
                                                                     libc::c_int)
                                                                    <<
                                                                    10 as
                                                                        libc::c_int)
                                                       } else {
                                                           0 as libc::c_int
                                                       }) as libc::c_ulong);
                if bufferNeeded > (*dctx).maxBufferSize {
                    /* tmp buffers too small */
                    (*dctx).maxBufferSize =
                        0 as libc::c_int as
                            size_t; /* ensure allocation will be re-attempted on next entry*/
                    free((*dctx).tmpIn as
                             *mut libc::c_void); /* block checksum */
                    (*dctx).tmpIn =
                        malloc((*dctx).maxBlockSize.wrapping_add(BFSize)) as
                            *mut BYTE;
                    if (*dctx).tmpIn.is_null() {
                        return err0r(LZ4F_ERROR_allocation_failed)
                    }
                    free((*dctx).tmpOutBuffer as *mut libc::c_void);
                    (*dctx).tmpOutBuffer = malloc(bufferNeeded) as *mut BYTE;
                    if (*dctx).tmpOutBuffer.is_null() {
                        return err0r(LZ4F_ERROR_allocation_failed)
                    }
                    (*dctx).maxBufferSize = bufferNeeded
                }
                (*dctx).tmpInSize = 0 as libc::c_int as size_t;
                (*dctx).tmpInTarget = 0 as libc::c_int as size_t;
                (*dctx).tmpOut = (*dctx).tmpOutBuffer;
                (*dctx).tmpOutStart = 0 as libc::c_int as size_t;
                (*dctx).tmpOutSize = 0 as libc::c_int as size_t;
                (*dctx).dStage = dstage_getBlockHeader;
                current_block_212 = 15624425105095617444;
            }
            3 => { current_block_212 = 15624425105095617444; }
            4 => { current_block_212 = 6560072651652764009; }
            5 => {
                /* uncompressed block */
                let minBuffSize: size_t =
                    if (srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                            size_t) <
                           dstEnd.wrapping_offset_from(dstPtr) as libc::c_long
                               as size_t {
                        srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                            size_t
                    } else {
                        dstEnd.wrapping_offset_from(dstPtr) as libc::c_long as
                            size_t
                    };
                let sizeToCopy_1: size_t =
                    if (*dctx).tmpInTarget < minBuffSize {
                        (*dctx).tmpInTarget
                    } else { minBuffSize };
                memcpy(dstPtr as *mut libc::c_void,
                       srcPtr as *const libc::c_void, sizeToCopy_1);
                if (*dctx).frameInfo.blockChecksumFlag as u64 != 0 {
                    LZ4_XXH32_update(&mut (*dctx).blockChecksum,
                                     srcPtr as *const libc::c_void,
                                     sizeToCopy_1);
                }
                if (*dctx).frameInfo.contentChecksumFlag as u64 != 0 {
                    LZ4_XXH32_update(&mut (*dctx).xxh,
                                     srcPtr as *const libc::c_void,
                                     sizeToCopy_1);
                }
                if (*dctx).frameInfo.contentSize != 0 {
                    (*dctx).frameRemainingSize =
                        ((*dctx).frameRemainingSize as
                             libc::c_ulong).wrapping_sub(sizeToCopy_1) as U64
                            as U64
                }
                /* history management (linked blocks only)*/
                if (*dctx).frameInfo.blockMode as libc::c_uint ==
                       LZ4F_blockLinked as libc::c_int as libc::c_uint {
                    LZ4F_updateDict(dctx, dstPtr, sizeToCopy_1, dstStart,
                                    0 as libc::c_int as libc::c_uint);
                }
                srcPtr = srcPtr.offset(sizeToCopy_1 as isize);
                dstPtr = dstPtr.offset(sizeToCopy_1 as isize);
                if sizeToCopy_1 == (*dctx).tmpInTarget {
                    /* all done */
                    if (*dctx).frameInfo.blockChecksumFlag as u64 != 0 {
                        (*dctx).tmpInSize =
                            0 as libc::c_int as size_t; /* new block */
                        (*dctx).dStage = dstage_getBlockChecksum
                    } else { (*dctx).dStage = dstage_getBlockHeader }
                } else {
                    (*dctx).tmpInTarget =
                        ((*dctx).tmpInTarget as
                             libc::c_ulong).wrapping_sub(sizeToCopy_1) as
                            size_t as size_t; /* need to copy more */
                    nextSrcSizeHint =
                        (*dctx).tmpInTarget.wrapping_add((if (*dctx).frameInfo.blockChecksumFlag
                                                                 as
                                                                 libc::c_uint
                                                                 != 0 {
                                                              BFSize
                                                          } else {
                                                              0 as libc::c_int
                                                                  as
                                                                  libc::c_ulong
                                                          })).wrapping_add(BHSize); /* next header size */
                    doAnotherStage = 0 as libc::c_int as libc::c_uint
                }
                current_block_212 = 4309244811846205759;
            }
            6 => {
                /* check block checksum for recently transferred uncompressed block */
                let mut crcSrc: *const libc::c_void =
                    0 as *const libc::c_void;
                if srcEnd.wrapping_offset_from(srcPtr) as libc::c_long >=
                       4 as libc::c_int as libc::c_long &&
                       (*dctx).tmpInSize == 0 as libc::c_int as libc::c_ulong
                   {
                    crcSrc = srcPtr as *const libc::c_void;
                    srcPtr = srcPtr.offset(4 as libc::c_int as isize);
                    current_block_212 = 14648249180243006330;
                } else {
                    let stillToCopy: size_t =
                        (4 as libc::c_int as
                             libc::c_ulong).wrapping_sub((*dctx).tmpInSize);
                    let sizeToCopy_2: size_t =
                        if stillToCopy <
                               srcEnd.wrapping_offset_from(srcPtr) as
                                   libc::c_long as size_t {
                            stillToCopy
                        } else {
                            srcEnd.wrapping_offset_from(srcPtr) as
                                libc::c_long as size_t
                        };
                    memcpy((*dctx).header.as_mut_ptr().offset((*dctx).tmpInSize
                                                                  as isize) as
                               *mut libc::c_void,
                           srcPtr as *const libc::c_void, sizeToCopy_2);
                    (*dctx).tmpInSize =
                        ((*dctx).tmpInSize as
                             libc::c_ulong).wrapping_add(sizeToCopy_2) as
                            size_t as size_t;
                    srcPtr = srcPtr.offset(sizeToCopy_2 as isize);
                    if (*dctx).tmpInSize < 4 as libc::c_int as libc::c_ulong {
                        /* all input consumed */
                        doAnotherStage =
                            0 as libc::c_int as libc::c_uint; /* new block */
                        current_block_212 = 4309244811846205759;
                    } else {
                        crcSrc =
                            (*dctx).header.as_mut_ptr() as
                                *const libc::c_void;
                        current_block_212 = 14648249180243006330;
                    }
                }
                match current_block_212 {
                    4309244811846205759 => { }
                    _ => {
                        let readCRC: U32 = LZ4F_readLE32(crcSrc);
                        let calcCRC: U32 =
                            LZ4_XXH32_digest(&mut (*dctx).blockChecksum);
                        if readCRC != calcCRC {
                            return err0r(LZ4F_ERROR_blockChecksum_invalid)
                        }
                        (*dctx).dStage = dstage_getBlockHeader;
                        current_block_212 = 4309244811846205759;
                    }
                }
            }
            7 => {
                if (srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                        size_t) < (*dctx).tmpInTarget {
                    (*dctx).tmpInSize = 0 as libc::c_int as size_t;
                    (*dctx).dStage = dstage_storeCBlock;
                    current_block_212 = 4309244811846205759;
                } else {
                    /* input large enough to read full block directly */
                    selectedIn = srcPtr;
                    srcPtr = srcPtr.offset((*dctx).tmpInTarget as isize);
                    current_block_212 = 5089124893069931607;
                }
            }
            8 => {
                /* jump over next block */
                let wantedData_0: size_t =
                    (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize);
                let inputLeft: size_t =
                    srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                        size_t;
                let sizeToCopy_3: size_t =
                    if wantedData_0 < inputLeft {
                        wantedData_0
                    } else { inputLeft };
                memcpy((*dctx).tmpIn.offset((*dctx).tmpInSize as isize) as
                           *mut libc::c_void, srcPtr as *const libc::c_void,
                       sizeToCopy_3);
                (*dctx).tmpInSize =
                    ((*dctx).tmpInSize as
                         libc::c_ulong).wrapping_add(sizeToCopy_3) as size_t
                        as size_t;
                srcPtr = srcPtr.offset(sizeToCopy_3 as isize);
                if (*dctx).tmpInSize < (*dctx).tmpInTarget {
                    /* need more input */
                    nextSrcSizeHint =
                        (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize).wrapping_add((if (*dctx).frameInfo.blockChecksumFlag
                                                                                                 as
                                                                                                 libc::c_uint
                                                                                                 !=
                                                                                                 0
                                                                                             {
                                                                                              BFSize
                                                                                          } else {
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_ulong
                                                                                          })).wrapping_add(BHSize); /* next header size */
                    doAnotherStage =
                        0 as libc::c_int as
                            libc::c_uint; /* incorrect frame size decoded */
                    current_block_212 = 4309244811846205759;
                } else {
                    selectedIn = (*dctx).tmpIn;
                    current_block_212 = 5089124893069931607;
                }
            }
            9 => { current_block_212 = 14557636130817708122; }
            10 => {
                if (*dctx).frameRemainingSize != 0 {
                    return err0r(LZ4F_ERROR_frameSize_wrong)
                }
                if (*dctx).frameInfo.contentChecksumFlag as u64 == 0 {
                    /* no checksum, frame is completed */
                    nextSrcSizeHint = 0 as libc::c_int as size_t;
                    LZ4F_resetDecompressionContext(dctx);
                    doAnotherStage = 0 as libc::c_int as libc::c_uint;
                    current_block_212 = 4309244811846205759;
                } else {
                    if (srcEnd.wrapping_offset_from(srcPtr) as libc::c_long) <
                           4 as libc::c_int as libc::c_long {
                        /* not enough size for entire CRC */
                        (*dctx).tmpInSize =
                            0 as libc::c_int as
                                size_t; /* if (dctx->dStage == dstage_storeSuffix) */
                        (*dctx).dStage = dstage_storeSuffix
                    } else {
                        selectedIn = srcPtr;
                        srcPtr = srcPtr.offset(4 as libc::c_int as isize)
                    }
                    if (*dctx).dStage as libc::c_uint ==
                           dstage_storeSuffix as libc::c_int as libc::c_uint {
                        current_block_212 = 162359820444412279;
                    } else { current_block_212 = 9350489878244555550; }
                }
            }
            11 => { current_block_212 = 162359820444412279; }
            12 => {
                if srcEnd.wrapping_offset_from(srcPtr) as libc::c_long >=
                       4 as libc::c_int as libc::c_long {
                    selectedIn = srcPtr;
                    srcPtr = srcPtr.offset(4 as libc::c_int as isize)
                } else {
                    /* not enough input to read cBlockSize field */
                    (*dctx).tmpInSize =
                        4 as libc::c_int as
                            size_t; /* if (dctx->dStage == dstage_storeSFrameSize) */
                    (*dctx).tmpInTarget =
                        8 as libc::c_int as size_t; /* still more to skip */
                    (*dctx).dStage = dstage_storeSFrameSize
                }
                if (*dctx).dStage as libc::c_uint ==
                       dstage_storeSFrameSize as libc::c_int as libc::c_uint {
                    current_block_212 = 13675415059418534261;
                } else { current_block_212 = 6744494640291411773; }
            }
            13 => { current_block_212 = 13675415059418534261; }
            14 => {
                let skipSize: size_t =
                    if (*dctx).tmpInTarget <
                           srcEnd.wrapping_offset_from(srcPtr) as libc::c_long
                               as size_t {
                        (*dctx).tmpInTarget
                    } else {
                        srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                            size_t
                    };
                srcPtr = srcPtr.offset(skipSize as isize);
                (*dctx).tmpInTarget =
                    ((*dctx).tmpInTarget as
                         libc::c_ulong).wrapping_sub(skipSize) as size_t as
                        size_t;
                doAnotherStage = 0 as libc::c_int as libc::c_uint;
                nextSrcSizeHint = (*dctx).tmpInTarget;
                if nextSrcSizeHint != 0 {
                    current_block_212 = 4309244811846205759;
                } else {
                    /* frame fully skipped : prepare context for a new frame */
                    LZ4F_resetDecompressionContext(dctx);
                    current_block_212 = 4309244811846205759;
                }
            }
            _ => { current_block_212 = 4309244811846205759; }
        }
        match current_block_212 {
            13675415059418534261 => {
                let sizeToCopy_6: size_t =
                    if (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize) <
                           srcEnd.wrapping_offset_from(srcPtr) as libc::c_long
                               as size_t {
                        (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize)
                    } else {
                        srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                            size_t
                    };
                memcpy((*dctx).header.as_mut_ptr().offset((*dctx).tmpInSize as
                                                              isize) as
                           *mut libc::c_void, srcPtr as *const libc::c_void,
                       sizeToCopy_6);
                srcPtr = srcPtr.offset(sizeToCopy_6 as isize);
                (*dctx).tmpInSize =
                    ((*dctx).tmpInSize as
                         libc::c_ulong).wrapping_add(sizeToCopy_6) as size_t
                        as size_t;
                if (*dctx).tmpInSize < (*dctx).tmpInTarget {
                    /* not enough input to get full sBlockSize; wait for more */
                    nextSrcSizeHint =
                        (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize);
                    doAnotherStage = 0 as libc::c_int as libc::c_uint;
                    current_block_212 = 4309244811846205759;
                } else {
                    selectedIn =
                        (*dctx).header.as_mut_ptr().offset(4 as libc::c_int as
                                                               isize);
                    current_block_212 = 6744494640291411773;
                }
            }
            162359820444412279 =>
            /* can be skipped */
            {
                let remainingInput_0: size_t =
                    srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                        size_t;
                let wantedData_1: size_t =
                    (4 as libc::c_int as
                         libc::c_ulong).wrapping_sub((*dctx).tmpInSize);
                let sizeToCopy_5: size_t =
                    if wantedData_1 < remainingInput_0 {
                        wantedData_1
                    } else { remainingInput_0 };
                memcpy((*dctx).tmpIn.offset((*dctx).tmpInSize as isize) as
                           *mut libc::c_void, srcPtr as *const libc::c_void,
                       sizeToCopy_5);
                srcPtr = srcPtr.offset(sizeToCopy_5 as isize);
                (*dctx).tmpInSize =
                    ((*dctx).tmpInSize as
                         libc::c_ulong).wrapping_add(sizeToCopy_5) as size_t
                        as size_t;
                if (*dctx).tmpInSize < 4 as libc::c_int as libc::c_ulong {
                    /* not enough input to read complete suffix */
                    nextSrcSizeHint =
                        (4 as libc::c_int as
                             libc::c_ulong).wrapping_sub((*dctx).tmpInSize);
                    doAnotherStage = 0 as libc::c_int as libc::c_uint;
                    current_block_212 = 4309244811846205759;
                } else {
                    selectedIn = (*dctx).tmpIn;
                    current_block_212 = 9350489878244555550;
                }
            }
            5089124893069931607 => {
                /* At this stage, input is large enough to decode a block */
                if (*dctx).frameInfo.blockChecksumFlag as u64 != 0 {
                    (*dctx).tmpInTarget =
                        ((*dctx).tmpInTarget as
                             libc::c_ulong).wrapping_sub(4 as libc::c_int as
                                                             libc::c_ulong) as
                            size_t as size_t;
                    /* selectedIn is defined at this stage (either srcPtr, or dctx->tmpIn) */
                    let readBlockCrc: U32 =
                        LZ4F_readLE32(selectedIn.offset((*dctx).tmpInTarget as
                                                            isize) as
                                          *const libc::c_void);
                    let calcBlockCrc: U32 =
                        LZ4_XXH32(selectedIn as *const libc::c_void,
                                  (*dctx).tmpInTarget,
                                  0 as libc::c_int as libc::c_uint);
                    if readBlockCrc != calcBlockCrc {
                        return err0r(LZ4F_ERROR_blockChecksum_invalid)
                    }
                }
                if dstEnd.wrapping_offset_from(dstPtr) as libc::c_long as
                       size_t >= (*dctx).maxBlockSize {
                    let mut dict: *const libc::c_char =
                        (*dctx).dict as *const libc::c_char;
                    let mut dictSize: size_t = (*dctx).dictSize;
                    let mut decodedSize: libc::c_int = 0;
                    if !dict.is_null() &&
                           dictSize >
                               (1 as libc::c_int *
                                    ((1 as libc::c_int) << 30 as libc::c_int))
                                   as libc::c_ulong {
                        /* the dictSize param is an int, avoid truncation / sign issues */
                        dict =
                            dict.offset(dictSize.wrapping_sub((64 as
                                                                   libc::c_int
                                                                   *
                                                                   ((1 as
                                                                         libc::c_int)
                                                                        <<
                                                                        10 as
                                                                            libc::c_int))
                                                                  as
                                                                  libc::c_ulong)
                                            as isize);
                        dictSize =
                            (64 as libc::c_int *
                                 ((1 as libc::c_int) << 10 as libc::c_int)) as
                                size_t
                    }
                    /* enough capacity in `dst` to decompress directly there */
                    decodedSize =
                        LZ4_decompress_safe_usingDict(selectedIn as
                                                          *const libc::c_char,
                                                      dstPtr as
                                                          *mut libc::c_char,
                                                      (*dctx).tmpInTarget as
                                                          libc::c_int,
                                                      (*dctx).maxBlockSize as
                                                          libc::c_int, dict,
                                                      dictSize as
                                                          libc::c_int); /* decompression failed */
                    if decodedSize < 0 as libc::c_int {
                        return err0r(LZ4F_ERROR_GENERIC)
                    }
                    if (*dctx).frameInfo.contentChecksumFlag as u64 != 0 {
                        LZ4_XXH32_update(&mut (*dctx).xxh,
                                         dstPtr as *const libc::c_void,
                                         decodedSize as size_t);
                    }
                    if (*dctx).frameInfo.contentSize != 0 {
                        (*dctx).frameRemainingSize =
                            ((*dctx).frameRemainingSize as
                                 libc::c_ulong).wrapping_sub(decodedSize as
                                                                 size_t) as
                                U64 as U64
                    }
                    /* dictionary management */
                    if (*dctx).frameInfo.blockMode as libc::c_uint ==
                           LZ4F_blockLinked as libc::c_int as libc::c_uint {
                        LZ4F_updateDict(dctx, dstPtr, decodedSize as size_t,
                                        dstStart,
                                        0 as libc::c_int as libc::c_uint);
                    }
                    dstPtr = dstPtr.offset(decodedSize as isize);
                    (*dctx).dStage = dstage_getBlockHeader;
                    current_block_212 = 4309244811846205759;
                } else {
                    /* not enough place into dst : decode into tmpOut */
            /* ensure enough place for tmpOut */
                    if (*dctx).frameInfo.blockMode as libc::c_uint ==
                           LZ4F_blockLinked as libc::c_int as libc::c_uint {
                        if (*dctx).dict == (*dctx).tmpOutBuffer as *const BYTE
                           {
                            if (*dctx).dictSize >
                                   (128 as libc::c_int *
                                        ((1 as libc::c_int) <<
                                             10 as libc::c_int)) as
                                       libc::c_ulong {
                                memcpy((*dctx).tmpOutBuffer as
                                           *mut libc::c_void,
                                       (*dctx).dict.offset((*dctx).dictSize as
                                                               isize).offset(-((64
                                                                                    as
                                                                                    libc::c_int
                                                                                    *
                                                                                    ((1
                                                                                          as
                                                                                          libc::c_int)
                                                                                         <<
                                                                                         10
                                                                                             as
                                                                                             libc::c_int))
                                                                                   as
                                                                                   isize))
                                           as *const libc::c_void,
                                       (64 as libc::c_int *
                                            ((1 as libc::c_int) <<
                                                 10 as libc::c_int)) as
                                           libc::c_ulong); /* dict not within tmp */
                                (*dctx).dictSize =
                                    (64 as libc::c_int *
                                         ((1 as libc::c_int) <<
                                              10 as libc::c_int)) as size_t
                            }
                            (*dctx).tmpOut =
                                (*dctx).tmpOutBuffer.offset((*dctx).dictSize
                                                                as isize)
                        } else {
                            let reservedDictSpace: size_t =
                                if (*dctx).dictSize <
                                       (64 as libc::c_int *
                                            ((1 as libc::c_int) <<
                                                 10 as libc::c_int)) as
                                           libc::c_ulong {
                                    (*dctx).dictSize
                                } else {
                                    (64 as libc::c_int *
                                         ((1 as libc::c_int) <<
                                              10 as libc::c_int)) as
                                        libc::c_ulong
                                };
                            (*dctx).tmpOut =
                                (*dctx).tmpOutBuffer.offset(reservedDictSpace
                                                                as isize)
                        }
                    }
                    /* Decode block */
                    let mut dict_0: *const libc::c_char =
                        (*dctx).dict as *const libc::c_char;
                    let mut dictSize_0: size_t = (*dctx).dictSize;
                    let mut decodedSize_0: libc::c_int = 0;
                    if !dict_0.is_null() &&
                           dictSize_0 >
                               (1 as libc::c_int *
                                    ((1 as libc::c_int) << 30 as libc::c_int))
                                   as libc::c_ulong {
                        /* the dictSize param is an int, avoid truncation / sign issues */
                        dict_0 =
                            dict_0.offset(dictSize_0.wrapping_sub((64 as
                                                                       libc::c_int
                                                                       *
                                                                       ((1 as
                                                                             libc::c_int)
                                                                            <<
                                                                            10
                                                                                as
                                                                                libc::c_int))
                                                                      as
                                                                      libc::c_ulong)
                                              as isize);
                        dictSize_0 =
                            (64 as libc::c_int *
                                 ((1 as libc::c_int) << 10 as libc::c_int)) as
                                size_t
                    }
                    decodedSize_0 =
                        LZ4_decompress_safe_usingDict(selectedIn as
                                                          *const libc::c_char,
                                                      (*dctx).tmpOut as
                                                          *mut libc::c_char,
                                                      (*dctx).tmpInTarget as
                                                          libc::c_int,
                                                      (*dctx).maxBlockSize as
                                                          libc::c_int, dict_0,
                                                      dictSize_0 as
                                                          libc::c_int);
                    if decodedSize_0 < 0 as libc::c_int {
                        /* decompression failed */
                        return err0r(LZ4F_ERROR_decompressionFailed)
                    }
                    if (*dctx).frameInfo.contentChecksumFlag as u64 != 0 {
                        LZ4_XXH32_update(&mut (*dctx).xxh,
                                         (*dctx).tmpOut as
                                             *const libc::c_void,
                                         decodedSize_0 as size_t);
                    }
                    if (*dctx).frameInfo.contentSize != 0 {
                        (*dctx).frameRemainingSize =
                            ((*dctx).frameRemainingSize as
                                 libc::c_ulong).wrapping_sub(decodedSize_0 as
                                                                 size_t) as
                                U64 as U64
                    }
                    (*dctx).tmpOutSize = decodedSize_0 as size_t;
                    (*dctx).tmpOutStart = 0 as libc::c_int as size_t;
                    (*dctx).dStage = dstage_flushOut;
                    /* fall-through */
                    current_block_212 = 14557636130817708122;
                }
            }
            15624425105095617444 =>
            /* fall-through */
            {
                if srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                       size_t >= BHSize {
                    selectedIn = srcPtr;
                    srcPtr = srcPtr.offset(BHSize as isize)
                } else {
                    /* not enough input to read cBlockSize field */
                    (*dctx).tmpInSize =
                        0 as libc::c_int as
                            size_t; /* if (dctx->dStage == dstage_storeBlockHeader) */
                    (*dctx).dStage = dstage_storeBlockHeader
                }
                if (*dctx).dStage as libc::c_uint ==
                       dstage_storeBlockHeader as libc::c_int as libc::c_uint
                   {
                    current_block_212 = 6560072651652764009;
                } else { current_block_212 = 12264624100856317061; }
            }
            9725544835206080425 =>
            /* fall-through */
            {
                let sizeToCopy: size_t =
                    if (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize) <
                           srcEnd.wrapping_offset_from(srcPtr) as libc::c_long
                               as size_t {
                        (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize)
                    } else {
                        srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                            size_t
                    }; /* rest of header + nextBlockHeader */
                memcpy((*dctx).header.as_mut_ptr().offset((*dctx).tmpInSize as
                                                              isize) as
                           *mut libc::c_void, srcPtr as *const libc::c_void,
                       sizeToCopy); /* not enough src data, ask for some more */
                (*dctx).tmpInSize =
                    ((*dctx).tmpInSize as
                         libc::c_ulong).wrapping_add(sizeToCopy) as size_t as
                        size_t; /* will update dStage appropriately */
                srcPtr = srcPtr.offset(sizeToCopy as isize);
                if (*dctx).tmpInSize < (*dctx).tmpInTarget {
                    nextSrcSizeHint =
                        (*dctx).tmpInTarget.wrapping_sub((*dctx).tmpInSize).wrapping_add(BHSize);
                    doAnotherStage = 0 as libc::c_int as libc::c_uint
                } else {
                    let hSize_0: size_t =
                        LZ4F_decodeHeader(dctx,
                                          (*dctx).header.as_mut_ptr() as
                                              *const libc::c_void,
                                          (*dctx).tmpInTarget);
                    if LZ4F_isError(hSize_0) != 0 { return hSize_0 }
                }
                current_block_212 = 4309244811846205759;
            }
            _ => { }
        }
        match current_block_212 {
            6744494640291411773 =>
            /* case dstage_decodeSFrameSize: */
            /* no direct entry */
            {
                let SFrameSize: size_t =
                    LZ4F_readLE32(selectedIn as *const libc::c_void) as
                        size_t;
                (*dctx).frameInfo.contentSize =
                    SFrameSize as libc::c_ulonglong;
                (*dctx).tmpInTarget = SFrameSize;
                (*dctx).dStage = dstage_skipSkippable;
                current_block_212 = 4309244811846205759;
            }
            9350489878244555550 =>
            /* case dstage_checkSuffix: */
            /* no direct entry, avoid initialization risks */
            {
                let readCRC_0: U32 =
                    LZ4F_readLE32(selectedIn as *const libc::c_void);
                let resultCRC: U32 = LZ4_XXH32_digest(&mut (*dctx).xxh);
                if readCRC_0 != resultCRC {
                    return err0r(LZ4F_ERROR_contentChecksum_invalid)
                }
                nextSrcSizeHint = 0 as libc::c_int as size_t;
                LZ4F_resetDecompressionContext(dctx);
                doAnotherStage = 0 as libc::c_int as libc::c_uint;
                current_block_212 = 4309244811846205759;
            }
            14557636130817708122 =>
            /* flush decoded data from tmpOut to dstBuffer */
            {
                let sizeToCopy_4: size_t =
                    if (*dctx).tmpOutSize.wrapping_sub((*dctx).tmpOutStart) <
                           dstEnd.wrapping_offset_from(dstPtr) as libc::c_long
                               as size_t {
                        (*dctx).tmpOutSize.wrapping_sub((*dctx).tmpOutStart)
                    } else {
                        dstEnd.wrapping_offset_from(dstPtr) as libc::c_long as
                            size_t
                    };
                memcpy(dstPtr as *mut libc::c_void,
                       (*dctx).tmpOut.offset((*dctx).tmpOutStart as isize) as
                           *const libc::c_void, sizeToCopy_4);
                /* dictionary management */
                if (*dctx).frameInfo.blockMode as libc::c_uint ==
                       LZ4F_blockLinked as libc::c_int as libc::c_uint {
                    LZ4F_updateDict(dctx, dstPtr, sizeToCopy_4, dstStart,
                                    1 as libc::c_int as libc::c_uint);
                }
                (*dctx).tmpOutStart =
                    ((*dctx).tmpOutStart as
                         libc::c_ulong).wrapping_add(sizeToCopy_4) as size_t
                        as size_t;
                dstPtr = dstPtr.offset(sizeToCopy_4 as isize);
                if (*dctx).tmpOutStart == (*dctx).tmpOutSize {
                    /* all flushed */
                    (*dctx).dStage = dstage_getBlockHeader
                } else { /* get next block */
                    /* could not flush everything : stop there, just request a block header */
                    doAnotherStage = 0 as libc::c_int as libc::c_uint;
                    nextSrcSizeHint = BHSize
                }
                current_block_212 = 4309244811846205759;
            }
            6560072651652764009 =>
            /* can be skipped */
            {
                let remainingInput: size_t =
                    srcEnd.wrapping_offset_from(srcPtr) as libc::c_long as
                        size_t;
                let wantedData: size_t =
                    BHSize.wrapping_sub((*dctx).tmpInSize);
                let sizeToCopy_0: size_t =
                    if wantedData < remainingInput {
                        wantedData
                    } else { remainingInput };
                memcpy((*dctx).tmpIn.offset((*dctx).tmpInSize as isize) as
                           *mut libc::c_void, srcPtr as *const libc::c_void,
                       sizeToCopy_0);
                srcPtr = srcPtr.offset(sizeToCopy_0 as isize);
                (*dctx).tmpInSize =
                    ((*dctx).tmpInSize as
                         libc::c_ulong).wrapping_add(sizeToCopy_0) as size_t
                        as size_t;
                if (*dctx).tmpInSize < BHSize {
                    /* not enough input for cBlockSize */
                    nextSrcSizeHint = BHSize.wrapping_sub((*dctx).tmpInSize);
                    doAnotherStage = 0 as libc::c_int as libc::c_uint;
                    current_block_212 = 4309244811846205759;
                } else {
                    selectedIn = (*dctx).tmpIn;
                    current_block_212 = 12264624100856317061;
                }
            }
            _ => { }
        }
        match current_block_212 {
            12264624100856317061 =>
            /* decode block header */
            {
                let nextCBlockSize: size_t =
                    (LZ4F_readLE32(selectedIn as *const libc::c_void) &
                         0x7fffffff as libc::c_uint) as size_t;
                let crcSize: size_t =
                    ((*dctx).frameInfo.blockChecksumFlag as
                         libc::c_ulong).wrapping_mul(BFSize);
                if nextCBlockSize == 0 as libc::c_int as libc::c_ulong {
                    /* frameEnd signal, no more block */
                    (*dctx).dStage = dstage_getSuffix
                } else {
                    if nextCBlockSize > (*dctx).maxBlockSize {
                        return err0r(LZ4F_ERROR_maxBlockSize_invalid)
                    }
                    if LZ4F_readLE32(selectedIn as *const libc::c_void) &
                           0x80000000 as libc::c_uint != 0 {
                        /* next block is uncompressed */
                        (*dctx).tmpInTarget = nextCBlockSize;
                        if (*dctx).frameInfo.blockChecksumFlag as u64 != 0 {
                            LZ4_XXH32_reset(&mut (*dctx).blockChecksum,
                                            0 as libc::c_int as libc::c_uint);
                        }
                        (*dctx).dStage = dstage_copyDirect
                    } else {
                        /* next block is a compressed block */
                        (*dctx).tmpInTarget =
                            nextCBlockSize.wrapping_add(crcSize);
                        (*dctx).dStage = dstage_getCBlock;
                        if dstPtr == dstEnd || srcPtr == srcEnd {
                            nextSrcSizeHint =
                                BHSize.wrapping_add(nextCBlockSize).wrapping_add(crcSize);
                            doAnotherStage = 0 as libc::c_int as libc::c_uint
                        }
                    }
                }
            }
            _ => { }
        }
        /* switch (dctx->dStage) */
    }
    /* preserve history within tmp whenever necessary */
    if (*dctx).frameInfo.blockMode as libc::c_uint ==
           LZ4F_blockLinked as libc::c_int as libc::c_uint &&
           (*dctx).dict != (*dctx).tmpOutBuffer as *const BYTE &&
           (*decompressOptionsPtr).stableDst == 0 &&
           ((*dctx).dStage as
                libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint) <
               (dstage_getSuffix as libc::c_int as
                    libc::c_uint).wrapping_sub(2 as libc::c_int as
                                                   libc::c_uint) {
        /* valid stages : [init ... getSuffix[ */
        if (*dctx).dStage as libc::c_uint ==
               dstage_flushOut as libc::c_int as libc::c_uint {
            let preserveSize: size_t =
                (*dctx).tmpOut.wrapping_offset_from((*dctx).tmpOutBuffer) as
                    libc::c_long as size_t;
            let mut copySize: size_t =
                ((64 as libc::c_int *
                      ((1 as libc::c_int) << 10 as libc::c_int)) as
                     libc::c_ulong).wrapping_sub((*dctx).tmpOutSize);
            let mut oldDictEnd: *const BYTE =
                (*dctx).dict.offset((*dctx).dictSize as
                                        isize).offset(-((*dctx).tmpOutStart as
                                                            isize));
            if (*dctx).tmpOutSize >
                   (64 as libc::c_int *
                        ((1 as libc::c_int) << 10 as libc::c_int)) as
                       libc::c_ulong {
                copySize = 0 as libc::c_int as size_t
            }
            if copySize > preserveSize { copySize = preserveSize }
            if copySize > 0 as libc::c_int as libc::c_ulong {
                memcpy((*dctx).tmpOutBuffer.offset(preserveSize as
                                                       isize).offset(-(copySize
                                                                           as
                                                                           isize))
                           as *mut libc::c_void,
                       oldDictEnd.offset(-(copySize as isize)) as
                           *const libc::c_void, copySize);
            }
            (*dctx).dict = (*dctx).tmpOutBuffer;
            (*dctx).dictSize = preserveSize.wrapping_add((*dctx).tmpOutStart)
        } else {
            let oldDictEnd_0: *const BYTE =
                (*dctx).dict.offset((*dctx).dictSize as isize);
            let newDictSize: size_t =
                if (*dctx).dictSize <
                       (64 as libc::c_int *
                            ((1 as libc::c_int) << 10 as libc::c_int)) as
                           libc::c_ulong {
                    (*dctx).dictSize
                } else {
                    (64 as libc::c_int *
                         ((1 as libc::c_int) << 10 as libc::c_int)) as
                        libc::c_ulong
                };
            if newDictSize > 0 as libc::c_int as libc::c_ulong {
                memcpy((*dctx).tmpOutBuffer as *mut libc::c_void,
                       oldDictEnd_0.offset(-(newDictSize as isize)) as
                           *const libc::c_void, newDictSize);
            }
            (*dctx).dict = (*dctx).tmpOutBuffer;
            (*dctx).dictSize = newDictSize;
            (*dctx).tmpOut = (*dctx).tmpOutBuffer.offset(newDictSize as isize)
        }
    }
    *srcSizePtr =
        srcPtr.wrapping_offset_from(srcStart) as libc::c_long as size_t;
    *dstSizePtr =
        dstPtr.wrapping_offset_from(dstStart) as libc::c_long as size_t;
    return nextSrcSizeHint;
}
/* ! LZ4F_decompress_usingDict() :
 *  Same as LZ4F_decompress(), using a predefined dictionary.
 *  Dictionary is used "in place", without any preprocessing.
 *  It must remain accessible throughout the entire frame decoding.
 */
#[no_mangle]
pub unsafe extern "C" fn LZ4F_decompress_usingDict(mut dctx: *mut LZ4F_dctx,
                                                   mut dstBuffer:
                                                       *mut libc::c_void,
                                                   mut dstSizePtr:
                                                       *mut size_t,
                                                   mut srcBuffer:
                                                       *const libc::c_void,
                                                   mut srcSizePtr:
                                                       *mut size_t,
                                                   mut dict:
                                                       *const libc::c_void,
                                                   mut dictSize: size_t,
                                                   mut decompressOptionsPtr:
                                                       *const LZ4F_decompressOptions_t)
 -> size_t {
    if (*dctx).dStage as libc::c_uint <=
           dstage_init as libc::c_int as libc::c_uint {
        (*dctx).dict = dict as *const BYTE;
        (*dctx).dictSize = dictSize
    }
    return LZ4F_decompress(dctx, dstBuffer, dstSizePtr, srcBuffer, srcSizePtr,
                           decompressOptionsPtr);
}
