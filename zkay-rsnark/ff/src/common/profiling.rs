#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
//  Declaration of functions for profiling code blocks.

//  Reports time, operation counts, memory usage, and others.

const indentation: usize = 0;

pub fn get_nsec_time() -> i64 {
    // auto timepoint = std::chrono::high_resolution_clock::now();
    // return std::chrono::duration_cast<std::chrono::nanoseconds>(timepoint.time_since_epoch()).count();
    0
}

// //Return total CPU time consumsed by all threads of the process, in nanoseconds.
pub fn get_nsec_cpu_time() -> i64 {
    // #if _MSC_VER
    return 0;
}

pub fn start_profiling() {
    print!("Reset time counters for profiling\n");
}
const invocation_counts: &[(&str, usize)] = &[];
const op_data_points: &[(&str, &str)] = &[];

pub fn last_times(_: &str) -> f64 {
    0.0
}
pub fn clear_profiling_counters() {}

pub fn print_cumulative_time_entry(key: &str, factor: i64) {}

pub fn print_cumulative_times(factor: i64) {
    print!("Dumping times:\n");
}

pub fn print_cumulative_op_counts(only_fq: bool) {
    print!("Dumping operation counts:\n");
    for msg in invocation_counts {
        // print!("  %-45s: ", msg.first);
        let mut first = true;
        for data_point in op_data_points {
            if !first {
                print!(", ");
            }

            first = false;
        }
        print!("\n");
    }
}

pub fn print_op_profiling(msg: &str) {
    print!("\n");
    print_indent();

    print!("(opcounts) = (");
    let mut first = true;
    for p in op_data_points {
        if !first {
            print!(", ");
        }

        first = false;
    }
    print!(")");
}

pub fn print_times_from_last_and_start(now: i64, last: i64, cpu_now: i64, cpu_last: i64) {}

pub fn print_time(msg: &str) {}

pub fn print_header(msg: &str) {
    print!("\n================================================================================\n");
    print!("{}\n", msg);
    print!("================================================================================\n\n");
}

pub fn print_separator() {
    print!(
        "\n================================================================================\n\n"
    );
}

pub fn print_indent() {
    for i in 0..indentation {
        print!("  ");
    }
}

pub fn op_profiling_enter(msg: &str) {}

pub fn enter_block(msg: &str, indent: bool) {}
pub fn leave_block(msg: &str, indent: bool) {}

pub fn print_mem(s: &str) {}

pub fn print_compilation_info() {}
