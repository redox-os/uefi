// SPDX-License-Identifier: MIT

//! # Status codes
//!
//! Status codes for UEFI interfaces.
//!
//! Most `usize` values can represent a valid status, but only a few are
//! defined by the UEFI specification. Ranges are reserved for use by the UEFI
//! specification itself, the [Platform Initialization specification][PI Spec],
//! and OEM usage. On a 64-bit platform, these ranges are:
//!
//! | Start                   | End                     | Usage         |
//! | ----------------------- | ----------------------- | ------------- |
//! | `0x0000_0000_0000_0000` | `0x1FFF_FFFF_FFFF_FFFF` | UEFI warnings |
//! | `0x2000_0000_0000_0000` | `0x3FFF_FFFF_FFFF_FFFF` | PI warnings   |
//! | `0x4000_0000_0000_0000` | `0x7FFF_FFFF_FFFF_FFFF` | OEM warnings  |
//! | `0x8000_0000_0000_0000` | `0x9FFF_FFFF_FFFF_FFFF` | UEFI errors   |
//! | `0xA000_0000_0000_0000` | `0xBFFF_FFFF_FFFF_FFFF` | PI errors     |
//! | `0xC000_0000_0000_0000` | `0xCFFF_FFFF_FFFF_FFFF` | OEM errors    |
//!
//! Error codes always have the highest bit set, while success and warnings
//! always have the highest bit cleared.
//!
//! ## References
//!
//! - [UEFI Specification, Version 2.9][UEFI Spec]: Appendix D - Status Codes
//!
//! [PI Spec]: https://uefi.org/sites/default/files/resources/PI_Spec_1_7_A_final_May1.pdf
//! [UEFI Spec]: https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf

use core::fmt;

/// A value that represents success, a warning, or an error.
#[must_use]
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Status(pub usize);

impl Status {
    pub const ERROR_BIT: usize = 1 << (usize::BITS - 1);

    // Success codes
    /// The operation completed successfully.
    pub const SUCCESS: Self = Self(0);

    // Warning codes
    /// The string contained one or more characters that the device could not
    /// render and were skipped.
    pub const WARN_UNKNOWN_GLYPH: Self = Self(1);
    /// The handle was closed, but the file was not deleted.
    pub const WARN_DELETE_FAILURE: Self = Self(2);
    /// The handle was closed, but the data to the file was not flushed.
    pub const WARN_WRITE_FAILURE: Self = Self(3);
    /// The resulting buffer was too small, and the data was truncated to fit.
    pub const WARN_BUFFER_TOO_SMALL: Self = Self(4);
    /// The data has not been updated without the timeframe set by local policy
    /// for this type of data.
    pub const WARN_STALE_DATA: Self = Self(5);
    /// The resulting buffer contains a UEFI-compliant file system.
    pub const WARN_FILE_SYSTEM: Self = Self(6);
    /// The operation will be processed across a system reset.
    pub const WARN_RESET_REQUIRED: Self = Self(7);

    // Error codes
    /// The image failed to load.
    pub const LOAD_ERROR: Self = Self(Self::ERROR_BIT | 1);
    /// A parameter was incorrect.
    pub const INVALID_PARAMETER: Self = Self(Self::ERROR_BIT | 2);
    /// The operation is not supported.
    pub const UNSUPPORTED: Self = Self(Self::ERROR_BIT | 3);
    /// The buffer was not the proper size for the request.
    pub const BAD_BUFFER_SIZE: Self = Self(Self::ERROR_BIT | 4);
    /// The buffer is not large enough to hold the requested data.
    pub const BUFFER_TOO_SMALL: Self = Self(Self::ERROR_BIT | 5);
    /// There is no data pending upon return.
    pub const NOT_READY: Self = Self(Self::ERROR_BIT | 6);
    /// The physical device reported an error while attempting the operation.
    pub const DEVICE_ERROR: Self = Self(Self::ERROR_BIT | 7);
    /// The device cannot be written to.
    pub const WRITE_PROTECTED: Self = Self(Self::ERROR_BIT | 8);
    /// A resource has run out.
    pub const OUT_OF_RESOURCES: Self = Self(Self::ERROR_BIT | 9);
    /// An inconsistency was detected on the file system.
    pub const VOLUME_CORRUPTED: Self = Self(Self::ERROR_BIT | 10);
    /// There is no more space on the file system.
    pub const VOLUME_FULL: Self = Self(Self::ERROR_BIT | 11);
    /// The device does not contain any medium to perform the operation.
    pub const NO_MEDIA: Self = Self(Self::ERROR_BIT | 12);
    /// The medium in the device has changed since the last access.
    pub const MEDIA_CHANGED: Self = Self(Self::ERROR_BIT | 13);
    /// The item was not found.
    pub const NOT_FOUND: Self = Self(Self::ERROR_BIT | 14);
    /// Access was denied.
    pub const ACCESS_DENIED: Self = Self(Self::ERROR_BIT | 15);
    /// The server was not found or did not respond to the request.
    pub const NO_RESPONSE: Self = Self(Self::ERROR_BIT | 16);
    /// A mapping to a device does not exist.
    pub const NO_MAPPING: Self = Self(Self::ERROR_BIT | 17);
    /// The timeout time expired.
    pub const TIMEOUT: Self = Self(Self::ERROR_BIT | 18);
    /// The protocol has not been started.
    pub const NOT_STARTED: Self = Self(Self::ERROR_BIT | 19);
    /// The protocol has already been started.
    pub const ALREADY_STARTED: Self = Self(Self::ERROR_BIT | 20);
    /// The operation was aborted.
    pub const ABORTED: Self = Self(Self::ERROR_BIT | 21);
    /// An ICMP error occurred during the network operation.
    pub const ICMP_ERROR: Self = Self(Self::ERROR_BIT | 22);
    /// A TFTP error occurred during the network operation.
    pub const TFTP_ERROR: Self = Self(Self::ERROR_BIT | 23);
    /// A protocol error occurred during the network operation.
    pub const PROTOCOL_ERROR: Self = Self(Self::ERROR_BIT | 24);
    /// The function encountered an internal version that was incompatible with
    /// a version requested by the caller.
    pub const INCOMPATIBLE_VERSION: Self = Self(Self::ERROR_BIT | 25);
    /// The function was not performed due to a security violation.
    pub const SECURITY_VIOLATION: Self = Self(Self::ERROR_BIT | 26);
    /// A CRC error was detected.
    pub const CRC_ERROR: Self = Self(Self::ERROR_BIT | 27);
    /// Beginning or end of media was reached.
    pub const END_OF_MEDIA: Self = Self(Self::ERROR_BIT | 28);
    // No meaning defined for 29-30.
    /// The end of the file was reached.
    pub const END_OF_FILE: Self = Self(Self::ERROR_BIT | 31);
    /// The language specified was invalid.
    pub const INVALID_LANGUAGE: Self = Self(Self::ERROR_BIT | 32);
    /// The security status of the data is unknown or compromised and the data
    /// must be updated or replaced to restore a valid security status.
    pub const COMPROMISED_DATA: Self = Self(Self::ERROR_BIT | 33);
    /// There is an address conflict allocation.
    pub const IP_ADDRESS_CONFLICT: Self = Self(Self::ERROR_BIT | 34);
    /// An HTTP error occurred during the network operation.
    pub const HTTP_ERROR: Self = Self(Self::ERROR_BIT | 35);

    /// Returns true if the value represents a success.
    pub fn is_success(&self) -> bool {
        self == &Self::SUCCESS
    }

    /// Returns true if the value represents an error.
    pub fn is_error(&self) -> bool {
        (self.0 & Self::ERROR_BIT) == Self::ERROR_BIT
    }

    /// Returns true if the value represents a warning.
    pub fn is_warning(&self) -> bool {
        !self.is_error() && !self.is_success()
    }
}

impl From<usize> for Status {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::SUCCESS => write!(f, "success"),

            Self::WARN_UNKNOWN_GLYPH => write!(f, "unknown glyph"),
            Self::WARN_DELETE_FAILURE => write!(f, "delete failure"),
            Self::WARN_WRITE_FAILURE => write!(f, "write failure"),
            Self::WARN_STALE_DATA => write!(f, "stale data"),
            Self::WARN_FILE_SYSTEM => write!(f, "file system"),
            Self::WARN_RESET_REQUIRED => write!(f, "reset required"),

            Self::LOAD_ERROR => write!(f, "load error"),
            Self::INVALID_PARAMETER => write!(f, "invalid paramter"),
            Self::UNSUPPORTED => write!(f, "unsupported"),
            Self::BAD_BUFFER_SIZE => write!(f, "bad buffer size"),
            Self::WARN_BUFFER_TOO_SMALL | Self::BUFFER_TOO_SMALL => write!(f, "buffer too small"),
            Self::NOT_READY => write!(f, "not ready"),
            Self::DEVICE_ERROR => write!(f, "device error"),
            Self::WRITE_PROTECTED => write!(f, "write protected"),
            Self::OUT_OF_RESOURCES => write!(f, "out of resources"),
            Self::VOLUME_CORRUPTED => write!(f, "volume corrupted"),
            Self::VOLUME_FULL => write!(f, "volume full"),
            Self::NO_MEDIA => write!(f, "no media"),
            Self::MEDIA_CHANGED => write!(f, "media changed"),
            Self::NOT_FOUND => write!(f, "not found"),
            Self::ACCESS_DENIED => write!(f, "acccess denied"),
            Self::NO_RESPONSE => write!(f, "no response"),
            Self::NO_MAPPING => write!(f, "no mapping"),
            Self::TIMEOUT => write!(f, "timeout"),
            Self::NOT_STARTED => write!(f, "not started"),
            Self::ALREADY_STARTED => write!(f, "already started"),
            Self::ABORTED => write!(f, "aborted"),
            Self::ICMP_ERROR => write!(f, "ICMP error"),
            Self::TFTP_ERROR => write!(f, "TFTP error"),
            Self::PROTOCOL_ERROR => write!(f, "protocol error"),
            Self::INCOMPATIBLE_VERSION => write!(f, "incompatible version"),
            Self::SECURITY_VIOLATION => write!(f, "security violation"),
            Self::CRC_ERROR => write!(f, "CRC error"),
            Self::END_OF_MEDIA => write!(f, "end of media"),
            Self::END_OF_FILE => write!(f, "end of file"),
            Self::INVALID_LANGUAGE => write!(f, "invalid language"),
            Self::COMPROMISED_DATA => write!(f, "compromised data"),
            Self::IP_ADDRESS_CONFLICT => write!(f, "IP address conflict"),
            Self::HTTP_ERROR => write!(f, "HTTP error"),

            _ => write!(f, "{:#X}", self.0),
        }
    }
}

impl fmt::LowerHex for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

pub type Result<T> = core::result::Result<T, Status>;

impl From<Status> for Result<()> {
    fn from(status: Status) -> Self {
        match status {
            Status::SUCCESS => Ok(()),
            e => Err(e),
        }
    }
}

impl<T> From<Result<T>> for Status {
    fn from(res: Result<T>) -> Self {
        match res {
            Ok(_) => Status::SUCCESS,
            Err(e) => e,
        }
    }
}
