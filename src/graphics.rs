//! Provides a basic abstraction to set video modes and copy pixels to and from the graphics
//! controller’s frame buffer. The linear address of the hardware frame buffer is also exposed so
//! software can write directly to the video hardware.

use crate::status::Status;

/// Represents a pixel when doing a Blt.
///
/// Blt stands for BLock Transfer.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GraphicsBltPixel {
    /// The blue part of the pixel.
    pub Blue: u8,
    /// The green part of the pixel.
    pub Green: u8,
    /// The red part of the pixel.
    pub Red: u8,
    /// The reserved part of the pixel.
    pub Reserved: u8,
}

/// Describes the BltOperations that are supported on rectangles. Rectangles have
/// coordinates (left, upper) (right, bottom).
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum GraphicsBltOp {
    /// Write data from the BltBuffer pixel (0,0) directly to every pixel of
    /// the video display rectangle (DestinationX, DestinationY)
    /// (DestinationX + Width, DestinationY + Height). Only one
    /// pixel will be used from the BltBuffer. Delta is NOT used.
    VideoFill,
    /// Read data from the video display rectangle (SourceX, SourceY)
    /// (SourceX + Width, SourceY + Height) and place it in the
    /// BltBuffer rectangle (DestinationX, DestinationY)
    /// (DestinationX + Width, DestinationY + Height). If
    /// DestinationX or DestinationY is not zero then Delta must
    /// be set to the length in bytes of a row in the BltBuffer.
    VideoToBuffer,
    /// Write data from the BltBuffer rectangle (SourceX, SourceY)
    /// (SourceX + Width, SourceY + Height) directly to the video
    /// display rectangle (DestinationX, DestinationY)
    /// (DestinationX + Width, DestinationY + Height). If
    /// SourceX or SourceY is not zero then Delta must be set to the
    /// length in bytes of a row in the BltBuffer.
    BufferToVideo,
    /// Copy from the video display rectangle (SourceX, SourceY)
    /// (SourceX + Width, SourceY + Height) to the video display
    /// rectangle (DestinationX, DestinationY) ( DestinationX +
    /// Width, DestinationY + Height). The BltBuffer and Delta
    /// are not used in this mode. There is no limitation on the overlapping of
    /// the source and destination rectangles.
    VideoToVideo,
}

/// Defines various pixel formats.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum GraphicsPixelFormat {
    /// A pixel is 32-bits and byte zero represents red, byte one represents green,
    /// byte two represents blue, and byte three is reserved. This is the definition
    /// for the physical frame buffer. The byte values for the red, green, and blue
    /// components represent the color intensity. This color intensity value range
    /// from a minimum intensity of 0 to maximum intensity of 255.
    PixelRedGreenBlueReserved8BitPerColor,
    /// A pixel is 32-bits and byte zero represents blue, byte one represents green,
    /// byte two represents red, and byte three is reserved. This is the definition
    /// for the physical frame buffer. The byte values for the red, green, and blue
    /// components represent the color intensity. This color intensity value range
    /// from a minimum intensity of 0 to maximum intensity of 255.
    PixelBlueGreenRedReserved8BitPerColor,
    /// The pixel definition of the physical frame buffer is defined by EFI_PIXEL_BITMASK.
    PixelBitMask,
    /// This mode does not support a physical frame buffer.
    PixelBltOnly,
    /// Valid EFI_GRAPHICS_PIXEL_FORMAT enum values are less than this value.
    PixelFormatMax,
}

/// If a bit is set in RedMask, GreenMask, or BlueMask then those bits of the pixel represent the
/// corresponding color. Bits in RedMask, GreenMask, BlueMask, and ReserverdMask must not over
/// lap bit positions. The values for the red, green, and blue components in the bit mask represent the
/// color intensity. The color intensities must increase as the color values for a each color mask
/// increase with a minimum intensity of all bits in a color mask clear to a maximum intensity of all bits
/// in a color mask set.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GraphicsPixelBitmask {
    /// The bits which represent red in the pixel layout of the physical frame buffer.
    pub RedMask: u32,
    /// The bits which represent green in the pixel layout of the physical frame buffer.
    pub GreenMask: u32,
    /// The bits which represent blue in the pixel layout of the physical frame buffer.
    pub BlueMask: u32,
    /// The bits which are reserved in the pixel layout of the physical frame buffer.
    pub ReservedMask: u32,
}

/// Provides information about the graphics output mode.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GraphicsOutputModeInfo {
    /// The version of this data structure. A value of zero represents the
    /// EFI_GRAPHICS_OUTPUT_MODE_INFORMATION structure as defined in this specification.
    /// Future version of this specification may extend this data structure in a
    /// backwards compatible way and increase the value of Version.
    pub Version: u32,
    /// The size of video screen in pixels in the X dimension.
    pub HorizontalResolution: u32,
    /// The size of video screen in pixels in the Y dimension.
    pub VerticalResolution: u32,
    /// Enumeration that defines the physical format of the pixel. A value of PixelBltOnly
    /// implies that a linear frame buffer is not available for this mode.
    pub PixelFormat: GraphicsPixelFormat,
    /// This bit-mask is only valid if PixelFormat is set to PixelPixelBitMask.
    /// A bit being set defines what bits are used for what purpose such as Red, Green, Blue, or Reserved.
    pub PixelInformation: GraphicsPixelBitmask,
    /// Defines the number of pixel elements per video memory line.
    pub PixelsPerScanLine: u32,
}

/// The EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE is read-only and values are only changed by using
/// the appropriate interface functions.
#[derive(Debug)]
#[repr(C)]
pub struct GraphicsOutputMode {
    /// The number of modes supported by QueryMode() and SetMode().
    pub MaxMode: u32,
    /// Current Mode of the graphics device. Valid mode numbers are 0 to MaxMode -1.
    pub Mode: u32,
    /// Reference to read-only EFI_GRAPHICS_OUTPUT_MODE_INFORMATION data.
    pub Info: &'static GraphicsOutputModeInfo,
    /// Size of Info structure in bytes. Future versions of this specification
    /// may increase the size of the EFI_GRAPHICS_OUTPUT_MODE_INFORMATION data.
    pub SizeOfInfo: usize,
    /// Base address of graphics linear frame buffer. Info contains
    /// information required to allow software to draw directly to the
    /// frame buffer without using Blt().Offset zero in FrameBufferBase
    /// represents the upper left pixel of the display.
    pub FrameBufferBase: usize,
    /// Amount of frame buffer needed to support the active mode as defined by
    /// PixelsPerScanLine x VerticalResolution x PixelElementSize.
    pub FrameBufferSize: usize,
}

/// Provides a basic abstraction to set video modes and copy pixels to and from the graphics
/// controller’s frame buffer. The linear address of the hardware frame buffer is also exposed so
/// software can write directly to the video hardware.
#[repr(C)]
pub struct GraphicsOutput {
    /// Returns information for an available graphics mode that the graphics
    /// device and the set of active video output devices supports.
    pub QueryMode:
        extern "win64" fn(&mut GraphicsOutput, u32, &mut usize, &mut *mut GraphicsOutputModeInfo)
            -> Status,
    /// Set the video device into the specified mode and clears the visible portions of the output display to black.
    pub SetMode: extern "win64" fn(&mut GraphicsOutput, u32) -> Status,
    /// Software abstraction to draw on the video device’s frame buffer.
    pub Blt: extern "win64" fn(
        &mut GraphicsOutput,
        *mut GraphicsBltPixel,
        GraphicsBltOp,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) -> Status,
    /// Reference to EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE data.
    pub Mode: &'static mut GraphicsOutputMode,
}
