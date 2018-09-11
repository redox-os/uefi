//! Provides types for dealing with resets.

/// The types of reset that UEFI supports.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum ResetType {
    /// EfiResetCold causes a system-wide reset. This sets all
    /// circuitry within the system to its initial state. This type of reset is asynchronous to system operation
    /// and operates without regard to cycle boundaries. EfiResetCold is tantamount to a system
    /// power cycle.
    Cold,
    /// EfiResetWarm causes a system-wide initialization. The
    /// processors are set to their in itial state, and pending cycles are not corrupted. If the system does not
    /// support this reset type, then an EfiResetCold must be performed.
    Warm,
    /// EfiResetShutdown causes the system to enter a power
    /// state equivalent to the ACPI G2/S5 or G3 states. If the system does not support this reset type, then
    /// when the system is rebooted, it should exhibit the EfiResetCold attributes.
    Shutdown,
}
