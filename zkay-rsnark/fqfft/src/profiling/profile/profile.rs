//  Implementation of functions for profiler.

// // use ff_curves::algebra::curves::edwards::edwards_pp;
// use ffec::common::double;
// // //#include <omp.h>

// use crate::evaluation_domain::domains::arithmetic_sequence_domain;
// use crate::evaluation_domain::domains::basic_radix2_domain;
// use crate::evaluation_domain::domains::extended_radix2_domain;
// use crate::evaluation_domain::domains::geometric_sequence_domain;
// use crate::evaluation_domain::domains::step_radix2_domain;

// /* https://stackoverflow.com/questions/26237419/faster-than-rand */
// 5149:  seed :u32=,
// #[inline]
// pub fn  fastrand()->u32
// {
//   let seed = (214013 * seed + 2531011);
//   return (seed >> 16) & 0x7FFF;
// }
// trait  FieldTConfig{
//       0:add_cnt:i32 =,
//       0:sub_cnt:i32 =,
//       0:mul_cnt:i32 =,
//       0:inv_cnt:i32 =,
// }
// /*
//  * @params:
//  *   domain_sizes - size of the input vectors for specified domain
//  *   profiling_type - vector specifing logging of runtime, memory, and operators
//  *   path - vector specifying directory paths for logging our profiling_type
//  *   type - vector of all domain types
//  *   num_threads - number of threads
//  *   key - specify which domain to profile on
//  */
// // template <FieldT>
// pub fn  profile< FieldT: FieldTConfig+num_traits::Zero>(domain_sizes:String,
//              profiling_type:Vec<i32>,
//              path:Vec<String>,
//              types:Vec<String>,
//              num_threads:i32,
//              key:i32)
// {
//   /* Determine domain sizes and precompute domain vectors */
//   let  dom_sizes:Vec<_>=domain_sizes.split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();

//   let mut  domain=vec![vec![];dom_sizes.len()];
//   for i in 0..dom_sizes.len()
//   {
//     let mut temp=vec![FieldT::zero();dom_sizes[i]];
//     for j in 0..dom_sizes[i]
//       {temp[j] = FieldT::from(fastrand());}
//     domain[i] = temp;
//   }

// //   /* Runtime File */
//   let mut  runtime_file="";
// //   if profiling_type[0]
// //   {
// //     runtime_file.open(path[0] + types[key] + "-" + std::to_string(num_threads) + ".csv");
// //     runtime_file << "size, time (in sec) \n";
// //   }
// //   /* Memory File */
// //   std::ofstream memory_file;
// //   if profiling_type[1]
// //   {
// //     memory_file.open(path[1] + types[key] + "-" + std::to_string(num_threads) + ".csv");
// //     memory_file << "size, memory (in kilobytes) \n";
// //   }
// //   /* Operators File (only on single-thread case) */
//   let mut  operators_file;
// //   if profiling_type[2] && num_threads == 1
// //   {
// //     operators_file.open(path[2] + types[key] + ".csv");
// //     operators_file << "size, addition, subtraction, multiplication, inverse \n";
// //   }

//   print!("\n{}-{}\n", types[key as usize], num_threads);

//   /* Assess on varying domain sizes */
//   for s in 0..domain.len() {
//     /* Initialization */
//     let mut a=&domain[s];
//     let  n = a.len();

//     if num_threads == 1
//     {
//     //   FieldT::add_cnt = 0;
//     //   FieldT::sub_cnt = 0;
//     //   FieldT::mul_cnt = 0;
//     //   FieldT::inv_cnt = 0;
//     }

//     /* Start time */
//     let  start = omp_get_wtime();

//     /* Perform operation */
//     match key {
//      0=> {basic_radix2_domain::<FieldT>(n).FFT(a);}
//    1=>{ extended_radix2_domain::<FieldT>(n).FFT(a);}
//     2=> {step_radix2_domain::<FieldT>(n).FFT(a);}
//      3=> {geometric_sequence_domain::<FieldT>(n).FFT(a);}
//      4=> {arithmetic_sequence_domain::<FieldT>(n).FFT(a);}
//     _=>{
//         println!("key");
//     }
// }
//     /* End time */
//     let  runtime = (omp_get_wtime() - start) as f64;
//     if profiling_type[0] { println!("{runtime_file} {n} , {runtime} \n");}

//     /* Memory usage */
//     struct rusage;
//     let mut  r_usage=rusage;
//     // getrusage(RUSAGE_SELF, &r_usage);
//     // if profiling_type[1] {println!("{memory_file}  {n} , {r_usage.ru_maxrss} \n");}

//     /* Operator count */
//     if profiling_type[2] && num_threads == 1
//       {
//         write!(operators_file,  "{n}  ,
//                       {}  ,
//                       { } ,
//                       {}  ,
//                       {}  \n",FieldT::add_cnt,FieldT::sub_cnt,FieldT::mul_cnt,FieldT::inv_cnt);
//         }

//     print!("{}: {} seconds, {} kilobytes\n", n, runtime, r_usage.ru_maxrss);
//   }

//   /* Close files */
// //   if profiling_type[0]) runtime_file.close(;
// //   if profiling_type[1]) memory_file.close(;
// //   if profiling_type[2] && num_threads == 1) operators_file.close(;
// }
// use std::process;
// pub fn  main( argc:i32,  argv:[&str])->i32
// {
//   if argc < 6
//   {
//     print!("./perform {{key}} {{num_threads}} {{datetime}} {{profile_type}} {{domain_sizes}}\n");
//     print!("{{key}}: 0 - 5 \n{{num_threads}}: 1, 2, 4, 8 \n{{datetime}}: datetime\n");
//     print!("{{profile_type}}: 0, 1, 2 \n{{domain_sizes}}: '32768 65536 131072 262144'\n");
//     process::exit(0);
//   }

//   /* Parse input arguments */
//   let  key = argv[1].parse::<i32>().unwrap();
//   let num_threads = argv[2].parse::<i32>().unwrap();
//   let  datetime = argv[3];
//   let profile_type = argv[4];
//   let domain_sizes = argv[5];

//   /* Make log file directories */
//   let mut  path=vec![String::new();3];
//   path[0] = format!("libfqfft/profiling/logs/runtime/{datetime}/");
//   path[1] = format!("libfqfft/profiling/logs/memory/{datetime}/");
//   path[2] = format!("libfqfft/profiling/logs/operators/{datetime}/");

//   /* Determine profiling type */
//   let mut  m=0;
//   let mut  profiling_type =vec![false;3];
//   for m in profile_type.split_ascii_whitespace()
//     {profiling_type[m.parse::<usize>().unwrap() - 1] = true;}
//   if profiling_type[0]
//     {if system( ("mkdir -p " + path[0]) ) {return 0;}}
//   if profiling_type[1]
//     {if system( ("mkdir -p " + path[1]) ) {return 0;}}
//   if profiling_type[2]
//     {if system( ("mkdir -p " + path[2]) ) {return 0;}}

//   /* Domain types */
//   let mut  types=vec![];
//   types.push("basic-radix2-fft");
//   types.push("extended-radix2-fft");
//   types.push("step-radix2-fft");
//   types.push("geometric-fft");
//   // types.push("arithmetic-fft");

//   /* Profile on 1, 2, 4, or 8 threads */
//   let  max_threads = omp_get_max_threads();
//   if num_threads >= 1 && num_threads <= max_threads
//   {
//     /* Fix number of threads, no dynamic adjustment */
//     omp_set_dynamic(0);
//     omp_set_num_threads(num_threads);

// // #ifdef PROF_DOUBLE
// //     profile<Double>(domain_sizes, profiling_type, path, type, num_threads, key);
// // #else
//     edwards_pp::init_public_params();
//     profile::<Fr::<edwards_pp> >(domain_sizes, profiling_type, path, types, num_threads, key);
// 
//   }

//   return 0;
// }
