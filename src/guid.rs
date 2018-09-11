use core::fmt;

pub const NULL_GUID: Guid = Guid(
    0x00000000,
    0x0000,
    0x0000,
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
);
pub const MPS_TABLE_GUID: Guid = Guid(
    0xeb9d2d2f,
    0x2d88,
    0x11d3,
    [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);
pub const ACPI_TABLE_GUID: Guid = Guid(
    0xeb9d2d30,
    0x2d88,
    0x11d3,
    [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);
pub const ACPI_20_TABLE_GUID: Guid = Guid(
    0x8868e871,
    0xe4f1,
    0x11d3,
    [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
);
pub const SMBIOS_TABLE_GUID: Guid = Guid(
    0xeb9d2d31,
    0x2d88,
    0x11d3,
    [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);
pub const SMBIOS3_TABLE_GUID: Guid = Guid(
    0xf2fd1544,
    0x9794,
    0x4a2c,
    [0x99, 0x2e, 0xe5, 0xbb, 0xcf, 0x20, 0xe3, 0x94],
);
pub const SAL_SYSTEM_TABLE_GUID: Guid = Guid(
    0xeb9d2d32,
    0x2d88,
    0x11d3,
    [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);
pub const HCDP_TABLE_GUID: Guid = Guid(
    0xf951938d,
    0x620b,
    0x42ef,
    [0x82, 0x79, 0xa8, 0x4b, 0x79, 0x61, 0x78, 0x98],
);
pub const UGA_IO_PROTOCOL_GUID: Guid = Guid(
    0x61a4d49e,
    0x6f68,
    0x4f1b,
    [0xb9, 0x22, 0xa8, 0x6e, 0xed, 0x0b, 0x07, 0xa2],
);
pub const SIMPLE_TEXT_OUTPUT_GUID: Guid = Guid(
    0x387477c2,
    0x69c7,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const GLOBAL_VARIABLE_GUID: Guid = Guid(
    0x8be4df61,
    0x93ca,
    0x11d2,
    [0xaa, 0x0d, 0x00, 0xe0, 0x98, 0x03, 0x2b, 0x8c],
);
pub const UV_SYSTEM_TABLE_GUID: Guid = Guid(
    0x3b13a7d4,
    0x633e,
    0x11dd,
    [0x93, 0xec, 0xda, 0x25, 0x56, 0xd8, 0x95, 0x93],
);
pub const LINUX_EFI_CRASH_GUID: Guid = Guid(
    0xcfc8fc79,
    0xbe2e,
    0x4ddc,
    [0x97, 0xf0, 0x9f, 0x98, 0xbf, 0xe2, 0x98, 0xa0],
);
/// Can be used on any image handle to obtain information about the loaded image.
pub const LOADED_IMAGE_PROTOCOL_GUID: Guid = Guid(
    0x5b1b31a1,
    0x9562,
    0x11d2,
    [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
/// Provides a basic abstraction to set video modes and copy pixels to and from the graphics
/// controller’s frame buffer. The linear address of the hardware frame buffer is also exposed so
/// software can write directly to the video hardware.
pub const GRAPHICS_OUTPUT_PROTOCOL_GUID: Guid = Guid(
    0x9042a9de,
    0x23dc,
    0x4a38,
    [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
);
pub const UGA_PROTOCOL_GUID: Guid = Guid(
    0x982c298b,
    0xf4fa,
    0x41cb,
    [0xb8, 0x38, 0x77, 0xaa, 0x68, 0x8f, 0xb8, 0x39],
);
pub const PCI_IO_PROTOCOL_GUID: Guid = Guid(
    0x4cf5b200,
    0x68b8,
    0x4ca5,
    [0x9e, 0xec, 0xb2, 0x3e, 0x3f, 0x50, 0x02, 0x9a],
);
pub const FILE_INFO_ID: Guid = Guid(
    0x09576e92,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const SYSTEM_RESOURCE_TABLE_GUID: Guid = Guid(
    0xb122a263,
    0x3661,
    0x4f68,
    [0x99, 0x29, 0x78, 0xf8, 0xb0, 0xd6, 0x21, 0x80],
);
/// This protocol provides control over block devices.
pub const BLOCK_IO_GUID: Guid = Guid(
    0x964e5b21,
    0x6459,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const FILE_SYSTEM_GUID: Guid = Guid(
    0x964e5b22,
    0x6459,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const LOAD_FILE_GUID: Guid = Guid(
    0x56ec3091,
    0x954c,
    0x11d2,
    [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const DEVICE_PATH_GUID: Guid = Guid(
    0x09576e91,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const DEVICE_TREE_GUID: Guid = Guid(
    0xb1b621d5,
    0xf19c,
    0x41a5,
    [0x83, 0x0b, 0xd9, 0x15, 0x2c, 0x69, 0xaa, 0xe0],
);
pub const PROPERTIES_TABLE_GUID: Guid = Guid(
    0x880aaca3,
    0x4adc,
    0x4a04,
    [0x90, 0x79, 0xb7, 0x47, 0x34, 0x08, 0x25, 0xe5],
);
pub const RNG_PROTOCOL_GUID: Guid = Guid(
    0x3152bca5,
    0xeade,
    0x433d,
    [0x86, 0x2e, 0xc0, 0x1c, 0xdc, 0x29, 0x1f, 0x44],
);
pub const RNG_ALGORITHM_RAW: Guid = Guid(
    0xe43176d7,
    0xb6e8,
    0x4827,
    [0xb7, 0x84, 0x7f, 0xfd, 0xc4, 0xb6, 0x85, 0x61],
);
pub const MEMORY_ATTRIBUTES_TABLE_GUID: Guid = Guid(
    0xdcfa911d,
    0x26eb,
    0x469f,
    [0xa2, 0x20, 0x38, 0xb7, 0xdc, 0x46, 0x12, 0x20],
);
pub const CONSOLE_OUT_DEVICE_GUID: Guid = Guid(
    0xd3b36f2c,
    0xd551,
    0x11d4,
    [0x9a, 0x46, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);
pub const SECTION_TIANO_COMPRESS_GUID: Guid = Guid(
    0xa31280ad,
    0x481e,
    0x41b6,
    [0x95, 0xe8, 0x12, 0x7f, 0x4c, 0x98, 0x47, 0x79],
);
pub const SECTION_LZMA_COMPRESS_GUID: Guid = Guid(
    0xee4e5898,
    0x3914,
    0x4259,
    [0x9d, 0x6e, 0xdc, 0x7b, 0xd7, 0x94, 0x03, 0xcf],
);
pub const DXE_SERVICES_TABLE_GUID: Guid = Guid(
    0x05ad34ba,
    0x6f02,
    0x4214,
    [0x95, 0x2e, 0x4d, 0xa0, 0x39, 0x8e, 0x2b, 0xb9],
);
pub const HOB_LIST_GUID: Guid = Guid(
    0x7739f24c,
    0x93d7,
    0x11d4,
    [0x9a, 0x3a, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);
pub const MEMORY_TYPE_INFORMATION_GUID: Guid = Guid(
    0x4c19049f,
    0x4137,
    0x4dd3,
    [0x9c, 0x10, 0x8b, 0x97, 0xa8, 0x3f, 0xfd, 0xfa],
);
pub const DEBUG_IMAGE_INFO_TABLE_GUID: Guid = Guid(
    0x49152e77,
    0x1ada,
    0x4764,
    [0xb7, 0xa2, 0x7a, 0xfe, 0xfe, 0xd9, 0x5e, 0x8b],
);
pub const SHELL_GUID: Guid = Guid(
    0x6302d008,
    0x7f9b,
    0x4f30,
    [0x87, 0xac, 0x60, 0xc9, 0xfe, 0xf5, 0xda, 0x4e],
);
pub const SHELL_PARAMETERS_GUID: Guid = Guid(
    0x752f3136,
    0x4e16,
    0x4fdc,
    [0xa2, 0x2a, 0xe5, 0xf4, 0x68, 0x12, 0xf4, 0xca],
);
/// Provides services that allow information about a pointer device to be retrieved.
pub const SIMPLE_POINTER_GUID: Guid = Guid(
    0x31878c87,
    0x0b75,
    0x11d5,
    [0x9a, 0x4f, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GuidKind {
    Null,
    Mps,
    Acpi,
    Acpi2,
    Smbios,
    Smbios3,
    SalSystem,
    Hcdp,
    UgaIo,
    GlobalVariable,
    SimpleTextOutput,
    UvSystem,
    LinuxEfiCrash,
    /// Can be used on any image handle to obtain information about the loaded image.
    LoadedImage,
    /// Provides a basic abstraction to set video modes and copy pixels to and from the graphics
    /// controller’s frame buffer. The linear address of the hardware frame buffer is also exposed so
    /// software can write directly to the video hardware.
    GraphicsOutput,
    Uga,
    PciIo,
    FileInfo,
    SystemResource,
    /// This protocol provides control over block devices.
    BlockIo,
    FileSystem,
    LoadFile,
    DevicePath,
    DeviceTree,
    Properties,
    Rng,
    RngAlrorithm,
    MemoryAttributes,
    ConsoleOut,
    SectionTianoCompress,
    SectionLzmaCompress,
    DxeServices,
    HobList,
    MemoryTypeInformation,
    DebugImageInfo,
    Shell,
    ShellParameters,
    /// Provides services that allow information about a pointer device to be retrieved.
    SimplePointer,
    Unknown,
}

impl Guid {
    pub fn kind(&self) -> GuidKind {
        match *self {
            NULL_GUID => GuidKind::Null,
            MPS_TABLE_GUID => GuidKind::Mps,
            ACPI_TABLE_GUID => GuidKind::Acpi,
            ACPI_20_TABLE_GUID => GuidKind::Acpi2,
            SMBIOS_TABLE_GUID => GuidKind::Smbios,
            SMBIOS3_TABLE_GUID => GuidKind::Smbios3,
            SAL_SYSTEM_TABLE_GUID => GuidKind::SalSystem,
            HCDP_TABLE_GUID => GuidKind::Hcdp,
            UGA_IO_PROTOCOL_GUID => GuidKind::UgaIo,
            GLOBAL_VARIABLE_GUID => GuidKind::GlobalVariable,
            SIMPLE_TEXT_OUTPUT_GUID => GuidKind::SimpleTextOutput,
            UV_SYSTEM_TABLE_GUID => GuidKind::UvSystem,
            LINUX_EFI_CRASH_GUID => GuidKind::LinuxEfiCrash,
            LOADED_IMAGE_PROTOCOL_GUID => GuidKind::LoadedImage,
            GRAPHICS_OUTPUT_PROTOCOL_GUID => GuidKind::GraphicsOutput,
            UGA_PROTOCOL_GUID => GuidKind::Uga,
            PCI_IO_PROTOCOL_GUID => GuidKind::PciIo,
            FILE_INFO_ID => GuidKind::FileInfo,
            SYSTEM_RESOURCE_TABLE_GUID => GuidKind::SystemResource,
            BLOCK_IO_GUID => GuidKind::BlockIo,
            FILE_SYSTEM_GUID => GuidKind::FileSystem,
            LOAD_FILE_GUID => GuidKind::LoadFile,
            DEVICE_PATH_GUID => GuidKind::DevicePath,
            DEVICE_TREE_GUID => GuidKind::DeviceTree,
            PROPERTIES_TABLE_GUID => GuidKind::Properties,
            RNG_PROTOCOL_GUID => GuidKind::Rng,
            RNG_ALGORITHM_RAW => GuidKind::RngAlrorithm,
            MEMORY_ATTRIBUTES_TABLE_GUID => GuidKind::MemoryAttributes,
            CONSOLE_OUT_DEVICE_GUID => GuidKind::ConsoleOut,
            SECTION_TIANO_COMPRESS_GUID => GuidKind::SectionTianoCompress,
            SECTION_LZMA_COMPRESS_GUID => GuidKind::SectionLzmaCompress,
            DXE_SERVICES_TABLE_GUID => GuidKind::DxeServices,
            HOB_LIST_GUID => GuidKind::HobList,
            MEMORY_TYPE_INFORMATION_GUID => GuidKind::MemoryTypeInformation,
            DEBUG_IMAGE_INFO_TABLE_GUID => GuidKind::DebugImageInfo,
            SHELL_GUID => GuidKind::Shell,
            SHELL_PARAMETERS_GUID => GuidKind::ShellParameters,
            SIMPLE_POINTER_GUID => GuidKind::SimplePointer,
            _ => GuidKind::Unknown,
        }
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:>08x}, {:>04x}, {:>04x}, [", self.0, self.1, self.2)?;
        for (i, b) in self.3.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{:>02x}", b)?;
        }
        write!(f, "])")?;
        Ok(())
    }
}
