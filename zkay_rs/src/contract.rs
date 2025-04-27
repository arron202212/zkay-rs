#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(non_camel_case_types)]

#[macro_export]
macro_rules! state_get {
    ( $self:ident.state().lock()[$exp:expr] ) => {
        $self.state().lock().get_item($exp).await
    };
}

#[macro_export]
macro_rules! state_set {
    ( $self:ident.state().lock()[$exp:expr]=$v:expr ) => {
        $self.state().lock().set_item($exp, $v)
    };
}

// //////////////////////////////////////////////////////////////////////////////////////
// // THIS CODE WAS GENERATED AUTOMATICALLY ////
// // Creation Time: 08:02:21 20-Nov-2024   ////
// //////////////////////////////////////////////////////////////////////////////////////
// from __future__ import annotations

// import os
// from enum import IntEnum
// from typing import Dict, List, Tuple, Optional, Union, Any
use alloy_primitives::Address;
use foundry_cli::opts::{EthereumOpts, RpcOpts};
use foundry_compilers::Project;
use foundry_config::Config;
use my_logging;
use proving_scheme::backends::groth16::ProvingSchemeGroth16;
use proving_scheme::proving_scheme::ProvingScheme;
use rccell::RcCell;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use zkay_config::config_user::UserConfig;
use zkay_config::{
    config::{CFG, zk_print_banner},
    with_context_block,
};
use zkay_transaction::blockchain::web3::Web3Tx;
use zkay_transaction::offchain::ApiWrapper;
use zkay_transaction::runtime::BlockchainClass;
use zkay_transaction::runtime::Runtime;
use zkay_transaction::runtime::{
    _blockchain_classes, _crypto_classes, _prover_classes, CryptoClass,
};
use zkay_transaction_crypto_params::params::CryptoParams;
// use yansi::paint::Paint;
// use zkay_transaction::blockchain::web3rs::Web3Blockchain;
use alloy_signer::Signer;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use zkay_transaction::int_casts::*;
use zkay_transaction::interface::{
    ZkayBlockchainInterface, ZkayCryptoInterface, ZkayHomomorphicCryptoInterface,
    ZkayKeystoreInterface, ZkayProverInterface,
};
use zkay_transaction::keystore::simple::SimpleKeystore;
use zkay_transaction::offchain::{
    ContractSimulator, ContractSimulatorConfig, ContractSimulatorRef, new_contract_simulator,
};
use zkay_transaction::prover::jsnark::JsnarkProver;
use zkay_transaction::solidity_math::*;
use zkay_transaction::{
    arc_cell_new,
    types::{
        ARcCell, AddressValue, CipherValue, DataType, PrivateKeyValue, PublicKeyValue,
        RandomnessValue, Value,
    },
};
use zkay_utils::timer::time_measure;
// me = None
// use ark_ff::{BigInteger, BigInteger256, Field, MontFp, PrimeField};
struct Survey<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    contract_simulator: ARcCell<ContractSimulator<C, P, B, K>>,
}

// class Survey(ContractSimulator){
#[derive(PartialEq, Clone)]
enum Choice {
    none = 0,
    a = 1,
    b = 2,
    c = 3,
}
use std::fmt;
impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.clone() as u8)
    }
}

impl<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> ContractSimulatorRef<C, P, B, K> for Survey<C, P, B, K>
{
    fn contract_simulator_ref(&self) -> ARcCell<ContractSimulator<C, P, B, K>> {
        self.contract_simulator.clone()
    }
}
impl<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> Survey<C, P, B, K>
{
    fn new(
        _project_dir: &str,
        _user_addr: &str,
        mut contract_simulator: ARcCell<ContractSimulator<C, P, B, K>>,
    ) -> Self {
        // super().__init__(project_dir, user_addr, "Survey");
        contract_simulator.lock().state.lock().decl(
            "organizer()(address)",
            convert_type,
            false,
            "",
        );
        contract_simulator.lock().state.lock().decl(
            "current_votes(address,uint)(uint)",
            convert_type,
            true,
            "ecdh-chaskey",
        );
        contract_simulator.lock().state.lock().decl(
            "a_count(uint)(uint)",
            convert_type,
            true,
            "elgamal",
        );
        contract_simulator.lock().state.lock().decl(
            "b_count(uint)(uint)",
            convert_type,
            true,
            "elgamal",
        );
        contract_simulator.lock().state.lock().decl(
            "c_count(uint)(uint)",
            convert_type,
            true,
            "elgamal",
        );
        contract_simulator
            .lock()
            .state
            .lock()
            .decl("min_votes()(uint)", convert_type, false, "");
        contract_simulator
            .lock()
            .state
            .lock()
            .decl("vote_count()(uint)", convert_type, false, "");
        contract_simulator.lock().state.lock().decl(
            "packed_results()(uint)",
            convert_type,
            false,
            "",
        );
        Self { contract_simulator }
    }

    //@staticmethod
    async fn connect<PS: ProvingScheme>(
        address: &Address,
        user: &str,
        project_dir: &str,
        mut contract_simulator: ARcCell<ContractSimulator<C, P, B, K>>,
        compile_zkay_file: fn(
            input_file_path: &str,
            output_dir: &str,
            import_keys: bool,
        ) -> anyhow::Result<()>,
        get_verification_contract_names: fn(code_or_ast: String) -> Vec<String>,
    ) -> Survey<C, P, B, K> {
        // = os.path.dirname(os.path.realpath(__file__))
        let mut c = Survey::new(project_dir, user, contract_simulator.clone());
        c.api()
            .lock()
            .connect::<PS>(address, compile_zkay_file, get_verification_contract_names)
            .await;
        contract_simulator.lock().initialize_keys_for(user).await;
        c
    }

    //@staticmethod
    async fn deploy(
        _min_votes: i32,
        user: &str,
        project_dir: &str,
        mut contract_simulator: ARcCell<ContractSimulator<C, P, B, K>>,
    ) -> Survey<C, P, B, K> {
        println!("=========deploy=======");
        //= os.path.dirname(os.path.realpath(__file__))
        contract_simulator.lock().initialize_keys_for(user).await;
        let mut c = Survey::new(project_dir, user, contract_simulator);
        c.constructor(_min_votes).await;
        c
    }
    async fn constructor(&self, _min_votes: i32) {
        println!("=========constructor=======");
        with_context_block!(var _fc= self._function_ctx(-1,0,"constructor").await=> {
            let (zk__is_ext,_fc)=_fc;
             with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                let (msg, block, _tx) = self.api().lock().get_special_variables();
                let _now = block.lock().as_ref().unwrap().timestamp;
                let actual_params = vec![_min_votes.to_string()];
                // BEGIN Simulate body
                 with_context_block!(var _sc= self._scope()=>{
                    assert!(_min_votes > 0,"require(_min_votes > 0) failed");
                    state_set!(self.state().lock()[&["organizer()(address)"]] = DataType::String(msg.lock().as_ref().unwrap().sender.clone()));
                    state_set!(self.state().lock()[&["min_votes()(uint)"]] =  DataType::Int(BigInt::from(_min_votes)));
                });
                // END Simulate body
                println!("====zk__is_ext==before==deploy========");
                if zk__is_ext{
                    println!("=====zk__is_ext=========deploy=======");
                    // Deploy contract
                    self.api().lock().deploy(actual_params, vec![false],None).await;
                }

            });
        });
    }
    // constructor._can_be_external = True

    async fn get_result_for(&self, option: &DataType) -> DataType {
        with_context_block!(var _fc=self._function_ctx(-1,0,"get_result_for").await=>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                         let (_msg, block, _tx) = self.api().lock().get_special_variables();
                        let _now = block.lock().as_ref().unwrap().timestamp;

                        let actual_params = vec![option.clone()];

                        // BEGIN Simulate body
                         with_context_block!(var _sc0=self._scope()=>{
                            assert!(self.is_result_published().await,"require(is_result_published()) failed");
                            self.locals().lock().decl("res", DataType::Int(BigInt::zero()));
                            if option != &DataType::Int(BigInt::from(Choice::none as u8)){
                                 with_context_block!(var _sc=self._scope()=>{
                                    let o=option.try_as_int_ref().unwrap().to_string().parse::<i32>().unwrap();
                                    self.locals().lock()["res"] = DataType::Int(state_get!(self.state().lock()[&["packed_results()(uint)"]]).try_as_int_ref().unwrap() >> (64i32 * (o - 1)));
                                });
                            }
                            if ! zk__is_ext{
                                return self.locals().lock()["res"].clone()
                            }
                        });
                        // END Simulate body

                        if zk__is_ext{
                            // Call pure/view function and return value
                            return self.api().lock().call("get_result_for(Survey.Choice option) public view returns (uint64)", actual_params, vec![(false, "None".to_string(), convert_type)]).await;
                        }
                    });
                });
        DataType::String(String::new())
    }
    // get_result_for._can_be_external = True

    async fn get_winning_choice(&self) -> DataType {
        with_context_block!(var _fc=self._function_ctx(-1,0,"get_winning_choice").await => {
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        let (_msg, block,_tx) = self.api().lock().get_special_variables();
                        let _now = block.lock().as_ref().unwrap().timestamp;

                        let actual_params = vec![];

                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            self.locals().lock().decl("c", DataType::Int(BigInt::from(Choice::none as u8)));
                            self.locals().lock().decl("votes", DataType::Int(BigInt::zero()));
                            self.locals().lock().decl("i",DataType::Int( BigInt::from(Choice::a as u8)));
                            while self.locals().lock()["i"] <=DataType::Int(BigInt::from(Choice::c as u8)){
                                // try{
                                     with_context_block!(var _sc1=self._scope()=>{
                                        self.locals().lock().decl("res", self.get_result_for(&self.locals().lock()["i"]).await);
                                        if self.locals().lock()["res"] > self.locals().lock()["votes"]{
                                             with_context_block!(var _sc2=self._scope()=>{
                                                self.locals().lock()["c"] = self.locals().lock()["i"].clone();
                                                self.locals().lock()["votes"] = self.locals().lock()["res"].clone();
                                            });
                                        }
                                    });
                                // finally{
                                    self.locals().lock()["i"] =DataType::Int(self.locals().lock()["i"].try_as_int_ref().unwrap().clone()+ 1);
                            }
                            if ! zk__is_ext{
                                return self.locals().lock()["c"].clone()
                            }
                        });
                        // END Simulate body

                        if zk__is_ext{
                            // Call pure/view function and return value
                            return self.api().lock().call("get_winning_choice() public view returns (Survey.Choice)", actual_params, vec![(false, "None".to_string(), convert_type)]).await
                        }
                    });
                });
        DataType::String(String::new())
    }
    // get_winning_choice._can_be_external = True

    async fn min_votes_reached(&self) -> bool {
        with_context_block!(var _fc=self._function_ctx(-1,0,"min_votes_reached").await => {
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        let (_msg, block,_tx) = self.api().lock().get_special_variables();
                        let _now = block.lock().as_ref().unwrap().timestamp;

                        let actual_params = vec![];

                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            if ! zk__is_ext{
                                return state_get!(self.state().lock()[&["vote_count()(uint)"]]) >= state_get!(self.state().lock()[&["min_votes()(uint)"]])
                            }
                        });
                        // END Simulate body

                        if zk__is_ext{
                            // Call pure/view function and return value
                            let ret= self.api().lock().call("min_votes_reached() public view returns (bool)", actual_params,vec![(false, "None".to_string(), convert_bool_type)]).await;
                            return ret.clone().try_as_bool().expect(&format!("==ret===={ret:?}"))
                        }
                    });
                });
        false
    }

    // min_votes_reached._can_be_external = True

    async fn is_result_published(&self) -> bool {
        with_context_block!(var _fc=self._function_ctx(-1,0,"is_result_published").await=>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", ! zk__is_ext,false)=>{
                        let (_msg, block,_tx) = self.api().lock().get_special_variables();
                        let _now = block.lock().as_ref().unwrap().timestamp;

                        let actual_params = vec![];

                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            if ! zk__is_ext{
                                // let pr=state_get!(self.state().lock()[&["packed_results()(uint)"]]) ;
                                // println!("=====packed_results=====is_result_published================={pr:?}");
                                return state_get!(self.state().lock()[&["packed_results()(uint)"]]) != DataType::String("0".to_owned())
                            }
                        });
                        // END Simulate body

                        if zk__is_ext{
                            // Call pure/view function and return value
                            return *self.api().lock().call("is_result_published() public view returns (bool)", actual_params, vec![(false, "None".to_string(), convert_bool_type)]).await.try_as_bool_ref().unwrap()
                        }
                    });
                });
        false
    }
    // is_result_published._can_be_external = True

    async fn vote(&self, votum: u8) {
        with_context_block!(var _fc=self._function_ctx(7,0,"vote").await =>{
                let (zk__is_ext,_fc)=_fc;
                             with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                                assert!( zk__is_ext);
                                let (msg, block,_tx) = self.api().lock().get_special_variables();
                                let _now = block.lock().as_ref().unwrap().timestamp;

                                let mut zk__priv =
                                    BTreeMap::from([("glob_sk_Ecdh_Chaskey__me", PublicKeyValue::data_type("ecdh-chaskey")),
        ("votum", DataType::Int(BigInt::zero()))]);


                                // Encrypt parameters
                                zk__priv.insert("votum",DataType::Int(BigInt::from(votum as u8)));
                                let mut d=self.api().lock().enc(zk__priv["votum"].try_as_int_ref().unwrap().to_string(),None, "ecdh-chaskey").await.0.contents.clone();
                                d.push(self.api().lock().get_my_pk("ecdh-chaskey")[0].to_string());
                                let votum=  DataType::CipherValue(Value::<String,CipherValue>::new(d,None,Some("ecdh-chaskey".to_owned())));
                                let mut actual_params = vec![votum.clone()];
                                let zk__out = vec![DataType::String("0".to_owned());25];
                                actual_params.push(DataType::List(zk__out.clone()));
                                let mut zk__in = vec![DataType::String("0".to_owned());28];
                                let (zk_in,zk_out);//:(Vec<String>,Vec<String>)
                                // BEGIN Simulate body
                                 with_context_block!(var _sc=self._scope()=> {
                                    zk__priv.insert("glob_sk_Ecdh_Chaskey__me",DataType::PrivateKeyValue(self.api().lock().get_my_sk("ecdh-chaskey")));
                                    assert!(zk__out.len() == 25,"require(zk__out.length == 25) failed");

                                    // Request static public keys
                                    // {
                                    let mut _tmp_key_Ecdh_Chaskey = PublicKeyValue::data_type("ecdh-chaskey");
                                    let mut _tmp_key_Elgamal  = PublicKeyValue::data_type("elgamal");
                                     _tmp_key_Ecdh_Chaskey = DataType::PublicKeyValue(self.api().lock().get_keystore("ecdh-chaskey").lock().getPk(&msg.lock().as_ref().unwrap().sender).await);
                                    zk__in[0] = DataType::String(_tmp_key_Ecdh_Chaskey.try_as_public_key_value_ref().unwrap()[0].clone());
                                    let organizer=state_get!(self.state().lock()[&["organizer()(address)"]]).try_as_string_ref().unwrap().clone();
                                    _tmp_key_Elgamal =  DataType::PublicKeyValue(self.api().lock().get_keystore("elgamal").lock().getPk(&organizer).await);
                                    zk__in[1..3].clone_from_slice(&_tmp_key_Elgamal.try_as_public_key_value_ref().unwrap()[..2].iter().map(|s|DataType::String(s.clone())).collect::<Vec<_>>()) ;
                                    // }

                                    // Backup private arguments for verification
                                    // {
                                    zk__in[3..5].clone_from_slice( &votum.try_as_cipher_value_ref().unwrap()[..2].iter().map(|s|DataType::String(s.clone())).collect::<Vec<_>>());

                                    // Copy from calldata to memory and set sender field
                                    self.locals().lock().decl("votum", DataType::CipherValue(Value::<String,CipherValue>::new(vec![votum.try_as_cipher_value_ref().unwrap()[0].clone(), votum.try_as_cipher_value_ref().unwrap()[1].clone(), zk__in[0].try_as_string_ref().unwrap().clone()],None,Some("ecdh-chaskey".to_owned()))));
                                    // }
                                        //println!("={s:?}=");
                                    let zkin:Vec<_>=zk__in.iter().map(|s|{s.try_as_string_ref().unwrap().clone()}).collect();
                                    let zkout:Vec<_>=zk__out.iter().map(|s|s.try_as_string_ref().unwrap().clone()).collect();
                                    let (zkin,zkout)=(arc_cell_new!(zkin),arc_cell_new!(zkout));
                                    let votum= self.locals().lock()["votum"].clone();
                                    // Call internal function
                                    let api=self.api().lock().clone();

                                    api.call_fct(2, async ||{self._zk__vote(votum.clone(), zkin.clone(), 5, zkout.clone(), 0).await;} ).await;
                                    (zk_in,zk_out)=(zkin.lock().clone(),zkout.lock().clone());
                                });
                                // END Simulate body

                                // Serialize circuit outputs and/or secret circuit inputs
                                self.api().lock().serialize_private_inputs(zk__priv.into_iter().map(|(k,v)|(k.to_owned(),v)).collect(), vec![0, 256]);

                                //Generate proof
                                let proof = self.api().lock().gen_proof("vote", zk_in, zk_out);
                                actual_params.push(DataType::List(proof.into_iter().map(|s|DataType::String(s)).collect()));
                                // let actual_params:Vec<_>=actual_params.into_iter().flatten().collect();
                                // Invoke public transaction
                                 let _=self.api().lock().transact("vote(uint[3] calldata votum, uint[] calldata zk__out, uint[8] calldata zk__proof) external", actual_params, vec![true, false, false],None);
                            });
                        });
    }
    // vote._can_be_external = True

    async fn _zk__vote(
        &self,
        votum: DataType,
        mut zk__ins: ARcCell<Vec<String>>,
        zk__in_start_idx: i32,
        mut zk__outs: ARcCell<Vec<String>>,
        zk__out_start_idx: i32,
    ) {
        let (mut zk__in, mut zk__out): (Vec<String>, Vec<String>) =
            (zk__ins.lock().clone(), zk__outs.lock().clone());
        println!("====_zk__vote==========================");
        let (zk__in_start_idx, zk__out_start_idx) =
            (zk__in_start_idx as usize, zk__out_start_idx as usize);
        println!("====_zk__vote==========================");
        with_context_block!(var _fc=self._function_ctx(5,0,"?").await =>{
                        let (zk__is_ext,_fc)=_fc;
                                    assert! (!zk__is_ext);
                println!("====_zk__vote==========================");
                                    let (msg, block,_tx) = self.api().lock().get_special_variables();
                                    let _now = block.lock().as_ref().unwrap().timestamp;
                println!("====_zk__vote==========================");
                                    let mut zk__priv =
                                        BTreeMap::from([("secret0_plain_votum", DataType::Int(BigInt::zero())), ("secret1_plain", DataType::Int(BigInt::zero())),
                                        ("zk__out1_cipher_R", RandomnessValue::data_type("elgamal")), ("zk__out3_cipher_R", RandomnessValue::data_type("elgamal")),
                                        ("zk__out5_cipher_R", RandomnessValue::data_type("elgamal"))]);
                                    let  mut zk__data;// = BTreeMap::new();
                println!("====_zk__vote==========================");
                                    // BEGIN Simulate body
                                     with_context_block!(var _sc=self._scope()=>{
                                        assert!((zk__out_start_idx + 25) <= zk__out.len() ,"require(zk__out_start_idx + 25 <= zk__out.length) failed");
                                        assert!((zk__in_start_idx + 23) <= zk__in.len(),"require(zk__in_start_idx + 23 <= zk__in.length) failed");
                println!("====_zk__vote==========================");
                                        zk__data = BTreeMap::from([
                                            ("zk__out0_plain", DataType::Bool(false)), ("zk__out1_cipher", CipherValue::data_type("elgamal")),
                                            ("zk__out2_cipher", CipherValue::data_type("elgamal")), ("zk__out3_cipher", CipherValue::data_type("elgamal")),
                                            ("zk__out4_cipher", CipherValue::data_type("elgamal")), ("zk__out5_cipher",CipherValue::data_type("elgamal")),
                                             ("zk__out6_cipher", CipherValue::data_type("elgamal")), ("zk__in0_cipher_votum", CipherValue::data_type("ecdh-chaskey")),
                                            ("zk__in1_key_sender", PublicKeyValue::data_type("ecdh-chaskey")), ("zk__in2_plain", DataType::Int(BigInt::zero())),
                                            ("zk__in3_cipher", CipherValue::data_type("ecdh-chaskey")), ("zk__in4_key_sender", PublicKeyValue::data_type("ecdh-chaskey")),
                                            ("zk__in5_plain", DataType::Int(BigInt::zero())), ("zk__in6_cipher_a_count", CipherValue::data_type("elgamal")),
                                            ("zk__in7_plain", DataType::Int(BigInt::zero())), ("zk__in8_cipher_b_count", CipherValue::data_type("elgamal")),
                                            ("zk__in9_plain", DataType::Int(BigInt::zero())), ("zk__in10_cipher_c_count", CipherValue::data_type("elgamal")),( "zk__in11_plain",DataType::Int(BigInt::zero())),
                                        ]);
                println!("====_zk__vote==========================");
                                        // require(reveal(votum != reveal(Choice::None.to_string(), me) && current_votes[me] == reveal(Choice::None.to_string(), me), all));
                                        // {
                                        zk__data.insert("zk__in0_cipher_votum",votum.clone());
                                        println!("====_zk__vote====************************************=====votum======{votum:?}===========");
                                        zk__priv.insert("secret0_plain_votum", self.api().lock().dec(zk__data["zk__in0_cipher_votum"].clone(), convert_type, "ecdh-chaskey").0);
                                        zk__data.insert("zk__in1_key_sender", DataType::PublicKeyValue(Value::<String,PublicKeyValue>::new(vec![zk__data["zk__in0_cipher_votum"].try_as_cipher_value_ref().unwrap()[2].clone()],None, Some("ecdh-chaskey".to_owned()))));
                                        zk__data.insert("zk__in2_plain", DataType::String("0".to_owned()));//DataType::Int(Choice::none as u128));
                                        let zk__in3_cipher=state_get!(self.state().lock()[&["current_votes(address,uint)(uint)", &msg.lock().as_ref().unwrap().sender]]).clone();
                                        println!("===zk__in3_cipher==========={zk__in3_cipher:?}============");
                                        zk__data.insert("zk__in3_cipher",zk__in3_cipher );
                                        zk__priv.insert("secret1_plain",self.api().lock().dec(zk__data["zk__in3_cipher"].clone(), convert_type, "ecdh-chaskey").0);
                                        zk__data.insert("zk__in4_key_sender",DataType::PublicKeyValue(Value::<String,PublicKeyValue>::new(vec![zk__data["zk__in3_cipher"].try_as_cipher_value_ref().unwrap()[2].clone()],None, Some("ecdh-chaskey".to_owned()))));
                                        zk__data.insert("zk__in5_plain",DataType::String("0".to_owned()));// DataType::Int(Choice::none as u128));
                                    println!("====_zk__vote====secret0_plain_votum=={:?}==={:?}====={:?}====={:?}======", zk__priv["secret0_plain_votum"] , zk__data["zk__in2_plain"],zk__priv["secret1_plain"], zk__data["zk__in5_plain"]);
                                        zk__data.insert("zk__out0_plain",DataType::Bool(zk__priv["secret0_plain_votum"] != zk__data["zk__in2_plain"]&&zk__priv["secret1_plain"] == zk__data["zk__in5_plain"]));
                                        assert!(*zk__data["zk__out0_plain"].try_as_bool_ref().unwrap(),"require(reveal(votum != Choice::none && current_votes[me] == Choice::None, all)) failed");
                                        // }
                println!("====_zk__vote==========================");
                                        assert!(!self.is_result_published().await,"require(!is_result_published()) failed");
        println!("====_zk__vote======================={}===",line!());
                                        state_set!(self.state().lock()[&["current_votes(address,uint)(uint)", &msg.lock().as_ref().unwrap().sender]] = votum);
        println!("====_zk__vote======================={}===",line!());
                                        let vote_count=state_get!(self.state().lock()[&["vote_count()(uint)"]]);
                                        println!("=====vote_count============={vote_count:?}============");
                                        let vote_count=state_get!(self.state().lock()[&["vote_count()(uint)"]]).try_as_string_ref().unwrap().parse::<u128>().unwrap()+1;
                                        state_set!(self.state().lock()[&["vote_count()(uint)"]] = DataType::Int(BigInt::from(vote_count)));
                                        // a_count = a_count + reveal<+>(votum == reveal(Choice::a.to_string(), me) ? reveal(1, me) : reveal(0, me), organizer);
                                        // {
        println!("====_zk__vote======================={}===",line!());
                                        zk__data.insert("zk__in6_cipher_a_count",state_get!(self.state().lock()[&["a_count(uint)(uint)"]]).clone());
                                        zk__data.insert("zk__in7_plain",DataType::String((Choice::a as u8).to_string()));
                                        let organizer=state_get!(self.state().lock()[&["organizer()(address)"]]).try_as_string_ref().unwrap().clone();
                                        let (zk__out1_cipher, zk__out1_cipher_r) = self.api().lock().enc( (if zk__priv["secret0_plain_votum"] == zk__data["zk__in7_plain"]{1} else{ 0}).to_string(), Some(organizer), "elgamal").await;
                                        zk__data.insert("zk__out1_cipher",DataType::CipherValue(zk__out1_cipher));
                                        zk__priv.insert("zk__out1_cipher_R",DataType::RandomnessValue(zk__out1_cipher_r.unwrap()));
                println!("====_zk__vote==========================");
                                        let organizer=state_get!(self.state().lock()[&["organizer()(address)"]]).try_as_string_ref().unwrap().clone();
                                        zk__data.insert("zk__out2_cipher", DataType::CipherValue(self.api().lock().do_homomorphic_op("+", "elgamal", organizer, vec![zk__data["zk__in6_cipher_a_count"].clone(), zk__data["zk__out1_cipher"].clone()]).await));

                                        state_set!(self.state().lock()[&["a_count(uint)(uint)"]] = zk__data["zk__out2_cipher"].clone());
                                        // }
                println!("====_zk__vote==========================");
                                        // b_count = b_count + reveal<+>(votum == reveal(Choice::b, me) ? reveal(1, me) : reveal(0, me), organizer);
                                        // {
                                        zk__data.insert("zk__in8_cipher_b_count",state_get!(self.state().lock()[&["b_count(uint)(uint)"]]).clone());
                                        zk__data.insert("zk__in9_plain",DataType::String((Choice::b as u8).to_string()));
                                        let organizer=state_get!(self.state().lock()[&["organizer()(address)"]]).try_as_string_ref().unwrap().clone();
                                        let (zk__out3_cipher,zk__out3_cipher_r) = self.api().lock().enc( (if zk__priv["secret0_plain_votum"] == zk__data["zk__in9_plain"]{1} else{ 0}).to_string(), Some(organizer), "elgamal").await;
                                        zk__data.insert("zk__out3_cipher",DataType::CipherValue(zk__out3_cipher));
                                        zk__priv.insert("zk__out3_cipher_R",DataType::RandomnessValue(zk__out3_cipher_r.unwrap()));
                                        let organizer=state_get!(self.state().lock()[&["organizer()(address)"]]).try_as_string_ref().unwrap().clone();
                                        zk__data.insert("zk__out4_cipher",DataType::CipherValue(self.api().lock().do_homomorphic_op("+", "elgamal", organizer, vec![zk__data["zk__in8_cipher_b_count"].clone(), zk__data["zk__out3_cipher"].clone()]).await));
                println!("====_zk__vote==========================");
                                        state_set!(self.state().lock()[&["b_count(uint)(uint)"]] = zk__data["zk__out4_cipher"].clone());
                                        // }
                println!("====_zk__vote==========================");
                                        // c_count = c_count + reveal<+>(votum == reveal(Choice::c, me) ? reveal(1, me) : reveal(0, me), organizer);
                                        // {
                                        zk__data.insert("zk__in10_cipher_c_count",state_get!(self.state().lock()[&["c_count(uint)(uint)"]]).clone());
                                        zk__data.insert("zk__in11_plain",DataType::String((Choice::c as u8).to_string()));
        let organizer=state_get!(self.state().lock()[&["organizer()(address)"]]).try_as_string_ref().unwrap().clone();
                                        let (zk__out5_cipher,zk__out5_cipher_r) = self.api().lock().enc((if zk__priv["secret0_plain_votum"] == zk__data["zk__in11_plain"]{1 } else {0}).to_string(), Some(organizer), "elgamal").await;
                                        zk__data.insert("zk__out5_cipher",DataType::CipherValue(zk__out5_cipher));
                                        zk__priv.insert("zk__out5_cipher_R",DataType::RandomnessValue(zk__out5_cipher_r.unwrap()));
        let organizer=state_get!(self.state().lock()[&["organizer()(address)"]]).try_as_string_ref().unwrap().clone();
                                        zk__data.insert("zk__out6_cipher",DataType::CipherValue(self.api().lock().do_homomorphic_op("+", "elgamal", organizer, vec![zk__data["zk__in10_cipher_c_count"].clone(), zk__data["zk__out5_cipher"].clone()]).await));

                                        state_set!(self.state().lock()[&["c_count(uint)(uint)"]] = zk__data["zk__out6_cipher"].clone());
                                        // }

                                        // Serialize input values
                                        // {
                                        zk__in[zk__in_start_idx ..zk__in_start_idx + 2].clone_from_slice(&zk__data["zk__in0_cipher_votum"].try_as_cipher_value_ref().unwrap()[..2]);
                                        zk__in[zk__in_start_idx + 2] = zk__data["zk__in1_key_sender"].try_as_public_key_value_ref().unwrap()[0].clone();
                                        zk__in[zk__in_start_idx + 3] = zk__data["zk__in2_plain"].try_as_string_ref().unwrap().clone();
                                        zk__in[zk__in_start_idx + 4..zk__in_start_idx + 6].clone_from_slice(&zk__data["zk__in3_cipher"].try_as_cipher_value_ref().unwrap()[..2]) ;
                                        zk__in[zk__in_start_idx + 6] = zk__data["zk__in4_key_sender"].try_as_public_key_value_ref().unwrap()[0].clone();
                                        zk__in[zk__in_start_idx + 7] = zk__data["zk__in5_plain"].try_as_string_ref().unwrap().clone();
                                        zk__in[zk__in_start_idx + 8..zk__in_start_idx + 12].clone_from_slice(&zk__data["zk__in6_cipher_a_count"].try_as_cipher_value_ref().unwrap()[..4]);
                                        zk__in[zk__in_start_idx + 12] = zk__data["zk__in7_plain"].try_as_string_ref().unwrap().clone();
                                        zk__in[zk__in_start_idx + 13..zk__in_start_idx + 17].clone_from_slice (&zk__data["zk__in8_cipher_b_count"].try_as_cipher_value_ref().unwrap()[..4]);
                                        zk__in[zk__in_start_idx + 17] = zk__data["zk__in9_plain"].try_as_string_ref().unwrap().clone();
                                        zk__in[zk__in_start_idx + 18..zk__in_start_idx + 22].clone_from_slice(&zk__data["zk__in10_cipher_c_count"].try_as_cipher_value_ref().unwrap()[..4]);
                                        zk__in[zk__in_start_idx + 22] = zk__data["zk__in11_plain"].try_as_string_ref().unwrap().clone();
                                        // }
                                    });
                                    // END Simulate body

                                    // Serialize circuit outputs and/or secret circuit inputs
                                    zk__out[zk__out_start_idx..zk__out_start_idx + 25].clone_from_slice(&self.api().lock().serialize_circuit_outputs(zk__data.into_iter().map(|(k,v)|(k.to_owned(),v)).collect(), vec![1, 0, 0, 0, 0, 0, 0])) ;
                                    self.api().lock().serialize_private_inputs(zk__priv.into_iter().map(|(k,v)|(k.to_owned(),v)).collect(), vec![256, 256, 0, 0, 0])
                                });
        (*zk__ins.lock(), *zk__outs.lock()) = (zk__in, zk__out);
    }
    // _zk__vote._can_be_external =

    async fn publish_results(&self) {
        with_context_block!(var _fc=self._function_ctx(6,0,"publish_results").await =>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        assert! (zk__is_ext);
                        let (msg, block,_tx) = self.api().lock().get_special_variables();
                        let _now = block.lock().as_ref().unwrap().timestamp;

                        let mut actual_params = vec![];
                        let mut zk__out = vec![DataType::Int(BigInt::zero());1];
                        actual_params.push(DataType::List(zk__out.clone()));
                         let mut zk__in= vec![DataType::Int(BigInt::zero());14];
                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            assert!(zk__out.len() == 1,"require(zk__out.length == 1) failed");

                            // Request static public keys
                            // {
                            let mut _tmp_key_Ecdh_Chaskey = PublicKeyValue::data_type("ecdh-chaskey");
                            let mut _tmp_key_Elgamal  = PublicKeyValue::data_type("elgamal");
                            _tmp_key_Elgamal = DataType::PublicKeyValue(self.api().lock().get_keystore("elgamal").lock().getPk(&msg.lock().as_ref().unwrap().sender).await);
                            zk__in[..2].clone_from_slice(&_tmp_key_Elgamal.try_as_public_key_value_ref().unwrap()[..2].iter().map(|s|DataType::String(s.clone())).collect::<Vec<_>>());
                            // }

                            // Call internal function
                            self.api().lock().call_fct(0, async ||{self._zk__publish_results(zk__in.iter().map(|s|s.try_as_string_ref().unwrap().clone()).collect(), 2, zk__out.iter().map(|s|s.try_as_string_ref().unwrap().clone()).collect(), 0).await;} ).await;
                        });
                        // END Simulate body

                        //Generate proof
                        let proof = self.api().lock().gen_proof("publish_results", zk__in.into_iter().map(|s|s.try_as_string_ref().unwrap().clone()).collect(), zk__out.into_iter().map(|s|s.try_as_string_ref().unwrap().clone()).collect());
                        actual_params.push(DataType::List(proof.into_iter().map(|s|DataType::String(s)).collect()));

                        // Invoke public transaction
                        let _=self.api().lock().transact("publish_results(uint[] calldata zk__out, uint[8] calldata zk__proof) external", actual_params, vec![false, false],None);
                    });
                });
    }
    // publish_results._can_be_external = True

    async fn _zk__publish_results(
        &self,
        mut zk__in: Vec<String>,
        zk__in_start_idx: i32,
        mut zk__out: Vec<String>,
        zk__out_start_idx: i32,
    ) {
        let (zk__in_start_idx, zk__out_start_idx) =
            (zk__in_start_idx as usize, zk__out_start_idx as usize);
        with_context_block!(var _fc=self._function_ctx(6,0,"?").await =>{
        let (zk__is_ext,_fc)=_fc;
                    assert! (!zk__is_ext);

                   let (msg, block,_tx) = self.api().lock().get_special_variables();
                   let _now = block.lock().as_ref().unwrap().timestamp;

                    let mut zk__priv =
                       BTreeMap::from( [("secret0_plain_c_count",DataType::Int(BigInt::zero())), ("zk__in0_cipher_c_count_R", RandomnessValue::data_type("elgamal")),
                        ("secret2_plain_b_count",DataType::Int(BigInt::zero())), ("zk__in1_cipher_b_count_R", RandomnessValue::data_type("elgamal")),
                        ("secret4_plain_a_count",DataType::Int(BigInt::zero())), ("zk__in2_cipher_a_count_R", RandomnessValue::data_type("elgamal")),
                    ]);
           let mut zk__data =
                            BTreeMap::from([("zk__out0_plain",DataType::Int(BigInt::zero())), ("zk__in0_cipher_c_count", CipherValue::data_type("elgamal")),
                            ("zk__in1_cipher_b_count",CipherValue::data_type("elgamal")), ("zk__in2_cipher_a_count", CipherValue::data_type("elgamal"))]);

                    // BEGIN Simulate body
                     with_context_block!(var _sc=self._scope()=>{
                        assert!((zk__out_start_idx + 1) <= zk__out.len(),"require(zk__out_start_idx + 1 <= zk__out.length) failed");
                        assert!((zk__in_start_idx + 12) <= zk__in.len(),"require(zk__in_start_idx + 12 <= zk__in.length) failed");


                         assert!(DataType::String(msg.lock().as_ref().unwrap().sender.clone() )== state_get!(self.state().lock()[&["organizer()(address)"]]),"require(me == organizer) failed");
                         assert!(self.min_votes_reached().await,"require(min_votes_reached()) failed");
                         assert!(!self.is_result_published().await,"require(!is_result_published()) failed");
                        // packed_results = reveal(((unhom(c_count)) << 128) | ((unhom(b_count)) << 64) | (unhom(a_count)), all);
                        // {
                        zk__data.insert("zk__in0_cipher_c_count",state_get!(self.state().lock()[&["c_count(uint)(uint)"]]).clone());
                        let (secret0_plain_c_count, zk__in0_cipher_c_count_r) = self.api().lock().dec(zk__data["zk__in0_cipher_c_count"].clone(), convert_type, "elgamal");
                        zk__priv.insert("secret0_plain_c_count",secret0_plain_c_count);
                        zk__priv.insert("zk__in0_cipher_c_count_R",DataType::RandomnessValue(zk__in0_cipher_c_count_r.unwrap()));

                        zk__data.insert("zk__in1_cipher_b_count", state_get!(self.state().lock()[&["b_count(uint)(uint)"]]).clone());

                        let (secret2_plain_b_count, zk__in1_cipher_b_count_r) = self.api().lock().dec(zk__data["zk__in1_cipher_b_count"].clone(), convert_type, "elgamal");
                        zk__priv.insert("secret2_plain_b_count",secret2_plain_b_count);
                        zk__priv.insert("zk__in1_cipher_b_count_R",DataType::RandomnessValue(zk__in1_cipher_b_count_r.unwrap()));

                        zk__data.insert("zk__in2_cipher_a_count", state_get!(self.state().lock()[&["a_count(uint)(uint)"]]).clone());

                        let (secret4_plain_a_count, zk__in2_cipher_a_count_r) = self.api().lock().dec(zk__data["zk__in2_cipher_a_count"].clone(), convert_type, "elgamal");
                        zk__priv.insert("secret4_plain_a_count",secret4_plain_a_count);
                        zk__priv.insert("zk__in2_cipher_a_count_R",DataType::RandomnessValue(zk__in2_cipher_a_count_r.unwrap()));
                           use alloy_primitives::{Address, Bytes, U256};
                        let zk__out0_plain:BigInt=zk__priv["secret0_plain_c_count"].try_as_int_ref().unwrap().clone() << 128 | zk__priv["secret2_plain_b_count"].try_as_int_ref().unwrap().clone() << 64 | zk__priv["secret4_plain_a_count"].try_as_int_ref().unwrap().clone();
                        zk__data.insert("zk__out0_plain",DataType::String( zk__out0_plain.to_string()));

                        state_set!(self.state().lock()[&["packed_results()(uint)"]] = zk__data["zk__out0_plain"].clone());
                        // }

                        // Serialize input values
                        // {
                        zk__in[zk__in_start_idx + 0..zk__in_start_idx + 4].clone_from_slice(&zk__data["zk__in0_cipher_c_count"].try_as_cipher_value_ref().unwrap()[..4]);
                        zk__in[zk__in_start_idx + 4..zk__in_start_idx + 8].clone_from_slice(&zk__data["zk__in1_cipher_b_count"].try_as_cipher_value_ref().unwrap()[..4]);
                        zk__in[zk__in_start_idx + 8..zk__in_start_idx + 12].clone_from_slice(&zk__data["zk__in2_cipher_a_count"].try_as_cipher_value_ref().unwrap()[..4]);
                        // }
                     });
                    // END Simulate body

                    // Serialize circuit outputs and/or secret circuit inputs
                    zk__out[zk__out_start_idx] = self.api().lock().serialize_circuit_outputs(zk__data.into_iter().map(|(k,v)|(k.to_owned(),v)).collect(), vec![192]).concat();
                    self.api().lock().serialize_private_inputs(zk__priv.into_iter().map(|(k,v)| (k.to_owned(),v)).collect(), vec![32, 0, 32, 0, 32, 0]);
                    });
    }
    // _zk__publish_results._can_be_external = false

    async fn check_if_agree_with_majority(&self) -> DataType {
        with_context_block!(var _fc=self._function_ctx(2,0,"check_if_agree_with_majority").await =>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        assert! (zk__is_ext);
                        let (msg, block,_tx) = self.api().lock().get_special_variables();
                        let _now = block.lock().as_ref().unwrap().timestamp;

                        let mut zk__priv =
                             BTreeMap::from([("glob_sk_Ecdh_Chaskey__me", PublicKeyValue::data_type("ecdh-chaskey"))]);


                        let mut actual_params = vec![];
                        let mut zk__out = vec![DataType::Int(BigInt::zero());2];
                        actual_params.push(DataType::List(zk__out.clone()));

                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            zk__priv.insert("glob_sk_Ecdh_Chaskey__me",DataType::PrivateKeyValue(self.api().lock().get_my_sk("ecdh-chaskey")));
                            assert!(zk__out.len() == 2,"require(zk__out.length == 2) failed");
                            let mut zk__in = vec![0.to_string();5];

                            // Request static public keys
                            // {
                            // let mut _tmp_key_Ecdh_Chaskey = PublicKeyValue::data_type("ecdh-chaskey");

                            let mut _tmp_key_Ecdh_Chaskey = self.api().lock().get_keystore("ecdh-chaskey").lock().getPk(&msg.lock().as_ref().unwrap().sender).await;
                            zk__in[0] = _tmp_key_Ecdh_Chaskey[0].clone();
                            // }

                            // Declare return variables
                            // let mut zk__ret_0 = CipherValue::data_type("ecdh-chaskey");

                            // Call internal function
                            // zk__ret_0 =
                             self.api().lock().call_fct(1,async ||{self._zk__check_if_agree_with_majority( zk__in.clone(), 1, zk__out.clone().into_iter().map(|s|s.try_as_string().unwrap()).collect(), 0).await;}).await;
                         });
                        // END Simulate body

                        // Serialize circuit outputs and/or secret circuit inputs
                        self.api().lock().serialize_private_inputs(zk__priv.into_iter().map(|(k,v)|(k.to_owned(),v)).collect(), vec![0]);

                        // Call pure/view function and return value
                        return self.api().lock().call("check_if_agree_with_majority(uint[] calldata zk__out) external view returns (uint[3] memory)", actual_params, vec![(true, "ecdh-chaskey".to_owned(), convert_type)]).await

                    });
                });
    }
    // check_if_agree_with_majority._can_be_external = True

    async fn _zk__check_if_agree_with_majority(
        &self,
        mut zk__in: Vec<String>,
        zk__in_start_idx: usize,
        mut zk__out: Vec<String>,
        zk__out_start_idx: usize,
    ) -> String {
        with_context_block!(var _fc=self._function_ctx(1,0,"?").await =>{
        let (zk__is_ext,_fc)=_fc;
                    assert! (!zk__is_ext);

                   let (msg, block,_tx) = self.api().lock().get_special_variables();
                   let _now = block.lock().as_ref().unwrap().timestamp;

                    let mut zk__priv =
                         BTreeMap::from([("secret0_plain", DataType::Int(BigInt::zero()))]);

        // Declare return variables
                        let  zk__ret_0;//= CipherValue::data_type("ecdh-chaskey");
          let mut zk__data =
                           BTreeMap::from ([("zk__out0_cipher", CipherValue::data_type("ecdh-chaskey")),
                        ("zk__in0_plain_c", DataType::Int(BigInt::zero())), ("zk__in1_cipher", CipherValue::data_type("ecdh-chaskey")),
                    ("zk__in2_key_sender", PublicKeyValue::data_type("ecdh-chaskey"))]);


                    // BEGIN Simulate body
                     with_context_block!(var _sc=self._scope()=>{
                        assert!((zk__out_start_idx + 2) <= zk__out.len(),"require(zk__out_start_idx + 2 <= zk__out.length) failed");
                        assert!((zk__in_start_idx + 4) <= zk__in.len(),"require(zk__in_start_idx + 4 <= zk__in.length) failed");


                        self.locals().lock().decl("c", self.get_winning_choice().await );
                        // return (reveal(c, me) == current_votes[me]);
                        // {
                        zk__data.insert("zk__in0_plain_c",self.locals().lock()["c"].clone());
                        zk__data.insert("zk__in1_cipher",state_get!(self.state().lock()[&["current_votes(address,uint)(uint)", &msg.lock().as_ref().unwrap().sender]]).clone());
                        zk__priv.insert("secret0_plain",self.api().lock().dec(zk__data["zk__in1_cipher"].clone(), convert_type, "ecdh-chaskey").0);
                        zk__data.insert("zk__in2_key_sender",DataType::PublicKeyValue(Value::<String,PublicKeyValue>::new(vec![zk__data["zk__in1_cipher"].try_as_cipher_value_ref().unwrap()[2].clone()],None, Some("ecdh-chaskey".to_owned()))));
                        //msg.lock().as_ref().unwrap().sender
                        let mut s=self.api().lock().enc((if zk__data["zk__in0_plain_c"] == zk__priv["secret0_plain"]{1}else{0}).to_string(),Some(msg.lock().as_ref().unwrap().sender.clone()) , "ecdh-chaskey").await.0;
                        s.contents.pop();
                        s.contents.push(self.api().lock().get_my_pk("ecdh-chaskey")[0].clone());
                        zk__data.insert("zk__out0_cipher",DataType::CipherValue(s));

                        zk__ret_0 = zk__data["zk__out0_cipher"].clone();
                        // }

                        // Serialize input values
                        // {
                        zk__in[zk__in_start_idx] = zk__data["zk__in0_plain_c"].try_as_int_ref().unwrap().to_string();
                        zk__in[zk__in_start_idx + 1..zk__in_start_idx + 3].clone_from_slice(&zk__data["zk__in1_cipher"].try_as_cipher_value_ref().unwrap()[..2]);
                        zk__in[zk__in_start_idx + 3] = zk__data["zk__in2_key_sender"].try_as_public_key_value_ref().unwrap()[0].clone();
                        // }
                     });
                    // END Simulate body

                    // Serialize circuit outputs and/or secret circuit inputs
                    zk__out[zk__out_start_idx..zk__out_start_idx + 2].clone_from_slice(&self.api().lock().serialize_circuit_outputs(zk__data.into_iter().map(|(k,v)|(k.to_owned(),v)).collect(), vec![0]));
                    self.api().lock().serialize_private_inputs(zk__priv.into_iter().map(|(k,v)|(k.to_owned(),v)).collect(), vec![256]);

                    return zk__ret_0.try_as_string().unwrap()
                        });
    }
    // _zk__check_if_agree_with_majority._can_be_external = false
}

async fn deploy<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
>(
    _min_votes: i32,
    user: &str,
    mut cs: ARcCell<ContractSimulator<C, P, B, K>>,
) -> Survey<C, P, B, K> {
    assert!(!user.is_empty(), "user.is_empty() ");
    Survey::deploy(_min_votes, user, "", cs).await
}

async fn connect<
    PS: ProvingScheme,
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone + std::marker::Send + std::marker::Sync,
    B: ZkayBlockchainInterface<P> + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
>(
    address: &Address,
    user: &str,
    mut cs: ARcCell<ContractSimulator<C, P, B, K>>,
    compile_zkay_file: fn(
        input_file_path: &str,
        output_dir: &str,
        import_keys: bool,
    ) -> anyhow::Result<()>,
    get_verification_contract_names: fn(code_or_ast: String) -> Vec<String>,
) -> Survey<C, P, B, K> {
    assert!(!user.is_empty(), "user.is_empty() ");
    Survey::connect::<PS>(
        address,
        &user,
        "",
        cs,
        compile_zkay_file,
        get_verification_contract_names,
    )
    .await
}

// fn create_dummy_accounts(count:i32) -> Vec<String>{
//     contract_simulator.create_dummy_accounts(count)
// }

fn help(val: Option<String>) {
    if val.is_none() {
        // import sys
        // ContractSimulator::help("sys.modules[__name__]", Survey, "Survey");
    }
    // else{
    //     import builtins
    //     builtins.help(val)
    // }
    // }
}
fn convert_type(v: String) -> DataType {
    DataType::String(v)
}
fn convert_bool_type(v: String) -> DataType {
    DataType::Bool(
        serde_json::from_str::<Vec<JsonValue>>(&v).unwrap()[0]
            .as_str()
            .unwrap()
            .parse::<bool>()
            .unwrap(),
    )
}
use crate::zkay_frontend::compile_zkay_file;
use ast_builder::process_ast::{get_processed_ast, get_verification_contract_names};
use zkay_ast::global_defs::{
    GlobalDefs, GlobalVars, array_length_member, global_defs, global_vars,
};
pub async fn main0(web3tx: Web3Tx) -> eyre::Result<()> {
    println!("==========================00000==============");
    // contract_simulator.use_config_from_manifest(file!());
    // os.path.dirname(os.path.realpath(__file__);
    // let me = contract_simulator.default_address();
    // if me.is_some(){
    //     me = me.val;
    // }
    // import code
    // code.interact(local=globals())

    // let contract_simulator=ContractSimulator::new(".","","",runtime.clone());
    // arc_cell_new!(_crypto_classes::<P, B, K>(&crypto_backend, keystore)),
    let sender = if let Some(address) = web3tx.eth.wallet.from {
        address
    } else {
        // println!("=============signer");
        let signer = web3tx.eth.wallet.signer().await?;
        signer.address()
    };
    println!("===================22=======00000==============");
    let _contract_simulator = new_contract_simulator(
        "/Users/lisheng/mygit/arron/zkay-rs/survey_compiled/",
        "Survey",
        sender.clone(),
        web3tx,
    );
    // let survey = connect::<ProvingSchemeGroth16, _, _, _, _>(
    //     &Address::default(),
    //     "",
    //     arc_cell_new!(_contract_simulator),
    //     compile_zkay_file,
    //     |s: String| {
    //         let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
    //         get_verification_contract_names((Some(s), None), global_vars)
    //     },
    // );

    let survey =
        deploy::<_, _, _, _>(2, &sender.to_string(), arc_cell_new!(_contract_simulator)).await;
    let min_votes = survey.min_votes_reached().await;
    println!("{min_votes:?}");
    let is_result_published = survey.is_result_published().await;
    println!("{is_result_published}");
    survey.vote(1).await;

    Ok(())
}
