#![allow(dead_code)]
#![allow(non_snake_case)]
#![no_std]
#![feature(try_trait)]

pub mod block_io;
pub mod boot;
pub mod capsule;
pub mod config;
pub mod device;
pub mod fs;
pub mod graphics;
pub mod guid;
pub mod loaded_image;
pub mod memory;
pub mod pointer;
pub mod reset;
pub mod runtime;
pub mod shell;
pub mod status;
pub mod system;
pub mod text;
pub mod time;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Event(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Handle(pub usize);

#[repr(C)]
pub struct TableHeader {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    CRC32: u32,
    Reserved: u32
}
