use crate::components::{
    ANDGate3, ANDer, Adder, Bit, Bus, BusOne, Comparator, Component, Decoder2x4, Decoder3x8,
    Enableable, Enabler, IOBus, IsZero, LeftShifter, NOTer, ORGate3, ORGate4, ORGate5, ORGate6,
    ORer, Register, RightShifter, Settable, Stepper, Updatable, XORer, BUS_WIDTH,
};

use crate::gates::{Wire, AND, NOT, OR};

mod alu;
mod cpu;

use alu::ALU;

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

struct InstructionDecoder3x8 {
    decoder: Decoder3x8,
    selector_gates: [AND; 8],
    bit0_not_gate: NOT,
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

pub enum Instruction {
    // LOADS
    // ----------------------
    // arg A = memory address to load from
    // arg B = register to store value in
    LD00 = 0x0000, // 0x0000 = LD R0, R0
    LD01 = 0x0001, // 0x0001 = LD R0, R1
    LD02 = 0x0002, // 0x0002 = LD R0, R2
    LD03 = 0x0003, // 0x0003 = LD R0, R3

    LD10 = 0x0004, // 0x0004 = LD R1, R0
    LD11 = 0x0005, // 0x0005 = LD R1, R1
    LD12 = 0x0006, // 0x0006 = LD R1, R2
    LD13 = 0x0007, // 0x0007 = LD R1, R3

    LD20 = 0x0008, // 0x0008 = LD R2, R0
    LD21 = 0x0009, // 0x0009 = LD R2, R1
    LD22 = 0x000A, // 0x000A = LD R2, R2
    LD23 = 0x000B, // 0x000B = LD R2, R3

    LD30 = 0x000C, // 0x000C = LD R3, R0
    LD31 = 0x000D, // 0x000D = LD R3, R1
    LD32 = 0x000E, // 0x000E = LD R3, R2
    LD33 = 0x000F, // 0x000F = LD R3, R3

    // STORES
    // ----------------------
    // arg A = memory address for value
    // arg B = value to store in memory
    ST00 = 0x0010, // 0x0010 = ST R0, R0
    ST01 = 0x0011, // 0x0011 = ST R0, R1
    ST02 = 0x0012, // 0x0012 = ST R0, R2
    ST03 = 0x0013, // 0x0013 = ST R0, R3

    ST10 = 0x0014, // 0x0014 = ST R1, R0
    ST11 = 0x0015, // 0x0015 = ST R1, R1
    ST12 = 0x0016, // 0x0016 = ST R1, R2
    ST13 = 0x0017, // 0x0017 = ST R1, R3

    ST20 = 0x0018, // 0x0018 = ST R2, R0
    ST21 = 0x0019, // 0x0019 = ST R2, R1
    ST22 = 0x001A, // 0x001A = ST R2, R2
    ST23 = 0x001B, // 0x001B = ST R2, R3

    ST30 = 0x001C, // 0x001C = ST R3, R0
    ST31 = 0x001D, // 0x001D = ST R3, R1
    ST32 = 0x001E, // 0x001E = ST R3, R2
    ST33 = 0x001F, // 0x001F = ST R3, R3

    // DATA
    // put value in memory into register (2 byte instruction)
    // ----------------------
    DATA0 = 0x0020, // 0x0020 = DATA R0
    DATA1 = 0x0021, // 0x0021 = DATA R1
    DATA2 = 0x0022, // 0x0022 = DATA R2
    DATA3 = 0x0023, // 0x0023 = DATA R3

    // JR
    // set instruction address register to value in register
    // ----------------------
    JR0 = 0x0030, // 0x0030 = JR R0
    JR1 = 0x0031, // 0x0031 = JR R1
    JR2 = 0x0032, // 0x0032 = JR R2
    JR3 = 0x0033, // 0x0033 = JR R3

    // JMP
    // set instruction address register to next byte (2 byte instruction)
    // ----------------------
    JMP = 0x0040, // 0x0040 = JMP <value>

    // JMP(CAEZ)
    // set instruction address register to next byte (2 byte instruction)
    // jump if <flag(s)> are true
    // ----------------------
    JMPZ = 0x0051,    // 0x0051 = JMPZ <value>
    JMPE = 0x0052,    // 0x0052 = JMPE <value>
    JMPEZ = 0x0053,   // 0x0053 = JMPEZ <value>
    JMPA = 0x0054,    // 0x0054 = JMPA <value>
    JMPAZ = 0x0055,   // 0x0055 = JMPAZ <value>
    JMPAE = 0x0056,   // 0x0056 = JMPAE <value>
    JMPAEZ = 0x0057,  // 0x0057 = JMPAEZ <value>
    JMPC = 0x0058,    // 0x0058 = JMPC <value>
    JMPCZ = 0x0059,   // 0x0059 = JMPCZ <value>
    JMPCE = 0x005A,   // 0x005A = JMPCE <value>
    JMPCEZ = 0x005B,  // 0x005B = JMPCEZ <value>
    JMPCA = 0x005C,   // 0x005C = JMPCA <value>
    JMPCAZ = 0x005D,  // 0x005D = JMPCAZ <value>
    JMPCAE = 0x005E,  // 0x005E = JMPCAE <value>
    JMPCAEZ = 0x005F, // 0x005F = JMPCAEZ <value>

    // CLF (CLEAR FLAGS)
    // ----------------------
    CLF = 0x0060, // 0x0060 CLF

    // IN
    // ----------------------
    INDATA0 = 0x0070, // 0x0070 = IN Data, R0
    INDATA1 = 0x0071, // 0x0071 = IN Data, R1
    INDATA2 = 0x0072, // 0x0072 = IN Data, R2
    INDATA3 = 0x0073, // 0x0073 = IN Data, R3
    INADDR0 = 0x0074, // 0x0074 = IN Addr, R0
    INADDR1 = 0x0075, // 0x0075 = IN Addr, R1
    INADDR2 = 0x0076, // 0x0076 = IN Addr, R2
    INADDR3 = 0x0077, // 0x0077 = IN Addr, R3

    // OUT
    // ----------------------
    OUTDATA0 = 0x0078, // 0x0078 = OUT Data, R0
    OUTDATA1 = 0x0079, // 0x0079 = OUT Data, R1
    OUTDATA2 = 0x007A, // 0x007A = OUT Data, R2
    OUTDATA3 = 0x007B, // 0x007B = OUT Data, R3
    OUTADDR0 = 0x007C, // 0x007C = OUT Addr, R0
    OUTADDR1 = 0x007D, // 0x007D = OUT Addr, R1
    OUTADDR2 = 0x007E, // 0x007E = OUT Addr, R2
    OUTADDR3 = 0x007F, // 0x007F = OUT Addr, R3

    // ADDS
    // ----------------------
    ADD00 = 0x0080, // 0x0080 = ADD R0, R0
    ADD01 = 0x0081, // 0x0081 = ADD R0, R1
    ADD02 = 0x0082, // 0x0082 = ADD R0, R2
    ADD03 = 0x0083, // 0x0083 = ADD R0, R3

    ADD10 = 0x0084, // 0x0084 = ADD R1, R0
    ADD11 = 0x0085, // 0x0085 = ADD R1, R1
    ADD12 = 0x0086, // 0x0086 = ADD R1, R2
    ADD13 = 0x0087, // 0x0087 = ADD R1, R3

    ADD20 = 0x0088, // 0x0088 = ADD R2, R0
    ADD21 = 0x0089, // 0x0089 = ADD R2, R1
    ADD22 = 0x008A, // 0x008A = ADD R2, R2
    ADD23 = 0x008B, // 0x008B = ADD Rr, R3

    ADD30 = 0x008C, // 0x008C = ADD R3, R0
    ADD31 = 0x008D, // 0x008D = ADD R3, R1
    ADD32 = 0x008E, // 0x008E = ADD R3, R2
    ADD33 = 0x008F, // 0x008F = ADD R3, R3

    // SHL
    // ----------------------
    SHL0 = 0x0090, // 0x0090 = SHL R0
    SHL1 = 0x0095, // 0x0095 = SHL R1
    SHL2 = 0x009A, // 0x009A = SHL R2
    SHL3 = 0x009F, // 0x009F = SHL R3

    // SHR
    // ----------------------
    SHR0 = 0x00A0, // 0x00A0 = SHR R0
    SHR1 = 0x00A5, // 0x00A5 = SHR R1
    SHR2 = 0x00AA, // 0x00AA = SHR R2
    SHR3 = 0x00AF, // 0x00AF = SHR R3

    // NOT
    // ----------------------
    NOT0 = 0x00B0, // 0x00B0 = NOT R0
    NOT1 = 0x00B5, // 0x00B5 = NOT R1
    NOT2 = 0x00BA, // 0x00BA = NOT R2
    NOT3 = 0x00BF, // 0x00BF = NOT R3

    // ANDS
    // ----------------------
    AND00 = 0x00C0, // 0x00C0 = AND R0, R0
    AND01 = 0x00C1, // 0x00C1 = AND R0, R1
    AND02 = 0x00C2, // 0x00C2 = AND R0, R2
    AND03 = 0x00C3, // 0x00C3 = AND R0, R3

    AND10 = 0x00C4, // 0x00C4 = AND R1, R0
    AND11 = 0x00C5, // 0x00C5 = AND R1, R1
    AND12 = 0x00C6, // 0x00C6 = AND R1, R2
    AND13 = 0x00C7, // 0x00C7 = AND R1, R3

    AND20 = 0x00C8, // 0x00C8 = AND R2, R0
    AND21 = 0x00C9, // 0x00C9 = AND R2, R1
    AND22 = 0x00CA, // 0x00CA = AND R2, R2
    AND23 = 0x00CB, // 0x00CB = AND R2, R3

    AND30 = 0x00CC, // 0x00CC = AND R3, R0
    AND31 = 0x00CD, // 0x00CD = AND R3, R1
    AND32 = 0x00CE, // 0x00CE = AND R3, R2
    AND33 = 0x00CF, // 0x00CF = AND R3, R3

    // ORS
    // ----------------------
    OR00 = 0x00D0, // 0x00D0 = OR R0, R0
    OR01 = 0x00D1, // 0x00D1 = OR R0, R1
    OR02 = 0x00D2, // 0x00D2 = OR R0, R2
    OR03 = 0x00D3, // 0x00D3 = OR R0, R3

    OR10 = 0x00D4, // 0x00D4 = OR R1, R0
    OR11 = 0x00D5, // 0x00D5 = OR R1, R1
    OR12 = 0x00D6, // 0x00D6 = OR R1, R2
    OR13 = 0x00D7, // 0x00D7 = OR R1, R3

    OR20 = 0x00D8, // 0x00D8 = OR R2, R0
    OR21 = 0x00D9, // 0x00D9 = OR R2, R1
    OR22 = 0x00DA, // 0x00DA = OR R2, R2
    OR23 = 0x00DB, // 0x00DB = OR R2, R3

    OR30 = 0x00DC, // 0x00DC = OR R3, R0
    OR31 = 0x00DD, // 0x00DD = OR R3, R1
    OR32 = 0x00DE, // 0x00DE = OR R3, R2
    OR33 = 0x00DF, // 0x00DF = OR R3, R3

    // XORS
    // ----------------------
    XOR00 = 0x00E0, // 0x00E0 = XOR R0, R0
    XOR01 = 0x00E1, // 0x00E1 = XOR R0, R1
    XOR02 = 0x00E2, // 0x00E2 = XOR R0, R2
    XOR03 = 0x00E3, // 0x00E3 = XOR R0, R3

    XOR10 = 0x00E4, // 0x00E4 = XOR R1, R0
    XOR11 = 0x00E5, // 0x00E5 = XOR R1, R1
    XOR12 = 0x00E6, // 0x00E6 = XOR R1, R2
    XOR13 = 0x00E7, // 0x00E7 = XOR R1, R3

    XOR20 = 0x00E8, // 0x00E8 = XOR R2, R0
    XOR21 = 0x00E9, // 0x00E9 = XOR R2, R1
    XOR22 = 0x00EA, // 0x00EA = XOR R2, R2
    XOR23 = 0x00EB, // 0x00EB = XOR R2, R3

    XOR30 = 0x00EC, // 0x00EC = XOR R3, R0
    XOR31 = 0x00ED, // 0x00ED = XOR R3, R1
    XOR32 = 0x00EE, // 0x00EE = XOR R3, R2
    XOR33 = 0x00EF, // 0x00EF = XOR R3, R3

    // CMP
    // ----------------------
    CMP00 = 0x00F0, // 0x00F0 = CMP R0, R0
    CMP01 = 0x00F1, // 0x00F1 = CMP R0, R1
    CMP02 = 0x00F2, // 0x00F2 = CMP R0, R2
    CMP03 = 0x00F3, // 0x00F3 = CMP R0, R3

    CMP10 = 0x00F4, // 0x00F4 = CMP R1, R0
    CMP11 = 0x00F5, // 0x00F5 = CMP R1, R1
    CMP12 = 0x00F6, // 0x00F6 = CMP R1, R2
    CMP13 = 0x00F7, // 0x00F7 = CMP R1, R3

    CMP20 = 0x00F8, // 0x00F8 = CMP R2, R0
    CMP21 = 0x00F9, // 0x00F9 = CMP R2, R1
    CMP22 = 0x00FA, // 0x00FA = CMP R2, R2
    CMP23 = 0x00FB, // 0x00FB = CMP R2, R3

    CMP30 = 0x00FC, // 0x00FC = CMP R3, R0
    CMP31 = 0x00FD, // 0x00FD = CMP R3, R1
    CMP32 = 0x00FE, // 0x00FE = CMP R3, R2
    CMP33 = 0x00FF, // 0x00FF = CMP R3, R3
}
