#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
}
