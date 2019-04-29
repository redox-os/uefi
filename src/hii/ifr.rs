use crate::guid::Guid;
use super::{FormId, QuestionId, StringId, VarStoreId};

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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HiiTime {
    pub Hour: u8,
    pub Minute: u8,
    pub Second: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HiiDate {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HiiRef {
    pub QuestionId: QuestionId,
    pub FormId: FormId,
    pub FormSetGuid: Guid,
    pub DevicePath: StringId,
}

#[repr(C)]
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

#[repr(C)]
pub struct HiiValue {
    pub Kind: IfrTypeKind,
    pub Buffer: *const u8,
    pub BufferLen: u16,
    pub Value: IfrTypeValue,
}

#[derive(Debug)]
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
#[repr(C)]
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
}

#[derive(Debug)]
#[repr(C)]
pub struct IfrAction {
    pub Header: IfrOpHeader,
    pub QuestionHeader: IfrQuestionHeader,
    // TODO: Make a function returning Option since this header could be
    // sizeof(Action) - sizeof(QuestionConfig)
    pub QuestionConfig: StringId,
}

#[derive(Debug)]
#[repr(C)]
pub struct IfrForm {
    pub Header: IfrOpHeader,
    pub FormId: FormId,
    pub FormTitle: StringId,
}

#[repr(C)]
pub struct IfrOneOfOption {
    pub Header: IfrOpHeader,
    pub Option: StringId,
    pub Flags: u8,
    pub Type: u8,
    pub Value: IfrTypeValue,
}
