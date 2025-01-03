// SPDX-License-Identifier: MIT

use core::ops::RangeInclusive;

use crate::memory::PhysicalAddress;
use crate::prelude::*;

/// EFI_FVB_ATTRIBUTES_2
pub type FvbAttributes2 = u32;

/// EFI_FV_ATTRIBUTES
pub type FvAttributes = u64;

/// EFI_FV_FILE_ATTRIBUTES
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct FvFileAttributes(pub u32);

impl FvFileAttributes {
    pub const ALIGNMENT: Self = Self(0x1F);
    pub const FIXED: Self = Self(1 << 8);
    pub const MEMORY_MAPPED: Self = Self(1 << 9);
}

/// EFI_FV_WRITE_POLICY
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct FvWritePolicy(pub u32);

impl FvWritePolicy {
    pub const UNRELIABLE_WRITE: Self = Self(0);
    pub const RELIABLE_WRITE: Self = Self(1);
}

/// EFI_FV_FILETYPE
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct FvFiletype(pub u8);

impl FvFiletype {
    pub const ALL: Self = Self(0x00);
    pub const RAW: Self = Self(0x01);
    pub const FREEFORM: Self = Self(0x02);
    pub const SECURITY_CORE: Self = Self(0x03);
    pub const PEI_CORE: Self = Self(0x04);
    pub const DXE_CORE: Self = Self(0x05);
    pub const PEIM: Self = Self(0x06);
    pub const DRIVER: Self = Self(0x07);
    pub const COMBINED_PEIM_DRIVER: Self = Self(0x08);
    pub const APPLICATION: Self = Self(0x09);
    pub const MM: Self = Self(0x0A);
    pub const FIRMWARE_VOLUME_IMAGE: Self = Self(0x0B);
    pub const COMBINED_MM_DXE: Self = Self(0x0C);
    pub const MM_CORE: Self = Self(0x0D);
    pub const MM_STANDALONE: Self = Self(0x0E);
    pub const MM_CORE_STANDALONE: Self = Self(0x0F);
    pub const FFS_PAD: Self = Self(0xF0);

    pub const OEM_RANGE: RangeInclusive<u8> = 0xC0..=0xDF;
    pub const DEBUG_RANGE: RangeInclusive<u8> = 0xE0..=0xEF;
    pub const FFS_RANGE: RangeInclusive<u8> = 0xF0..=0xFF;
}

/// EFI_SECTION_TYPE
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct SectionType(pub u8);

impl SectionType {
    pub const ALL: Self = Self(0x00);
    pub const COMPRESSION: Self = Self(0x01);
    pub const GUID_DEFINED: Self = Self(0x02);
    pub const DISPOSABLE: Self = Self(0x03);
    pub const PE32: Self = Self(0x10);
    pub const PIC: Self = Self(0x11);
    pub const TE: Self = Self(0x12);
    pub const DXE_DEPEX: Self = Self(0x13);
    pub const VERSION: Self = Self(0x14);
    pub const USER_INTERFACE: Self = Self(0x15);
    pub const COMPATIBILITY16: Self = Self(0x16);
    pub const FIRMWARE_VOLUME_IMAGE: Self = Self(0x17);
    pub const FREEFORM_SUBTYPE_GUID: Self = Self(0x18);
    pub const RAW: Self = Self(0x19);
    pub const PEI_DEPEX: Self = Self(0x1B);
    pub const MM_DEPEX: Self = Self(0x1C);
}

/// EFI_FV_WRITE_FILE_DATA
#[derive(Debug)]
#[repr(C)]
pub struct FvWriteFileData {
    pub NameGuid: *const Guid,
    pub Type: FvFiletype,
    pub FileAttributes: FvFileAttributes,
    pub Buffer: *const u8,
    pub BufferSize: u32,
}

/// EFI_FIRMWARE_VOLUME2_PROTOCOL
#[derive(Debug)]
#[repr(C)]
pub struct FirmwareVolume2 {
    pub GetVolumeAttributes:
        extern "efiapi" fn(This: &Self, FvAttributes: &mut FvAttributes) -> Status,
    pub SetVolumeAttributes:
        extern "efiapi" fn(This: &Self, FvAttributes: &mut FvAttributes) -> Status,
    pub ReadFile: extern "efiapi" fn(
        This: &Self,
        NameGuid: &Guid,
        Buffer: &mut *mut u8,
        BufferSize: &mut usize,
        FoundType: &mut FvFiletype,
        FileAttributes: &mut FvFileAttributes,
        AuthenticationStatus: &mut u32,
    ) -> Status,
    pub ReadSection: extern "efiapi" fn(
        This: &Self,
        NameGuid: &Guid,
        SectionType: SectionType,
        SectionInstance: usize,
        Buffer: &mut *mut u8,
        BufferSize: &mut usize,
        AuthenticationStatus: &mut u32,
    ) -> Status,
    pub WriteFile: extern "efiapi" fn(
        This: &Self,
        NumberOfFiles: u32,
        WritePolicy: FvWritePolicy,
        FileData: &FvWriteFileData,
    ) -> Status,
    pub GetNextFile: extern "efiapi" fn(
        This: &Self,
        Key: *mut u8,
        FileType: &mut FvFiletype,
        NameGuid: &mut Guid,
        Attributes: &mut FvFileAttributes,
        Size: &mut usize,
    ) -> Status,
    pub KeySize: u32,
    pub ParentHandle: Handle,
    pub GetInfo: extern "efiapi" fn(
        This: &Self,
        InformationType: &Guid,
        BufferSize: &mut usize,
        Buffer: *mut u8,
    ) -> Status,
    pub SetInfo: extern "efiapi" fn(
        This: &Self,
        InformationType: &Guid,
        BufferSize: usize,
        Buffer: *const u8,
    ) -> Status,
}

impl FirmwareVolume2 {
    pub const GUID: Guid = guid!("220e73b6-6bdb-4413-8405-b974b108619a");
}

/// EFI_FIRMWARE_VOLUME_BLOCK2_PROTOCOL
#[derive(Debug)]
#[repr(C)]
pub struct FirmwareVolumeBlock2 {
    pub GetAttributes: extern "efiapi" fn(This: &Self, Attributes: &mut FvbAttributes2) -> Status,
    pub SetAttributes: extern "efiapi" fn(This: &Self, Attributes: &mut FvbAttributes2) -> Status,
    pub GetPhysicalAddress:
        extern "efiapi" fn(This: &Self, Address: &mut PhysicalAddress) -> Status,
    pub GetBlockSize: extern "efiapi" fn(
        This: &Self,
        Lba: u64,
        BlockSize: &mut usize,
        NumberOfBlocks: &mut usize,
    ) -> Status,
    pub Read: extern "efiapi" fn(
        This: &Self,
        Lba: u64,
        Offset: usize,
        NumBytes: &mut usize,
        Buffer: *mut u8,
    ) -> Status,
    pub Write: extern "efiapi" fn(
        This: &Self,
        Lba: u64,
        Offset: usize,
        NumBytes: &mut usize,
        Buffer: *mut u8,
    ) -> Status,
    // TODO: Change to efiapi
    EraseBlocks: extern "C" fn(This: &Self, ...) -> Status,
    pub ParentHandle: Handle,
}

impl FirmwareVolumeBlock2 {
    pub const GUID: Guid = guid!("8f644fa9-e850-4db1-9ce2-0b44698e8da4");
    pub const LBA_LIST_TERMINATOR: u64 = u64::MAX;
}
