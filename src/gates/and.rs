use super::Wire;

#[derive(Debug)]
pub struct AND {
    output: Wire,
}

impl AND {
    pub fn new() -> Self {
        Self {
            output: Wire::new("Z".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(&mut self, a: bool, b: bool) -> bool {
        self.output.update(a & b);
        self.output.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_gate() {
        let mut and_gate = AND::new();
        assert_eq!(and_gate.update(false, false), false);
        assert_eq!(and_gate.update(false, true), false);
        assert_eq!(and_gate.update(true, false), false);
        assert_eq!(and_gate.update(true, true), true);
    }
}
