use core::ops::{ControlFlow, FromResidual, Try};

pub const ERROR_BIT: usize = 1 << 63;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Error {
    Success,
    LoadError,
    InvalidParameter,
    Unsupported,
    BadBufferSize,
    BufferTooSmall,
    NotReady,
    DeviceError,
    WriteProtected,
    OutOfResources,
    VolumeCorrupted,
    VolumeFull,
    NoMedia,
    MediaChanged,
    NotFound,
    AccessDenied,
    NoResponse,
    NoMapping,
    Timeout,
    NotStarted,
    AlreadyStarted,
    Aborted,
    IcmpError,
    TftpError,
    ProtocolError,
    IncompatibleVersion,
    SecurityViolation,
    CrcError,
    EndOfMedia,
    Error29,
    Error30,
    EndOfFile,
    InvalidLanguage,
    CompromisedData,
    Error34,
    HttpError,
    Unknown
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
            _ => Unknown
        }
    }
}

pub type Result<T> = ::core::result::Result<T, Error>;

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
    type Output = usize;
    type Residual = Error;

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        if self.0 & ERROR_BIT == 0 {
            ControlFlow::Continue(self.0)
        } else {
            ControlFlow::Break(Error::from(self.0 & !ERROR_BIT))
        }
    }

    fn from_output(v: Self::Output) -> Self {
        Self(v & !ERROR_BIT)
    }
}

impl FromResidual for Status {
    fn from_residual(v: Error) -> Self {
        Self(v as usize | ERROR_BIT)
    }
}

impl<T> FromResidual<Error> for Result<T> {
    fn from_residual(err: Error) -> Self {
        Err(err)
    }
}
