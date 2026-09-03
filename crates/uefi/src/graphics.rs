// SPDX-License-Identifier: MIT

use crate::prelude::*;

/// `EFI_GRAPHICS_OUTPUT_BLT_PIXEL`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct GraphicsBltPixel {
    pub Blue: u8,
    pub Green: u8,
    pub Red: u8,
    pub Reserved: u8,
}

/// `EFI_GRAPHICS_OUTPUT_BLT_OPERATION`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct GraphicsBltOp(pub u32);

impl GraphicsBltOp {
    /// Write data from the first buffer pixel to every pixel of the display.
    pub const VIDEO_FILL: Self = Self(0);
    /// Copy from the display to the buffer.
    pub const VIDEO_TO_BUFFER: Self = Self(1);
    /// Copy from the buffer to the display.
    pub const BUFFER_TO_VIDEO: Self = Self(2);
    /// Copy from the display to the display.
    pub const VIDEO_TO_VIDEO: Self = Self(3);
    /// Valid operation values are less than this value.
    pub const MAX: Self = Self(4);
}

/// `EFI_GRAPHICS_PIXEL_FORMAT`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct GraphicsPixelFormat(pub u32);

impl GraphicsPixelFormat {
    /// A pixel is 32-bits and byte zero represents red, byte one represents green,
    /// byte two represents blue, and byte three is reserved. This is the definition
    /// for the physical frame buffer. The byte values for the red, green, and blue
    /// components represent the color intensity. This color intensity value range
    /// from a minimum intensity of 0 to maximum intensity of 255.
    pub const PIXEL_RED_GREEN_BLUE_RESERVED_8_BIT_PER_COLOR: Self = Self(0);
    /// A pixel is 32-bits and byte zero represents blue, byte one represents green,
    /// byte two represents red, and byte three is reserved. This is the definition
    /// for the physical frame buffer. The byte values for the red, green, and blue
    /// components represent the color intensity. This color intensity value range
    /// from a minimum intensity of 0 to maximum intensity of 255.
    pub const PIXEL_BLUE_GREEN_RED_RESERVED_8_BIT_PER_COLOR: Self = Self(1);
    /// The Pixel definition of the physical frame buffer.
    pub const PIXEL_BIT_MASK: Self = Self(2);
    /// This mode does not support a physical frame buffer.
    pub const PIXEL_BLT_ONLY: Self = Self(3);
    /// Valid format values are less than this value.
    pub const MAX: Self = Self(4);
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GraphicsPixelBitmask {
    pub RedMask: u32,
    pub GreenMask: u32,
    pub BlueMask: u32,
    pub ReservedMask: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GraphicsOutputModeInfo {
    /// The version of this data structure. A value of zero represents the
    /// EFI_GRAPHICS_OUTPUT_MODE_INFORMATION structure as defined in this specification.
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

#[derive(Debug)]
#[repr(C)]
pub struct GraphicsOutputMode {
    /// The number of modes supported by QueryMode() and SetMode().
    pub MaxMode: u32,
    /// Current Mode of the graphics device. Valid mode numbers are 0 to MaxMode -1.
    pub Mode: u32,
    /// Pointer to read-only EFI_GRAPHICS_OUTPUT_MODE_INFORMATION data.
    pub Info: &'static GraphicsOutputModeInfo,
    /// Size of Info structure in bytes.
    pub SizeOfInfo: usize,
    /// Base address of graphics linear frame buffer.
    /// Offset zero in FrameBufferBase represents the upper left pixel of the display.
    pub FrameBufferBase: usize,
    /// Amount of frame buffer needed to support the active mode as defined by
    /// PixelsPerScanLine xVerticalResolution x PixelElementSize.
    pub FrameBufferSize: usize,
}

#[repr(C)]
pub struct GraphicsOutput {
    pub QueryMode: unsafe extern "efiapi" fn(
        &mut GraphicsOutput,
        u32,
        &mut usize,
        &mut *mut GraphicsOutputModeInfo,
    ) -> Status,
    pub SetMode: unsafe extern "efiapi" fn(&mut GraphicsOutput, u32) -> Status,
    pub Blt: unsafe extern "efiapi" fn(
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
    pub Mode: &'static mut GraphicsOutputMode,
}

impl GraphicsOutput {
    pub const GUID: Guid = guid!("9042a9de-23dc-4a38-96fb-7aded080516a");
}
