use core::mem;
use uefi::boot::LocateSearchType;

use crate::prelude::*;
use crate::system_table;

pub trait Protocol<T: 'static> {
    fn guid() -> Guid;

    fn new(fs: &'static mut T) -> Self
    where
        Self: Sized;

    fn locate_protocol() -> Result<Self>
    where
        Self: Sized,
    {
        let guid = Self::guid();
        let mut interface = 0;
        let status = (system_table().BootServices.LocateProtocol)(&guid, 0, &mut interface);

        match status {
            Status::SUCCESS => Ok(Self::new(unsafe { &mut *(interface as *mut T) })),
            _ => Err(status),
        }
    }

    fn handle_protocol(handle: Handle) -> Result<Self>
    where
        Self: Sized,
    {
        let guid = Self::guid();
        let mut interface = 0;
        let status = (system_table().BootServices.HandleProtocol)(handle, &guid, &mut interface);

        match status {
            Status::SUCCESS => Ok(Self::new(unsafe { &mut *(interface as *mut T) })),
            _ => Err(status),
        }
    }

    fn locate_handle() -> Result<Vec<Handle>> {
        let guid = Self::guid();
        let mut handles = Vec::with_capacity(256);
        let mut len = handles.capacity() * mem::size_of::<Handle>();
        let status = (system_table().BootServices.LocateHandle)(
            LocateSearchType::ByProtocol,
            &guid,
            0,
            &mut len,
            handles.as_mut_ptr(),
        );

        match status {
            Status::SUCCESS => {
                unsafe {
                    handles.set_len(len / mem::size_of::<Handle>());
                }
                Ok(handles)
            }
            _ => Err(status),
        }
    }

    fn one() -> Result<Self>
    where
        Self: Sized,
    {
        Self::locate_protocol()
    }

    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        let mut instances = Vec::new();
        for handle in Self::locate_handle().unwrap_or_default() {
            if let Ok(instance) = Self::handle_protocol(handle) {
                instances.push(instance);
            }
        }
        instances
    }
}
