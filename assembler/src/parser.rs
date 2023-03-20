use crate::{error::Error, instruction::Instruction, Register};
use lazy_static::lazy_static;
use std::{collections::HashMap, io::Read};

lazy_static! {
    static ref REGISTERS: HashMap<String, Register> = {
        let mut map = HashMap::new();
        map.insert("0".to_string(), Register::REG0);
        map.insert("1".to_string(), Register::REG1);
        map.insert("2".to_string(), Register::REG2);
        map.insert("3".to_string(), Register::REG3);
        map
    };
}

struct Parser;

impl Parser {
    //TODO: Parser function
    fn parse(_input: Box<dyn Read>) -> Result<Vec<Box<dyn Instruction>>, Error> {
        Ok(vec![])
    }
}
