use core::mem;

use crate::guid::Guid;
use super::{FormId, QuestionId, StringId, VarStoreId};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IfrTypeKind {
    U8 = 0x00,
    U16 = 0x01,
    U32 = 0x02,
    U64 = 0x03,
    Bool = 0x04,
    Time = 0x05,
    Date = 0x06,
    String = 0x07,
    Other = 0x08,
    Undefined = 0x09,
    Action = 0x0A,
    Buffer = 0x0B,
    Ref = 0x0C,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct HiiTime {
    pub Hour: u8,
    pub Minute: u8,
    pub Second: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct HiiDate {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(packed)] // Fails to have correct size with repr(C)
pub struct HiiRef {
    pub QuestionId: QuestionId,
    pub FormId: FormId,
    pub FormSetGuid: Guid,
    pub DevicePath: StringId,
}

#[derive(Clone, Copy)]
#[repr(packed)] // Fails to have correct size with repr(C)
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
    // buffer: [u8]
}

impl IfrTypeValue {
    pub unsafe fn to_enum(self, kind: IfrTypeKind) -> IfrTypeValueEnum {
        match kind {
            IfrTypeKind::U8 => IfrTypeValueEnum::U8(self.u8),
            IfrTypeKind::U16 => IfrTypeValueEnum::U16(self.u16),
            IfrTypeKind::U32 => IfrTypeValueEnum::U32(self.u32),
            IfrTypeKind::U64 => IfrTypeValueEnum::U64(self.u64),
            IfrTypeKind::Bool => IfrTypeValueEnum::Bool(self.b),
            IfrTypeKind::Time => IfrTypeValueEnum::Time(self.time),
            IfrTypeKind::Date => IfrTypeValueEnum::Date(self.date),
            IfrTypeKind::String => IfrTypeValueEnum::String(self.string),
            IfrTypeKind::Ref => IfrTypeValueEnum::Ref(self.reference),
            _ => IfrTypeValueEnum::Other(
                kind,
                mem::transmute(self)
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IfrTypeValueEnum {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Bool(bool),
    Time(HiiTime),
    Date(HiiDate),
    String(StringId),
    Ref(HiiRef),
    Other(IfrTypeKind, [u8; 22])
}

impl IfrTypeValueEnum {
    pub unsafe fn to_union(self) -> (IfrTypeKind, IfrTypeValue) {
        match self {
            IfrTypeValueEnum::U8(u8) => (IfrTypeKind::U8, IfrTypeValue { u8 }),
            IfrTypeValueEnum::U16(u16) => (IfrTypeKind::U16, IfrTypeValue { u16 }),
            IfrTypeValueEnum::U32(u32) => (IfrTypeKind::U32, IfrTypeValue { u32 }),
            IfrTypeValueEnum::U64(u64) => (IfrTypeKind::U64, IfrTypeValue { u64 }),
            IfrTypeValueEnum::Bool(b) => (IfrTypeKind::Bool, IfrTypeValue { b }),
            IfrTypeValueEnum::Time(time) => (IfrTypeKind::Time, IfrTypeValue { time }),
            IfrTypeValueEnum::Date(date) => (IfrTypeKind::Date, IfrTypeValue { date }),
            IfrTypeValueEnum::String(string) => (IfrTypeKind::String, IfrTypeValue { string }),
            IfrTypeValueEnum::Ref(reference) => (IfrTypeKind::Ref, IfrTypeValue { reference }),
            IfrTypeValueEnum::Other(kind, data) => (kind, mem::transmute(data)),
        }
    }
}

#[repr(C)]
pub struct HiiValue {
    pub Kind: IfrTypeKind,
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

#[derive(Debug)]
#[repr(packed)] // Has incorrect size if not packed
pub struct IfrQuestionHeader {
    pub Header: IfrStatementHeader,
    pub QuestionId: QuestionId,
    pub VarStoreId: VarStoreId,
    // union { VarName: StringId, VarOffset: u16 }
    pub VarStoreInfo: u16,
    pub Flags: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum IfrOpCode {
    Form = 0x01,
    Subtitle = 0x02,
    Text = 0x03,
    Image = 0x04,
    OneOf = 0x05,
    Checkbox = 0x06,
    Numeric = 0x07,
    Password = 0x08,
    OneOfOption = 0x09,
    SuppressIf = 0x0A,
    Locked = 0x0B,
    Action = 0x0C,
    ResetButton = 0x0D,
    FormSet = 0x0E,
    Ref = 0x0F,
    NoSubmitIf = 0x10,
    InconsistentIf = 0x11,
    EqIdVal = 0x12,
    EqIdId = 0x13,
    EqIdValList = 0x14,
    And = 0x15,
    Or = 0x16,
    Not = 0x17,
    Rule = 0x18,
    GrayOutIf = 0x19,
    Date = 0x1A,
    Time = 0x1B,
    String = 0x1C,
    Refresh = 0x1D,
    DisableIf = 0x1E,
    Animation = 0x1F,
    ToLower = 0x20,
    ToUpper = 0x21,
    Map = 0x22,
    OrderedList = 0x23,
    VarStore = 0x24,
    VarStoreNameValue = 0x25,
    VarStoreEfi = 0x26,
    VarStoreDevice = 0x27,
    Version = 0x28,
    End = 0x29,
    Match = 0x2A,
    Get = 0x2B,
    Set = 0x2C,
    Read = 0x2D,
    Write = 0x2E,
    Equal = 0x2F,
    NotEqual = 0x30,
    GreaterThan = 0x31,
    GreaterEqual = 0x32,
    LessThan = 0x33,
    LessEqual = 0x34,
    BitwiseAnd = 0x35,
    BitwiseOr = 0x36,
    BitwiseNot = 0x37,
    ShiftLeft = 0x38,
    ShiftRight = 0x39,
    Add = 0x3A,
    Subtract = 0x3B,
    Multiply = 0x3C,
    Divide = 0x3D,
    Modulo = 0x3E,
    RuleRef = 0x3F,
    QuestionRef1 = 0x40,
    QuestionRef2 = 0x41,
    Uint8 =0x42,
    Uint16 = 0x43,
    Uint32 = 0x44,
    Uint64 = 0x45,
    True = 0x46,
    False = 0x47,
    ToUint = 0x48,
    ToString = 0x49,
    ToBoolean = 0x4A,
    Mid = 0x4B,
    Find = 0x4C,
    Token = 0x4D,
    StringRef1 = 0x4E,
    StringRef2 = 0x4F,
    Conditional = 0x50,
    QuestionRef3 = 0x51,
    Zero = 0x52,
    One = 0x53,
    Ones = 0x54,
    Undefined = 0x55,
    Length = 0x56,
    Dup = 0x57,
    This = 0x58,
    Span = 0x59,
    Value = 0x5A,
    Default = 0x5B,
    DefaultStore = 0x5C,
    FormMap = 0x5D,
    Catenate = 0x5E,
    Guid = 0x5F,
    Security = 0x60,
    ModelTag = 0x61,
    RefreshId = 0x62,
    WarningIf = 0x63,
    Match2 = 0x64,
}

#[derive(Debug)]
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
    ($struct: ident, $field:ident, $type:ty) => (
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
    );
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
#[repr(packed)] // Has incorrect size if not packed
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
#[repr(packed)] // Has incorrect size if not packed
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
    pub Kind: IfrTypeKind,
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
    pub Question: IfrQuestionHeader
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

#[repr(packed)] // Has incorrect size if not packed
pub struct IfrSubtitle {
    pub Header: IfrOpHeader,
    pub Statement: IfrStatementHeader,
    pub Flags: u8,
}
