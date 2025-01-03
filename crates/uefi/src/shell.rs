use crate::prelude::*;

#[repr(C)]
pub struct Shell {
    pub Execute: extern "efiapi" fn(
        ImageHandle: &Handle,
        CommandLine: *const u16,
        Environment: *const *const u16,
        Status: *mut Status,
    ) -> Status,
    //TODO
}

impl Shell {
    pub const GUID: Guid = guid!("6302d008-7f9b-4f30-87ac-60c9fef5da4e");
}

#[repr(C)]
pub struct ShellParameters {
    pub Argv: *const *const u16,
    pub Argc: usize,
    pub StdIn: Handle,
    pub StdOut: Handle,
    pub StdErr: Handle,
}

impl ShellParameters {
    pub const GUID: Guid = guid!("752f3136-4e16-4fdc-a22a-e5f46812f4ca");
}
