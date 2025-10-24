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



// template<typename T1, typename T2, mp_size_t n>
// knowledge_commitment<T1,T2> opt_window_wnaf_exp(base:&knowledge_commitment<T1,T2>
//                                                 scalar:&ffec::bigint<n> scalar_bits:size_t);

// template<typename T1, typename T2, typename FieldT, ffec::multi_exp_method Method>
// knowledge_commitment<T1, T2> kc_multi_exp_with_mixed_addition(vec:&knowledge_commitment_vector<T1, T2>
//                                                                 min_idx:size_t
//                                                                 max_idx:size_t
//                                                                 typename std::vector<FieldT>::const_iterator scalar_start,
//                                                                 typename std::vector<FieldT>::const_iterator scalar_end,
//                                                                 chunks:size_t);

// template<typename T1, typename T2, typename FieldT>
// knowledge_commitment_vector<T1, T2> kc_batch_exp(scalar_size:size_t
//                                                  T1_window:size_t
//                                                  T2_window:size_t
//                                                  T1_table:&ffec::window_table<T1>
//                                                  T2_table:&ffec::window_table<T2>
//                                                  T1_coeff:&FieldT
//                                                  T2_coeff:&FieldT
//                                                  v:&std::vector<FieldT>
//                                                  suggested_num_chunks:size_t);



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



// template<typename T1, typename T2, mp_size_t n>
// knowledge_commitment<T1,T2> opt_window_wnaf_exp(base:&knowledge_commitment<T1,T2>
//                                                 scalar:&ffec::bigint<n> scalar_bits:size_t)
// {
//     return knowledge_commitment<T1,T2>(opt_window_wnaf_exp(base.g, scalar, scalar_bits),
//                                        opt_window_wnaf_exp(base.h, scalar, scalar_bits));
// }

// template<typename T1, typename T2, typename FieldT, ffec::multi_exp_method Method>
// knowledge_commitment<T1, T2> kc_multi_exp_with_mixed_addition(vec:&knowledge_commitment_vector<T1, T2>
//                                                                 min_idx:size_t
//                                                                 max_idx:size_t
//                                                                 typename std::vector<FieldT>::const_iterator scalar_start,
//                                                                 typename std::vector<FieldT>::const_iterator scalar_end,
//                                                                 chunks:size_t)
// {
//     ffec::enter_block("Process scalar vector");
//     auto index_it = std::lower_bound(vec.indices.begin(), vec.indices.end(), min_idx);
//     vec.indices.begin(:size_t offset = index_it -);

//     auto value_it = vec.values.begin() + offset;

//     FieldT::zero(:FieldT zero =);
//     FieldT::one(:FieldT one =);

//     std::vector<FieldT> p;
//     std::vector<knowledge_commitment<T1, T2> > g;

//     knowledge_commitment<T1, T2> acc = knowledge_commitment<T1, T2>::zero();

//     size_t num_skip = 0;
//     size_t num_add = 0;
//     size_t num_other = 0;

//     std::distance(scalar_start:size_t scalar_length = scalar_end);

//     while (index_it != vec.indices.end() && *index_it < max_idx)
//     {
//         (*index_it:size_t scalar_position =) - min_idx;
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

// template<typename T1, typename T2, typename FieldT>
// knowledge_commitment_vector<T1, T2> kc_batch_exp_internal(scalar_size:size_t
//                                                           T1_window:size_t
//                                                           T2_window:size_t
//                                                           T1_table:&ffec::window_table<T1>
//                                                           T2_table:&ffec::window_table<T2>
//                                                           T1_coeff:&FieldT
//                                                           T2_coeff:&FieldT
//                                                           v:&std::vector<FieldT>
//                                                           start_pos:size_t
//                                                           end_pos:size_t
//                                                           expected_size:size_t)
// {
//     knowledge_commitment_vector<T1, T2> res;

//     res.values.reserve(expected_size);
//     res.indices.reserve(expected_size);

//     for (size_t pos = start_pos; pos != end_pos; ++pos)
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

// template<typename T1, typename T2, typename FieldT>
// knowledge_commitment_vector<T1, T2> kc_batch_exp(scalar_size:size_t
//                                                  T1_window:size_t
//                                                  T2_window:size_t
//                                                  T1_table:&ffec::window_table<T1>
//                                                  T2_table:&ffec::window_table<T2>
//                                                  T1_coeff:&FieldT
//                                                  T2_coeff:&FieldT
//                                                  v:&std::vector<FieldT>
//                                                  suggested_num_chunks:size_t)
// {
//     knowledge_commitment_vector<T1, T2> res;
//     res.domain_size_ = v.len();

//     size_t nonzero = 0;
//     for i in 0..v.len()
//     {
//         nonzero += if v[i].is_zero() {0} else{1};
//     }

//     std::min(nonzero:size_t num_chunks = std::max((size_t)1, suggested_num_chunks));

//     if !ffec::inhibit_profiling_info
//     {
//         ffec::print_indent(); print!("Non-zero coordinate count: {}/{} {}\n", nonzero, v.len(), 100.*nonzero/v.len());
//     }

//     std::vector<knowledge_commitment_vector<T1, T2> > tmp(num_chunks);
//     std::vector<size_t> chunk_pos(num_chunks+1);

//     const size_t chunk_size = nonzero / num_chunks;
//     1:size_t last_chunk = nonzero - chunk_size * (num_chunks -);

//     chunk_pos[0] = 0;

//     size_t cnt = 0;
//     size_t chunkno = 1;

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
