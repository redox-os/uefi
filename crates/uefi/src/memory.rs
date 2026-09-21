// SPDX-License-Identifier: MIT

use core::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysicalAddress(pub u64);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtualAddress(pub u64);

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub Type: u32,
    pub PhysicalStart: PhysicalAddress,
    pub VirtualStart: VirtualAddress,
    pub NumberOfPages: u64,
    pub Attribute: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct MemoryType(pub u32);

impl MemoryType {
    /// Not used.
    pub const RESERVED: Self = Self(0);
    /// The code portions of a loaded application.
    /// (Note that UEFI OS loaders are UEFI applications.)
    pub const LOADER_CODE: Self = Self(1);
    /// The data portions of a loaded application and the default data allocation
    /// type used by an application to allocate pool memory.
    pub const LOADER_DATA: Self = Self(2);
    /// The code portions of a loaded Boot Services Driver.
    pub const BOOT_SERVICES_CODE: Self = Self(3);
    /// The data portions of a loaded Boot Serves Driver, and the default data
    /// allocation type used by a Boot Services Driver to allocate pool memory.
    pub const BOOT_SERVICES_DATA: Self = Self(4);
    /// The code portions of a loaded Runtime Services Driver.
    pub const RUNTIME_SERVICES_CODE: Self = Self(5);
    /// The data portions of a loaded Runtime Services Driver and the default
    /// data allocation type used by a Runtime Services Driver to allocate pool memory.
    pub const RUNTIME_SERVICES_DATA: Self = Self(6);
    /// Free (unallocated) memory.
    pub const CONVENTIONAL: Self = Self(7);
    /// Memory in which errors have been detected.
    pub const UNUSABLE: Self = Self(8);
    /// Memory that holds the ACPI tables.
    pub const ACPI_RECLAIM: Self = Self(9);
    /// Address space reserved for use by the firmware.
    pub const ACPI_NON_VOLATILE: Self = Self(10);
    /// Used by system firmware to request that a memory-mapped IO region
    /// be mapped by the OS to a virtual address so it can be accessed by EFI runtime services.
    pub const MMIO: Self = Self(11);
    /// System memory-mapped IO region that is used to translate memory
    /// cycles to IO cycles by the processor.
    pub const MMIO_PORT_SPACE: Self = Self(12);
    /// Address space reserved by the firmware for code that is part of the processor.
    pub const PAL_CODE: Self = Self(13);
    /// A memory region that operates as EfiConventionalMemory,
    /// however it happens to also support byte-addressable non-volatility.
    pub const PERSISTENT: Self = Self(14);
    /// System memory that has not been accepted by a corresponding call to
    /// the underlying isolation architecture.
    pub const UNACCEPTED: Self = Self(16);
    pub const MAX: Self = Self(16);

    pub const RESERVED_FOR_OEM: RangeInclusive<u32> = 0x7000_0000..=0x7FFF_FFFF;
    pub const RESERVED_FOR_OS_LOADER: RangeInclusive<u32> = 0x8000_0000..=0xFFFF_FFFF;
}
