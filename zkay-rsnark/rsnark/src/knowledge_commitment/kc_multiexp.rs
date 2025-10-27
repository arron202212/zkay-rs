// /** @file
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

//#ifndef KC_MULTIEXP_HPP_
// #define KC_MULTIEXP_HPP_

/*
  Split out from multiexp to prevent cyclical
  dependencies. I.e. previously multiexp dependend on
  knowledge_commitment, which dependend on sparse_vector, which
  dependend on multiexp (to do accumulate).

  Will probably go away in more general exp refactoring.
*/

 use ffec::algebra::scalar_multiplication::multiexp;

use crate::knowledge_commitment::knowledge_commitment;



// 
// knowledge_commitment<T1,T2> opt_window_wnaf_exp(base:&knowledge_commitment<T1,T2>
//                                                 scalar:&ffec::bigint<n> scalar_bits:usize);

// 
// knowledge_commitment<T1, T2> kc_multi_exp_with_mixed_addition(vec:&knowledge_commitment_vector<T1, T2>
//                                                                 min_idx:usize
//                                                                 max_idx:usize
//                                                                 Vec<FieldT>::const_iterator scalar_start,
//                                                                 Vec<FieldT>::const_iterator scalar_end,
//                                                                 chunks:usize);

// 
// knowledge_commitment_vector<T1, T2> kc_batch_exp(scalar_size:usize
//                                                  T1_window:usize
//                                                  T2_window:usize
//                                                  T1_table:&ffec::window_table<T1>
//                                                  T2_table:&ffec::window_table<T2>
//                                                  T1_coeff:&FieldT
//                                                  T2_coeff:&FieldT
//                                                  v:&Vec<FieldT>
//                                                  suggested_num_chunks:usize);



// use crate::knowledge_commitment::kc_multiexp;

//#endif // KC_MULTIEXP_HPP_


// /** @file
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

//#ifndef KC_MULTIEXP_TCC_
// #define KC_MULTIEXP_TCC_



// 
// knowledge_commitment<T1,T2> opt_window_wnaf_exp(base:&knowledge_commitment<T1,T2>
//                                                 scalar:&ffec::bigint<n> scalar_bits:usize)
// {
//     return knowledge_commitment<T1,T2>(opt_window_wnaf_exp(base.g, scalar, scalar_bits),
//                                        opt_window_wnaf_exp(base.h, scalar, scalar_bits));
// }

// 
// knowledge_commitment<T1, T2> kc_multi_exp_with_mixed_addition(vec:&knowledge_commitment_vector<T1, T2>
//                                                                 min_idx:usize
//                                                                 max_idx:usize
//                                                                 Vec<FieldT>::const_iterator scalar_start,
//                                                                 Vec<FieldT>::const_iterator scalar_end,
//                                                                 chunks:usize)
// {
//     ffec::enter_block("Process scalar vector");
//     auto index_it = std::lower_bound(vec.indices.begin(), vec.indices.end(), min_idx);
//     vec.indices.begin(:usize offset = index_it -);

//     auto value_it = vec.values.begin() + offset;

//     FieldT::zero(:FieldT zero =);
//     FieldT::one(:FieldT one =);

//     Vec<FieldT> p;
//     Vec<knowledge_commitment<T1, T2> > g;

//     knowledge_commitment<T1, T2> acc = knowledge_commitment<T1, T2>::zero();

//     usize num_skip = 0;
//     usize num_add = 0;
//     usize num_other = 0;

//     std::distance(scalar_start:usize scalar_length = scalar_end);

//     while (index_it != vec.indices.end() && *index_it < max_idx)
//     {
//         (*index_it:usize scalar_position =) - min_idx;
//         assert!(scalar_position < scalar_length);

//         scalar_position:FieldT scalar = *(scalar_start +);

//         if scalar == zero
//         {
//             // do nothing
//             num_skip+=1;
//         }
//         else if scalar == one
//         {
// // #ifdef USE_MIXED_ADDITION
//             acc.g = acc.g.mixed_add(value_it->g);
//             acc.h = acc.h.mixed_add(value_it->h);
// #else
//             acc.g = acc.g + value_it->g;
//             acc.h = acc.h + value_it->h;
// //#endif
//             num_add+=1;
//         }
//         else
//         {
//             p.push(scalar);
//             g.push(*value_it);
//             num_other+=1;
//         }

//         index_it+=1;
//         value_it+=1;
//     }

//     ffec::print_indent(); print!("* Elements of w skipped: {} {}\n", num_skip, 100.*num_skip/(num_skip+num_add+num_other));
//     ffec::print_indent(); print!("* Elements of w processed with special addition: {} {}\n", num_add, 100.*num_add/(num_skip+num_add+num_other));
//     ffec::print_indent(); print!("* Elements of w remaining: {} {}\n", num_other, 100.*num_other/(num_skip+num_add+num_other));
//     ffec::leave_block("Process scalar vector");

//     return acc + ffec::multi_exp<knowledge_commitment<T1, T2>, FieldT, Method>(g.begin(), g.end(), p.begin(), p.end(), chunks);
// }

// 
// knowledge_commitment_vector<T1, T2> kc_batch_exp_internal(scalar_size:usize
//                                                           T1_window:usize
//                                                           T2_window:usize
//                                                           T1_table:&ffec::window_table<T1>
//                                                           T2_table:&ffec::window_table<T2>
//                                                           T1_coeff:&FieldT
//                                                           T2_coeff:&FieldT
//                                                           v:&Vec<FieldT>
//                                                           start_pos:usize
//                                                           end_pos:usize
//                                                           expected_size:usize)
// {
//     knowledge_commitment_vector<T1, T2> res;

//     res.values.reserve(expected_size);
//     res.indices.reserve(expected_size);

//     for (usize pos = start_pos; pos != end_pos; ++pos)
//     {
//         if !v[pos].is_zero()
//         {
//             res.values.push(knowledge_commitment<T1, T2>(windowed_exp(scalar_size, T1_window, T1_table, T1_coeff * v[pos]),
//                                                                  windowed_exp(scalar_size, T2_window, T2_table, T2_coeff * v[pos])));
//             res.indices.push(pos);
//         }
//     }

//     return res;
// }

// 
// knowledge_commitment_vector<T1, T2> kc_batch_exp(scalar_size:usize
//                                                  T1_window:usize
//                                                  T2_window:usize
//                                                  T1_table:&ffec::window_table<T1>
//                                                  T2_table:&ffec::window_table<T2>
//                                                  T1_coeff:&FieldT
//                                                  T2_coeff:&FieldT
//                                                  v:&Vec<FieldT>
//                                                  suggested_num_chunks:usize)
// {
//     knowledge_commitment_vector<T1, T2> res;
//     res.domain_size_ = v.len();

//     usize nonzero = 0;
//     for i in 0..v.len()
//     {
//         nonzero += if v[i].is_zero() {0} else{1};
//     }

//     std::min(nonzero:usize num_chunks = std::max((usize)1, suggested_num_chunks));

//     if !ffec::inhibit_profiling_info
//     {
//         ffec::print_indent(); print!("Non-zero coordinate count: {}/{} {}\n", nonzero, v.len(), 100.*nonzero/v.len());
//     }

//     Vec<knowledge_commitment_vector<T1, T2> > tmp(num_chunks);
//     Vec<usize> chunk_pos(num_chunks+1);

//     let chunk_size = nonzero / num_chunks;
//     1:usize last_chunk = nonzero - chunk_size * (num_chunks -);

//     chunk_pos[0] = 0;

//     usize cnt = 0;
//     usize chunkno = 1;

//     for i in 0..v.len()
//     {
//         cnt += if v[i].is_zero() {0} else{1};
//         if cnt == chunk_size && chunkno < num_chunks
//         {
//             chunk_pos[chunkno] = i;
//             cnt = 0;
//             chunkno+=1;
//         }
//     }

//     chunk_pos[num_chunks] = v.len();

// // #ifdef MULTICORE
// //#pragma omp parallel for
// //#endif
//     for i in 0..num_chunks
//     {
//         tmp[i] = kc_batch_exp_internal<T1, T2, FieldT>(scalar_size, T1_window, T2_window, T1_table, T2_table, T1_coeff, T2_coeff, v,
//                                                        chunk_pos[i], chunk_pos[i+1], if i == num_chunks - 1  {last_chunk} else {chunk_size});
// // #ifdef USE_MIXED_ADDITION
//         ffec::batch_to_special<knowledge_commitment<T1, T2>>(tmp[i].values);
// //#endif
//     }

//     if num_chunks == 1
//     {
//         tmp[0].domain_size_ = v.len();
//         return tmp[0];
//     }
//     else
//     {
//         for i in 0..num_chunks
//         {
//             res.values.insert(res.values.end(), tmp[i].values.begin(), tmp[i].values.end());
//             res.indices.insert(res.indices.end(), tmp[i].indices.begin(), tmp[i].indices.end());
//         }
//         return res;
//     }
// }



//#endif // KC_MULTIEXP_TCC_
