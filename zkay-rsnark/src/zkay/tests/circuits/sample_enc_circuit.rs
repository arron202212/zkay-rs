
use zkay::zkay_circuit_base;
use zkay::homomorphic_input;
use  zkay::zkay_type::zk_uint;

pub struct SampleEncCircuit extends ZkayCircuitBase {
    pub  SampleEncCircuit() {
        super("zk__Verify_Test_foo", 6, 8, 1, true);
        addCryptoBackend("elgamal", "elgamal", 508);
    }

      __zk__foo() {
        stepIn("_zk__foo");
        addS("zk__out0_cipher_R", 1, ZkUint(256));
        addIn("zk__in0_cipher_val", 4, ZkUint(256));
        addOut("zk__out0_cipher", 4, ZkUint(256));
        addOut("zk__out1_cipher", 4, ZkUint(256));

        //[ --- val + reveal<+>(3, owner) ---
        // zk__in0_cipher_val = val
        //[ --- 3 ---
        decl("tmp0_plain", cast(val(3, ZkUint(8)), ZkUint(32)));
        // zk__out0_cipher = enc(tmp0_plain, glob_key_Elgamal__owner)
        checkEnc("elgamal", "tmp0_plain", "glob_key_Elgamal__owner", "zk__out0_cipher_R", "zk__out0_cipher");
        //] --- 3 ---

        decl("tmp1_cipher", o_hom("elgamal", "glob_key_Elgamal__owner", HomomorphicInput.of(getCipher("zk__in0_cipher_val")), '+', HomomorphicInput.of(getCipher("zk__out0_cipher"))));
        checkEq("tmp1_cipher", "zk__out1_cipher");
        //] --- val + reveal<+>(3, owner) ---

        stepOut();
    }

    
      fn buildCircuit() {
        super.buildCircuit();
        addK("elgamal", "glob_key_Elgamal__owner", 2);

        __zk__foo();
    }

    pub    main(args:Vec<String>) {
        SampleEncCircuit circuit = SampleEncCircuit::new();
        circuit.run(args);
    }
}
