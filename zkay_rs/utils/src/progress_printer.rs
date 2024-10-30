// import contextlib
// from enum import Enum
// from typing import ContextManager

use zkay_config::{config::CFG, zk_print};

pub struct WithPrintStep;
impl WithPrintStep {
    pub fn new(name: &str) -> Self {
        zk_print!("{name}... ");
        Self
    }
}
impl Drop for WithPrintStep {
    fn drop(&mut self) {
        zk_print!("done");
    }
}

// @contextlib.contextmanager
pub fn print_step(name: &str) -> WithPrintStep {
    // yield
    WithPrintStep::new(name)
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

pub struct WithColoredPrint;
impl WithColoredPrint {
    pub fn new(color: TermColor) -> Self {
        print!("{},''", color.value());
        Self
    }
}
impl Drop for WithColoredPrint {
    fn drop(&mut self) {
        print!("{},''", TermColor::ENDC.value());
    }
}

// @contextlib.contextmanager
pub fn colored_print(color: TermColor) -> WithColoredPrint {
    // yield
    WithColoredPrint::new(color)
}

pub fn fail_print() -> WithColoredPrint {
    colored_print(TermColor::FAIL)
}

pub fn warn_print() -> WithColoredPrint {
    colored_print(TermColor::WARNING)
}

pub fn success_print() -> WithColoredPrint {
    colored_print(TermColor::OKGREEN)
}
