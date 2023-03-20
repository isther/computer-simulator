use instructions::{
    Error, Instructions, Label, Marker, Resolver, Symbol, CURRENTINSTRUCTION, DEFLABEL, DEFSYMBOL,
    NEXTINSTRUCTION,
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

#[derive(Clone)]
pub struct Assembler {
    reserved_symbols: ReservedSymbols,
    labels: HashMap<String, u16>,
    symbols: HashMap<String, u16>,
}

impl Resolver for Assembler {
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
    pub fn new() -> Self {
        Self {
            reserved_symbols: ReservedSymbols::new(),
            labels: HashMap::new(),
            symbols: HashMap::new(),
        }
    }

    pub fn process(
        &mut self,
        code_start_offset: u16,
        instructions: Option<Instructions>,
    ) -> Result<Vec<u16>, Error> {
        self.labels = HashMap::new();
        self.symbols = HashMap::new();

        let mut position: u16 = 0;

        //calculate labels and symbols
        for instruction in instructions.as_ref().unwrap().instructions.iter() {
            position += instruction.size();

            if instruction.as_any().type_id() == TypeId::of::<DEFLABEL>() {
                match instruction.as_any().downcast_ref::<DEFLABEL>() {
                    Some(lable) => {
                        if self.labels.contains_key(&lable.name) {
                            return Err(Error::LabelExist(lable.name.clone()));
                        }

                        self.labels
                            .insert(lable.name.clone(), position + code_start_offset);
                    }
                    None => return Err(Error::DowncastError),
                }
            }
            if instruction.as_any().type_id() == TypeId::of::<DEFSYMBOL>() {
                match instruction.as_any().downcast_ref::<DEFSYMBOL>() {
                    Some(symbol) => {
                        if self.symbols.contains_key(&symbol.name) {
                            return Err(Error::SymbolExist(symbol.name.clone()));
                        }

                        if self.reserved_symbols.is_reserved_symbol(&symbol.name) {
                            return Err(Error::SymbolReserved(symbol.name.clone()));
                        }

                        self.symbols.insert(symbol.name.clone(), symbol.value);
                    }
                    None => return Err(Error::DowncastError),
                };
            }
        }

        let mut emitted = Vec::new();
        let mut i = 0;
        position = 0;
        for instruction in instructions.clone().unwrap().instructions {
            if instruction.as_any().type_id() == TypeId::of::<DEFLABEL>()
                || instruction.as_any().type_id() == TypeId::of::<DEFSYMBOL>()
            {
                continue;
            }

            self.symbols
                .insert(CURRENTINSTRUCTION.to_string(), position + code_start_offset);
            self.symbols.insert(
                NEXTINSTRUCTION.to_string(),
                get_next_executable_instruction_loc(
                    *self.symbols.get(CURRENTINSTRUCTION).unwrap(),
                    i,
                    instructions.clone().unwrap(),
                ),
            );

            emitted.append(&mut instruction.emit(Some(Rc::new(self.clone())))?);

            position += instruction.size();
            i += 1;
        }

        Ok(emitted)
    }

    pub fn string(
        &mut self,
        code_start_offset: u16,
        instructions: Option<Instructions>,
    ) -> Result<String, Error> {
        self.labels = HashMap::new();
        self.symbols = HashMap::new();

        let mut position: u16 = 0;

        //calculate labels and symbols
        for instruction in instructions.as_ref().unwrap().instructions.iter() {
            position += instruction.size();

            if instruction.as_any().type_id() == TypeId::of::<DEFLABEL>() {
                match instruction.as_any().downcast_ref::<DEFLABEL>() {
                    Some(lable) => {
                        self.labels
                            .insert(lable.name.clone(), position + code_start_offset);
                    }
                    None => return Err(Error::DowncastError),
                }
            }
            if instruction.as_any().type_id() == TypeId::of::<DEFSYMBOL>() {
                match instruction.as_any().downcast_ref::<DEFSYMBOL>() {
                    Some(symbol) => {
                        self.symbols.insert(symbol.name.clone(), symbol.value);
                    }
                    None => return Err(Error::DowncastError),
                };
            }
        }

        let mut result = String::new();
        let mut i = 0;
        position = 0;
        for instruction in instructions.clone().unwrap().instructions.iter() {
            if instruction.as_any().type_id() == TypeId::of::<DEFLABEL>() {
                match instruction.as_any().downcast_ref::<DEFLABEL>() {
                    Some(symbol) => {
                        result += "\n";
                        result += symbol.to_string().as_str();
                        result += ":";
                    }
                    None => return Err(Error::DowncastError),
                };
            } else if instruction.as_any().type_id() == TypeId::of::<DEFSYMBOL>() {
                match instruction.as_any().downcast_ref::<DEFSYMBOL>() {
                    Some(symbol) => {
                        result += symbol.to_string().as_str();
                    }
                    None => return Err(Error::DowncastError),
                };
            } else {
                self.symbols
                    .insert(CURRENTINSTRUCTION.to_string(), position + code_start_offset);
                self.symbols.insert(
                    NEXTINSTRUCTION.to_string(),
                    get_next_executable_instruction_loc(
                        *self.symbols.get(CURRENTINSTRUCTION).unwrap(),
                        i,
                        instructions.clone().unwrap(),
                    ),
                );
                result += "\t";
                result += format!("0x{:>04X}:\t", position + code_start_offset)
                    .to_string()
                    .as_str();

                let emit = instruction.emit(Some(Rc::new(self.clone())))?;
                result += "{";
                for i in 0..instruction.size() {
                    result += format!("{}", emit[i as usize]).to_string().as_str();
                    if i < instruction.size() - 1 {
                        result += " ";
                    }
                }
                result += "}\t";
                // TODO: Output format
                match instruction.size() {
                    4 => result += "\t",
                    _ => result += "\t".repeat(3).as_str(),
                }
                result += instruction.to_string().as_str();
                position += instruction.size();
            }
            i += 1;
            result += "\n";
        }

        Ok(result)
    }
}

#[derive(Clone)]
struct ReservedSymbols(HashMap<String, Box<dyn Marker>>);

impl ReservedSymbols {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn is_reserved_symbol(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }
}

fn get_next_executable_instruction_loc(
    current_offset: u16,
    current_instr_index: usize,
    instructions: Instructions,
) -> u16 {
    // if at the end then just return the location outside the loop
    if current_instr_index == instructions.instructions.len() {
        return current_offset + instructions.instructions[current_instr_index].size();
    }

    let mut next_instruction_pos = 0;
    for i in current_instr_index..instructions.instructions.len() {
        let instruction = &instructions.instructions[i];
        if instruction.type_id() == TypeId::of::<DEFLABEL>()
            || instruction.type_id() == TypeId::of::<DEFSYMBOL>()
        {
            continue;
        } else {
            if current_instr_index == i {
                next_instruction_pos += instruction.size();
            } else {
                break;
            }
        }
    }
    return current_offset + next_instruction_pos;
}
