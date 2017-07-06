use ::Handle;

#[repr(C)]
pub struct ShellParameters {
    pub Argv: * const * const u16,
    pub Argc: usize,
    pub StdIn: Handle,
    pub StdOut: Handle,
    pub StdErr: Handle
}
