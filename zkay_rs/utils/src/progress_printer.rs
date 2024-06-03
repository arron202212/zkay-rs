// import contextlib
// from enum import Enum
// from typing import ContextManager

use zkay_config::{config::CFG, zk_print};
// @contextlib.contextmanager
pub fn print_step(name: &str) {
    zk_print!("{name}... ");
    // yield
    zk_print!("done");
}

pub enum TermColor {
    HEADER,
    OKBLUE,
    OKGREEN,
    WARNING,
    FAIL,
    ENDC,
    BOLD,
    UNDERLINE,
}
impl TermColor {
    pub fn value(&self) -> &'static str {
        match self {
            Self::HEADER => "\x1b[95m",
            Self::OKBLUE => "\x1b[94m",
            Self::OKGREEN => "\x1b[92m",
            Self::WARNING => "\x1b[93m",
            Self::FAIL => "\x1b[91m",
            Self::ENDC => "\x1b[0m",
            Self::BOLD => "\x1b[1m",
            Self::UNDERLINE => "\x1b[4m",
        }
    }
}

// @contextlib.contextmanager
pub fn colored_print(color: TermColor) {
    print!("{},''", color.value());
    // yield
    print!("{},''", TermColor::ENDC.value());
}

// def fail_print() -> ContextManager:
//     return colored_print(TermColor.FAIL)

pub fn warn_print() {
    colored_print(TermColor::WARNING);
}

// def success_print() -> ContextManager:
//     return colored_print(TermColor.OKGREEN)
