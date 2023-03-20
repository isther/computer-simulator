use crate::{
    error::Error,
    instruction::{Instruction, Resolver},
    markers::{Label, Marker, Symbol},
    CURRENTINSTRUCTION, DEFLABEL, DEFSYMBOL, NEXTINSTRUCTION,
};

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

struct Assembler {
    assembler_resolver: Rc<RefCell<AssemblerResolver>>,
    reserved_symbols: ReservedSymbols,
}

struct AssemblerResolver {
    labels: HashMap<String, u16>,
    symbols: HashMap<String, u16>,
}

impl Resolver for AssemblerResolver {
    fn label_resolver(&self, label: &Label) -> Result<u16, Error> {
        if let Some(v) = self.labels.get(&label.name) {
            Ok(*v)
        } else {
            Err(Error::UnknownLabel(label.name.clone()))
        }
    }

    fn symbol_resolver(&self, symbol: &Symbol) -> Result<u16, Error> {
        if let Some(v) = self.symbols.get(&symbol.name) {
            Ok(*v)
        } else {
            Err(Error::UnknownSymbol(symbol.name.clone()))
        }
    }
}

impl Assembler {
    fn process(
        &mut self,
        code_start_offset: u16,
        instructions: Option<Vec<Rc<RefCell<dyn Instruction>>>>,
    ) -> Result<Vec<u16>, Error> {
        self.assembler_resolver.borrow_mut().labels = HashMap::new();
        self.assembler_resolver.borrow_mut().symbols = HashMap::new();

        let mut position: u16 = 0;
        //calculate labels and symbols
        for instruction in instructions.as_ref().unwrap() {
            position += instruction.borrow().size();

            if instruction.type_id() == TypeId::of::<DEFLABEL>() {
                match instruction.borrow().as_any().downcast_ref::<DEFLABEL>() {
                    Some(lable) => {
                        if self
                            .assembler_resolver
                            .borrow()
                            .labels
                            .contains_key(&lable.name)
                        {
                            return Err(Error::LabelExist(lable.name.clone()));
                        }

                        self.assembler_resolver
                            .borrow_mut()
                            .labels
                            .insert(lable.name.clone(), position + code_start_offset);
                    }
                    None => return Err(Error::DowncastError),
                }
            }
            if instruction.type_id() == TypeId::of::<DEFSYMBOL>() {
                match instruction.borrow().as_any().downcast_ref::<DEFSYMBOL>() {
                    Some(symbol) => {
                        if self
                            .assembler_resolver
                            .borrow()
                            .symbols
                            .contains_key(&symbol.name)
                        {
                            return Err(Error::SymbolExist(symbol.name.clone()));
                        }

                        if self.reserved_symbols.is_reserved_symbol(&symbol.name) {
                            return Err(Error::SymbolReserved(symbol.name.clone()));
                        }

                        self.assembler_resolver
                            .borrow_mut()
                            .symbols
                            .insert(symbol.name.clone(), symbol.value);
                    }
                    None => return Err(Error::DowncastError),
                };
            }
        }

        let mut emitted = Vec::new();
        let mut i = 0;
        position = 0;
        for instruction in instructions.as_ref().unwrap().iter() {
            if i.type_id() == TypeId::of::<DEFLABEL>() || i.type_id() == TypeId::of::<DEFSYMBOL>() {
                continue;
            }

            self.assembler_resolver
                .borrow_mut()
                .symbols
                .insert(CURRENTINSTRUCTION.to_string(), position + code_start_offset);
            self.assembler_resolver.borrow_mut().symbols.insert(
                NEXTINSTRUCTION.to_string(),
                get_next_executable_instruction_loc(
                    *self
                        .assembler_resolver
                        .borrow()
                        .symbols
                        .get(CURRENTINSTRUCTION)
                        .unwrap(),
                    i,
                    instructions.as_ref().unwrap().clone(),
                ),
            );
            emitted.append(
                &mut instruction
                    .borrow()
                    .emit(Some(self.assembler_resolver.clone()))?,
            );

            position += instruction.borrow().size();
            i += 1;
        }

        Ok(vec![])
    }
}

struct ReservedSymbols(HashMap<String, Box<dyn Marker>>);

impl ReservedSymbols {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn is_reserved_symbol(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }
}

fn get_next_executable_instruction_loc(
    current_offset: u16,
    current_instr_index: usize,
    instructions: Vec<Rc<RefCell<dyn Instruction>>>,
) -> u16 {
    // if at the end then just return the location outside the loop
    if current_instr_index == instructions.len() {
        return current_offset + instructions[current_instr_index].borrow().size();
    }

    let mut next_instruction_pos = 0;
    for i in current_instr_index..instructions.len() {
        let instruction = &instructions[i];
        if instruction.type_id() == TypeId::of::<DEFLABEL>()
            || instruction.type_id() == TypeId::of::<DEFSYMBOL>()
        {
            continue;
        } else {
            if current_instr_index == i {
                next_instruction_pos += instruction.borrow().size();
            } else {
                break;
            }
        }
    }
    return current_offset + next_instruction_pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler() {}
}
