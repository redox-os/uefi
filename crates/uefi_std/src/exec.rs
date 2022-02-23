use uefi::Handle;
use uefi::status::Result;

use crate::ffi::wstr;
use crate::fs::load;
use crate::loaded_image::LoadedImage;
use crate::proto::Protocol;

pub fn exec_data(data: &[u8], name: &str, args: &[&str]) -> Result<usize> {
    let handle = crate::handle();
    let st = crate::system_table();

    let mut image_handle = Handle(0);
    (st.BootServices.LoadImage)(false, handle, 0, data.as_ptr(), data.len(), &mut image_handle)?;

    let mut cmdline = format!("\"{}\"", name);
    for arg in args.iter() {
        cmdline.push_str(" \"");
        cmdline.push_str(arg);
        cmdline.push_str("\"");
    }
    cmdline.push('\0');

    let wcmdline = wstr(&cmdline);

    if let Ok(loaded_image) = LoadedImage::handle_protocol(image_handle) {
        loaded_image.0.LoadOptionsSize = (wcmdline.len() as u32) * 2;
        loaded_image.0.LoadOptions = wcmdline.as_ptr();
    }

    let mut exit_size = 0;
    let mut exit_ptr = ::core::ptr::null_mut();
    let ret = (st.BootServices.StartImage)(image_handle, &mut exit_size, &mut exit_ptr)?;

    Ok(ret)
}

pub fn exec_path(path: &str, args: &[&str]) -> Result<usize> {
    let data = load(path)?;
    exec_data(&data, path, args)
}
