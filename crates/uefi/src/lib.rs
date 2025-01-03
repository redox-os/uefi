#![no_std]
#![allow(dead_code)]
#![allow(non_snake_case)]

#[macro_use]
pub mod macros;

pub mod block_io;
pub mod boot;
pub mod capsule;
pub mod component_name;
pub mod config;
pub mod device;
pub mod firmware_volume;
pub mod fs;
pub mod graphics;
pub mod guid;
pub mod hii;
pub mod loaded_image;
pub mod memory;
pub mod pointer;
pub mod prelude;
pub mod reset;
pub mod runtime;
pub mod shell;
pub mod status;
pub mod system;
pub mod text;
pub mod time;

pub(crate) mod util;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Event(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Handle(pub usize);

#[repr(C)]
pub struct TableHeader {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    CRC32: u32,
    Reserved: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Tpl(pub usize);
