#![allow(dead_code)]
#![allow(non_snake_case)]
#![no_std]
#![feature(try_trait)]

pub mod boot;
pub mod config;
pub mod fs;
pub mod graphics;
pub mod guid;
pub mod memory;
pub mod runtime;
pub mod shell;
pub mod status;
pub mod system;
pub mod text;
pub mod time;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Event(pub usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Handle(pub usize);

#[repr(C)]
pub struct TableHeader {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    CRC32: u32,
    Reserved: u32
}
