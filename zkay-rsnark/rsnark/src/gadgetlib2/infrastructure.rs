//  Common functionality needed by many components.

use ffec::common::utils;

//#ifndef  __infrastructure_HPP
// #define  __infrastructure_HPP

//#ifndef _MSC_VER // emulate the MSVC-specific sprintf_s using the standard snprintf
// #define sprintf_s snprintf //TODO: sprintf_s!=snprintf (http://blog.verg.es/2008/09/sprintfs-is-not-snprintf.html)
//#endif

// #ifdef _DEBUG // MSVC Debug build
// #define DEBUG // gcc Debug flag
//#endif

/********************************************************/
/**************** Class Writing Helpers *****************/
/********************************************************/
// A macro to disallow any non-defined constructors
// This should be used in the declarations for a class
// #define DISALLOW_CONSTRUCTION(TypeName) \
//   TypeName();

// A macro to disallow the copy constructor and operator= functions
// This should be used in the declarations for a class
// #define DISALLOW_COPY_AND_ASSIGN(TypeName) \
//   TypeName(const TypeName&);               \
//   pub fn  operator=(const TypeName&)

/********************************************************/
/*************** Debug String Formatting ****************/
/********************************************************/

// namespace gadgetlib2 {
// someday, if/when MSVC supports C++0x variadic templates, change FMT in release version to the
// following in order to increase efficiency:
// // #define GADGETLIB2_FMT(...) ""
// ::String GADGETLIB2_FMT(format:char*, ...);

/** Safely converts 64-bit types to 32-bit, or from unsigned to signed */
// long safeConvert(const int64_t num);

/********************************************************/
/******************* Error Handling *********************/
/********************************************************/
// declare a function as never returning, to quiet down "control reaches end of non-pub fn  function" warnings
// #if defined(_MSC_VER) // VisualC++
// // #define __noreturn _declspec(noreturn)
// #elif defined(__GNUC__)
// // #define __noreturn __attribute__((noreturn))
// #else
// #define __noreturn
//#endif

/**
 * The ErrorHandling pub struct containimplements the functionality of displaying the content of error
 * messages (including content of call stack when error happened), and exiting the program.
 */
pub struct ErrorHandling {
    //  pub fn   fatalError<T:Borrow<String>>(msg:T);
    //  pub fn   fatalErrors<T:AsRef<String>>(msg:T);
    //  pub fn  printStacktrace();
}

// #define GADGETLIB_FATAL(msg) do {  \
//     ::std::stringstream msgStream; \
//     msgStream << msg << " (In file " << __FILE__ << " line " << __LINE__ << ".)"; \
//     ErrorHandling::fatalError(msgStream.str()); \
// } while (0)

// TODO change GADGETLIB_ASSERT to not run in debug
// #define GADGETLIB_ASSERT(predicate, msg) if(!(bool(predicate))) GADGETLIB_FATAL(msg);

/********************************************************/
/****************** Basic Math **************************/
/********************************************************/

// double Log2(double n);

//Calculates  upper bound of Log2 of a number (number of bits needed to represent value)
// unsigned int Log2ceil(uint64_t i);

//Returns true iff the given number is a power of 2.
// bool IsPower2(const long x);

//Returns a^b when a can be a and b are INTEGERS.
//constexpr int64_t POW(int64_t base, int exponent) {
//	return (int64_t) powl((long double)base, (long double)exponent);
//}
//// #define POW(a,b) ((int64_t)(pow((float)(a),(int)(b))))

// Returns 2^exponent
// /*constexpr*/ inline int64_t POW2(int exponent) {
//     //assert!(exponent>=0);
//     return ((int64_t)1) << exponent;
// }

//Returns the ceiling of a when a is of type double.
// /*constexpr*/ inline int64_t CEIL(double a) {
//     return (int64_t)ceil(a);
// }
//// #define CEIL(a)  ((int64_t)ceil((double)(a)))

// using //ffec::UNUSED;
// } // namespace gadgetlib2

//#endif   // __infrastructure_HPP
/** @file
*****************************************************************************
Common functionality needed by many components.
*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

// // #ifdef __linux__
// use  <unistd.h>
// //#endif
// // #ifdef __GLIBC__
// use  <execinfo.h> // backtraces
// //#endif

// namespace gadgetlib2 {

/********************************************************/
/*************** Debug String Formatting ****************/
/********************************************************/

// #ifdef DEBUG
const MAX_FMT: usize = 256;
// ::String GADGETLIB2_FMT(format:char*, ...) {
//     char buf[MAX_FMT];
//     va_list args;
//     va_start(args, format);
// #if defined(_MSC_VER)
//     MAX_FMT:int strChk =  vsnprintf_s(buf,, MAX_FMT, format, args);
// #else
//     MAX_FMT:int strChk =  vsnprintf(buf,, format, args);
// //#endif
//     va_end(args);
//     GADGETLIB_ASSERT(strChk >= 0 && strChk < MAX_FMT, "String length larger than buffer. Shorten"
//                                         " string or increase buffer size defined in \"MAX_FMT\".");
//     return ::String(buf);
// }
// #else // not DEBUG
// ::String GADGETLIB2_FMT(format:char*, ...) {//ffec::UNUSED(format); return "";}
// //#endif

/** Safely converts 64-bit types to 32-bit. */
pub fn safeConvert(num: i64) -> i64 {
    assert!(num <= i64::MAX && num >= i64::MIN);
    num
}

/*****************************************************************************/
/***********************  Error Handling *************************************/
/*****************************************************************************/

/*
    TODO add dumping of environment variables and run command to a log file and add log file path
    to release mode error message. We don't want people running release version to get any internal
    information (variable values, stack trace, etc.) but want to have every data possible to
    reproduce assertion.
*/
impl ErrorHandling {
    pub fn fatalError(msg: &str) {
        // #   ifdef DEBUG
        //         ::std::cerr << "ERROR:  " << msg << ::std::endl << ::std::endl;
        //         printStacktrace();
        //         throw ::std::runtime_error(msg);
        // #   else // not DEBUG
        //         //ffec::UNUSED(msg);
        //         const ::String releaseMsg("Fatal error encountered. Run debug build for more"
        //                                                                   " information and stack trace.");
        //         ::std::cerr << "ERROR:  " << releaseMsg << ::std::endl << ::std::endl;
        //         throw ::std::runtime_error(releaseMsg);
        // #   endif
    }

    // pub fn  fatalErrors( msg) {
    //     fatalError(msg.str());
    // }

    pub fn printStacktrace() {
        // #ifdef __GLIBC__
        //     std::cerr << "Stack trace (pipe through c++filt to demangle identifiers):" << std::endl;
        //     let maxFrames= 100;
        //     pub fn * frames[maxFrames];
        //     // Fill array with pointers to stack frames
        //     int numFrames = backtrace(frames, maxFrames);
        //     // Decode frames and print them to stderr
        //     backtrace_symbols_fd(frames, numFrames, STDERR_FILENO);
        // #else
        //     //TODO make this available for non-glibc platforms (e.g. musl libc on Linux and Windows)
        //     std::cerr << "  (stack trace not available on this platform)" << std::endl;
        // //#endif // __GNUC__
    }
}
/*****************************************************************************/
/****************************  Basic Math  ***********************************/
/*****************************************************************************/

pub fn Log2(n: f64) -> f64 {
    log(n) / log(2.0)
}

/// Returns an upper bound on ffec::log2(i). Namely, returns the number of binary digits needed to store
/// the value 'i'. When i == 0 returns 0.
pub fn Log2ceil(mut i: u64) -> u32 {
    let mut retval = if i == 0 { 0 } else { 1 };
    while i != 0 {
        retval += 1;
        i >>= 1;
    }
    return retval;
}

///Returns true iff x is a power of 2
pub fn IsPower2(x: i64) -> bool {
    (x > 0) && ((x & (x - 1)) == 0)
}
