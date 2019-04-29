use crate::hii::{FontStyle, StringId};
use crate::util::{c_str_at_end, w_str_at_end};

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum SibtKind {
    End = 0x00,
    StringScsu = 0x10,
    StringScsuFont = 0x11,
    StringsScsu = 0x12,
    StringsScsuFont = 0x13,
    StringUcs2 = 0x14,
    StringUcs2Font = 0x15,
    StringsUcs2 = 0x16,
    StringsUcs2Font = 0x17,
    Duplicate = 0x20,
    Skip2 = 0x21,
    Skip1 = 0x22,
    Ext1 = 0x30,
    Ext2 = 0x31,
    Ext4 = 0x32,
    Font = 0x40,
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtHeader {
    pub BlockType: SibtKind
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtDuplicate {
    pub Header: SibtHeader,
    pub StringId: StringId,
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtEnd {
    pub Header: SibtHeader,
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtExt1 {
    pub Header: SibtHeader,
    pub BlockType2: u8,
    pub Length: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtExt2 {
    pub Header: SibtHeader,
    pub BlockType2: u8,
    pub Length: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtExt4 {
    pub Header: SibtHeader,
    pub BlockType2: u8,
    pub Length: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtFont {
    pub Header: SibtHeader,
    pub FontId: u8,
    pub FontSize: u16,
    pub FontStyle: FontStyle
}

impl SibtFont {
    pub fn FontName(&self) -> &[u8] {
        unsafe { c_str_at_end(self, 0) }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtSkip1 {
    pub Header: SibtHeader,
    pub SkipCount: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtSkip2 {
    pub Header: SibtHeader,
    // For some reason using u16 caused the size of the struct to be 4
    // It seems that it should actually be 3
    pub SkipCountBytes: [u8; 2],
}

impl SibtSkip2 {
    // TODO: big endian?
    pub fn SkipCount(&self) -> u16 {
        self.SkipCountBytes[0] as u16 |
        (self.SkipCountBytes[1] as u16) << 8
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtStringScsu {
    pub Header: SibtHeader,
}

impl SibtStringScsu {
    pub fn StringText(&self) -> &[u8] {
        unsafe { c_str_at_end(self, 0) }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtStringScsuFont {
    pub Header: SibtHeader,
    pub FontIdentifier: u8,
}

impl SibtStringScsuFont {
    pub fn StringText(&self) -> &[u8] {
        unsafe { c_str_at_end(self, 0) }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtStringsScsu {
    pub Header: SibtHeader,
    pub StringCount: u16,
}

impl SibtStringsScsu {
    pub fn StringText(&self, string: u16) -> &[u8] {
        if string < self.StringCount {
            unsafe { c_str_at_end(self, string as usize) }
        } else {
            &[]
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtStringsScsuFont {
    pub Header: SibtHeader,
    pub FontIdentifier: u8,
    pub StringCount: u16,
}

impl SibtStringsScsuFont {
    pub fn StringText(&self, string: u16) -> &[u8] {
        if string < self.StringCount {
            unsafe { c_str_at_end(self, string as usize) }
        } else {
            &[]
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtStringUcs2 {
    pub Header: SibtHeader,
}

impl SibtStringUcs2 {
    pub fn StringText(&self) -> &[u16] {
        unsafe { w_str_at_end(self, 0) }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtStringUcs2Font {
    pub Header: SibtHeader,
    pub FontIdentifier: u8,
}

impl SibtStringUcs2Font {
    pub fn StringText(&self) -> &[u16] {
        unsafe { w_str_at_end(self, 0) }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtStringsUcs2 {
    pub Header: SibtHeader,
    pub StringCount: u16,
}

impl SibtStringsUcs2 {
    pub fn StringText(&self, string: u16) -> &[u16] {
        if string < self.StringCount {
            unsafe { w_str_at_end(self, string as usize) }
        } else {
            &[]
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SibtStringsUcs2Font {
    pub Header: SibtHeader,
    pub FontIdentifier: u8,
    pub StringCount: u16,
}

impl SibtStringsUcs2Font {
    pub fn StringText(&self, string: u16) -> &[u16] {
        if string < self.StringCount {
            unsafe { w_str_at_end(self, string as usize) }
        } else {
            &[]
        }
    }
}
