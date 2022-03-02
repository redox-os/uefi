#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Time {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
    pub Hour: u8,
    pub Minute: u8,
    pub Second: u8,
    _Pad1: u8,
    pub Nanosecond: u32,
    pub TimeZone: u16,
    pub Daylight: u8,
    _Pad2: u8
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct TimeCapabilities {
  pub Resolution: u32,
  pub Accuracy: u32,
  pub SetsToZero: bool,
}
