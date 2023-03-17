use std::any::Any;

pub trait Marker: Any {
    fn string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone)]
pub struct Label {
    pub name: String,
}

impl Label {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Marker for Label {
    fn string(&self) -> String {
        format!("{}", self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Marker for Symbol {
    fn string(&self) -> String {
        format!("%{}", self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Number {
    pub value: u16,
}

impl Number {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}

impl Marker for Number {
    fn string(&self) -> String {
        format!("0x{:>04X}", self.value)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
