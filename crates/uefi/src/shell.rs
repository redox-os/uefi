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

#[repr(C)]
pub struct ShellParameters {
    pub Argv: *const *const u16,
    pub Argc: usize,
    pub StdIn: Handle,
    pub StdOut: Handle,
    pub StdErr: Handle,
}
