// SPDX-License-Identifier: MIT

pub mod database;
pub mod ifr;
pub mod package;
pub mod sibt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct HiiHandle(pub *mut core::ffi::c_void);

pub type AnimationId = u16;
pub type FontStyle = u32;
pub type FormId = u16;
pub type ImageId = u16;
pub type QuestionId = u16;
pub type StringId = u16;
pub type VarStoreId = u16;
