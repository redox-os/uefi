use crate::prelude::*;

enum_try_from_int! {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum DevicePathType {
        Hardware = 0x01,
        Acpi = 0x02,
        Messaging = 0x03,
        Media = 0x04,
        Bbs = 0x05,
        End = 0x7f,
    }
}

enum_try_from_int! {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum DevicePathHardwareType {
        Pci = 0x01,
        Pccard = 0x02,
        Memmap = 0x03,
        Vendor = 0x04,
        Controller = 0x05,
    }
}

enum_try_from_int! {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum DevicePathAcpiType {
        Acpi = 0x01,
        Extended = 0x02,
        Adr = 0x03,
    }
}

enum_try_from_int! {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum DevicePathMessagingType {
        Atapi = 0x01,
        Scsi = 0x02,
        Fiberchannel = 0x03,
        Serial1394 = 0x04,
        Usb = 0x05,
        Sata = 0x12,
        Wwid = 0x10,
        Dlu = 0x11,
        Usbclass = 0x0f,
        I2o = 0x06,
        Mac = 0x0b,
        Ipv4 = 0x0c,
        Ipv6 = 0x0d,
        Infiniband = 0x09,
        Uart = 0x0e,
        Vendordefined = 0x0a,
        Iscsi = 0x13,
        Nvme = 0x17,
    }
}

enum_try_from_int! {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum DevicePathMediaType {
        Harddrive = 0x01,
        Cdrom = 0x02,
        Vendor = 0x03,
        Filepath = 0x04,
        Protocol = 0x05,
    }
}

enum_try_from_int! {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum DevicePathBbsType {
        Bbs = 0x01,
    }
}

enum_try_from_int! {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum DevicePathEndType {
        Instance = 0x01,
        Entire = 0xff,
    }
}

#[repr(C)]
pub struct DevicePath {
    pub Type: u8,
    pub SubType: u8,
    pub Length: u16,
}

impl DevicePath {
    pub const GUID: Guid = guid!("09576e91-6d3f-11d2-8e39-00a0c969723b");
}

#[repr(C)]
pub struct VendorDevicePath {
    pub Header: DevicePath,
    pub Guid: Guid,
}
