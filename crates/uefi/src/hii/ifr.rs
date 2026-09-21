// SPDX-License-Identifier: MIT

use core::{fmt, mem};

use super::{FormId, QuestionId, StringId, VarStoreId};
use crate::guid::Guid;

/// `EFI_IFR_TYPE_*`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct IfrType(pub u8);

impl IfrType {
    pub const NUM_SIZE_8: Self = Self(0x00);
    pub const NUM_SIZE_16: Self = Self(0x01);
    pub const NUM_SIZE_32: Self = Self(0x02);
    pub const NUM_SIZE_64: Self = Self(0x03);
    pub const BOOLEAN: Self = Self(0x04);
    pub const TIME: Self = Self(0x05);
    pub const DATE: Self = Self(0x06);
    pub const STRING: Self = Self(0x07);
    pub const OTHER: Self = Self(0x08);
    pub const UNDEFINED: Self = Self(0x09);
    pub const ACTION: Self = Self(0x0A);
    pub const BUFFER: Self = Self(0x0B);
    pub const REF: Self = Self(0x0C);
}

/// `EFI_HII_TIME`
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct HiiTime {
    pub Hour: u8,
    pub Minute: u8,
    pub Second: u8,
}

/// `EFI_HII_DATE`
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct HiiDate {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
}

/// `EFI_HII_REF`
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C, packed)] // Fails to have correct size with repr(C)
pub struct HiiRef {
    pub QuestionId: QuestionId,
    pub FormId: FormId,
    pub FormSetGuid: Guid,
    pub DevicePath: StringId,
}

/// `EFI_IFR_TYPE_VALUE`
#[derive(Clone, Copy)]
#[repr(C, packed)] // Fails to have correct size with repr(C)
pub union IfrTypeValue {
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    b: bool,
    time: HiiTime,
    date: HiiDate,
    string: StringId,
    reference: HiiRef,
}

impl fmt::Debug for IfrTypeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IfrTypeValue").finish()
    }
}

// XXX: Part of `EDKII_FORM_DISPLAY_ENGINE_PROTOCOL`
/// `EFI_HII_VALUE`
#[repr(C)]
pub struct HiiValue {
    pub Kind: IfrType,
    pub Buffer: *mut u8,
    pub BufferLen: u16,
    pub Value: IfrTypeValue,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct IfrStatementHeader {
    pub Prompt: StringId,
    pub Help: StringId,
}

pub const IFR_FLAG_READ_ONLY: u8 = 0x01;
pub const IFR_FLAG_CALLBACK: u8 = 0x04;
pub const IFR_FLAG_RESET_REQUIRED: u8 = 0x10;
pub const IFR_FLAG_RECONNECT_REQUIRED: u8 = 0x40;
pub const IFR_FLAG_OPTIONS_ONLY: u8 = 0x80;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)] // Has incorrect size if not packed
pub struct IfrQuestionHeader {
    pub Header: IfrStatementHeader,
    pub QuestionId: QuestionId,
    pub VarStoreId: VarStoreId,
    // union { VarName: StringId, VarOffset: u16 }
    pub VarStoreInfo: u16,
    pub Flags: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct IfrOpCode(pub u8);

impl IfrOpCode {
    pub const FORM: Self = Self(0x01);
    pub const SUBTITLE: Self = Self(0x02);
    pub const TEXT: Self = Self(0x03);
    pub const IMAGE: Self = Self(0x04);
    pub const ONE_OF: Self = Self(0x05);
    pub const CHECKBOX: Self = Self(0x06);
    pub const NUMERIC: Self = Self(0x07);
    pub const PASSWORD: Self = Self(0x08);
    pub const ONE_OF_OPTION: Self = Self(0x09);
    pub const SUPPRESS_IF: Self = Self(0x0A);
    pub const LOCKED: Self = Self(0x0B);
    pub const ACTION: Self = Self(0x0C);
    pub const RESET_BUTTON: Self = Self(0x0D);
    pub const FORM_SET: Self = Self(0x0E);
    pub const REF: Self = Self(0x0F);
    pub const NO_SUBMIT_IF: Self = Self(0x10);
    pub const INCONSISTENT_IF: Self = Self(0x11);
    pub const EQ_ID_VAL: Self = Self(0x12);
    pub const EQ_ID_ID: Self = Self(0x13);
    pub const EQ_ID_VAL_LIST: Self = Self(0x14);
    pub const AND: Self = Self(0x15);
    pub const OR: Self = Self(0x16);
    pub const NOT: Self = Self(0x17);
    pub const RULE: Self = Self(0x18);
    pub const GRAY_OUT_IF: Self = Self(0x19);
    pub const DATE: Self = Self(0x1A);
    pub const TIME: Self = Self(0x1B);
    pub const STRING: Self = Self(0x1C);
    pub const REFRESH: Self = Self(0x1D);
    pub const DISABLE_IF: Self = Self(0x1E);
    pub const ANIMATION: Self = Self(0x1F);
    pub const TO_LOWER: Self = Self(0x20);
    pub const TO_UPPER: Self = Self(0x21);
    pub const MAP: Self = Self(0x22);
    pub const ORDERED_LIST: Self = Self(0x23);
    pub const VARSTORE: Self = Self(0x24);
    pub const VARSTORE_NAME_VALUE: Self = Self(0x25);
    pub const VARSTORE_EFI: Self = Self(0x26);
    pub const VARSTORE_DEVICE: Self = Self(0x27);
    pub const VERSION: Self = Self(0x28);
    pub const END: Self = Self(0x29);
    pub const MATCH: Self = Self(0x2A);
    pub const GET: Self = Self(0x2B);
    pub const SET: Self = Self(0x2C);
    pub const READ: Self = Self(0x2D);
    pub const WRITE: Self = Self(0x2E);
    pub const EQUAL: Self = Self(0x2F);
    pub const NOT_EQUAL: Self = Self(0x30);
    pub const GREATER_THAN: Self = Self(0x31);
    pub const GREATER_EQUAL: Self = Self(0x32);
    pub const LESS_THAN: Self = Self(0x33);
    pub const LESS_EQUAL: Self = Self(0x34);
    pub const BITWISE_AND: Self = Self(0x35);
    pub const BITWISE_OR: Self = Self(0x36);
    pub const BITWISE_NOT: Self = Self(0x37);
    pub const SHIFT_LEFT: Self = Self(0x38);
    pub const SHIFT_RIGHT: Self = Self(0x39);
    pub const ADD: Self = Self(0x3A);
    pub const SUBTRACT: Self = Self(0x3B);
    pub const MULTIPLY: Self = Self(0x3C);
    pub const DIVIDE: Self = Self(0x3D);
    pub const MODULO: Self = Self(0x3E);
    pub const RULE_REF: Self = Self(0x3F);
    pub const QUESTION_REF1: Self = Self(0x40);
    pub const QUESTION_REF2: Self = Self(0x41);
    pub const UINT8: Self = Self(0x42);
    pub const UINT16: Self = Self(0x43);
    pub const UINT32: Self = Self(0x44);
    pub const UINT64: Self = Self(0x45);
    pub const TRUE: Self = Self(0x46);
    pub const FALSE: Self = Self(0x47);
    pub const TO_UINT: Self = Self(0x48);
    pub const TO_STRING: Self = Self(0x49);
    pub const TO_BOOLEAN: Self = Self(0x4A);
    pub const MID: Self = Self(0x4B);
    pub const FIND: Self = Self(0x4C);
    pub const TOKEN: Self = Self(0x4D);
    pub const STRING_REF1: Self = Self(0x4E);
    pub const STRING_REF2: Self = Self(0x4F);
    pub const CONDITIONAL: Self = Self(0x50);
    pub const QUESTION_REF3: Self = Self(0x51);
    pub const ZERO: Self = Self(0x52);
    pub const ONE: Self = Self(0x53);
    pub const ONES: Self = Self(0x54);
    pub const UNDEFINED: Self = Self(0x55);
    pub const LENGTH: Self = Self(0x56);
    pub const DUP: Self = Self(0x57);
    pub const THIS: Self = Self(0x58);
    pub const SPAN: Self = Self(0x59);
    pub const VALUE: Self = Self(0x5A);
    pub const DEFAULT: Self = Self(0x5B);
    pub const DEFAULT_STORE: Self = Self(0x5C);
    pub const FORM_MAP: Self = Self(0x5D);
    pub const CATENATE: Self = Self(0x5E);
    pub const GUID: Self = Self(0x5F);
    pub const SECURITY: Self = Self(0x60);
    pub const MODEL_TAG: Self = Self(0x61);
    pub const REFRESH_ID: Self = Self(0x62);
    pub const WARNING_IF: Self = Self(0x63);
    pub const MATCH2: Self = Self(0x64);
}

/// `EFI_IFR_OP_HEADER`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct IfrOpHeader {
    pub OpCode: IfrOpCode,
    pub Length_Scope: u8,
}

impl IfrOpHeader {
    pub fn Length(&self) -> u8 {
        self.Length_Scope & 0x7F
    }

    pub fn Scope(&self) -> bool {
        self.Length_Scope & 0x80 == 0x80
    }

    pub unsafe fn cast<T>(&self) -> Option<&T> {
        if self.Length() as usize >= mem::size_of::<T>() {
            Some(&*(self as *const Self as *const T))
        } else {
            None
        }
    }
}

macro_rules! unsafe_field {
    ($struct: ident, $field:ident, $type:ty) => {
        pub fn $field(&self) -> Option<&$type> {
            unsafe {
                let self_ptr = self as *const Self;
                let unsafe_self = &*(self_ptr as *const $struct);
                let field_ptr = &unsafe_self.$field as *const $type;
                let length = field_ptr.add(1) as usize - self_ptr as usize;
                if self.Header.Length() as usize >= length {
                    Some(&*field_ptr)
                } else {
                    None
                }
            }
        }
    };
}

#[derive(Debug)]
#[repr(C)]
pub struct IfrAction {
    pub Header: IfrOpHeader,
    pub QuestionHeader: IfrQuestionHeader,
}

#[repr(C)]
struct UnsafeIfrAction {
    safe: IfrAction,
    QuestionConfig: StringId,
}

impl IfrAction {
    unsafe_field!(UnsafeIfrAction, QuestionConfig, StringId);
}

#[derive(Debug)]
#[repr(C, packed)] // Has incorrect size if not packed
pub struct IfrCheckbox {
    pub Header: IfrOpHeader,
    pub Question: IfrQuestionHeader,
    pub Flags: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct IfrForm {
    pub Header: IfrOpHeader,
    pub FormId: FormId,
    pub FormTitle: StringId,
}

#[derive(Debug)]
#[repr(C, packed)] // Has incorrect size if not packed
pub struct IfrNumeric {
    pub Header: IfrOpHeader,
    pub Question: IfrQuestionHeader,
    pub Flags: u8,
    //TODO: MinValue, MaxValue, Step
}

#[derive(Debug)]
#[repr(C)]
pub struct IfrOneOf {
    pub Header: IfrOpHeader,
    pub Question: IfrQuestionHeader,
    pub Flags: u8,
    //TODO: MinValue, MaxValue, Step
}

#[repr(C)]
pub struct IfrOneOfOption {
    pub Header: IfrOpHeader,
    pub Option: StringId,
    pub Flags: u8,
    pub Kind: IfrType,
    pub Value: IfrTypeValue,
}

#[derive(Debug)]
#[repr(C)]
pub struct IfrOrderedList {
    pub Header: IfrOpHeader,
    pub Question: IfrQuestionHeader,
    pub MaxContainers: u8,
    pub Flags: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct IfrRef {
    pub Header: IfrOpHeader,
    pub Question: IfrQuestionHeader,
}

#[repr(C)]
struct UnsafeIfrRef {
    safe: IfrRef,
    FormId: FormId,
    QuestionId: QuestionId,
    FormSetId: Guid,
    DevicePath: *const u16,
}

impl IfrRef {
    unsafe_field!(UnsafeIfrRef, FormId, FormId);
    unsafe_field!(UnsafeIfrRef, QuestionId, QuestionId);
    unsafe_field!(UnsafeIfrRef, FormSetId, Guid);
    unsafe_field!(UnsafeIfrRef, DevicePath, *const u16);
}

#[repr(C, packed)] // Has incorrect size if not packed
pub struct IfrSubtitle {
    pub Header: IfrOpHeader,
    pub Statement: IfrStatementHeader,
    pub Flags: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct IfrText {
    pub Header: IfrOpHeader,
    pub Statement: IfrStatementHeader,
    pub TextTwo: StringId,
}
