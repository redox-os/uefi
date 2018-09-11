//! The EFI Configuration Table is the ConfigurationTable field in the EFI System Table. This table
//! contains a set of GUID/pointer pairs. Each element of this table is described by the
//! EFI_CONFIGURATION_TABLE structure below. The number of types of configuration tables is
//! expected to grow over time. This is why a GUID is used to identify the configuration table type. The
//! EFI Configuration Table may contain at most once instance of each table type.

use crate::guid::Guid;

/// Contains a set of GUID/pointer pairs comprised of the ConfigurationTable field in the EFI
/// System Table.
#[repr(C)]
pub struct ConfigurationTable {
    /// The 128-bit GUID value that uniquely identifies the system
    /// configuration table.
    pub VendorGuid: Guid,
    /// A pointer to the table associated with VendorGuid. Whether
    /// this pointer is a physical address or a virtual address during
    /// runtime is determined by the VendorGuid. The VendorGuid
    /// associated with a given VendorTable pointer defines whether
    /// or not a particular address reported in the table gets fixed up
    /// when a call to SetVirtualAddressMap() is made. It is the
    /// responsibility of the specification defining the VendorTable to
    /// specify whether to convert the addresses reported in the table.
    pub VendorTable: usize,
}
