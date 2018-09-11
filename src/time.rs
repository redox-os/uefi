/// This represents the current time information.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Time {
    /// The current local date.
    pub Year: u16,
    /// The current local date.
    pub Month: u8,
    /// The current local date.
    pub Day: u8,
    /// The current local time. Nanoseconds report the current
    /// fraction of a second in the device. The format of the time is
    /// hh:mm:ss.nnnnnnnnn. A battery backed real time clock
    /// device maintains the date and time.
    pub Hour: u8,
    /// The current local time. Nanoseconds report the current
    /// fraction of a second in the device. The format of the time is
    /// hh:mm:ss.nnnnnnnnn. A battery backed real time clock
    /// device maintains the date and time.
    pub Minute: u8,
    /// The current local time. Nanoseconds report the current
    /// fraction of a second in the device. The format of the time is
    /// hh:mm:ss.nnnnnnnnn. A battery backed real time clock
    /// device maintains the date and time.
    pub Second: u8,
    _Pad1: u8,
    /// The current local time. Nanoseconds report the current
    /// fraction of a second in the device. The format of the time is
    /// hh:mm:ss.nnnnnnnnn. A battery backed real time clock
    /// device maintains the date and time.
    pub Nanosecond: u32,
    /// The time's offset in minutes from UTC. If the value is
    /// EFI_UNSPECIFIED_TIMEZONE, then the time is interpreted
    /// as a local time. The TimeZone is the number of minutes
    /// that the local time is relative to UTC. To calculate the
    /// TimeZone value, follow this equation: Localtime = UTC -
    /// TimeZone.
    ///
    /// To further illustrate this, an example is given below:
    /// PST (Pacific Standard Time is 12PM) = UTC (8PM) - 8
    /// hours (480 minutes)
    ///
    /// In this case, the value for Timezone would be 480 if
    /// referencing PST.
    pub TimeZone: u16,
    /// A bitmask containing the daylight savings time
    /// information for the time.
    ///
    /// The EFI_TIME_ADJUST_DAYLIGHT bit indicates if the time
    /// is affected by daylight savings time or not. This value does
    /// not indicate that the time has been adjusted for daylight
    /// savings time. It indicates only that it should be adjusted
    /// when the EFI_TIME enters daylight savings time.
    ///
    /// If EFI_TIME_IN_DAYLIGHT is set, the time has been
    /// adjusted for daylight savings time.
    ///
    /// All other bits must be zero.
    /// When entering daylight saving time, if the time is affected,
    /// but hasn't been adjusted (DST = 1), use the new
    /// calculation:
    /// 
    /// 1. The date/time should be increased by the appropriate
    /// amount.
    /// 2. The TimeZone should be decreased by the appropriate
    /// amount (EX: +480 changes to +420 when moving from
    /// PST to PDT).
    /// 3. The Daylight value changes to 3.
    /// 
    /// When exiting daylight saving time, if the time is affected
    /// and has been adjusted (DST = 3), use the new calculation:
    ///
    /// 1. The date/time should be decreased by the appropriate
    /// amount.
    /// 2. The TimeZone should be increased by the appropriate
    /// amount.
    /// 3. The Daylight value changes to 1.
    pub Daylight: u8,
    _Pad2: u8,
}

/// This provides the capabilities of the
/// real time clock device as exposed through the EFI interfaces.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct TimeCapabilities {
    /// Provides the reporting resolution of the real-time clock
    /// device in counts per second. For a normal PC-AT CMOS
    /// RTC device, this value would be 1 Hz, or 1, to indicate that
    /// the device only reports the time to the resolution of 1
    /// second.
    pub Resolution: u32,
    /// Provides the timekeeping accuracy of the real-time clock
    /// in an error rate of 1E-6 parts per million. For a clock with
    /// an accuracy of 50 parts per million, the value in this field
    /// would be 50,000,000.
    pub Accuracy: u32,
    /// A TRUE indicates that a time set operation clears the
    /// device’s time below the Resolution reporting level. A
    /// FALSE indicates that the state below the Resolution level
    /// of the device is not cleared when the time is set. Normal
    /// PC-AT CMOS RTC devices set this value to FALSE.
    pub SetsToZero: bool,
}
