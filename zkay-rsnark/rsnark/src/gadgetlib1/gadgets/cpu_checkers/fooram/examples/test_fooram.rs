/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <libff/common/utils.hpp>

use  <libsnark/common/default_types/r1cs_ppzkpcd_pp.hpp>
use  <libsnark/common/default_types/r1cs_ppzksnark_pp.hpp>
use  <libsnark/relations/ram_computations/rams/fooram/fooram_params.hpp>
use  <libsnark/zk_proof_systems/ppzksnark/ram_ppzksnark/examples/run_ram_ppzksnark.hpp>
use  <libsnark/zk_proof_systems/zksnark/ram_zksnark/examples/run_ram_zksnark.hpp>

namespace libsnark {

class default_fooram_zksnark_pp {
public:
    type default_r1cs_ppzkpcd_pp PCD_pp;
    type typename PCD_pp::scalar_field_A FieldT;
    type ram_fooram<FieldT> machine_pp;

    static void init_public_params() { PCD_pp::init_public_params(); }
};

class default_fooram_ppzksnark_pp {
public:
    type default_r1cs_ppzksnark_pp snark_pp;
    type libff::Fr<default_r1cs_ppzksnark_pp> FieldT;
    type ram_fooram<FieldT> machine_pp;

    static void init_public_params() { snark_pp::init_public_params(); }
};

} // libsnark

using namespace libsnark;

template<typename ppT>
void profile_ram_zksnark(const size_t w)
{
    type ram_zksnark_machine_pp<ppT> ramT;

    ram_example<ramT> example;
    example.ap = ram_architecture_params<ramT>(w);
    example.boot_trace_size_bound = 0;
    example.time_bound = 10;
    const bool test_serialization = true;
    const bool bit = run_ram_zksnark<ppT>(example, test_serialization);
    assert(bit);
}

template<typename ppT>
void profile_ram_ppzksnark(const size_t w)
{
    type ram_ppzksnark_machine_pp<ppT> ramT;

    ram_example<ramT> example;
    example.ap = ram_architecture_params<ramT>(w);
    example.boot_trace_size_bound = 0;
    example.time_bound = 100;
    const bool test_serialization = true;
    const bool bit = run_ram_ppzksnark<ppT>(example, test_serialization);
    assert(bit);
}

int main(int argc, const char* argv[])
{
    libff::UNUSED(argv);
    libff::start_profiling();
    default_fooram_ppzksnark_pp::init_public_params();
    default_fooram_zksnark_pp::init_public_params();

    if (argc == 1)
    {
        profile_ram_zksnark<default_fooram_zksnark_pp>(32);
    }
    else
    {
        profile_ram_ppzksnark<default_fooram_ppzksnark_pp>(8);
    }
}
