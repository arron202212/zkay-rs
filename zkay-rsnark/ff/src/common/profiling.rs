#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
//  *****************************************************************************
//  Declaration of functions for profiling code blocks.

//  Reports time, operation counts, memory usage, and others.

// //#ifndef PROFILING_HPP_
// // #define PROFILING_HPP_

// //#include <cstddef>
// //#include <map>
// //#include <string>
// //#include <vector>

// // namespace libff {

// pub fn  start_profiling();
// i64 get_nsec_time();
// pub fn  print_time(const char* msg);
// pub fn  print_header(const char* msg);
// pub fn  print_separator();

// pub fn  print_indent();

// extern bool inhibit_profiling_info;
// extern bool inhibit_profiling_counters;
// extern BTreeMap<String, std::usize> invocation_counts;
// extern BTreeMap<String, i64> last_times;
// extern BTreeMap<String, i64> cumulative_times;

// pub fn  clear_profiling_counters();

// pub fn  print_cumulative_time_entry(key:&String, const i64 factor=1);
// pub fn  print_cumulative_times(const i64 factor=1);
// pub fn  print_cumulative_op_counts(only_fq:bool=false);

// pub fn  enter_block(msg:&String, indent:bool=true);
// pub fn  leave_block(msg:&String, indent:bool=true);

// pub fn  print_mem(s:&String = "");
// pub fn  print_compilation_info();

// // } // namespace libff

// //#endif // PROFILING_HPP_

//  Implementation of functions for profiling code blocks.

// //  See profiling.hpp .

// //#include <cassert>
// //#include <chrono>
// //#include <cstdio>
// //#include <ctime>
// //#include <list>
// //#include <stdexcept>
// //#include <vector>

// use crate::common::default_types/ec_pp;
// use crate::common::profiling;
// use crate::common::utils;

// //#ifndef NO_PROCPS
// //#include <proc/readproc.h>
// //#endif
const indentation: usize = 0;

// pub struct Profiling;

// impl Profiling {

// using std::usize;

// i64 get_nsec_time()
// {
//     auto timepoint = std::chrono::high_resolution_clock::now();
//     return std::chrono::duration_cast<std::chrono::nanoseconds>(timepoint.time_since_epoch()).count();
// }

// /* Return total CPU time consumsed by all threads of the process, in nanoseconds. */
// i64 get_nsec_cpu_time()
// {
// #if _MSC_VER
// 	return 0;
// #else
//     ::timespec ts;
//     if  ::clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &ts) != 0  {
//         throw ::std::runtime_error("clock_gettime(CLOCK_PROCESS_CPUTIME_ID) failed");
//     }
//         // If we expected this to work, don't silently ignore failures, because that would hide the problem and incur an unnecessarily system-call overhead. So if we ever observe this exception, we should probably add a suitable // #ifdef .
//         //TODO: clock_gettime(CLOCK_PROCESS_CPUTIME_ID) is not supported by native Windows. What about Cygwin? Should we // #ifdef on CLOCK_PROCESS_CPUTIME_ID or on __linux__?
//     return ts.tv_sec * 1000000000LL + ts.tv_nsec;
// //#endif
// }

// i64 start_time, last_time;
// i64 start_cpu_time, last_cpu_time;

// pub fn  start_profiling()
// {
//     print!("Reset time counters for profiling\n");

//     last_time = start_time = get_nsec_time();
//     last_cpu_time = start_cpu_time = get_nsec_cpu_time();
// }

// BTreeMap<String, usize> invocation_counts;
// BTreeMap<String, i64> enter_times;
// BTreeMap<String, i64> last_times;
// BTreeMap<String, i64> cumulative_times;
// //TODO: Instead of analogous maps for time and cpu_time, use a single struct-valued map
// BTreeMap<String, i64> enter_cpu_times;
// BTreeMap<String, i64> last_cpu_times;
// BTreeMap<std::pair<String, String>, i64> op_counts;
// BTreeMap<std::pair<String, String>, i64> cumulative_op_counts; // ((msg, data_point), value)
//     // TODO: Convert op_counts and cumulative_op_counts from pair to structs

// Vec<String> block_names;

// std::list<std::pair<String, i64*> > op_data_points = {
// // #ifdef PROFILE_OP_COUNTS
//     std::make_pair("Fradd", &Fr<default_ec_pp>::add_cnt),
//     std::make_pair("Frsub", &Fr<default_ec_pp>::sub_cnt),
//     std::make_pair("Frmul", &Fr<default_ec_pp>::mul_cnt),
//     std::make_pair("Frinv", &Fr<default_ec_pp>::inv_cnt),
//     std::make_pair("Fqadd", &Fq<default_ec_pp>::add_cnt),
//     std::make_pair("Fqsub", &Fq<default_ec_pp>::sub_cnt),
//     std::make_pair("Fqmul", &Fq<default_ec_pp>::mul_cnt),
//     std::make_pair("Fqinv", &Fq<default_ec_pp>::inv_cnt),
//     std::make_pair("G1add", &G1<default_ec_pp>::add_cnt),
//     std::make_pair("G1dbl", &G1<default_ec_pp>::dbl_cnt),
//     std::make_pair("G2add", &G2<default_ec_pp>::add_cnt),
//     std::make_pair("G2dbl", &G2<default_ec_pp>::dbl_cnt)
// //#endif
// };

// bool inhibit_profiling_info = false;
// bool inhibit_profiling_counters = false;

// pub fn  clear_profiling_counters()
// {
//     invocation_counts.clear();
//     last_times.clear();
//     last_cpu_times.clear();
//     cumulative_times.clear();
// }

// pub fn  print_cumulative_time_entry(key:&String, const i64 factor)
// {
//     let total_ms= (cumulative_times.at(key) * 1e-6);
//     let cnt = invocation_counts.at(key);
//     let avg_ms= total_ms / cnt;
//     print!("   %-45s: %12.5fms = %lld * %0.5fms ({} invocations, %0.5fms = %lld * %0.5fms per invocation)\n", key, total_ms, factor, total_ms/ (double) factor, cnt, avg_ms, factor, avg_ms/ (double) factor);
// }

// pub fn  print_cumulative_times(const i64 factor)
// {
//     print!("Dumping times:\n");
//     for kv in &cumulative_times
//     {
//         print_cumulative_time_entry(kv.first, factor);
//     }
// }

// pub fn  print_cumulative_op_counts(only_fq:bool)
// {
// // #ifdef PROFILE_OP_COUNTS
//     print!("Dumping operation counts:\n");
//     for msg in &invocation_counts
//     {
//         print!("  %-45s: ", msg.first);
//         bool first = true;
//         for data_point in &op_data_points
//         {
//             if only_fq && data_point.first.compare(0, 2, "Fq") != 0
//             {
//                 continue;
//             }

//             if !first
//             {
//                 print!(", ");
//             }
//             print!("%-5s = %7.0f (%3zu)",
//                    data_point.first,
//                    1. * cumulative_op_counts[std::make_pair(msg.first, data_point.first)] / msg.second,
//                    msg.second);
//             first = false;
//         }
//         print!("\n");
//     }
// #else
//     UNUSED(only_fq);
// //#endif
// }

// pub fn  print_op_profiling(msg:&String)
// {
// // #ifdef PROFILE_OP_COUNTS
//     print!("\n");
//     print_indent();

//     print!("(opcounts) = (");
//     bool first = true;
//     for p in &op_data_points
//     {
//         if !first
//         {
//             print!(", ");
//         }

//         print!("{}=%lld", p.first, *(p.second)-op_counts[std::make_pair(msg, p.first)]);
//         first = false;
//     }
//     print!(")");
// #else
//     UNUSED(msg);
// //#endif
// }

// static pub fn  print_times_from_last_and_start(i64     now, i64     last,
//                                             i64 cpu_now, i64 cpu_last)
// {
//     i64 time_from_start = now - start_time;
//     i64 time_from_last = now - last;

//     i64 cpu_time_from_start = cpu_now - start_cpu_time;
//     i64 cpu_time_from_last = cpu_now - cpu_last;

//     if time_from_last != 0 {
//         double parallelism_from_last = 1.0 * (double) cpu_time_from_last / (double) time_from_last;
//         print!("[%0.4fs x%0.2f]", (double) time_from_last * 1e-9, parallelism_from_last);
//     } else {
//         print!("[             ]");
//     }
//     if time_from_start != 0 {
//         double parallelism_from_start = 1.0 * (double) cpu_time_from_start / (double) time_from_start;
//         print!("\t(%0.4fs x%0.2f from start)", (double) time_from_start * 1e-9, parallelism_from_start);
//     }
// }

// pub fn  print_time(const char* msg)
// {
//     if inhibit_profiling_info
//     {
//         return;
//     }

//     i64 now = get_nsec_time();
//     i64 cpu_now = get_nsec_cpu_time();

//     print!("%-35s\t", msg);
//     print_times_from_last_and_start(now, last_time, cpu_now, last_cpu_time);
// // #ifdef PROFILE_OP_COUNTS
//     print_op_profiling(msg);
// //#endif
//     print!("\n");

//     fflush(stdout);
//     last_time = now;
//     last_cpu_time = cpu_now;
// }

// pub fn  print_header(const char *msg)
// {
//     print!("\n================================================================================\n");
//     print!("{}\n", msg);
//     print!("================================================================================\n\n");
// }

// pub fn  print_separator()
// {
//     print!("\n================================================================================\n\n");
// }

pub fn print_indent() {
    for i in 0..indentation {
        print!("  ");
    }
}

// pub fn  op_profiling_enter(msg:&String)
// {
//     for p in &op_data_points
//     {
//         op_counts[std::make_pair(msg, p.first)] = *(p.second);
//     }
// }

pub fn enter_block(msg: &str, indent: bool) {
    //     if inhibit_profiling_counters
    //     {
    //         return;
    //     }

    //     block_names.emplace_back(msg);
    //     i64 t = get_nsec_time();
    //     enter_times[msg] = t;
    //     i64 cpu_t = get_nsec_cpu_time();
    //     enter_cpu_times[msg] = cpu_t;

    //     if inhibit_profiling_info
    //     {
    //         return;
    //     }

    // // #ifdef MULTICORE
    // #pragma omp critical
    // //#endif
    //     {
    //         op_profiling_enter(msg);

    //         print_indent();
    //         print!("(enter) %-35s\t", msg);
    //         print_times_from_last_and_start(t, t, cpu_t, cpu_t);
    //         print!("\n");
    //         fflush(stdout);

    //         if indent
    //         {
    //             ++indentation;
    //         }
    //     }
}
pub fn leave_block(msg: &str, indent: bool) {
    //     if inhibit_profiling_counters
    //     {
    //         return;
    //     }

    // //#ifndef MULTICORE
    //     assert!(*(--block_names.end()) == msg);
    // //#endif
    //     block_names.pop_back();

    //     ++invocation_counts[msg];

    //     i64 t = get_nsec_time();
    //     last_times[msg] = (t - enter_times[msg]);
    //     cumulative_times[msg] += (t - enter_times[msg]);

    //     i64 cpu_t = get_nsec_cpu_time();
    //     last_cpu_times[msg] = (cpu_t - enter_cpu_times[msg]);

    // // #ifdef PROFILE_OP_COUNTS
    //     for p in &op_data_points
    //     {
    //         cumulative_op_counts[std::make_pair(msg, p.first)] += *(p.second)-op_counts[std::make_pair(msg, p.first)];
    //     }
    // //#endif

    //     if inhibit_profiling_info
    //     {
    //         return;
    //     }

    // // #ifdef MULTICORE
    // #pragma omp critical
    // //#endif
    //     {
    //         if indent
    //         {
    //             --indentation;
    //         }

    //         print_indent();
    //         print!("(leave) %-35s\t", msg);
    //         print_times_from_last_and_start(t, enter_times[msg], cpu_t, enter_cpu_times[msg]);
    //         print_op_profiling(msg);
    //         print!("\n");
    //         fflush(stdout);
    //     }
}

// pub fn  print_mem(s:&String)
// {
// //#ifndef NO_PROCPS
//     struct proc_t usage;
//     look_up_our_self(&usage);
//     if s.empty()
//     {
//         print!("* Peak vsize (physical memory+swap) in mebibytes: %lu\n", usage.vsize >> 20);
//     }
//     else
//     {
//         print!("* Peak vsize (physical memory+swap) in mebibytes ({}): %lu\n", s, usage.vsize >> 20);
//     }
// #else
//     UNUSED(s);
//     print!("* Memory profiling not supported in NO_PROCPS mode\n");
// //#endif
// }

// pub fn  print_compilation_info()
// {
// // #ifdef __GNUC__
//     print!("g++ version: {}\n", __VERSION__);
//     print!("Compiled on {} {}\n", __DATE__, __TIME__);
// //#endif
// // #ifdef STATIC
//     print!("STATIC: yes\n");
// #else
//     print!("STATIC: no\n");
// //#endif
// // #ifdef MULTICORE
//     print!("MULTICORE: yes\n");
// #else
//     print!("MULTICORE: no\n");
// //#endif
// // #ifdef DEBUG
//     print!("DEBUG: yes\n");
// #else
//     print!("DEBUG: no\n");
// //#endif
// // #ifdef PROFILE_OP_COUNTS
//     print!("PROFILE_OP_COUNTS: yes\n");
// #else
//     print!("PROFILE_OP_COUNTS: no\n");
// //#endif
// // #ifdef _GLIBCXX_DEBUG
//     print!("_GLIBCXX_DEBUG: yes\n");
// #else
//     print!("_GLIBCXX_DEBUG: no\n");
// //#endif
// }

// } // namespace ffec
