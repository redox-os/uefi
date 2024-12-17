use uefi::loaded_image::LoadedImage as UefiLoadedImage;

use crate::proto::Protocol;
use crate::prelude::*;

pub struct LoadedImage(pub &'static mut UefiLoadedImage);

impl Protocol<UefiLoadedImage> for LoadedImage {
    fn guid() -> Guid {
        UefiLoadedImage::GUID
    }

    fn new(inner: &'static mut UefiLoadedImage) -> Self {
        LoadedImage(inner)
    }
}
