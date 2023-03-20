use std::{any::Any, fmt::Display, rc::Rc};

mod error;
mod instructions;
mod markers;

pub use error::Error;
pub use instructions::*;
pub use markers::{Label, Marker, Number, Symbol};

pub const CURRENTINSTRUCTION: &'static str = "CURRENTINSTRUCTION";
pub const NEXTINSTRUCTION: &'static str = "NEXTINSTRUCTION";

pub trait Resolver: ResolverClone {
    fn label_resolver(&self, _: &Label) -> Result<u16, Error> {
        Ok(0)
    }
    fn symbol_resolver(&self, _: &Symbol) -> Result<u16, Error> {
        Ok(0)
    }
}

pub trait ResolverClone {
    fn clone_box(&self) -> Box<dyn Resolver>;
}

impl<T> ResolverClone for T
where
    T: 'static + Resolver + Clone,
{
    fn clone_box(&self) -> Box<dyn Resolver> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Resolver> {
    fn clone(&self) -> Box<dyn Resolver> {
        self.clone_box()
    }
}

pub trait Instruction: Any + Display {
    fn emit(&self, resolver: Option<Rc<dyn Resolver>>) -> Result<Vec<u16>, Error>;
    fn size(&self) -> u16;
    fn as_any(&self) -> &dyn Any;
}

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

// Instructions - useful list data structure for convienience
pub type SafeInstruction = Rc<dyn Instruction>;
#[derive(Clone)]
pub struct Instructions {
    pub instructions: Vec<SafeInstruction>,
}

impl Instructions {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add(&mut self, ins: Vec<SafeInstruction>) {
        for i in ins.iter() {
            self.instructions.push(i.clone());
        }
    }

    pub fn add_blocks(&mut self, blocks: Vec<Vec<SafeInstruction>>) {
        for block in blocks {
            self.add(block);
        }
    }

    pub fn get(&self) -> Vec<SafeInstruction> {
        self.instructions.clone()
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for instruction in &self.instructions {
            match instruction.as_any().downcast_ref::<DEFLABEL>() {
                Some(label) => {
                    result += "\n";
                    result += label.to_string().as_str();
                    result += ":\n";
                }
                None => match instruction.as_any().downcast_ref::<DEFSYMBOL>() {
                    Some(symbol) => {
                        result += "\n";
                        result += symbol.to_string().as_str();
                        result += "\n";
                    }
                    None => {
                        result += "\t";
                        result += instruction.to_string().as_str();
                        result += "\n";
                    }
                },
            }
        }

        write!(f, "{}", result)
    }
}
