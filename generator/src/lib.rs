use crate::error::Error;

use std::{
    any::{type_name, Any, TypeId},
    cell::RefCell,
    fmt::Display,
    rc::Rc,
};

mod common;
mod error;

pub use assembler::{
    IOMode, Instruction, Label, Number, Register, Symbol, ADD, AND, CALL, CLF, CMP, DATA, DEFLABEL,
    DEFSYMBOL, IN, JMP, JMPF, JR, LOAD, NOT, OUT, SHL, STORE, XOR,
};

pub use common::{
    call_routine, deselect_io, initialise_common_code, render_string, reset_linex,
    select_display_adapter, update_pen_position,
};

// Instructions - useful list data structure for convienience
pub type SafeInstruction = Rc<RefCell<dyn Instruction>>;
pub struct Instructions {
    instructions: Vec<SafeInstruction>,
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

        for instruction in self.instructions.iter() {
            // if instruction.type_id() == TypeId::of::<&Rc<RefCell<DEFLABEL>>>() {
            match instruction.borrow().as_any().downcast_ref::<DEFLABEL>() {
                Some(label) => {
                    result.push('\n');
                    result.push_str(label.to_string().as_str());
                    result.push('\n');
                    continue;
                }
                None => {}
            }
            // } else if instruction.type_id() == TypeId::of::<DEFSYMBOL>() {
            match instruction.borrow().as_any().downcast_ref::<DEFSYMBOL>() {
                Some(symbol) => {
                    result.push('\n');
                    result.push_str(symbol.to_string().as_str());
                    result.push('\n');
                    continue;
                }
                None => {}
            }
            // } else {
            result.push('\t');
            result.push_str(instruction.borrow().to_string().as_str());
            result.push('\n');
            // }
        }

        // write!(f, "{}", result)
        write!(f, "{}", "")
    }
}
