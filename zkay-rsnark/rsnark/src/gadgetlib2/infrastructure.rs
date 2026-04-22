//  Common functionality needed by many components.

use ffec::common::utils;


// /**
//  * The ErrorHandling pub struct containimplements the functionality of displaying the content of error
//  * messages (including content of call stack when error happened), and exiting the program.
//  */
pub struct ErrorHandling;

#[inline]
pub fn POW2(exponent: i32) -> i64 {
    1 << exponent
}

const MAX_FMT: usize = 256;


// /** Safely converts 64-bit types to 32-bit. */
pub fn safeConvert(num: i64) -> i64 {
    assert!(num <= i64::MAX && num >= i64::MIN);
    num
}

// /*
//     TODO add dumping of environment variables and run command to a log file and add log file path
//     to release mode error message. We don't want people running release version to get any internal
//     information (variable values, stack trace, etc.) but want to have every data possible to
//     reproduce assertion.
// */
impl ErrorHandling {
    pub fn fatalError(msg: &str) {
       
    }


    pub fn printStacktrace() {
       
    }
}

pub fn Log2(n: f64) -> f64 {
    n.ln() / 2.0f64.ln()
}

/// Returns an upper bound on ffec::log2(i). Namely, returns the number of binary digits needed to store
/// the value 'i'. When i == 0 returns 0.
pub fn Log2ceil(mut i: u64) -> u32 {
    let mut retval = if i == 0 { 0 } else { 1 };
    while i != 0 {
        retval += 1;
        i >>= 1;
    }
    retval
}

///Returns true iff x is a power of 2
pub fn IsPower2(x: i64) -> bool {
    (x > 0) && ((x & (x - 1)) == 0)
}
