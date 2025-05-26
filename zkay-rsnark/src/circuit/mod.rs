pub mod auxiliary;
pub mod config;
pub mod eval;
pub mod operations;
pub mod structure;
pub mod tests;
pub trait StructNameConfig {
    fn name(&self) -> String {
        String::new()
    }
}

pub trait OpCodeConfig {
    fn op_code(&self) -> String {
        String::new()
    }
}

pub trait InstanceOf: StructNameConfig {
    fn instance_of(&self, name: &str) -> bool {
        self.name() == name
    }
}
