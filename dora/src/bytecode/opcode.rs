use crate::bytecode::generate::{BytecodeIdx, Register, StrConstPoolIdx};
use crate::class::ClassDefId;
use crate::field::FieldId;
use crate::vm::{FctId, GlobalId};

#[derive(PartialEq, Debug)]
pub enum Bytecode {
    AddInt(Register, Register, Register),
    AddLong(Register, Register, Register),
    AddFloat(Register, Register, Register),
    AddDouble(Register, Register, Register),

    SubInt(Register, Register, Register),
    SubFloat(Register, Register, Register),

    NegInt(Register, Register),
    NegLong(Register, Register),

    MulInt(Register, Register, Register),
    MulFloat(Register, Register, Register),

    DivInt(Register, Register, Register),
    DivFloat(Register, Register, Register),

    ModInt(Register, Register, Register),
    AndInt(Register, Register, Register),
    OrInt(Register, Register, Register),
    XorInt(Register, Register, Register),
    NotBool(Register, Register),

    ShlInt(Register, Register, Register),
    ShrInt(Register, Register, Register),
    SarInt(Register, Register, Register),

    MovBool(Register, Register),
    MovByte(Register, Register),
    MovChar(Register, Register),
    MovInt(Register, Register),
    MovLong(Register, Register),
    MovFloat(Register, Register),
    MovDouble(Register, Register),
    MovPtr(Register, Register),

    LoadFieldBool(Register, Register, ClassDefId, FieldId),
    LoadFieldByte(Register, Register, ClassDefId, FieldId),
    LoadFieldChar(Register, Register, ClassDefId, FieldId),
    LoadFieldInt(Register, Register, ClassDefId, FieldId),
    LoadFieldLong(Register, Register, ClassDefId, FieldId),
    LoadFieldFloat(Register, Register, ClassDefId, FieldId),
    LoadFieldDouble(Register, Register, ClassDefId, FieldId),
    LoadFieldPtr(Register, Register, ClassDefId, FieldId),

    LoadGlobalBool(Register, GlobalId),
    LoadGlobalByte(Register, GlobalId),
    LoadGlobalChar(Register, GlobalId),
    LoadGlobalInt(Register, GlobalId),
    LoadGlobalLong(Register, GlobalId),
    LoadGlobalFloat(Register, GlobalId),
    LoadGlobalDouble(Register, GlobalId),
    LoadGlobalPtr(Register, GlobalId),

    ConstNil(Register),
    ConstTrue(Register),
    ConstFalse(Register),
    ConstZeroByte(Register),
    ConstZeroInt(Register),
    ConstZeroLong(Register),
    ConstZeroFloat(Register),
    ConstZeroDouble(Register),
    ConstChar(Register, char),
    ConstByte(Register, u8),
    ConstInt(Register, u32),
    ConstLong(Register, u64),
    ConstFloat(Register, f32),
    ConstDouble(Register, f64),
    ConstString(Register, StrConstPoolIdx),

    TestEqPtr(Register, Register, Register),
    TestNePtr(Register, Register, Register),

    TestEqInt(Register, Register, Register),
    TestNeInt(Register, Register, Register),
    TestGtInt(Register, Register, Register),
    TestGeInt(Register, Register, Register),
    TestLtInt(Register, Register, Register),
    TestLeInt(Register, Register, Register),

    TestEqFloat(Register, Register, Register),
    TestNeFloat(Register, Register, Register),
    TestGtFloat(Register, Register, Register),
    TestGeFloat(Register, Register, Register),
    TestLtFloat(Register, Register, Register),
    TestLeFloat(Register, Register, Register),

    JumpIfFalse(Register, BytecodeIdx),
    JumpIfTrue(Register, BytecodeIdx),
    Jump(BytecodeIdx),

    InvokeDirectVoid(FctId, Register, usize),
    InvokeDirectBool(Register, FctId, Register, usize),
    InvokeDirectByte(Register, FctId, Register, usize),
    InvokeDirectChar(Register, FctId, Register, usize),
    InvokeDirectInt(Register, FctId, Register, usize),
    InvokeDirectLong(Register, FctId, Register, usize),
    InvokeDirectFloat(Register, FctId, Register, usize),
    InvokeDirectDouble(Register, FctId, Register, usize),
    InvokeDirectPtr(Register, FctId, Register, usize),

    InvokeVirtualVoid(FctId, Register, usize),
    InvokeVirtualBool(Register, FctId, Register, usize),
    InvokeVirtualByte(Register, FctId, Register, usize),
    InvokeVirtualChar(Register, FctId, Register, usize),
    InvokeVirtualInt(Register, FctId, Register, usize),
    InvokeVirtualLong(Register, FctId, Register, usize),
    InvokeVirtualFloat(Register, FctId, Register, usize),
    InvokeVirtualDouble(Register, FctId, Register, usize),
    InvokeVirtualPtr(Register, FctId, Register, usize),

    InvokeStaticVoid(FctId, Register, usize),
    InvokeStaticBool(Register, FctId, Register, usize),
    InvokeStaticByte(Register, FctId, Register, usize),
    InvokeStaticChar(Register, FctId, Register, usize),
    InvokeStaticInt(Register, FctId, Register, usize),
    InvokeStaticLong(Register, FctId, Register, usize),
    InvokeStaticFloat(Register, FctId, Register, usize),
    InvokeStaticDouble(Register, FctId, Register, usize),
    InvokeStaticPtr(Register, FctId, Register, usize),

    NewObject(Register, ClassDefId),

    Throw(Register),

    RetBool(Register),
    RetByte(Register),
    RetChar(Register),
    RetInt(Register),
    RetLong(Register),
    RetFloat(Register),
    RetDouble(Register),
    RetPtr(Register),

    RetVoid,
}
