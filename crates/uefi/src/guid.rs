// SPDX-License-Identifier: MIT

//! # Globally Unique Identifier
//!
//! GUIDs are Microsoft's variant of [UUIDs][wiki]. The first 3 groups are
//! stored in the native endianness of the platform.
//!
//! ## References
//!
//! - [UEFI Specification, Version 2.10][UEFI Spec]: Appendix A - GUID and Time
//!   Formats
//! - [RFC 9562: Universally Unique IDentifiers (UUIDs)][rfc9562]
//! - [Microsoft GUID structure][guiddef]
//!
//! [wiki]: https://en.wikipedia.org/wiki/Universally_unique_identifier
//! [guiddef]: https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid
//! [rfc9562]: https://www.rfc-editor.org/rfc/rfc9562.html
//! [UEFI Spec]: https://uefi.org/sites/default/files/resources/UEFI_Spec_2_10_Aug29.pdf

use core::fmt;

#[macro_export]
macro_rules! guid {
    ($guid_str:literal) => {
        $crate::guid::Guid::parse_str($guid_str)
    };
}

/// String length of a GUID with hyphens.
const HYPHENATED_LEN: usize = 36;

pub const NULL_GUID: Guid = guid!("00000000-0000-0000-0000-000000000000");
pub const MPS_TABLE_GUID: Guid = guid!("eb9d2d2f-2d88-11d3-9a16-0090273fc14d");
pub const ACPI_TABLE_GUID: Guid = guid!("eb9d2d30-2d88-11d3-9a16-0090273fc14d");
pub const ACPI_20_TABLE_GUID: Guid = guid!("8868e871-e4f1-11d3-bc22-0080c73c8881");
pub const SMBIOS_TABLE_GUID: Guid = guid!("eb9d2d31-2d88-11d3-9a16-0090273fc14d");
pub const SMBIOS3_TABLE_GUID: Guid = guid!("f2fd1544-9794-4a2c-992e-e5bbcf20e394");
pub const SAL_SYSTEM_TABLE_GUID: Guid = guid!("eb9d2d32-2d88-11d3-9a16-0090273fc14d");
pub const HCDP_TABLE_GUID: Guid = guid!("f951938d-620b-42ef-8279-a84b79617898");
pub const UGA_IO_PROTOCOL_GUID: Guid = guid!("61a4d49e-6f68-4f1b-b922-a86eed0b07a2");
pub const SIMPLE_TEXT_OUTPUT_GUID: Guid = guid!("387477c2-69c7-11d2-8e39-00a0c969723b");
pub const GLOBAL_VARIABLE_GUID: Guid = guid!("8be4df61-93ca-11d2-aa0d-00e098032b8c");
pub const UV_SYSTEM_TABLE_GUID: Guid = guid!("3b13a7d4-633e-11dd-93ec-da2556d89593");
pub const LINUX_EFI_CRASH_GUID: Guid = guid!("cfc8fc79-be2e-4ddc-97f0-9f98bfe298a0");
pub const LOADED_IMAGE_PROTOCOL_GUID: Guid = guid!("5b1b31a1-9562-11d2-8e3f-00a0c969723b");
pub const LOADED_IMAGE_DEVICE_PATH_GUID: Guid = guid!("bc62157e-3e33-4fec-9920-2d3b36d750df");
pub const GRAPHICS_OUTPUT_PROTOCOL_GUID: Guid = guid!("9042a9de-23dc-4a38-96fb-7aded080516a");
pub const UGA_PROTOCOL_GUID: Guid = guid!("982c298b-f4fa-41cb-b838-77aa688fb839");
pub const PCI_IO_PROTOCOL_GUID: Guid = guid!("4cf5b200-68b8-4ca5-9eec-b23e3f50029a");
pub const FILE_INFO_ID: Guid = guid!("09576e92-6d3f-11d2-8e39-00a0c969723b");
pub const SYSTEM_RESOURCE_TABLE_GUID: Guid = guid!("b122a263-3661-4f68-9929-78f8b0d62180");
pub const BLOCK_IO_GUID: Guid = guid!("964e5b21-6459-11d2-8e39-00a0c969723b");
pub const FILE_SYSTEM_GUID: Guid = guid!("964e5b22-6459-11d2-8e39-00a0c969723b");
pub const LOAD_FILE_GUID: Guid = guid!("56ec3091-954c-11d2-8e3f-00a0c969723b");
pub const DEVICE_PATH_GUID: Guid = guid!("09576e91-6d3f-11d2-8e39-00a0c969723b");
pub const DEVICE_TREE_GUID: Guid = guid!("b1b621d5-f19c-41a5-830b-d9152c69aae0");
pub const PROPERTIES_TABLE_GUID: Guid = guid!("880aaca3-4adc-4a04-9079-b747340825e5");
pub const RNG_PROTOCOL_GUID: Guid = guid!("3152bca5-eade-433d-862e-c01cdc291f44");
pub const RNG_ALGORITHM_RAW: Guid = guid!("e43176d7-b6e8-4827-b784-7ffdc4b68561");
pub const MEMORY_ATTRIBUTES_TABLE_GUID: Guid = guid!("dcfa911d-26eb-469f-a220-38b7dc461220");
pub const CONSOLE_OUT_DEVICE_GUID: Guid = guid!("d3b36f2c-d551-11d4-9a46-0090273fc14d");
pub const SECTION_TIANO_COMPRESS_GUID: Guid = guid!("a31280ad-481e-41b6-95e8-127f4c984779");
pub const SECTION_LZMA_COMPRESS_GUID: Guid = guid!("ee4e5898-3914-4259-9d6e-dc7bd79403cf");
pub const DXE_SERVICES_TABLE_GUID: Guid = guid!("05ad34ba-6f02-4214-952e-4da0398e2bb9");
pub const HOB_LIST_GUID: Guid = guid!("7739f24c-93d7-11d4-9a3a-0090273fc14d");
pub const MEMORY_TYPE_INFORMATION_GUID: Guid = guid!("4c19049f-4137-4dd3-9c10-8b97a83ffdfa");
pub const DEBUG_IMAGE_INFO_TABLE_GUID: Guid = guid!("49152e77-1ada-4764-b7a2-7afefed95e8b");
pub const SHELL_GUID: Guid = guid!("6302d008-7f9b-4f30-87ac-60c9fef5da4e");
pub const SHELL_PARAMETERS_GUID: Guid = guid!("752f3136-4e16-4fdc-a22a-e5f46812f4ca");
pub const SIMPLE_POINTER_GUID: Guid = guid!("31878c87-0b75-11d5-9a4f-0090273fc14d");
pub const HII_DATABASE_GUID: Guid = guid!("ef9fc172-a1b2-4693-b327-6d32fc416042");
pub const COMPONENT_NAME2_GUID: Guid = guid!("6a7a5cff-e8d9-4f70-bada-75ab3025ce14");

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
    LoadedImage,
    GraphicsOutput,
    Uga,
    PciIo,
    FileInfo,
    SystemResource,
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
    SimplePointer,
    HiiDatabase,
    ComponentName2,
    Unknown,
}

impl Guid {
    /// Converts a string literal to a GUID.
    ///
    /// The string must be in the form "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".
    /// Hex digits may be either upper case or lower case.
    ///
    /// # Panics
    ///
    /// Panics if the string literal is not in the standard form or contains
    /// a value that is not a hex digit.
    pub const fn parse_str(literal: &str) -> Self {
        if literal.len() != HYPHENATED_LEN {
            panic!("invalid GUID length");
        }

        let bytes = literal.as_bytes();

        // Check hyphens
        if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' {
            panic!("invalid GUID format");
        }

        let mut raw = [0u8; 16];

        let mut i = 0;
        let mut j = 0;
        while i < HYPHENATED_LEN {
            if i == 8 || i == 13 || i == 18 || i == 23 {
                // Already checked hyphens; skip
                i += 1;
            }

            let hi = hex_to_u8(bytes[i]);
            let lo = hex_to_u8(bytes[i + 1]);

            let b = hi << 4 | lo;
            raw[j] = b;

            i += 2;
            j += 1;
        }

        let d1 = u32::from_be_bytes([raw[0], raw[1], raw[2], raw[3]]);
        let d2 = u16::from_be_bytes([raw[4], raw[5]]);
        let d3 = u16::from_be_bytes([raw[6], raw[7]]);
        let d4 = [
            raw[8], raw[9], raw[10], raw[11], raw[12], raw[13], raw[14], raw[15],
        ];

        Self(d1, d2, d3, d4)
    }

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
            HII_DATABASE_GUID => GuidKind::HiiDatabase,
            COMPONENT_NAME2_GUID => GuidKind::ComponentName2,
            _ => GuidKind::Unknown,
        }
    }
}

/// Converts a hex character to its integer value.
/// Panics on invalid values.
#[doc(hidden)]
const fn hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'A'..=b'F' => hex - b'A' + 10,
        b'a'..=b'f' => hex - b'a' + 10,
        _ => panic!("invalid hex value in GUID"),
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Per RFC 4122, hex digits are output as lower case characters.
        write!(f, "{:x}", &self)
    }
}

impl fmt::LowerHex for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            // Group 1
            self.0,
            // Group 2
            self.1,
            // Group 3
            self.2,
            // Group 4
            self.3[0],
            self.3[1],
            // Group 5
            self.3[2],
            self.3[3],
            self.3[4],
            self.3[5],
            self.3[6],
            self.3[7],
        )
    }
}

impl fmt::UpperHex for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            // Group 1
            self.0,
            // Group 2
            self.1,
            // Group 3
            self.2,
            // Group 4
            self.3[0],
            self.3[1],
            // Group 5
            self.3[2],
            self.3[3],
            self.3[4],
            self.3[5],
            self.3[6],
            self.3[7],
        )
    }
}
