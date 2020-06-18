#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]



extern crate f128;
extern crate libc;



pub mod src {
pub mod lib {
pub mod lz4;
pub mod lz4frame;
pub mod lz4hc;
pub mod xxhash;
} // mod lib
pub mod programs {
pub mod bench;
pub mod datagen;
pub mod lz4io;
} // mod programs
} // mod src

