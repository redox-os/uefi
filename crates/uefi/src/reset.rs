// SPDX-License-Identifier: MIT

/// `EFI_RESET_TYPE`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct ResetType(pub u32);

impl ResetType {
    pub const COLD: Self = Self(0);
    pub const WARM: Self = Self(1);
    pub const SHUTDOWN: Self = Self(2);
    pub const PLATFORM_SPECIFIC: Self = Self(3);
}
