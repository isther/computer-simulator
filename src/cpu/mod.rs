use crate::{
    components::Decoder3x8,
    gates::{AND, NOT},
};

mod alu;
mod cpu;

use alu::ALU;
pub use cpu::CPU;

pub enum FlagState {
    Carry = 0,
    ALarger = 1,
    Equal = 2,
    Zero = 3,
}

impl From<FlagState> for i32 {
    fn from(state: FlagState) -> Self {
        match state {
            FlagState::Carry => 0,
            FlagState::ALarger => 1,
            FlagState::Equal => 2,
            FlagState::Zero => 3,
        }
    }
}

pub struct InstructionDecoder3x8 {
    pub decoder: Decoder3x8,
    pub selector_gates: [AND; 8],
    pub bit0_not_gate: NOT,
}

impl InstructionDecoder3x8 {
    fn new() -> Self {
        Self {
            decoder: Decoder3x8::new(),
            selector_gates: (0..8)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
            bit0_not_gate: NOT::new(),
        }
    }
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

// DATA
// put value in memory into register (2 byte instruction)
// ----------------------
// 0x0020 = DATA R0
// 0x0021 = DATA R1
// 0x0022 = DATA R2
// 0x0023 = DATA R3

// JR
// set instruction address register to value in register
// ----------------------
// 0x0030 = JR R0
// 0x0031 = JR R1
// 0x0032 = JR R2
// 0x0033 = JR R3

// JMP
// set instruction address register to next byte (2 byte instruction)
// ----------------------
// 0x0040 = JMP <value>

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

// CLF (CLEAR FLAGS)
// ----------------------
// 0x0060 CLF

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

// SHL
// ----------------------
// 0x0090 = SHL R0
// 0x0095 = SHL R1
// 0x009A = SHL R2
// 0x009F = SHL R3

// SHR
// ----------------------
// 0x00A0 = SHR R0
// 0x00A5 = SHR R1
// 0x00AA = SHR R2
// 0x00AF = SHR R3

// NOT
// ----------------------
// 0x00B0 = NOT R0
// 0x00B5 = NOT R1
// 0x00BA = NOT R2
// 0x00BF = NOT R3

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
