//! This protocol provides control over block devices.
use crate::status::Status;

/// Represents block IO media information.
#[repr(C)]
pub struct BlockIoMedia {
    /// The current media ID. If the media changes, this value is
    /// changed.
    pub MediaId: u32,
    /// TRUE if the media is removable; otherwise, FALSE.
    pub RemovableMedia: bool,
    /// TRUE if there is a media current ly present in the device;
    /// otherwise, FALSE. This field shows the media present status as of
    /// the most recent ReadBlocks() or WriteBlocks() call.
    pub MediaPresent: bool,
    /// TRUE if the EFI_BLOCK_IO_PROTOCOL was produced to abstract
    /// partition structures on the disk. FALSE if the BLOCK_IO protocol
    /// was produced to abstract the logical blocks on a hardware
    /// device.
    pub LogicalPartition: bool,
    /// TRUE if the media is marked read-only otherwise, FALSE. This
    /// field shows the read-only status as of the most recent
    /// WriteBlocks() call.
    pub ReadOnly: bool,
    /// TRUE if the WriteBlocks() function caches write data.
    pub WriteCaching: bool,
    /// The intrinsic block size of the device. If the media changes, then
    /// this field is updated.Returns the number of bytes per logical
    /// block. For ATA devices, this is reported in IDENTIFY DEVICE data
    /// words 117-118 (i.e., Words per Logical Sector) (see ATA8-ACS).
    /// For SCSI devices, this is reported in the READ CAPACITY (16)
    /// parameter data Logical Block Length In Bytes field (see SBC-3).
    pub BlockSize: u32,
    /// Supplies the alignment requirement for any buffer used in a data
    /// transfer. IoAlign values of 0 and 1 mean that the buffer can be
    /// placed anywhere in memory. Otherwise, IoAlign must be a
    /// power of 2, and the requirement is that the start address of a
    /// buffer must be evenly divisible by IoAlign with no remainder.
    pub IoAlign: u32,
    /// The last LBA on the device. If the media changes, then this field is
    /// updated. For ATA devices, this is reported in IDENTIFY DEVICE
    /// data words 60-61 (i.e., Total number of user addressable logical
    /// sectors) (see ATA8-ACS) minus one. For SCSI devices, this is
    /// reported in the READ CAPACITY (16) parameter data Returned
    /// Logical Block Address field (see SBC-3) minus one.
    pub LastBlock: u64,
}

/// This protocol provides control over block devices.
#[repr(C)]
pub struct BlockIo {
    /// The revision to which the block IO interface adheres. All future
    /// revisions must be backwards compatible. If a future version is
    /// not back wards compatible it is not the same GUID.
    pub Revision: u64,
    /// A pointer to the EFI_BLOCK_IO_MEDIA data for this device.
    pub Media: &'static BlockIoMedia,
    /// Resets the block device hardware.
    pub Reset: extern "win64" fn(&BlockIo, ExtendedVerification: bool) -> Status,
    /// Reads the requested number of blocks from the device.
    pub ReadBlocks:
        extern "win64" fn(&BlockIo, MediaId: u32, LBA: u64, BufferSize: usize, Buffer: *mut u8)
            -> Status,
    /// Writes the requested number of blocks to the device.
    pub WriteBlocks:
        extern "win64" fn(&BlockIo, MediaId: u32, LBA: u64, BufferSize: usize, Buffer: *const u8)
            -> Status,
    /// Flushes any cache blocks. This function is optional and only
    /// needs to be supported on block devices that cache writes.
    pub FlushBlocks: extern "win64" fn(&BlockIo) -> Status,
}
