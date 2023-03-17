use crate::assembler::NEXTINSTRUCTION;
use crate::error::Error;
use crate::markers::{Label, Marker, Number, Symbol};
use std::any::{Any, TypeId};

type LabelResolver = fn(&Label) -> Result<u16, Error>;
type SymbolResolver = fn(&Symbol) -> Result<u16, Error>;
type Register = u16;
type IoMode = &'static str;

const REG0: Register = 0;
const REG1: Register = 1;
const REG2: Register = 2;
const REG3: Register = 3;
const ADDRESS_MODE: IoMode = "Addr";
const DATA_MODE: IoMode = "Data";

pub trait Instruction: Any {
    fn string(&self) -> String;
    fn emit(
        &self,
        label_resolver: Option<LabelResolver>,
        symbol_resolver: Option<SymbolResolver>,
    ) -> Result<Vec<u16>, Error>;
    fn size(&self) -> u16;
    fn as_any(&self) -> &dyn Any;
}

// LOADS
// ----------------------
// arg A = memory address to load from
// arg B = register to store value in
// 0x0000 = LD R0, R0
// 0x0001 = LD R0, R1
// 0x0002 = LD R0, R2
// 0x0003 = LD R0, R3

// 0x0004 = LD R1, R0
// 0x0005 = LD R1, R1
// 0x0006 = LD R1, R2
// 0x0007 = LD R1, R3

// 0x0008 = LD R2, R0
// 0x0009 = LD R2, R1
// 0x000A = LD R2, R2
// 0x000B = LD R2, R3

// 0x000C = LD R3, R0
// 0x000D = LD R3, R1
// 0x000E = LD R3, R2
// 0x000F = LD R3, R3

struct LOAD {
    memory_address_reg: Register,
    to_register: Register,
}

impl LOAD {
    fn new(memory_address_reg: Register, to_register: Register) -> Self {
        Self {
            memory_address_reg,
            to_register,
        }
    }
}

impl Instruction for LOAD {
    fn string(&self) -> String {
        format!("LD R{}, R{}", self.memory_address_reg, self.to_register)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.memory_address_reg {
                REG0 => Ok(0x0000),
                REG1 => Ok(0x0004),
                REG2 => Ok(0x0008),
                REG3 => Ok(0x000C),
                _ => Err(Error::UnknownRegister(self.memory_address_reg.to_string())),
            }? + self.to_register,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// STORES
// ----------------------
// arg A = memory address for value
// arg B = value to store in memory
// 0x0010 = ST R0, R0
// 0x0011 = ST R0, R1
// 0x0012 = ST R0, R2
// 0x0013 = ST R0, R3

// 0x0014 = ST R1, R0
// 0x0015 = ST R1, R1
// 0x0016 = ST R1, R2
// 0x0017 = ST R1, R3

// 0x0018 = ST R2, R0
// 0x0019 = ST R2, R1
// 0x001A = ST R2, R2
// 0x001B = ST R2, R3

// 0x001C = ST R3, R0
// 0x001D = ST R3, R1
// 0x001E = ST R3, R2
// 0x001F = ST R3, R3

struct STORE {
    from_register: Register,
    to_register: Register,
}

impl STORE {
    fn new(from_register: Register, to_register: Register) -> Self {
        Self {
            from_register,
            to_register,
        }
    }
}

impl Instruction for STORE {
    fn string(&self) -> String {
        format!("ST R{}, R{}", self.from_register, self.to_register)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.from_register {
                REG0 => Ok(0x0010),
                REG1 => Ok(0x0014),
                REG2 => Ok(0x0018),
                REG3 => Ok(0x001C),
                _ => Err(Error::UnknownRegister(self.from_register.to_string())),
            }? + self.to_register,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// DATA
// put value in memory into register (2 byte instruction)
// ----------------------
// 0x0020 = DATA R0
// 0x0021 = DATA R1
// 0x0022 = DATA R2
// 0x0023 = DATA R3

struct DATA<T: Marker> {
    to_register: Register,
    data: T,
}

impl<T: Marker> DATA<T> {
    fn new(to_register: Register, data: T) -> Self {
        Self { to_register, data }
    }
}

impl<T: Marker> Instruction for DATA<T> {
    fn string(&self) -> String {
        format!("DATA R{}, {}", self.to_register, self.data.string())
    }

    fn emit(
        &self,
        _: Option<LabelResolver>,
        symbol_resolver: Option<SymbolResolver>,
    ) -> Result<Vec<u16>, Error> {
        let instruction = match self.to_register {
            REG0 => Ok(0x0020),
            REG1 => Ok(0x0021),
            REG2 => Ok(0x0022),
            REG3 => Ok(0x0023),
            _ => Err(Error::UnknownRegister(self.to_register.to_string())),
        }?;

        if TypeId::of::<Symbol>() == self.data.type_id() {
            Ok(vec![
                instruction,
                symbol_resolver.unwrap()(self.data.as_any().downcast_ref::<Symbol>().unwrap())?,
            ])
        } else if TypeId::of::<Number>() == self.data.type_id() {
            Ok(vec![
                instruction,
                self.data.as_any().downcast_ref::<Number>().unwrap().value,
            ])
        } else {
            Err(Error::UnknownMarker(self.data.string()))
        }
    }

    fn size(&self) -> u16 {
        2
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// JR
// set instruction address register to value in register
// ----------------------
// 0x0030 = JR R0
// 0x0031 = JR R1
// 0x0032 = JR R2
// 0x0033 = JR R3
struct JR {
    register: Register,
}

impl JR {
    fn new(register: Register) -> Self {
        Self { register }
    }
}

impl Instruction for JR {
    fn string(&self) -> String {
        format!("JR R{}", self.register)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![match self.register {
            REG0 => Ok(0x0030),
            REG1 => Ok(0x0031),
            REG2 => Ok(0x0032),
            REG3 => Ok(0x0033),
            _ => Err(Error::UnknownRegister(self.register.to_string())),
        }?])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// JMP
// set instruction address register to next byte (2 byte instruction)
// ----------------------
// 0x0040 = JMP <value>

struct JMP {
    jump_location: Label,
}

impl JMP {
    fn new(jump_location: Label) -> Self {
        Self { jump_location }
    }
}

impl Instruction for JMP {
    fn string(&self) -> String {
        format!("JMP {}", self.jump_location.string())
    }

    fn emit(
        &self,
        label_resolver: Option<LabelResolver>,
        _: Option<SymbolResolver>,
    ) -> Result<Vec<u16>, Error> {
        Ok(vec![0x0040, label_resolver.unwrap()(&self.jump_location)?])
    }

    fn size(&self) -> u16 {
        2
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// JMP(CAEZ)
// set instruction address register to next byte (2 byte instruction)
// jump if <flag(s)> are true
// ----------------------
// 0x0051 = JMPZ <value>
// 0x0052 = JMPE <value>
// 0x0053 = JMPEZ <value>
// 0x0054 = JMPA <value>
// 0x0055 = JMPAZ <value>
// 0x0056 = JMPAE <value>
// 0x0057 = JMPAEZ <value>
// 0x0058 = JMPC <value>
// 0x0059 = JMPCZ <value>
// 0x005A = JMPCE <value>
// 0x005B = JMPCEZ <value>
// 0x005C = JMPCA <value>
// 0x005D = JMPCAZ <value>
// 0x005E = JMPCAE <value>
// 0x005F = JMPCAEZ <value>

struct JMPF {
    flags: Vec<String>,
    jump_location: Label,
}

impl JMPF {
    fn new(flags: Vec<String>, jump_location: Label) -> Self {
        Self {
            flags,
            jump_location,
        }
    }
}

impl Instruction for JMPF {
    fn string(&self) -> String {
        format!("JMP{} {}", self.flags.join(""), self.jump_location.string())
    }

    fn emit(
        &self,
        label_resolver: Option<LabelResolver>,
        _: Option<SymbolResolver>,
    ) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.flags.join("").as_str() {
                "Z" => Ok(0x0051),
                "E" => Ok(0x0052),
                "EZ" => Ok(0x0053),
                "A" => Ok(0x0054),
                "AZ" => Ok(0x0055),
                "AE" => Ok(0x0056),
                "AEZ" => Ok(0x0057),
                "C" => Ok(0x0058),
                "CZ" => Ok(0x0059),
                "CE" => Ok(0x005A),
                "CEZ" => Ok(0x005B),
                "CA" => Ok(0x005C),
                "CAZ" => Ok(0x005D),
                "CAE" => Ok(0x005E),
                "CAEZ" => Ok(0x005F),
                _ => Err(Error::UnknownFlag(self.flags.join(""))),
            }?,
            label_resolver.unwrap()(&self.jump_location)?,
        ])
    }

    fn size(&self) -> u16 {
        2
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// CLF (CLEAR FLAGS)
// ----------------------
// 0x0060 CLF
struct CLF {}

impl CLF {
    fn new() -> Self {
        Self {}
    }
}

impl Instruction for CLF {
    fn string(&self) -> String {
        "CLF".to_string()
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![0x0060])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// IN
// ----------------------
// 0x0070 = IN Data, R0
// 0x0071 = IN Data, R1
// 0x0072 = IN Data, R2
// 0x0073 = IN Data, R3
// 0x0074 = IN Addr, R0
// 0x0075 = IN Addr, R1
// 0x0076 = IN Addr, R2
// 0x0077 = IN Addr, R3
struct IN {
    io_mode: IoMode,
    to_register: Register,
}

impl IN {
    fn new(io_mode: IoMode, to_register: Register) -> Self {
        Self {
            io_mode,
            to_register,
        }
    }
}

impl Instruction for IN {
    fn string(&self) -> String {
        format!("IN {}, R{}", self.io_mode, self.to_register)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.io_mode {
                DATA_MODE => Ok(0x0070),
                ADDRESS_MODE => Ok(0x0074),
                _ => Err(Error::UnknownIoMode(self.io_mode.to_string())),
            }? + self.to_register,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// OUT
// ----------------------
// 0x0078 = OUT Data, R0
// 0x0079 = OUT Data, R1
// 0x007A = OUT Data, R2
// 0x007B = OUT Data, R3
// 0x007C = OUT Addr, R0
// 0x007D = OUT Addr, R1
// 0x007E = OUT Addr, R2
// 0x007F = OUT Addr, R3
struct OUT {
    io_mode: IoMode,
    to_register: Register,
}

impl OUT {
    fn new(io_mode: IoMode, to_register: Register) -> Self {
        Self {
            io_mode,
            to_register,
        }
    }
}

impl Instruction for OUT {
    fn string(&self) -> String {
        format!("OUT {}, R{}", self.io_mode, self.to_register)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.io_mode {
                DATA_MODE => Ok(0x0078),
                ADDRESS_MODE => Ok(0x007C),
                _ => Err(Error::UnknownIoMode(self.io_mode.to_string())),
            }? + self.to_register,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ADDS
// ----------------------
// 0x0080 = ADD R0, R0
// 0x0081 = ADD R0, R1
// 0x0082 = ADD R0, R2
// 0x0083 = ADD R0, R3

// 0x0084 = ADD R1, R0
// 0x0085 = ADD R1, R1
// 0x0086 = ADD R1, R2
// 0x0087 = ADD R1, R3

// 0x0088 = ADD R2, R0
// 0x0089 = ADD R2, R1
// 0x008A = ADD R2, R2
// 0x008B = ADD R2, R3

// 0x008C = ADD R3, R0
// 0x008D = ADD R3, R1
// 0x008E = ADD R3, R2
// 0x008F = ADD R3, R3
struct ADD {
    register_b: Register,
    register_a: Register,
}

impl ADD {
    fn new(register_a: Register, register_b: Register) -> ADD {
        ADD {
            register_a,
            register_b,
        }
    }
}

impl Instruction for ADD {
    fn string(&self) -> String {
        format!("ADD R{}, R{}", self.register_a, self.register_b)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.register_a {
                REG0 => Ok(0x0080),
                REG1 => Ok(0x0084),
                REG2 => Ok(0x0088),
                REG3 => Ok(0x008C),
                _ => Err(Error::UnknownRegister(self.register_a.to_string())),
            }? + self.register_b,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// SHL
// ----------------------
// 0x0090 = SHL R0
// 0x0095 = SHL R1
// 0x009A = SHL R2
// 0x009F = SHL R3
struct SHL {
    register: Register,
}

impl SHL {
    fn new(register: Register) -> SHL {
        SHL { register }
    }
}

impl Instruction for SHL {
    fn string(&self) -> String {
        format!("SHL R{}", self.register)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![match self.register {
            REG0 => Ok(0x0090),
            REG1 => Ok(0x0095),
            REG2 => Ok(0x009A),
            REG3 => Ok(0x009F),
            _ => Err(Error::UnknownRegister(self.register.to_string())),
        }?])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// SHR
// ----------------------
// 0x00A0 = SHR R0
// 0x00A5 = SHR R1
// 0x00AA = SHR R2
// 0x00AF = SHR R3
struct SHR {
    register: Register,
}

impl SHR {
    fn new(register: Register) -> SHR {
        SHR { register }
    }
}

impl Instruction for SHR {
    fn string(&self) -> String {
        format!("SHR R{}", self.register)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![match self.register {
            REG0 => Ok(0x00A0),
            REG1 => Ok(0x00A5),
            REG2 => Ok(0x00AA),
            REG3 => Ok(0x00AF),
            _ => Err(Error::UnknownRegister(self.register.to_string())),
        }?])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// NOT
// ----------------------
// 0x00B0 = NOT R0
// 0x00B5 = NOT R1
// 0x00BA = NOT R2
// 0x00BF = NOT R3
struct NOT {
    register: Register,
}

impl NOT {
    fn new(register: Register) -> NOT {
        NOT { register }
    }
}

impl Instruction for NOT {
    fn string(&self) -> String {
        format!("NOT R{}", self.register)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![match self.register {
            REG0 => Ok(0x00B0),
            REG1 => Ok(0x00B5),
            REG2 => Ok(0x00BA),
            REG3 => Ok(0x00BF),
            _ => Err(Error::UnknownRegister(self.register.to_string())),
        }?])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ANDS
// ----------------------
// 0x00C0 = AND R0, R0
// 0x00C1 = AND R0, R1
// 0x00C2 = AND R0, R2
// 0x00C3 = AND R0, R3

// 0x00C4 = AND R1, R0
// 0x00C5 = AND R1, R1
// 0x00C6 = AND R1, R2
// 0x00C7 = AND R1, R3

// 0x00C8 = AND R2, R0
// 0x00C9 = AND R2, R1
// 0x00CA = AND R2, R2
// 0x00CB = AND R2, R3

// 0x00CC = AND R3, R0
// 0x00CD = AND R3, R1
// 0x00CE = AND R3, R2
// 0x00CF = AND R3, R3
struct AND {
    register_a: Register,
    register_b: Register,
}

impl AND {
    fn new(register_a: Register, register_b: Register) -> AND {
        AND {
            register_a,
            register_b,
        }
    }
}

impl Instruction for AND {
    fn string(&self) -> String {
        format!("AND R{}, R{}", self.register_a, self.register_b)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.register_a {
                REG0 => Ok(0x00C0),
                REG1 => Ok(0x00C4),
                REG2 => Ok(0x00C8),
                REG3 => Ok(0x00CC),
                _ => Err(Error::UnknownRegister(self.register_a.to_string())),
            }? + self.register_b,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ORS
// ----------------------
// 0x00D0 = OR R0, R0
// 0x00D1 = OR R0, R1
// 0x00D2 = OR R0, R2
// 0x00D3 = OR R0, R3

// 0x00D4 = OR R1, R0
// 0x00D5 = OR R1, R1
// 0x00D6 = OR R1, R2
// 0x00D7 = OR R1, R3

// 0x00D8 = OR R2, R0
// 0x00D9 = OR R2, R1
// 0x00DA = OR R2, R2
// 0x00DB = OR R2, R3

// 0x00DC = OR R3, R0
// 0x00DD = OR R3, R1
// 0x00DE = OR R3, R2
// 0x00DF = OR R3, R3
struct OR {
    register_a: Register,
    register_b: Register,
}

impl OR {
    fn new(register_a: Register, register_b: Register) -> OR {
        OR {
            register_a,
            register_b,
        }
    }
}

impl Instruction for OR {
    fn string(&self) -> String {
        format!("OR R{}, R{}", self.register_a, self.register_b)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.register_a {
                REG0 => Ok(0x00D0),
                REG1 => Ok(0x00D4),
                REG2 => Ok(0x00D8),
                REG3 => Ok(0x00DC),
                _ => Err(Error::UnknownRegister(self.register_a.to_string())),
            }? + self.register_b,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// XORS
// ----------------------
// 0x00E0 = XOR R0, R0
// 0x00E1 = XOR R0, R1
// 0x00E2 = XOR R0, R2
// 0x00E3 = XOR R0, R3

// 0x00E4 = XOR R1, R0
// 0x00E5 = XOR R1, R1
// 0x00E6 = XOR R1, R2
// 0x00E7 = XOR R1, R3

// 0x00E8 = XOR R2, R0
// 0x00E9 = XOR R2, R1
// 0x00EA = XOR R2, R2
// 0x00EB = XOR R2, R3

// 0x00EC = XOR R3, R0
// 0x00ED = XOR R3, R1
// 0x00EE = XOR R3, R2
// 0x00EF = XOR R3, R3
struct XOR {
    register_a: Register,
    register_b: Register,
}

impl XOR {
    fn new(register_a: Register, register_b: Register) -> XOR {
        XOR {
            register_a,
            register_b,
        }
    }
}

impl Instruction for XOR {
    fn string(&self) -> String {
        format!("XOR R{}, R{}", self.register_a, self.register_b)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.register_a {
                REG0 => Ok(0x00E0),
                REG1 => Ok(0x00E4),
                REG2 => Ok(0x00E8),
                REG3 => Ok(0x00EC),
                _ => Err(Error::UnknownRegister(self.register_a.to_string())),
            }? + self.register_b,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// CMP
// ----------------------
// 0x00F0 = CMP R0, R0
// 0x00F1 = CMP R0, R1
// 0x00F2 = CMP R0, R2
// 0x00F3 = CMP R0, R3

// 0x00F4 = CMP R1, R0
// 0x00F5 = CMP R1, R1
// 0x00F6 = CMP R1, R2
// 0x00F7 = CMP R1, R3

// 0x00F8 = CMP R2, R0
// 0x00F9 = CMP R2, R1
// 0x00FA = CMP R2, R2
// 0x00FB = CMP R2, R3

// 0x00FC = CMP R3, R0
// 0x00FD = CMP R3, R1
// 0x00FE = CMP R3, R2
// 0x00FF = CMP R3, R3
struct CMP {
    register_a: Register,
    register_b: Register,
}

impl CMP {
    fn new(register_a: Register, register_b: Register) -> CMP {
        CMP {
            register_a,
            register_b,
        }
    }
}

impl Instruction for CMP {
    fn string(&self) -> String {
        format!("CMP R{}, R{}", self.register_a, self.register_b)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![
            match self.register_a {
                REG0 => Ok(0x00F0),
                REG1 => Ok(0x00F4),
                REG2 => Ok(0x00F8),
                REG3 => Ok(0x00FC),
                _ => Err(Error::UnknownRegister(self.register_a.to_string())),
            }? + self.register_b,
        ])
    }

    fn size(&self) -> u16 {
        1
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// PLACEHOLDER INSTRUCTIONS - these are used by the assembler
pub struct DEFLABEL {
    pub name: String,
}

impl Instruction for DEFLABEL {
    fn string(&self) -> String {
        self.name.clone()
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![])
    }

    fn size(&self) -> u16 {
        0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct DEFSYMBOL {
    pub name: String,
    pub value: u16,
}

impl Instruction for DEFSYMBOL {
    fn string(&self) -> String {
        format!("%{} = 0x{:X}", self.name, self.value)
    }

    fn emit(&self, _: Option<LabelResolver>, _: Option<SymbolResolver>) -> Result<Vec<u16>, Error> {
        Ok(vec![])
    }

    fn size(&self) -> u16 {
        0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// PSUEDO INSTRUCTIONS - these are  composite instructions that may map to multiple opcodes
struct CALL {
    routine: Label,
}

impl CALL {
    fn new(routine: Label) -> CALL {
        CALL { routine }
    }
}

impl Instruction for CALL {
    fn string(&self) -> String {
        format!("CALL {}", self.routine.string())
    }

    fn emit(
        &self,
        label_resolver: Option<LabelResolver>,
        symbol_resolver: Option<SymbolResolver>,
    ) -> Result<Vec<u16>, Error> {
        let next_instruction_address = symbol_resolver.unwrap()(&Symbol::new(NEXTINSTRUCTION))?;
        let composite_instructions: Vec<Box<dyn Instruction>> = vec![
            Box::new(DATA::new(REG3, Number::new(next_instruction_address))),
            Box::new(JMP::new(self.routine.clone())),
        ];

        let mut emiited = Vec::new();
        for i in composite_instructions.iter() {
            let mut e = i.emit(label_resolver, symbol_resolver)?;
            emiited.append(&mut e);
        }

        Ok(emiited)
    }

    fn size(&self) -> u16 {
        4
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Instructions - useful list data structure for convienience

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markers::Number;

    #[test]
    fn test_instruction_two_reg_string() {
        let instructions: Vec<(Box<dyn Instruction>, &str)> = vec![
            // LOAD
            (Box::new(LOAD::new(REG0, REG0)), "LD R0, R0"),
            (Box::new(LOAD::new(REG0, REG1)), "LD R0, R1"),
            (Box::new(LOAD::new(REG0, REG2)), "LD R0, R2"),
            (Box::new(LOAD::new(REG0, REG3)), "LD R0, R3"),
            (Box::new(LOAD::new(REG1, REG0)), "LD R1, R0"),
            (Box::new(LOAD::new(REG1, REG1)), "LD R1, R1"),
            (Box::new(LOAD::new(REG1, REG2)), "LD R1, R2"),
            (Box::new(LOAD::new(REG1, REG3)), "LD R1, R3"),
            (Box::new(LOAD::new(REG2, REG0)), "LD R2, R0"),
            (Box::new(LOAD::new(REG2, REG1)), "LD R2, R1"),
            (Box::new(LOAD::new(REG2, REG2)), "LD R2, R2"),
            (Box::new(LOAD::new(REG2, REG3)), "LD R2, R3"),
            (Box::new(LOAD::new(REG3, REG0)), "LD R3, R0"),
            (Box::new(LOAD::new(REG3, REG1)), "LD R3, R1"),
            (Box::new(LOAD::new(REG3, REG2)), "LD R3, R2"),
            (Box::new(LOAD::new(REG3, REG3)), "LD R3, R3"),
            // STORE
            (Box::new(STORE::new(REG0, REG0)), "ST R0, R0"),
            (Box::new(STORE::new(REG0, REG1)), "ST R0, R1"),
            (Box::new(STORE::new(REG0, REG2)), "ST R0, R2"),
            (Box::new(STORE::new(REG0, REG3)), "ST R0, R3"),
            (Box::new(STORE::new(REG1, REG0)), "ST R1, R0"),
            (Box::new(STORE::new(REG1, REG1)), "ST R1, R1"),
            (Box::new(STORE::new(REG1, REG2)), "ST R1, R2"),
            (Box::new(STORE::new(REG1, REG3)), "ST R1, R3"),
            (Box::new(STORE::new(REG2, REG0)), "ST R2, R0"),
            (Box::new(STORE::new(REG2, REG1)), "ST R2, R1"),
            (Box::new(STORE::new(REG2, REG2)), "ST R2, R2"),
            (Box::new(STORE::new(REG2, REG3)), "ST R2, R3"),
            (Box::new(STORE::new(REG3, REG0)), "ST R3, R0"),
            (Box::new(STORE::new(REG3, REG1)), "ST R3, R1"),
            (Box::new(STORE::new(REG3, REG2)), "ST R3, R2"),
            (Box::new(STORE::new(REG3, REG3)), "ST R3, R3"),
            // ADD
            (Box::new(ADD::new(REG0, REG0)), "ADD R0, R0"),
            (Box::new(ADD::new(REG0, REG1)), "ADD R0, R1"),
            (Box::new(ADD::new(REG0, REG2)), "ADD R0, R2"),
            (Box::new(ADD::new(REG0, REG3)), "ADD R0, R3"),
            (Box::new(ADD::new(REG1, REG0)), "ADD R1, R0"),
            (Box::new(ADD::new(REG1, REG1)), "ADD R1, R1"),
            (Box::new(ADD::new(REG1, REG2)), "ADD R1, R2"),
            (Box::new(ADD::new(REG1, REG3)), "ADD R1, R3"),
            (Box::new(ADD::new(REG2, REG0)), "ADD R2, R0"),
            (Box::new(ADD::new(REG2, REG1)), "ADD R2, R1"),
            (Box::new(ADD::new(REG2, REG2)), "ADD R2, R2"),
            (Box::new(ADD::new(REG2, REG3)), "ADD R2, R3"),
            (Box::new(ADD::new(REG3, REG0)), "ADD R3, R0"),
            (Box::new(ADD::new(REG3, REG1)), "ADD R3, R1"),
            (Box::new(ADD::new(REG3, REG2)), "ADD R3, R2"),
            (Box::new(ADD::new(REG3, REG3)), "ADD R3, R3"),
            // AND
            (Box::new(AND::new(REG0, REG0)), "AND R0, R0"),
            (Box::new(AND::new(REG0, REG1)), "AND R0, R1"),
            (Box::new(AND::new(REG0, REG2)), "AND R0, R2"),
            (Box::new(AND::new(REG0, REG3)), "AND R0, R3"),
            (Box::new(AND::new(REG1, REG0)), "AND R1, R0"),
            (Box::new(AND::new(REG1, REG1)), "AND R1, R1"),
            (Box::new(AND::new(REG1, REG2)), "AND R1, R2"),
            (Box::new(AND::new(REG1, REG3)), "AND R1, R3"),
            (Box::new(AND::new(REG2, REG0)), "AND R2, R0"),
            (Box::new(AND::new(REG2, REG1)), "AND R2, R1"),
            (Box::new(AND::new(REG2, REG2)), "AND R2, R2"),
            (Box::new(AND::new(REG2, REG3)), "AND R2, R3"),
            (Box::new(AND::new(REG3, REG0)), "AND R3, R0"),
            (Box::new(AND::new(REG3, REG1)), "AND R3, R1"),
            (Box::new(AND::new(REG3, REG2)), "AND R3, R2"),
            (Box::new(AND::new(REG3, REG3)), "AND R3, R3"),
            // OR
            (Box::new(OR::new(REG0, REG0)), "OR R0, R0"),
            (Box::new(OR::new(REG0, REG1)), "OR R0, R1"),
            (Box::new(OR::new(REG0, REG2)), "OR R0, R2"),
            (Box::new(OR::new(REG0, REG3)), "OR R0, R3"),
            (Box::new(OR::new(REG1, REG0)), "OR R1, R0"),
            (Box::new(OR::new(REG1, REG1)), "OR R1, R1"),
            (Box::new(OR::new(REG1, REG2)), "OR R1, R2"),
            (Box::new(OR::new(REG1, REG3)), "OR R1, R3"),
            (Box::new(OR::new(REG2, REG0)), "OR R2, R0"),
            (Box::new(OR::new(REG2, REG1)), "OR R2, R1"),
            (Box::new(OR::new(REG2, REG2)), "OR R2, R2"),
            (Box::new(OR::new(REG2, REG3)), "OR R2, R3"),
            (Box::new(OR::new(REG3, REG0)), "OR R3, R0"),
            (Box::new(OR::new(REG3, REG1)), "OR R3, R1"),
            (Box::new(OR::new(REG3, REG2)), "OR R3, R2"),
            (Box::new(OR::new(REG3, REG3)), "OR R3, R3"),
            // XOR
            (Box::new(XOR::new(REG0, REG0)), "XOR R0, R0"),
            (Box::new(XOR::new(REG0, REG1)), "XOR R0, R1"),
            (Box::new(XOR::new(REG0, REG2)), "XOR R0, R2"),
            (Box::new(XOR::new(REG0, REG3)), "XOR R0, R3"),
            (Box::new(XOR::new(REG1, REG0)), "XOR R1, R0"),
            (Box::new(XOR::new(REG1, REG1)), "XOR R1, R1"),
            (Box::new(XOR::new(REG1, REG2)), "XOR R1, R2"),
            (Box::new(XOR::new(REG1, REG3)), "XOR R1, R3"),
            (Box::new(XOR::new(REG2, REG0)), "XOR R2, R0"),
            (Box::new(XOR::new(REG2, REG1)), "XOR R2, R1"),
            (Box::new(XOR::new(REG2, REG2)), "XOR R2, R2"),
            (Box::new(XOR::new(REG2, REG3)), "XOR R2, R3"),
            (Box::new(XOR::new(REG3, REG0)), "XOR R3, R0"),
            (Box::new(XOR::new(REG3, REG1)), "XOR R3, R1"),
            (Box::new(XOR::new(REG3, REG2)), "XOR R3, R2"),
            (Box::new(XOR::new(REG3, REG3)), "XOR R3, R3"),
            // CMP
            (Box::new(CMP::new(REG0, REG0)), "CMP R0, R0"),
            (Box::new(CMP::new(REG0, REG1)), "CMP R0, R1"),
            (Box::new(CMP::new(REG0, REG2)), "CMP R0, R2"),
            (Box::new(CMP::new(REG0, REG3)), "CMP R0, R3"),
            (Box::new(CMP::new(REG1, REG0)), "CMP R1, R0"),
            (Box::new(CMP::new(REG1, REG1)), "CMP R1, R1"),
            (Box::new(CMP::new(REG1, REG2)), "CMP R1, R2"),
            (Box::new(CMP::new(REG1, REG3)), "CMP R1, R3"),
            (Box::new(CMP::new(REG2, REG0)), "CMP R2, R0"),
            (Box::new(CMP::new(REG2, REG1)), "CMP R2, R1"),
            (Box::new(CMP::new(REG2, REG2)), "CMP R2, R2"),
            (Box::new(CMP::new(REG2, REG3)), "CMP R2, R3"),
            (Box::new(CMP::new(REG3, REG0)), "CMP R3, R0"),
            (Box::new(CMP::new(REG3, REG1)), "CMP R3, R1"),
            (Box::new(CMP::new(REG3, REG2)), "CMP R3, R2"),
            (Box::new(CMP::new(REG3, REG3)), "CMP R3, R3"),
        ];

        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }
    }

    #[test]
    fn test_instruction_two_reg() {
        let instructions: Vec<(Box<dyn Instruction>, u16)> = vec![
            // LOAD
            (Box::new(LOAD::new(REG0, REG0)), 0x0000),
            (Box::new(LOAD::new(REG0, REG1)), 0x0001),
            (Box::new(LOAD::new(REG0, REG2)), 0x0002),
            (Box::new(LOAD::new(REG0, REG3)), 0x0003),
            (Box::new(LOAD::new(REG1, REG0)), 0x0004),
            (Box::new(LOAD::new(REG1, REG1)), 0x0005),
            (Box::new(LOAD::new(REG1, REG2)), 0x0006),
            (Box::new(LOAD::new(REG1, REG3)), 0x0007),
            (Box::new(LOAD::new(REG2, REG0)), 0x0008),
            (Box::new(LOAD::new(REG2, REG1)), 0x0009),
            (Box::new(LOAD::new(REG2, REG2)), 0x000A),
            (Box::new(LOAD::new(REG2, REG3)), 0x000B),
            (Box::new(LOAD::new(REG3, REG0)), 0x000C),
            (Box::new(LOAD::new(REG3, REG1)), 0x000D),
            (Box::new(LOAD::new(REG3, REG2)), 0x000E),
            (Box::new(LOAD::new(REG3, REG3)), 0x000F),
            // STORE
            (Box::new(STORE::new(REG0, REG0)), 0x0010),
            (Box::new(STORE::new(REG0, REG1)), 0x0011),
            (Box::new(STORE::new(REG0, REG2)), 0x0012),
            (Box::new(STORE::new(REG0, REG3)), 0x0013),
            (Box::new(STORE::new(REG1, REG0)), 0x0014),
            (Box::new(STORE::new(REG1, REG1)), 0x0015),
            (Box::new(STORE::new(REG1, REG2)), 0x0016),
            (Box::new(STORE::new(REG1, REG3)), 0x0017),
            (Box::new(STORE::new(REG2, REG0)), 0x0018),
            (Box::new(STORE::new(REG2, REG1)), 0x0019),
            (Box::new(STORE::new(REG2, REG2)), 0x001A),
            (Box::new(STORE::new(REG2, REG3)), 0x001B),
            (Box::new(STORE::new(REG3, REG0)), 0x001C),
            (Box::new(STORE::new(REG3, REG1)), 0x001D),
            (Box::new(STORE::new(REG3, REG2)), 0x001E),
            (Box::new(STORE::new(REG3, REG3)), 0x001F),
            // ADD
            (Box::new(ADD::new(REG0, REG0)), 0x0080),
            (Box::new(ADD::new(REG0, REG1)), 0x0081),
            (Box::new(ADD::new(REG0, REG2)), 0x0082),
            (Box::new(ADD::new(REG0, REG3)), 0x0083),
            (Box::new(ADD::new(REG1, REG0)), 0x0084),
            (Box::new(ADD::new(REG1, REG1)), 0x0085),
            (Box::new(ADD::new(REG1, REG2)), 0x0086),
            (Box::new(ADD::new(REG1, REG3)), 0x0087),
            (Box::new(ADD::new(REG2, REG0)), 0x0088),
            (Box::new(ADD::new(REG2, REG1)), 0x0089),
            (Box::new(ADD::new(REG2, REG2)), 0x008A),
            (Box::new(ADD::new(REG2, REG3)), 0x008B),
            (Box::new(ADD::new(REG3, REG0)), 0x008C),
            (Box::new(ADD::new(REG3, REG1)), 0x008D),
            (Box::new(ADD::new(REG3, REG2)), 0x008E),
            (Box::new(ADD::new(REG3, REG3)), 0x008F),
            // AND
            (Box::new(AND::new(REG0, REG0)), 0x00C0),
            (Box::new(AND::new(REG0, REG1)), 0x00C1),
            (Box::new(AND::new(REG0, REG2)), 0x00C2),
            (Box::new(AND::new(REG0, REG3)), 0x00C3),
            (Box::new(AND::new(REG1, REG0)), 0x00C4),
            (Box::new(AND::new(REG1, REG1)), 0x00C5),
            (Box::new(AND::new(REG1, REG2)), 0x00C6),
            (Box::new(AND::new(REG1, REG3)), 0x00C7),
            (Box::new(AND::new(REG2, REG0)), 0x00C8),
            (Box::new(AND::new(REG2, REG1)), 0x00C9),
            (Box::new(AND::new(REG2, REG2)), 0x00CA),
            (Box::new(AND::new(REG2, REG3)), 0x00CB),
            (Box::new(AND::new(REG3, REG0)), 0x00CC),
            (Box::new(AND::new(REG3, REG1)), 0x00CD),
            (Box::new(AND::new(REG3, REG2)), 0x00CE),
            (Box::new(AND::new(REG3, REG3)), 0x00CF),
            // OR
            (Box::new(OR::new(REG0, REG0)), 0x00D0),
            (Box::new(OR::new(REG0, REG1)), 0x00D1),
            (Box::new(OR::new(REG0, REG2)), 0x00D2),
            (Box::new(OR::new(REG0, REG3)), 0x00D3),
            (Box::new(OR::new(REG1, REG0)), 0x00D4),
            (Box::new(OR::new(REG1, REG1)), 0x00D5),
            (Box::new(OR::new(REG1, REG2)), 0x00D6),
            (Box::new(OR::new(REG1, REG3)), 0x00D7),
            (Box::new(OR::new(REG2, REG0)), 0x00D8),
            (Box::new(OR::new(REG2, REG1)), 0x00D9),
            (Box::new(OR::new(REG2, REG2)), 0x00DA),
            (Box::new(OR::new(REG2, REG3)), 0x00DB),
            (Box::new(OR::new(REG3, REG0)), 0x00DC),
            (Box::new(OR::new(REG3, REG1)), 0x00DD),
            (Box::new(OR::new(REG3, REG2)), 0x00DE),
            (Box::new(OR::new(REG3, REG3)), 0x00DF),
            // XOR
            (Box::new(XOR::new(REG0, REG0)), 0x00E0),
            (Box::new(XOR::new(REG0, REG1)), 0x00E1),
            (Box::new(XOR::new(REG0, REG2)), 0x00E2),
            (Box::new(XOR::new(REG0, REG3)), 0x00E3),
            (Box::new(XOR::new(REG1, REG0)), 0x00E4),
            (Box::new(XOR::new(REG1, REG1)), 0x00E5),
            (Box::new(XOR::new(REG1, REG2)), 0x00E6),
            (Box::new(XOR::new(REG1, REG3)), 0x00E7),
            (Box::new(XOR::new(REG2, REG0)), 0x00E8),
            (Box::new(XOR::new(REG2, REG1)), 0x00E9),
            (Box::new(XOR::new(REG2, REG2)), 0x00EA),
            (Box::new(XOR::new(REG2, REG3)), 0x00EB),
            (Box::new(XOR::new(REG3, REG0)), 0x00EC),
            (Box::new(XOR::new(REG3, REG1)), 0x00ED),
            (Box::new(XOR::new(REG3, REG2)), 0x00EE),
            (Box::new(XOR::new(REG3, REG3)), 0x00EF),
            // CMP
            (Box::new(CMP::new(REG0, REG0)), 0x00F0),
            (Box::new(CMP::new(REG0, REG1)), 0x00F1),
            (Box::new(CMP::new(REG0, REG2)), 0x00F2),
            (Box::new(CMP::new(REG0, REG3)), 0x00F3),
            (Box::new(CMP::new(REG1, REG0)), 0x00F4),
            (Box::new(CMP::new(REG1, REG1)), 0x00F5),
            (Box::new(CMP::new(REG1, REG2)), 0x00F6),
            (Box::new(CMP::new(REG1, REG3)), 0x00F7),
            (Box::new(CMP::new(REG2, REG0)), 0x00F8),
            (Box::new(CMP::new(REG2, REG1)), 0x00F9),
            (Box::new(CMP::new(REG2, REG2)), 0x00FA),
            (Box::new(CMP::new(REG2, REG3)), 0x00FB),
            (Box::new(CMP::new(REG3, REG0)), 0x00FC),
            (Box::new(CMP::new(REG3, REG1)), 0x00FD),
            (Box::new(CMP::new(REG3, REG2)), 0x00FE),
            (Box::new(CMP::new(REG3, REG3)), 0x00FF),
        ];

        for i in instructions {
            assert_eq!(i.0.emit(None, None).unwrap(), vec![i.1]);
        }
    }

    #[test]
    fn test_instruction_one_reg_string() {
        let instructions: Vec<(Box<dyn Instruction>, &str)> = vec![
            // JR
            (Box::new(JR::new(REG0)), "JR R0"),
            (Box::new(JR::new(REG1)), "JR R1"),
            (Box::new(JR::new(REG2)), "JR R2"),
            (Box::new(JR::new(REG3)), "JR R3"),
            // NOT
            (Box::new(NOT::new(REG0)), "NOT R0"),
            (Box::new(NOT::new(REG1)), "NOT R1"),
            (Box::new(NOT::new(REG2)), "NOT R2"),
            (Box::new(NOT::new(REG3)), "NOT R3"),
            // SHL
            (Box::new(SHL::new(REG0)), "SHL R0"),
            (Box::new(SHL::new(REG1)), "SHL R1"),
            (Box::new(SHL::new(REG2)), "SHL R2"),
            (Box::new(SHL::new(REG3)), "SHL R3"),
            // SHR
            (Box::new(SHR::new(REG0)), "SHR R0"),
            (Box::new(SHR::new(REG1)), "SHR R1"),
            (Box::new(SHR::new(REG2)), "SHR R2"),
            (Box::new(SHR::new(REG3)), "SHR R3"),
        ];

        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }
    }

    #[test]
    fn test_instruction_one_reg() {
        let instructions: Vec<(Box<dyn Instruction>, u16)> = vec![
            // JR
            (Box::new(JR::new(REG0)), 0x0030),
            (Box::new(JR::new(REG1)), 0x0031),
            (Box::new(JR::new(REG2)), 0x0032),
            (Box::new(JR::new(REG3)), 0x0033),
            // NOT
            (Box::new(NOT::new(REG0)), 0x00B0),
            (Box::new(NOT::new(REG1)), 0x00B5),
            (Box::new(NOT::new(REG2)), 0x00BA),
            (Box::new(NOT::new(REG3)), 0x00BF),
            // SHL
            (Box::new(SHL::new(REG0)), 0x0090),
            (Box::new(SHL::new(REG1)), 0x0095),
            (Box::new(SHL::new(REG2)), 0x009A),
            (Box::new(SHL::new(REG3)), 0x009F),
            // SHR
            (Box::new(SHR::new(REG0)), 0x00A0),
            (Box::new(SHR::new(REG1)), 0x00A5),
            (Box::new(SHR::new(REG2)), 0x00AA),
            (Box::new(SHR::new(REG3)), 0x00AF),
        ];

        for i in instructions {
            assert_eq!(i.0.emit(None, None).unwrap(), vec![i.1]);
        }
    }

    #[test]
    fn test_instruction_data_string() {
        let instructions: Vec<(DATA<Number>, &str)> = vec![
            (DATA::new(REG0, Number::new(0x0001)), "DATA R0, 0x0001"),
            (DATA::new(REG1, Number::new(0x0002)), "DATA R1, 0x0002"),
            (DATA::new(REG2, Number::new(0x0003)), "DATA R2, 0x0003"),
            (DATA::new(REG3, Number::new(0x0004)), "DATA R3, 0x0004"),
        ];
        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }

        let instructions: Vec<(DATA<Symbol>, &str)> = vec![
            (DATA::new(REG0, Symbol::new("aaa")), "DATA R0, %aaa"),
            (DATA::new(REG1, Symbol::new("bbb")), "DATA R1, %bbb"),
            (DATA::new(REG2, Symbol::new("ccc")), "DATA R2, %ccc"),
            (DATA::new(REG3, Symbol::new("ddd")), "DATA R3, %ddd"),
        ];

        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }
    }

    #[test]
    fn test_instruction_data() {
        let instructions: Vec<(DATA<Number>, Vec<u16>)> = vec![
            (DATA::new(REG0, Number::new(0x0001)), vec![0x0020, 0x0001]),
            (DATA::new(REG1, Number::new(0x0002)), vec![0x0021, 0x0002]),
            (DATA::new(REG2, Number::new(0x0003)), vec![0x0022, 0x0003]),
            (DATA::new(REG3, Number::new(0x0004)), vec![0x0023, 0x0004]),
        ];
        for i in instructions {
            assert_eq!(i.0.emit(None, None).unwrap(), i.1);
        }

        let instructions: Vec<(DATA<Symbol>, Vec<u16>)> = vec![
            (DATA::new(REG0, Symbol::new("aaa")), vec![0x0020, 0xA000]),
            (DATA::new(REG1, Symbol::new("bbb")), vec![0x0021, 0xB000]),
            (DATA::new(REG2, Symbol::new("ccc")), vec![0x0022, 0xC000]),
            (DATA::new(REG3, Symbol::new("ddd")), vec![0x0023, 0xD000]),
        ];

        let dummy_symbol_resolver: SymbolResolver = |s: &Symbol| -> Result<u16, Error> {
            match s.name.as_str() {
                "aaa" => Ok(0xA000),
                "bbb" => Ok(0xB000),
                "ccc" => Ok(0xC000),
                "ddd" => Ok(0xD000),
                _ => Err(Error::UnknownSymbol(s.name.clone())),
            }
        };

        for i in instructions {
            assert_eq!(i.0.emit(None, Some(dummy_symbol_resolver)).unwrap(), i.1);
        }
    }

    #[test]
    fn test_instruction_io_string() {
        let instructions: Vec<(Box<dyn Instruction>, &str)> = vec![
            // Data mode
            (Box::new(IN::new(DATA_MODE, REG0)), "IN Data, R0"),
            (Box::new(IN::new(DATA_MODE, REG1)), "IN Data, R1"),
            (Box::new(IN::new(DATA_MODE, REG2)), "IN Data, R2"),
            (Box::new(IN::new(DATA_MODE, REG3)), "IN Data, R3"),
            (Box::new(OUT::new(DATA_MODE, REG0)), "OUT Data, R0"),
            (Box::new(OUT::new(DATA_MODE, REG1)), "OUT Data, R1"),
            (Box::new(OUT::new(DATA_MODE, REG2)), "OUT Data, R2"),
            (Box::new(OUT::new(DATA_MODE, REG3)), "OUT Data, R3"),
            // Address mode
            (Box::new(IN::new(ADDRESS_MODE, REG0)), "IN Addr, R0"),
            (Box::new(IN::new(ADDRESS_MODE, REG1)), "IN Addr, R1"),
            (Box::new(IN::new(ADDRESS_MODE, REG2)), "IN Addr, R2"),
            (Box::new(IN::new(ADDRESS_MODE, REG3)), "IN Addr, R3"),
            (Box::new(OUT::new(ADDRESS_MODE, REG0)), "OUT Addr, R0"),
            (Box::new(OUT::new(ADDRESS_MODE, REG1)), "OUT Addr, R1"),
            (Box::new(OUT::new(ADDRESS_MODE, REG2)), "OUT Addr, R2"),
            (Box::new(OUT::new(ADDRESS_MODE, REG3)), "OUT Addr, R3"),
        ];

        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }
    }

    #[test]
    fn test_instruction_io() {
        let instructions: Vec<(Box<dyn Instruction>, u16)> = vec![
            // Data mode
            (Box::new(IN::new(DATA_MODE, REG0)), 0x0070),
            (Box::new(IN::new(DATA_MODE, REG1)), 0x0071),
            (Box::new(IN::new(DATA_MODE, REG2)), 0x0072),
            (Box::new(IN::new(DATA_MODE, REG3)), 0x0073),
            (Box::new(OUT::new(DATA_MODE, REG0)), 0x0078),
            (Box::new(OUT::new(DATA_MODE, REG1)), 0x0079),
            (Box::new(OUT::new(DATA_MODE, REG2)), 0x007A),
            (Box::new(OUT::new(DATA_MODE, REG3)), 0x007B),
            // Address mode
            (Box::new(IN::new(ADDRESS_MODE, REG0)), 0x0074),
            (Box::new(IN::new(ADDRESS_MODE, REG1)), 0x0075),
            (Box::new(IN::new(ADDRESS_MODE, REG2)), 0x0076),
            (Box::new(IN::new(ADDRESS_MODE, REG3)), 0x0077),
            (Box::new(OUT::new(ADDRESS_MODE, REG0)), 0x007C),
            (Box::new(OUT::new(ADDRESS_MODE, REG1)), 0x007D),
            (Box::new(OUT::new(ADDRESS_MODE, REG2)), 0x007E),
            (Box::new(OUT::new(ADDRESS_MODE, REG3)), 0x007F),
        ];

        for i in instructions {
            assert_eq!(i.0.emit(None, None).unwrap(), vec![i.1]);
        }
    }

    #[test]
    fn test_instruction_jmp_string() {
        let instructions: Vec<(JMP, &str)> = vec![
            (JMP::new(Label::new("foo")), "JMP foo"),
            (JMP::new(Label::new("bar")), "JMP bar"),
        ];

        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }
    }

    #[test]
    fn test_instruction_jmp() {
        let instructions: Vec<(JMP, Vec<u16>)> = vec![
            (JMP::new(Label::new("foo")), vec![0x0040, 0x0001]),
            (JMP::new(Label::new("bar")), vec![0x0040, 0x0002]),
        ];

        let dummy_label_resolver: LabelResolver = |l: &Label| -> Result<u16, Error> {
            match l.name.as_str() {
                "foo" => Ok(0x0001),
                "bar" => Ok(0x0002),
                _ => Err(Error::UnknownLabel(l.name.clone())),
            }
        };

        for i in instructions {
            assert_eq!(i.0.emit(Some(dummy_label_resolver), None).unwrap(), i.1);
        }
    }

    #[test]
    fn test_instruction_clf_string() {
        let instructions: Vec<(CLF, &str)> = vec![(CLF::new(), "CLF")];

        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }
    }

    #[test]
    fn test_instruction_clf() {
        let instructions: Vec<(CLF, u16)> = vec![(CLF::new(), 0x0060)];

        for i in instructions {
            assert_eq!(i.0.emit(None, None).unwrap(), vec![i.1]);
        }
    }

    #[test]
    fn test_instruction_call_string() {
        let instructions: Vec<(Box<dyn Instruction>, &str)> = vec![
            (Box::new(CALL::new(Label::new("foo"))), "CALL foo"),
            (Box::new(CALL::new(Label::new("bar"))), "CALL bar"),
        ];

        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }
    }

    #[test]
    fn test_instruction_call() {
        let instructions: Vec<(Box<dyn Instruction>, Vec<u16>)> = vec![
            (
                Box::new(CALL::new(Label::new("foo"))),
                vec![0x0023, 0x1234, 0x0040, 0x0001],
            ),
            (
                Box::new(CALL::new(Label::new("bar"))),
                vec![0x0023, 0x1234, 0x0040, 0x0002],
            ),
        ];

        let dummy_label_resolver: LabelResolver = |l: &Label| -> Result<u16, Error> {
            match l.name.as_str() {
                "foo" => Ok(0x0001),
                "bar" => Ok(0x0002),
                _ => Err(Error::UnknownLabel(l.name.clone())),
            }
        };
        let dummy_symbol_resolver: SymbolResolver = |l: &Symbol| -> Result<u16, Error> {
            match l.name.as_str() {
                "NEXTINSTRUCTION" => Ok(0x1234),
                _ => Err(Error::UnknownSymbol(l.name.clone())),
            }
        };

        for i in instructions {
            assert_eq!(
                i.0.emit(Some(dummy_label_resolver), Some(dummy_symbol_resolver))
                    .unwrap(),
                i.1
            );
        }
    }

    #[test]
    fn test_instruction_jmp_flag_string() {
        let instructions: Vec<(JMPF, &str)> = vec![
            (
                JMPF::new(vec!["Z".to_string()], Label::new("foo1")),
                "JMPZ foo1",
            ),
            (
                JMPF::new(vec!["E".to_string()], Label::new("foo2")),
                "JMPE foo2",
            ),
            (
                JMPF::new(vec!["E".to_string(), "Z".to_string()], Label::new("foo3")),
                "JMPEZ foo3",
            ),
            (
                JMPF::new(vec!["A".to_string()], Label::new("foo4")),
                "JMPA foo4",
            ),
            (
                JMPF::new(vec!["A".to_string(), "Z".to_string()], Label::new("foo5")),
                "JMPAZ foo5",
            ),
            (
                JMPF::new(vec!["A".to_string(), "E".to_string()], Label::new("foo6")),
                "JMPAE foo6",
            ),
            (
                JMPF::new(
                    vec!["A".to_string(), "E".to_string(), "Z".to_string()],
                    Label::new("foo7"),
                ),
                "JMPAEZ foo7",
            ),
            (
                JMPF::new(vec!["C".to_string()], Label::new("foo8")),
                "JMPC foo8",
            ),
            (
                JMPF::new(vec!["C".to_string(), "Z".to_string()], Label::new("foo9")),
                "JMPCZ foo9",
            ),
            (
                JMPF::new(vec!["C".to_string(), "E".to_string()], Label::new("fooA")),
                "JMPCE fooA",
            ),
            (
                JMPF::new(
                    vec!["C".to_string(), "E".to_string(), "Z".to_string()],
                    Label::new("fooB"),
                ),
                "JMPCEZ fooB",
            ),
            (
                JMPF::new(vec!["C".to_string(), "A".to_string()], Label::new("fooC")),
                "JMPCA fooC",
            ),
            (
                JMPF::new(
                    vec!["C".to_string(), "A".to_string(), "Z".to_string()],
                    Label::new("fooD"),
                ),
                "JMPCAZ fooD",
            ),
            (
                JMPF::new(
                    vec!["C".to_string(), "A".to_string(), "E".to_string()],
                    Label::new("fooE"),
                ),
                "JMPCAE fooE",
            ),
            (
                JMPF::new(
                    vec![
                        "C".to_string(),
                        "A".to_string(),
                        "E".to_string(),
                        "Z".to_string(),
                    ],
                    Label::new("fooF"),
                ),
                "JMPCAEZ fooF",
            ),
        ];

        for i in instructions {
            assert_eq!(i.0.string(), i.1);
        }
    }

    #[test]
    fn test_instruction_jmp_flag() {
        let instructions: Vec<(JMPF, Vec<u16>)> = vec![
            (
                JMPF::new(vec!["Z".to_string()], Label::new("foo1")),
                vec![0x0051, 0x0001],
            ),
            (
                JMPF::new(vec!["E".to_string()], Label::new("foo2")),
                vec![0x0052, 0x0002],
            ),
            (
                JMPF::new(vec!["E".to_string(), "Z".to_string()], Label::new("foo3")),
                vec![0x0053, 0x0003],
            ),
            (
                JMPF::new(vec!["A".to_string()], Label::new("foo4")),
                vec![0x0054, 0x0004],
            ),
            (
                JMPF::new(vec!["A".to_string(), "Z".to_string()], Label::new("foo5")),
                vec![0x0055, 0x0005],
            ),
            (
                JMPF::new(vec!["A".to_string(), "E".to_string()], Label::new("foo6")),
                vec![0x0056, 0x0006],
            ),
            (
                JMPF::new(
                    vec!["A".to_string(), "E".to_string(), "Z".to_string()],
                    Label::new("foo7"),
                ),
                vec![0x0057, 0x0007],
            ),
            (
                JMPF::new(vec!["C".to_string()], Label::new("foo8")),
                vec![0x0058, 0x0008],
            ),
            (
                JMPF::new(vec!["C".to_string(), "Z".to_string()], Label::new("foo9")),
                vec![0x0059, 0x0009],
            ),
            (
                JMPF::new(vec!["C".to_string(), "E".to_string()], Label::new("fooA")),
                vec![0x005A, 0x000A],
            ),
            (
                JMPF::new(
                    vec!["C".to_string(), "E".to_string(), "Z".to_string()],
                    Label::new("fooB"),
                ),
                vec![0x005B, 0x000B],
            ),
            (
                JMPF::new(vec!["C".to_string(), "A".to_string()], Label::new("fooC")),
                vec![0x005C, 0x000C],
            ),
            (
                JMPF::new(
                    vec!["C".to_string(), "A".to_string(), "Z".to_string()],
                    Label::new("fooD"),
                ),
                vec![0x005D, 0x000D],
            ),
            (
                JMPF::new(
                    vec!["C".to_string(), "A".to_string(), "E".to_string()],
                    Label::new("fooE"),
                ),
                vec![0x005E, 0x000E],
            ),
            (
                JMPF::new(
                    vec![
                        "C".to_string(),
                        "A".to_string(),
                        "E".to_string(),
                        "Z".to_string(),
                    ],
                    Label::new("fooF"),
                ),
                vec![0x005F, 0x000F],
            ),
        ];

        let dummy_label_resolver: LabelResolver = |l: &Label| -> Result<u16, Error> {
            match l.name.as_str() {
                "foo1" => Ok(0x0001),
                "foo2" => Ok(0x0002),
                "foo3" => Ok(0x0003),
                "foo4" => Ok(0x0004),
                "foo5" => Ok(0x0005),
                "foo6" => Ok(0x0006),
                "foo7" => Ok(0x0007),
                "foo8" => Ok(0x0008),
                "foo9" => Ok(0x0009),
                "fooA" => Ok(0x000A),
                "fooB" => Ok(0x000B),
                "fooC" => Ok(0x000C),
                "fooD" => Ok(0x000D),
                "fooE" => Ok(0x000E),
                "fooF" => Ok(0x000F),
                _ => Err(Error::UnknownLabel(l.name.clone())),
            }
        };

        for i in instructions {
            assert_eq!(i.0.emit(Some(dummy_label_resolver), None).unwrap(), i.1);
        }
    }
}
