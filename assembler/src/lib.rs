use std::{any::Any, fmt::Display, rc::Rc};

mod asm;
mod error;
mod instruction;
mod markers;
mod parser;

use error::Error;
use instruction::Resolver;
pub use instruction::{
    Instruction, ADD, AND, CALL, CLF, CMP, DATA, IN, JMP, JMPF, JR, LOAD, NOT, OR, OUT, SHL, SHR,
    STORE, XOR,
};
pub use markers::{Label, Number, Symbol};

pub const CURRENTINSTRUCTION: &'static str = "CURRENTINSTRUCTION";
pub const NEXTINSTRUCTION: &'static str = "NEXTINSTRUCTION";

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Register {
    REG0 = 0,
    REG1 = 1,
    REG2 = 2,
    REG3 = 3,
}

impl From<Register> for u16 {
    fn from(r: Register) -> u16 {
        r as u16
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::REG0 => write!(f, "0"),
            Register::REG1 => write!(f, "1"),
            Register::REG2 => write!(f, "2"),
            Register::REG3 => write!(f, "3"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum IOMode {
    AddressMode,
    DataMode,
}

impl Display for IOMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOMode::AddressMode => write!(f, "Addr"),
            IOMode::DataMode => write!(f, "Data"),
        }
    }
}
// PLACEHOLDER INSTRUCTIONS - these are used by the assembler
pub struct DEFLABEL {
    pub name: String,
}

impl DEFLABEL {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Display for DEFLABEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.name)
    }
}

impl Instruction for DEFLABEL {
    fn emit(&self, _: Option<Rc<dyn Resolver>>) -> Result<Vec<u16>, Error> {
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

impl DEFSYMBOL {
    pub fn new(name: &str, value: u16) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

impl Display for DEFSYMBOL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{} = 0x{:X}", self.name, self.value)
    }
}

impl Instruction for DEFSYMBOL {
    fn emit(&self, _: Option<Rc<dyn Resolver>>) -> Result<Vec<u16>, Error> {
        Ok(vec![])
    }

    fn size(&self) -> u16 {
        0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
