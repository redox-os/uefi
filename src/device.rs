use guid::Guid;

#[repr(C)]
pub enum DevicePathType {
    Hardware = 0x01,
    Acpi = 0x02,
    Messaging = 0x03,
    Media = 0x04,
    Bbs = 0x05,
    End = 0x7f,
}

#[repr(C)]
pub enum DevicePathHardwareType {
    Pci = 0x01,
    Pccard = 0x02,
    Memmap = 0x03,
    Vendor = 0x04,
    Controller = 0x05,
}

#[repr(C)]
pub enum DevicePathAcpiType {
    Acpi = 0x01,
    Extended = 0x02,
    Adr = 0x03,
}

//TODO: Messaging types

#[repr(C)]
pub enum DevicePathMediaType {
    Harddrive = 0x01,
    Cdrom = 0x02,
    Vendor = 0x03,
    Filepath = 0x04,
    Protocol = 0x05,
}

#[repr(C)]
pub enum DevicePathBbsType {
    Bbs = 0x01,
}


#[repr(C)]
pub enum DevicePathEndType {
    Instance = 0x01,
    Entire = 0xff,
}

#[repr(C)]
pub struct DevicePath {
    pub Type: u8,
    pub SubType: u8,
    pub Length: u16,
}

#[repr(C)]
pub struct VendorDevicePath {
    pub Header: DevicePath,
    pub Guid: Guid,
}
