use std::any::Any;
use std::fmt::Display;

pub trait Marker: Any + Display {
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.name)
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:>04X}", self.value)
    }
}
