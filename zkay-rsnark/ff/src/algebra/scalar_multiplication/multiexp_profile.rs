
//  Implementation of interfaces for multi-exponentiation routines.



use crate::algebra::curves::bn128::bn128_pp;
use crate::algebra::scalar_multiplication::multiexp;
use crate::common::profiling;
use crate::common::rng;


type run_result_t<GroupT> = (i64, Vec<GroupT>) ;


type test_instances_t<T> = Vec<Vec<T> >;


 fn generate_group_elements<GroupT>(count:usize, size:usize)->test_instances_t<GroupT>
{
    // generating a random group element is expensive,
    // so for now we only generate a single one and repeat it
    let mut result=test_instances_t::<GroupT> ::new(count);

    for i in 0..count {
        let  x = GroupT::random_element();
        x.to_special(); // djb requires input to be in special form
        for j in 0..size {
            result[i].push_back(x);
        }
    }

     result
}


fn generate_scalars(count:usize, size:usize)->test_instances_t<FieldT> 
{
    // we use SHA512_rng because it is much faster than
    // FieldT::random_element()
    let mut result=test_instances_t::<GroupT> ::new(count);

    for i in 0..count {
        for j in 0..size {
            result[i].push_back(SHA512_rng::<FieldT>(i * size + j));
        }
    }

     result
}


 fn profile_multiexp(
    group_elements:test_instances_t<GroupT>,
    scalars:test_instances_t<FieldT>)->run_result_t<GroupT>
{
    let  start_time = get_nsec_time();

    let  answers:Vec<GroupT>=vec![];
    for i in 0..group_elements.len() {
        answers.push_back(multi_exp::<GroupT, FieldT, Method>(
            &group_elements[i],
            &scalars[i],
            1));
    }

    let  time_delta = get_nsec_time() - start_time;

     run_result_t::<GroupT>(time_delta, answers)
}


pub fn  print_performance_csv(
    expn_start:usize,
    expn_end_fast:usize,
    expn_end_naive:usize,
    compare_answers:bool)
{
    for expn in expn_start..=expn_end_fast {
        print!("{}", expn); 

        let  group_elements =
            generate_group_elements::<GroupT>(10, 1 << expn);
        let scalars =
            generate_scalars::<FieldT>(10, 1 << expn);

        let result_bos_coster =
            profile_multiexp::<GroupT, FieldT, multi_exp_method_bos_coster>(
                group_elements, scalars);
        print!("\t{}", result_bos_coster.first); 

        let result_djb =
            profile_multiexp::<GroupT, FieldT, multi_exp_method_BDLO12>(
                group_elements, scalars);
        print!("\t{}", result_djb.first); 

        if compare_answers && (result_bos_coster.1!= result_djb.1) {
            eprint!("Answers NOT MATCHING (bos coster != djb)\n");
        }

        if expn <= expn_end_naive {
            let result_naive =
                profile_multiexp::<GroupT, FieldT, multi_exp_method_naive>(
                    group_elements, scalars);
            print!("\t{}", result_naive.first); 

            if compare_answers && (result_bos_coster.1!= result_naive.1) {
                print!(stderr, "Answers NOT MATCHING (bos coster != naive)\n");
            }
        }

        print!("\n");
    }
}

fn main()
{
    print_compilation_info();

    print!("Profiling BN128_G1\n");
    bn128_pp::init_public_params();
    print_performance_csv::<G1<bn128_pp>, Fr<bn128_pp> >(2, 20, 14, true);

    print!("Profiling BN128_G2\n");
    print_performance_csv::<G2<bn128_pp>, Fr<bn128_pp> >(2, 20, 14, true);

    
}
use sha2::{Sha512, Digest};
use rand_core::{RngCore, SeedableRng, Error};

pub struct Sha512Rng {
    state: [u8; 64],
    buffer: [u8; 64],
    buffer_ptr: usize,
}

impl SeedableRng for Sha512Rng {
    type Seed = [u8; 64];
    fn from_seed(seed: Self::Seed) -> Self {
        Self { 
            state: seed,
            buffer: [0u8; 64],
            buffer_ptr: 64, // 初始設為已耗盡，強制第一次調用時生成數據
        }
    }
}

impl RngCore for Sha512Rng {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        self.fill_bytes(&mut bytes);
        u32::from_le_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        self.fill_bytes(&mut bytes);
        u64::from_le_bytes(bytes)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut filled = 0;
        while filled < dest.len() {
            // 如果 buffer 已用完，生成新的雜湊塊
            if self.buffer_ptr >= 64 {
                let mut hasher = Sha512::new();
                hasher.update(&self.state);
                let result = hasher.finalize();
                
                self.buffer.copy_from_slice(&result);
                self.state.copy_from_slice(&result); // 更新內部狀態以確保不可預測性
                self.buffer_ptr = 0;
            }

            // 確定本次要拷貝的長度
            let remaining_in_buffer = 64 - self.buffer_ptr;
            let to_copy = std::cmp::min(remaining_in_buffer, dest.len() - filled);
            
            dest[filled..filled + to_copy]
                .copy_from_slice(&self.buffer[self.buffer_ptr..self.buffer_ptr + to_copy]);

            self.buffer_ptr += to_copy;
            filled += to_copy;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
