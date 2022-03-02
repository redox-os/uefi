use crate::guid::Guid;

#[repr(C)]
pub struct ConfigurationTable {
    pub VendorGuid: Guid,
    pub VendorTable: usize
}
