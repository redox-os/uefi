//! A library to write code that interacts with the UEFI firmware.
//!
//! The documentation is based on UEFI version 2.7A which can be found
//! [here](http://www.uefi.org/sites/default/files/resources/UEFI%20Spec%202_7_A%20Sept%206.pdf).
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

/// Handle to an event structure.
///
/// Type VOID *.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Event(pub usize);

/// A collection of related interfaces.
///
/// Type VOID *.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Handle(pub usize);

/// Data structure that precedes all of the standard EFI table types.
#[repr(C)]
pub struct TableHeader {
    /// A 64-bit signature that identifies the type of table that follows.
    /// Unique signatures have been generated for the EFI System
    /// Table, the EFI Boot Services Table, and the EFI Runtime Services
    /// Table.
    Signature: u64,
    /// The revision of the EFI Specification to which this table
    /// conforms. The upper 16 bits of this field contain the major
    /// revision value, and the lower 16 bits contain the minor revision
    /// value. The minor revision values are binary coded decimals and
    /// are limited to the range of 00..99.
    ///
    /// When printed or displayed UEFI spec revision is referred as
    /// (Major revision).(Minor revision upper decimal).(Minor revision
    /// lower decimal) or (Major rev ision).(Minor revision upper
    /// decimal) in case Minor revision lower decimal is set to 0. For
    /// example:
    ///
    /// A specification with the revision value ((2<<16) | (30)) would
    /// be referred as 2.3;
    ///
    /// A specification with the revision value ((2<<16) | (31)) would
    /// be referred as 2.3.1
    Revision: u32,
    /// The size, in bytes, of the entire table including the
    /// EFI_TABLE_HEADER.
    HeaderSize: u32,
    ///The 32-bit CRC for the entire table. This value is computed by
    /// setting this field to 0, and co mputing the 32-bit CRC for
    /// HeaderSize bytes.
    CRC32: u32,
    /// Reserved field that must be set to 0.
    Reserved: u32,
}
