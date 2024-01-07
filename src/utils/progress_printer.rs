// import contextlib
// from enum import Enum
// from typing import ContextManager

// from zkay.config import zk_print

// @contextlib.contextmanager
// def print_step(name):
//     zk_print(f'{name}... ', end='', flush=True)
//     yield
//     zk_print('done')

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
            Self::HEADER => "\033[95m",
            Self::OKBLUE => "\033[94m",
            Self::OKGREEN => "\033[92m",
            Self::WARNING => "\033[93m",
            Self::FAIL => "\033[91m",
            Self::ENDC => "\033[0m",
            Self::BOLD => "\033[1m",
            Self::UNDERLINE => "\033[4m",
            _ => "",
        }
    }
}

// @contextlib.contextmanager
pub fn colored_print(color: TermColor) {
    print!("{},{}", color.value(), "");
    // yield
    print!("{},{}", TermColor::ENDC.value(), "");
}

// def fail_print() -> ContextManager:
//     return colored_print(TermColor.FAIL)

pub fn warn_print() {
    colored_print(TermColor::WARNING);
}

// def success_print() -> ContextManager:
//     return colored_print(TermColor.OKGREEN)
