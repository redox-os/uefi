use core::{mem, slice};

use crate::guid::Guid;
use crate::util::c_str_at_end;

use super::StringId;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum HiiPackageKind {
    /// Pseudo-package type used when exporting package lists
    All = 0x00,
    /// Package type where the format of the data is specified using a GUID immediately following the package header
    Guid = 0x01,
    /// Forms package
    Forms = 0x02,
    /// Strings package
    Strings = 0x04,
    /// Fonts package
    Fonts = 0x05,
    /// Images package
    Images = 0x06,
    /// Simplified (8x19, 16x19) Fonts package
    SimpleFonts = 0x07,
    /// Binary-encoded device path
    DevicePath = 0x08,
    /// Keyboard Layout package
    KeyboardLayout = 0x09,
    /// Animations package
    Animations = 0x0A,
    /// Used to mark the end of a package list
    End = 0xDF,
    // Package types reserved for firmware implementations
    //SystemBegin = 0xE0,
    //SystemEnd = 0xFF,
}

#[derive(Debug)]
#[repr(C)]
pub struct HiiPackageHeader {
    pub Length_Kind: u32,
}

impl HiiPackageHeader {
    pub fn Length(&self) -> u32 {
        self.Length_Kind & 0xFFFFFF
    }

    pub fn Kind(&self) -> HiiPackageKind {
        unsafe { mem::transmute((self.Length_Kind >> 24) as u8) }
    }

    pub fn Data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const Self).add(1) as *const u8,
                self.Length() as usize - mem::size_of::<Self>()
            )
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct HiiStringPackageHeader {
    pub Header: HiiPackageHeader,
    pub HdrSize: u32,
    pub StringInfoOffset: u32,
    pub LanguageWindow: [u16; 16],
    pub LanguageName: StringId,
}

impl HiiStringPackageHeader {
    pub fn Language(&self) -> &[u8] {
        unsafe { c_str_at_end(self, 0) }
    }

    pub fn StringInfo(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const Self as *const u8).add(
                    self.StringInfoOffset as usize
                ),
                self.Header.Length() as usize - self.StringInfoOffset as usize
            )
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct HiiPackageListHeader {
    pub PackageListGuid: Guid,
    pub PackageLength: u32,
}

impl HiiPackageListHeader {
    pub fn Data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const Self).add(1) as *const u8,
                self.PackageLength as usize - mem::size_of::<Self>()
            )
        }
    }
}
