//! Handles UEFI status codes.

use core::ops::Try;

/// The hig bit of a status code to indicate an error.
#[cfg(target_pointer_width = "128")]
pub const ERROR_BIT: usize = 1 << 127;

/// The high bit of a status code to indicate an error.
#[cfg(target_pointer_width = "64")]
pub const ERROR_BIT: usize = 1 << 63;

/// The high bit of a status code to indicate an error.
#[cfg(target_pointer_width = "32")]
pub const ERROR_BIT: usize = 1 << 31;

/// The high bit of a status code to indicate an error.
#[cfg(target_pointer_width = "16")]
pub const ERROR_BIT: usize = 1 << 15;

/// The high bit of a status code to indicate an error.
#[cfg(target_pointer_width = "8")]
pub const ERROR_BIT: usize = 1 << 7;

/// Represents an error in a UEFI status code.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Error {
    /// The operation completed successfully.
    Success,
    /// The image failed to load.
    LoadError,
    /// A parameter was incorrect.
    InvalidParameter,
    /// The operation is not supported.
    Unsupported,
    /// The buffer was not the proper size for the request.
    BadBufferSize,
    /// The buffer is not large enough to hold the requested data. The
    /// required buffer size is returned in the appropriate parameter
    /// when this error occurs.
    BufferTooSmall,
    /// There is no data pending upon return.
    NotReady,
    /// The physical device reported an error while attempting the
    /// operation.
    DeviceError,
    /// The device cannot be written to.
    WriteProtected,
    /// A resource has run out.
    OutOfResources,
    /// An inconstancy was detected on the file system causing the
    /// operation to fail.
    VolumeCorrupted,
    /// There is no more space on the file system.
    VolumeFull,
    /// The device does not contain any medium to perform the
    /// operation.
    NoMedia,
    /// The medium in the device has changed since the last access.
    MediaChanged,
    /// The item was not found.
    NotFound,
    /// Access was denied.
    AccessDenied,
    /// The server was not found or did not respond to the request.
    NoResponse,
    /// A mapping to a device does not exist.
    NoMapping,
    /// The timeout time expired.
    Timeout,
    /// The protocol has not been started.
    NotStarted,
    /// The protocol has already been started.
    AlreadyStarted,
    /// The operation was aborted.
    Aborted,
    /// An ICMP error occurred during the network operation.
    IcmpError,
    /// A TFTP error occurred during the network operation.
    TftpError,
    /// A protocol error occurred during the network operation.
    ProtocolError,
    /// The function encountered an internal version that was
    /// incompatible with a version requested by the caller.
    IncompatibleVersion,
    /// The function was not performed due to a security violation.
    SecurityViolation,
    /// A CRC error was detected.
    CrcError,
    /// Beginning or end of media was reached.
    EndOfMedia,
    /// Error code 29 is not defined as of UEFI version 2.7A.
    Error29,
    /// Error code 29 is not defined as of UEFI version 2.7A.
    Error30,
    /// The end of the file was reached.
    EndOfFile,
    /// The language specified was invalid.
    InvalidLanguage,
    /// The security status of the data is unknown or compromised and
    /// the data must be updated or replaced to restore a valid security
    /// status.
    CompromisedData,
    /// There is an address conflict address allocation.
    Error34,
    /// A HTTP error occurred during the network operation.
    HttpError,
    /// There is an unknown error.
    Unknown,
}

impl From<usize> for Error {
    fn from(value: usize) -> Self {
        use self::Error::*;
        match value {
            0 => Success,
            1 => LoadError,
            2 => InvalidParameter,
            3 => Unsupported,
            4 => BadBufferSize,
            5 => BufferTooSmall,
            6 => NotReady,
            7 => DeviceError,
            8 => WriteProtected,
            9 => OutOfResources,
            10 => VolumeCorrupted,
            11 => VolumeFull,
            12 => NoMedia,
            13 => MediaChanged,
            14 => NotFound,
            15 => AccessDenied,
            16 => NoResponse,
            17 => NoMapping,
            18 => Timeout,
            19 => NotStarted,
            20 => AlreadyStarted,
            21 => Aborted,
            22 => IcmpError,
            23 => TftpError,
            24 => ProtocolError,
            25 => IncompatibleVersion,
            26 => SecurityViolation,
            27 => CrcError,
            28 => EndOfMedia,
            29 => Error29,
            30 => Error30,
            31 => EndOfFile,
            32 => InvalidLanguage,
            33 => CompromisedData,
            34 => Error34,
            35 => HttpError,
            _ => Unknown,
        }
    }
}

/// Represents a result with an UEFI status code as error.
pub type Result<T> = ::core::result::Result<T, Error>;

/// Success, error, and warning codes returned by boot services and runtime services
/// functions.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[must_use]
#[repr(transparent)]
pub struct Status(pub usize);

impl Status {
    pub fn new(value: usize) -> Self {
        Status(value)
    }
}

impl Try for Status {
    type Ok = usize;
    type Error = Error;

    fn into_result(self) -> Result<Self::Ok> {
        if self.0 & ERROR_BIT == 0 {
            Ok(self.0)
        } else {
            Err(Error::from(self.0 & !(ERROR_BIT)))
        }
    }

    fn from_error(v: Self::Error) -> Self {
        Status(v as usize | ERROR_BIT)
    }

    fn from_ok(v: Self::Ok) -> Self {
        Status(v & !(ERROR_BIT))
    }
}
