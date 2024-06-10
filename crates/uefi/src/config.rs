use crate::prelude::*;

#[repr(C)]
pub struct ConfigurationTable {
    pub VendorGuid: Guid,
    pub VendorTable: usize,
}
