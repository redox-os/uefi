use uefi::shell::Shell as UefiShell;

use crate::proto::Protocol;
use crate::prelude::*;

pub struct Shell(pub &'static mut UefiShell);

impl Protocol<UefiShell> for Shell {
    fn guid() -> Guid {
        UefiShell::GUID
    }

    fn new(inner: &'static mut UefiShell) -> Self {
        Shell(inner)
    }
}
