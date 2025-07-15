pub mod auxiliary;
pub mod config;
pub mod eval;
pub mod operations;
pub mod structure;
pub mod tests;
use crate::circuit::structure::wire_type::WireType;
use enum_dispatch::enum_dispatch;
#[enum_dispatch]
pub trait StructNameConfig {
    fn name(&self) -> String {
        String::new()
    }
}
#[enum_dispatch]
pub trait OpCodeConfig {
    fn op_code(&self) -> String {
        String::new()
    }
}
#[enum_dispatch]
pub trait InstanceOf: StructNameConfig {
    fn instance_of(&self, name: &str) -> bool {
        self.name() == name
    }
}

#[macro_export]
macro_rules! impl_struct_name_for {
    ($impl_type:ty) => {
        impl crate::circuit::StructNameConfig for $impl_type {
            fn name(&self) -> String {
                self.t.name()
            }
        }
    };
}
