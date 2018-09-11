/// Handles the memory management specific functionality of UEFI.

/// Represents a physical address.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhysicalAddress(pub u64);

/// Represents a virtual address.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VirtualAddress(pub u64);

/// Describes the different areas of memory in the memory map.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct MemoryDescriptor {
    /// Type of the memory region.
    pub Type: u32,
    /// Physical address of the first byte in the memory region.
    /// PhysicalStart must be aligned on a 4 KiB boundary, and must
    /// not be above 0xfffffffffffff000.
    pub PhysicalStart: PhysicalAddress,
    /// Virtual address of the first byte in the memory region.
    /// VirtualStart must be aligned on a 4 KiB boundary, and must not
    /// be above 0xfffffffffffff000.
    pub VirtualStart: VirtualAddress,
    /// Number of 4 KiB pages in the memory region.
    /// NumberOfPages must not be 0, and must not be any value that
    /// would represent a memory page with a start address, either physical
    /// or virtual, above 0xfffffffffffff000.
    pub NumberOfPages: u64,
    /// Attributes of the memory region that describe the bit mask of
    /// capabilities for that memory region, and not necessarily the current
    /// settings for that memory region.
    pub Attribute: u64,
}

/// Represents the different types memory can have.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum MemoryType {
    /// Not usable.
    EfiReservedMemoryType,
    /// The code portions of a loaded application.
    EfiLoaderCode,
    /// The data portions of a loaded application and the default data allocation
    /// type used by an application to allocate pool memory.
    EfiLoaderData,
    /// The code portions of a loaded Boot Service Driver.
    EfiBootServicesCode,
    /// The data portions of a loaded Boot Serve Driver, and the default data
    /// allocation type used by a Boot Services Driver to allocate pool memory.
    EfiBootServicesData,
    /// The code portions of a loaded Runtime Driver.
    EfiRuntimeServicesCode,
    /// The data portions of a loaded Runtime Driver and the default
    /// data allocation type used by a Runtime Driver to allocate pool memory.
    EfiRuntimeServicesData,
    /// Free (unallocated) memory.
    EfiConventionalMemory,
    /// Memory in which errors have been detected.
    EfiUnusableMemory,
    /// Memory that holds the ACPI tables.
    EfiACPIReclaimMemory,
    /// Address space reserved for use by the firmware.
    EfiACPIMemoryNVS,
    /// Used by system firmware to request that a memory-mapped IO region
    /// be mapped by the OS to a virtual address so it can be accessed by EFI runtime services.
    EfiMemoryMappedIO,
    /// System memory-mapped IO region that is used to translate memory
    /// cycles to IO cycles by the processor.
    EfiMemoryMappedIOPortSpace,
    /// Address space reserved by the firmware for code that is part of the processor.
    EfiPalCode,
    /// A memory region that operates as EfiConventionalMemory,
    /// however it happens to also support byte-addressable non-volatility.
    EfiPersistentMemory,
    EfiMaxMemoryType,
}
