#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(non_camel_case_types)]

// //////////////////////////////////////////////////////////////////////////////////////
// // THIS CODE WAS GENERATED AUTOMATICALLY ////
// // Creation Time: 08:02:21 20-Nov-2024   ////
// //////////////////////////////////////////////////////////////////////////////////////
// from __future__ import annotations

// import os
// from enum import IntEnum
// from typing import Dict, List, Tuple, Optional, Union, Any
use my_logging;
use proving_scheme::proving_scheme::ProvingScheme;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use zkay_config::{
    config::{zk_print_banner, CFG},
    with_context_block,
};
use zkay_transaction::blockchain::web3rs::Web3Blockchain;
use zkay_transaction::int_casts::*;
use zkay_transaction::interface::{
    ZkayBlockchainInterface, ZkayCryptoInterface, ZkayHomomorphicCryptoInterface,
    ZkayKeystoreInterface, ZkayProverInterface,
};
use zkay_transaction::offchain::{
    ContractSimulator, ContractSimulatorConfig, ContractSimulatorRef, BN128_SCALAR_FIELDS,
};
use zkay_transaction::solidity_math::*;
use zkay_transaction::types::{AddressValue,DataType};
use zkay_utils::timer::time_measure;
// me = None

struct Survey<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone,
    B: ZkayBlockchainInterface<P> + Web3Blockchain + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
> {
    contract_simulator: ContractSimulator<C, P, B, K>,
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
        P: ZkayProverInterface + Clone,
        B: ZkayBlockchainInterface<P> + Web3Blockchain + Clone,
        K: ZkayKeystoreInterface<P, B> + Clone,
    > ContractSimulatorRef<C, P, B, K> for Survey<C, P, B, K>
{
    fn contract_simulator_ref(&self) -> &ContractSimulator<C, P, B, K> {
        &self.contract_simulator
    }
    fn contract_simulator_mut(&mut self) -> &mut ContractSimulator<C, P, B, K> {
        &mut self.contract_simulator
    }
}
impl<
        C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
        P: ZkayProverInterface + Clone,
        B: ZkayBlockchainInterface<P> + Web3Blockchain + Clone,
        K: ZkayKeystoreInterface<P, B> + Clone,
    > Survey<C, P, B, K>
{
    fn new(
        _project_dir: &str,
        _user_addr: &str,
        mut contract_simulator: ContractSimulator<C, P, B, K>,
    ) -> Self {
        // super().__init__(project_dir, user_addr, "Survey");
        contract_simulator
            .state
            .borrow_mut()
            .decl("organizer", convert_type, false, "");
        contract_simulator.state.borrow_mut().decl(
            "current_votes",
            convert_type,
            true,
            "ecdh-chaskey",
        );
        contract_simulator
            .state
            .borrow_mut()
            .decl("a_count", convert_type, true, "elgamal");
        contract_simulator
            .state
            .borrow_mut()
            .decl("b_count", convert_type, true, "elgamal");
        contract_simulator
            .state
            .borrow_mut()
            .decl("c_count", convert_type, true, "elgamal");
        contract_simulator
            .state
            .borrow_mut()
            .decl("min_votes", convert_type, false, "");
        contract_simulator
            .state
            .borrow_mut()
            .decl("vote_count", convert_type, false, "");
        contract_simulator
            .state
            .borrow_mut()
            .decl("packed_results", convert_type, false, "");
        Self { contract_simulator }
    }

    //@staticmethod
    fn connect<PS: ProvingScheme>(
        address: &str,
        user: &str,
        project_dir: &str,
        mut contract_simulator: ContractSimulator<C, P, B, K>,
        compile_zkay_file: fn(
            input_file_path: &str,
            output_dir: &str,
            import_keys: bool,
        ) -> anyhow::Result<String>,
        get_verification_contract_names: fn(code_or_ast: String) -> Vec<String>,
    ) -> Survey<C, P, B, K> {
        // = os.path.dirname(os.path.realpath(__file__))
        let c = Survey::new(project_dir, user, contract_simulator.clone());
        c.api().connect::<PS>(
            address.into(),
            compile_zkay_file,
            get_verification_contract_names,
        );
        contract_simulator.initialize_keys_for(user);
        c
    }

    //@staticmethod
    fn deploy(
        _min_votes: i32,
        user: &str,
        project_dir: &str,
        mut contract_simulator: ContractSimulator<C, P, B, K>,
    ) -> Survey<C, P, B, K> {
        //= os.path.dirname(os.path.realpath(__file__))
        contract_simulator.initialize_keys_for(user);
        let mut c = Survey::new(project_dir, user, contract_simulator);
        c.constructor(_min_votes);
        c
    }
    fn constructor(&mut self, _min_votes: i32) {
        with_context_block!(var _fc= self._function_ctx(-1,0,"constructor")=> {
            let (zk__is_ext,_fc)=_fc;
             with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                let (msg, block, _tx) = self.api().get_special_variables();
                let _now = block.borrow().as_ref().unwrap().timestamp;
                let actual_params = vec![_min_votes.to_string()];
                // BEGIN Simulate body
                 with_context_block!(var _sc= self._scope()=>{
                    assert!(_min_votes > 0,"require(_min_votes > 0) failed");
                    self.state().borrow_mut()[&["organizer"]] = msg.borrow().as_ref().unwrap().sender.clone();
                    self.state().borrow_mut()[&["min_votes"]] = _min_votes.to_string();
                });
                // END Simulate body

                if zk__is_ext{
                    // Deploy contract
                    self.api().deploy(actual_params, vec![false],None);
                }

            });
        });
    }
    // constructor._can_be_external = True

    fn get_result_for(&mut self, option: &String) -> String {
        with_context_block!(var _fc=self._function_ctx(-1,0,"get_result_for")=>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                         let (_msg, block, _tx) = self.api().get_special_variables();
                        let _now = block.borrow().as_ref().unwrap().timestamp;

                        let actual_params = vec![option.to_string()];

                        // BEGIN Simulate body
                         with_context_block!(var _sc0=self._scope()=>{
                            assert!(self.is_result_published(),"require(is_result_published()) failed");
                            self.locals().borrow_mut().decl("res", 0.to_string());
                            if option != &Choice::none.to_string(){
                                 with_context_block!(var _sc=self._scope()=>{
                                    self.locals().borrow()["res"] = (self.state().borrow()[&["packed_results"]].parse::<u128>().unwrap() >> (64 * (((option.parse::<u64>().unwrap()) - 1)))).to_string();
                                });
                            }
                            if ! zk__is_ext{
                                return self.locals().borrow()["res"].clone()
                            }
                        });
                        // END Simulate body

                        if zk__is_ext{
                            // Call pure/view function and return value
                            return self.api().call("get_result_for", actual_params, vec![(false, "None".to_string(), convert_type)])[0].clone();
                        }
                    });
                });
        String::new()
    }
    // get_result_for._can_be_external = True

    fn get_winning_choice(&mut self) -> String {
        with_context_block!(var _fc=self._function_ctx(-1,0,"get_winning_choice") => {
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        let (_msg, block,_tx) = self.api().get_special_variables();
                        let _now = block.borrow().as_ref().unwrap().timestamp;

                        let actual_params = vec![];

                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            self.locals().borrow_mut().decl("c", Choice::none.to_string());
                            self.locals().borrow_mut().decl("votes", 0.to_string());
                            self.locals().borrow_mut().decl("i", Choice::a.to_string());
                            while self.locals().borrow()["i"] <=Choice::c.to_string(){
                                // try{
                                     with_context_block!(var _sc1=self._scope()=>{
                                        self.locals().borrow_mut().decl("res", self.get_result_for(&self.locals().borrow()["i"]));
                                        if self.locals().borrow()["res"] > self.locals().borrow()["votes"]{
                                             with_context_block!(var _sc2=self._scope()=>{
                                                self.locals().borrow_mut()["c"] = self.locals().borrow()["i"].clone();
                                                self.locals().borrow_mut()["votes"] = self.locals().borrow()["res"].clone();
                                            });
                                        }
                                    });
                                // finally{
                                    self.locals().borrow_mut()["i"] += &1.to_string();
                            }
                            if ! zk__is_ext{
                                return self.locals().borrow()["c"].clone()
                            }
                        });
                        // END Simulate body

                        if zk__is_ext{
                            // Call pure/view function and return value
                            return self.api().call("get_winning_choice", actual_params, vec![(false, "None".to_string(), convert_type)])[0].clone()
                        }
                    });
                });
        String::new()
    }
    // get_winning_choice._can_be_external = True

    fn min_votes_reached(&mut self) -> bool {
        with_context_block!(var _fc=self._function_ctx(-1,0,"min_votes_reached") => {
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        let (_msg, block,_tx) = self.api().get_special_variables();
                        let _now = block.borrow().as_ref().unwrap().timestamp;

                        let actual_params = vec![];

                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            if ! zk__is_ext{
                                return self.state().borrow()[&["vote_count"]] >= self.state().borrow()[&["min_votes"]]
                            }
                        });
                        // END Simulate body

                        if zk__is_ext{
                            // Call pure/view function and return value
                            return !self.api().call("min_votes_reached", actual_params,vec![(false, "None".to_string(), convert_type)]).is_empty()
                        }
                    });
                });
        false
    }

    // min_votes_reached._can_be_external = True

    fn is_result_published(&mut self) -> bool {
        with_context_block!(var _fc=self._function_ctx(-1,0,"is_result_published")=>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", ! zk__is_ext,false)=>{
                        let (_msg, block,_tx) = self.api().get_special_variables();
                        let _now = block.borrow().as_ref().unwrap().timestamp;

                        let actual_params = vec![];

                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            if ! zk__is_ext{
                                return self.state().borrow()[&["packed_results"]] != 0.to_string()
                            }
                        });
                        // END Simulate body

                        if zk__is_ext{
                            // Call pure/view function and return value
                            return self.api().call("is_result_published", actual_params, vec![(false, "None".to_string(), convert_type)])[0].parse::<bool>().unwrap()
                        }
                    });
                });
        false
    }
    // is_result_published._can_be_external = True

    fn vote(&self, votum: u8) {
        with_context_block!(var _fc=self._function_ctx(7,0,"vote") =>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        assert!( zk__is_ext);
                        let (msg, block,_tx) = self.api().get_special_variables();
                        let _now = block.borrow().as_ref().unwrap().timestamp;

                        let mut zk__priv =
                            BTreeMap::from([("glob_sk_Ecdh_Chaskey__me", PublicKeyValue::data_type("ecdh-chaskey")), 
("votum", DataType::Int(0))]);


                        // Encrypt parameters
                        zk__priv.insert("votum",DataType::Int(votum));
                        let mut d=self.api().enc(zk__priv["votum"].parse::<i32>().unwrap(),None, "ecdh-chaskey").0.contents.clone();
                        d.pop();
                        d.push(self.api().get_my_pk("ecdh-chaskey")[0].to_string());
                        let votum=  DataType::CryptoValue(Value<String,CryptoValue>::new(d,None,Some("ecdh-chaskey".to_owned())));
                        let mut actual_params = vec![votum];
                        let zk__out = vec![DataType::Int(0);25];
                        actual_params.push(DataType::List(zk__out));
                        let mut zk__in = vec![DataType::Int(0);28];
                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            zk__priv.insert("glob_sk_Ecdh_Chaskey__me",DataType::PrivateKeyValue(self.api().get_my_sk("ecdh-chaskey")));
                            assert!(zk__out.len() == 25,"require(zk__out.length == 25) failed");

                            // Request static public keys
                            // {
                            let mut _tmp_key_Ecdh_Chaskey = PublicKeyValue::data_type("ecdh-chaskey");
                            let mut _tmp_key_Elgamal  = PublicKeyValue::data_type("elgamal");
                             _tmp_key_Ecdh_Chaskey = DataType::PublicKeyValue(self.api().get_keystore("ecdh-chaskey").getPk(&msg.borrow().as_ref().unwrap().sender));
                            zk__in[0] = DataType::String(_tmp_key_Ecdh_Chaskey[0].clone());
                            _tmp_key_Elgamal =  DataType::PublicKeyValue(self.api().get_keystore("elgamal").getPk(&self.state().borrow()[&["organizer"]]));
                            zk__in[1..3].clone_from_slice(&_tmp_key_Elgamal[..2]) ;
                            // }

                            // Backup private arguments for verification
                            // {
                            zk__in[3..5].clone_from_slice( &votum[..2]);

                            // Copy from calldata to memory and set sender field
                            self.locals().borrow().decl("votum", DataType::CryptoValue(Value::<String,CryptoValue>::new(vec![votum[0], votum[1], zk__in[0]],None,Some("ecdh-chaskey".to_owned()))));
                            // }

                            // Call internal function
                            self.api().call_fct(2, ||{self._zk__vote(&self.locals().borrow()["votum"], zk__in.clone(), 5, zk__out.clone(), 0);} );
                        });
                        // END Simulate body

                        // Serialize circuit outputs and/or secret circuit inputs
                        self.api().serialize_private_inputs(zk__priv.into_iter().map(|(k,v)|(k.to_owned(),v)).collect(), vec![0, 256]);

                        //Generate proof
                        let proof = self.api().gen_proof("vote", zk__in, zk__out);
                        actual_params.push(proof);
                        let actual_params:Vec<_>=actual_params.into_iter().flatten().collect();
                        // Invoke public transaction
                        return self.api().transact("vote", actual_params, vec![true, false, false],None)
                    });
                });
    }
    // vote._can_be_external = True

    fn _zk__vote(
        &self,
        votum: DataType,
        zk__in: Vec<String>,
        zk__in_start_idx: i32,
        zk__out: Vec<String>,
        zk__out_start_idx: i32,
    ) {
        let (zk__in_start_idx,zk__out_start_idx)=(zk__in_start_idx as usize,zk__out_start_idx as usize);
        with_context_block!(var _fc=self._function_ctx(5,0,"?") =>{
        let (zk__is_ext,_fc)=_fc;
                    assert! (!zk__is_ext);

                    let (msg, block,_tx) = self.api().get_special_variables();
                    let _now = block.borrow().as_ref().unwrap().timestamp;

                    let mut zk__priv =
                        BTreeMap::from([("secret0_plain_votum", DataType::Int(0)), ("secret1_plain", DataType::Int(0)), 
                        ("zk__out1_cipher_R", RandomnessValue::data_type("elgamal")), ("zk__out3_cipher_R", RandomnessValue::data_type("elgamal")),
                        ("zk__out5_cipher_R", RandomnessValue::data_type("elgamal"))]);
                    let mut zk__data = BTreeMap::new();

                    // BEGIN Simulate body
                     with_context_block!(var _sc=self._scope()=>{
                        assert!((zk__out_start_idx + 25) <= zk__out.len() as i32,"require(zk__out_start_idx + 25 <= zk__out.length) failed");
                        assert!((zk__in_start_idx + 23) <= zk__in.len() as i32,"require(zk__in_start_idx + 23 <= zk__in.length) failed");
                        zk__data = BTreeMap::from([
                            ("zk__out0_plain", Data::Bool(false)), ("zk__out1_cipher", CryptoValue::data_type("elgamal")), 
                            ("zk__out2_cipher", CryptoValue::data_type("elgamal")), ("zk__out3_cipher", CryptoValue::data_type("elgamal")),
                            ("zk__out4_cipher", CryptoValue::data_type("elgamal")), ("zk__out5_cipher",CryptoValue::data_type("elgamal")),
                             ("zk__out6_cipher", CryptoValue::data_type("elgamal")), ("zk__in0_cipher_votum", CryptoValue::data_type("ecdh-chaskey")),
                            ("zk__in1_key_sender", PublicKeyValue::data_type("ecdh-chaskey")), ("zk__in2_plain", DataType::Int(0)), 
                            ("zk__in3_cipher", CryptoValue::data_type("ecdh-chaskey")), ("zk__in4_key_sender", PublicKeyValue::data_type("ecdh-chaskey")),
                            ("zk__in5_plain", DataType::Int(0)), ("zk__in6_cipher_a_count", CryptoValue::data_type("elgamal")), 
                            ("zk__in7_plain", DataType::Int(0)), ("zk__in8_cipher_b_count", CryptoValue::data_type("elgamal")),
                            ("zk__in9_plain", DataType::Int(0)), ("zk__in10_cipher_c_count", CryptoValue::data_type("elgamal")),( "zk__in11_plain",DataType::Int(0)),
                        ]);

                        // require(reveal(votum != reveal(Choice::None.to_string(), me) && current_votes[me] == reveal(Choice::None.to_string(), me), all));
                        // {
                        fn choice(v:String){
                                v
                        }
                        zk__data.insert("zk__in0_cipher_votum",votum.clone());
                        zk__priv.insert("secret0_plain_votum", self.api().dec(zk__data["zk__in0_cipher_votum"], choice, "ecdh-chaskey").0);
                        zk__data.insert("zk__in1_key_sender", DataType::PublicKeyValue(Value::<String,PublicKeyValue>::new(vec![zk__data["zk__in0_cipher_votum"].try_as_crypto_value_ref().unwrap()[2].clone()],None, Some("ecdh-chaskey".to_owned()))));
                        zk__data.insert("zk__in2_plain", DataType::Int(Choice::none as u128));
                        zk__data.insert("zk__in3_cipher", self.state().borrow()[&["current_votes", msg.borrow().as_ref().unwrap().sender]]);
                        zk__priv.insert("secret1_plain",self.api().dec(zk__data["zk__in3_cipher"], choice, "ecdh-chaskey").0);
                        zk__data.insert("zk__in4_key_sender",DataType::PublicKeyValue(Value::<String,PublicKeyValue>::new(vec![zk__data["zk__in3_cipher"].try_as_crypto_value_ref().unwrap()[2].clone()],None, Some("ecdh-chaskey".to_owned()))));
                        zk__data.insert("zk__in5_plain", DataType::Int(Choice::none as u128));
                        zk__data.insert("zk__out0_plain",DataType::Bool(zk__priv["secret0_plain_votum"] != zk__data["zk__in2_plain"]&&zk__priv["secret1_plain"] == zk__data["zk__in5_plain"]);

                        assert!(*zk__data["zk__out0_plain"].try_as_bool_ref().unwrap(),"require(reveal(votum != Choice::none && current_votes[me] == Choice::None.to_string(), all)) failed");
                        // }

                        assert!(!self.is_result_published(),"require(!is_result_published()) failed");
                        self.state().borrow_mut()[&["current_votes", msg.borrow().as_ref().unwrap().sender]] = votum;
                        self.state().borrow_mut()[&["vote_count"]] = DataType::Int(*self.state().borrow()[&["vote_count"]].try_as_int_ref().unwwrap()+1);
                        // a_count = a_count + reveal<+>(votum == reveal(Choice::a.to_string(), me) ? reveal(1, me) : reveal(0, me), organizer);
                        // {
                        zk__data.insert("zk__in6_cipher_a_count",self.state().borrow()[&["a_count"]]);
                        zk__data.insert("zk__in7_plain",DataType::Int(Choice::a as usize));
                        let (zk__out1_cipher, zk__out1_cipher_r) = self.api().enc( if zk__priv["secret0_plain_votum"] == zk__data["zk__in7_plain"]{1} else{ 0}, Some(self.state().borrow()[&["organizer"]].try_as_string_ref().unwrap().clone()), "elgamal");
                        zk__data.insert("zk__out1_cipher",zk__out1_cipher);
                        zk__priv.insert("zk__out1_cipher_R",zk__out1_cipher_r);

                        zk__data.insert("zk__out2_cipher", self.api().do_homomorphic_op("+", "elgamal", self.state().borrow()[&["organizer"]], vec![zk__data["zk__in6_cipher_a_count"].clone(), zk__data["zk__out1_cipher"].clone()]));

                        *self.state().borrow_mut()[&["a_count"]] = zk__data["zk__out2_cipher"];
                        // }

                        // b_count = b_count + reveal<+>(votum == reveal(Choice::b, me) ? reveal(1, me) : reveal(0, me), organizer);
                        // {
                        zk__data.insert("zk__in8_cipher_b_count",self.state().borrow()[&["b_count"]]);
                        zk__data.insert("zk__in9_plain",DataType::Int(Choice::b as u128));
                        let (zk__out3_cipher,zk__out3_cipher_r) = self.api().enc( if zk__priv["secret0_plain_votum"] == zk__data["zk__in9_plain"]{1} else{ 0}, Some(self.state().borrow()[&["organizer"]].try_as_string_ref().unwrap().clone()), "elgamal");
                        zk__data.insert("zk__out3_cipher",zk__out3_cipher);
                        zk__priv.insert("zk__out3_cipher_R",zk__out3_cipher_r);
                        zk__data.insert("zk__out4_cipher",self.api().do_homomorphic_op("+", "elgamal", self.state().borrow()[&["organizer"]], vec![zk__data["zk__in8_cipher_b_count"].clone(), zk__data["zk__out3_cipher"].clone()]));

                        *self.state().borrow_mut()[&["b_count"]] = zk__data["zk__out4_cipher"].clone();
                        // }

                        // c_count = c_count + reveal<+>(votum == reveal(Choice::c, me) ? reveal(1, me) : reveal(0, me), organizer);
                        // {
                        zk__data.insert("zk__in10_cipher_c_count",self.state().borrow()[&["c_count"]].clone());
                        zk__data.insert("zk__in11_plain",DataType::Int(Choice::c as u128));
                        let (zk__out5_cipher,zk__out5_cipher_r) = self.api().enc(if zk__priv["secret0_plain_votum"] == zk__data["zk__in11_plain"]{1 } else {0}, self.state().borrow()[&["organizer"]], "elgamal");
                        zk__data.insert("zk__out5_cipher",zk__out5_cipher);
                        zk__priv.insert("zk__out5_cipher_R",zk__out5_cipher_r);
                        zk__data.insert("zk__out6_cipher",self.api().do_homomorphic_op("+", "elgamal", Some(self.state().borrow()[&["organizer"]].try_as_string_ref().unwrap().clone()), vec![zk__data["zk__in10_cipher_c_count"].clone(), zk__data["zk__out5_cipher"].clone()]));

                        *self.state().borrow_mut()[&["c_count"]] = zk__data["zk__out6_cipher"].clone();
                        // }

                        // Serialize input values
                        // {
                        zk__in[zk__in_start_idx ..zk__in_start_idx + 2].clone_from_slice(&zk__data["zk__in0_cipher_votum"][..2]);
                        zk__in[zk__in_start_idx + 2] = zk__data["zk__in1_key_sender"][0].clone();
                        zk__in[zk__in_start_idx + 3] = zk__data["zk__in2_plain"].clone();
                        zk__in[zk__in_start_idx + 4..zk__in_start_idx + 6].clone_from_slice(&zk__data["zk__in3_cipher"][..2]) ;
                        zk__in[zk__in_start_idx + 6] = zk__data["zk__in4_key_sender"][0].clone();
                        zk__in[zk__in_start_idx + 7] = zk__data["zk__in5_plain"].clone();
                        zk__in[zk__in_start_idx + 8..zk__in_start_idx + 12].clone_from_slice(&zk__data["zk__in6_cipher_a_count"][..4]);
                        zk__in[zk__in_start_idx + 12] = zk__data["zk__in7_plain"].clone();
                        zk__in[zk__in_start_idx + 13..zk__in_start_idx + 17].clone_from_slice (&zk__data["zk__in8_cipher_b_count"][..4]);
                        zk__in[zk__in_start_idx + 17] = zk__data["zk__in9_plain"].clone();
                        zk__in[zk__in_start_idx + 18..zk__in_start_idx + 22].clone_from_slice(&zk__data["zk__in10_cipher_c_count"][..4]);
                        zk__in[zk__in_start_idx + 22] = zk__data["zk__in11_plain"].clone();
                        // }
                    });
                    // END Simulate body

                    // Serialize circuit outputs and/or secret circuit inputs
                    zk__out[zk__out_start_idx..zk__out_start_idx + 25].clone_from_slice(&self.api().serialize_circuit_outputs(zk__data, vec![1, 0, 0, 0, 0, 0, 0])) ;
                    self.api().serialize_private_inputs(zk__priv, vec![256, 256, 0, 0, 0])
                });
    }
    // _zk__vote._can_be_external =

    fn publish_results(&self) {
        with_context_block!(var _fc=self._function_ctx(6,0,"publish_results") =>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        assert! (zk__is_ext);
                        let (msg, block,_tx) = self.api().get_special_variables();
                        let _now = block.borrow().as_ref().unwrap().timestamp;

                        let mut actual_params = vec![];
                        let mut zk__out = vec![DataType::Int(0);1];
                        actual_params.push(zk__out);
                         let mut zk__in= vec![DataType::Int(0);14];
                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            assert!(zk__out.len() == 1,"require(zk__out.length == 1) failed");

                            // Request static public keys
                            // {
                            let mut _tmp_key_Ecdh_Chaskey = PublicKeyValue::data_type("ecdh-chaskey");
                            let mut _tmp_key_Elgamal  = PublicKeyValue::data_type("elgamal");
                            _tmp_key_Elgamal = self.api().get_keystore("elgamal").getPk(msg.borrow().as_ref().unwrap().sender);
                            zk__in[..2].clone_from_slice(&_tmp_key_Elgamal[..2]);
                            // }

                            // Call internal function
                            self.api().call_fct(0, ||{self._zk__publish_results(zk__in, 2, zk__out, 0);} );
                        });
                        // END Simulate body

                        //Generate proof
                        let proof = self.api().gen_proof("publish_results", zk__in, zk__out);
                        actual_params.push(proof);

                        // Invoke public transaction
                        return self.api().transact("publish_results", actual_params, vec![false, false],None)
                    });
                });
    }
    // publish_results._can_be_external = True

    fn _zk__publish_results(
        &self,
        zk__in: Vec<String>,
        zk__in_start_idx: i32,
        zk__out: Vec<String>,
        zk__out_start_idx: i32,
    ) {
        let (zk__in_start_idx,zk__out_start_idx)=(zk__in_start_idx as usize,zk__out_start_idx  as usize);
        with_context_block!(var _fc=self._function_ctx(6,0,"?") =>{
        let (zk__is_ext,_fc)=_fc;
                    assert! (!zk__is_ext);

                   let (msg, block,_tx) = self.api().get_special_variables();
                   let _now = block.borrow().as_ref().unwrap().timestamp;

                    let mut zk__priv =
                       BTreeMap::from( [("secret0_plain_c_count",DataType::Int(0)), ("zk__in0_cipher_c_count_R", RandomnessValue::data_type("elgamal")), 
                        ("secret2_plain_b_count",DataType::Int(0)), ("zk__in1_cipher_b_count_R", RandomnessValue::data_type("elgamal")),
                        ("secret4_plain_a_count",DataType::Int(0)), ("zk__in2_cipher_a_count_R", RandomnessValue::data_type("elgamal")),
                    ]);
           let mut zk__data =
                            BTreeMap::from([("zk__out0_plain",DataType::Int(0)), ("zk__in0_cipher_c_count", CipherValue::data_type("elgamal")), 
                            ("zk__in1_cipher_b_count",CipherValue::data_type("elgamal")), ("zk__in2_cipher_a_count", CipherValue::data_type("elgamal"))]);

                    // BEGIN Simulate body
                     with_context_block!(var _sc=self._scope()=>{
                        assert!((zk__out_start_idx + 1) <= zk__out.len(),"require(zk__out_start_idx + 1 <= zk__out.length) failed");
                        assert!((zk__in_start_idx + 12) <= zk__in.len(),"require(zk__in_start_idx + 12 <= zk__in.length) failed");


                         assert!(msg.borrow().as_ref().unwrap().sender == self.state().borrow()[&["organizer"]],"require(me == organizer) failed");
                         assert!(self.min_votes_reached(),"require(min_votes_reached()) failed");
                         assert!(!self.is_result_published(),"require(!is_result_published()) failed");
                        // packed_results = reveal(((unhom(c_count)) << 128) | ((unhom(b_count)) << 64) | (unhom(a_count)), all);
                        // {
                        zk__data.insert("zk__in0_cipher_c_count",self.state().borrow()[&["c_count"]].clone());
                        let (secret0_plain_c_count, zk__in0_cipher_c_count_r) = self.api().dec(zk__data["zk__in0_cipher_c_count"].clone(), convert_type, "elgamal");
                        zk__priv.insert("secret0_plain_c_count",secret0_plain_c_count);
                        zk__priv.insert("zk__in0_cipher_c_count_R",zk__in0_cipher_c_count_r);

                        zk__data.insert("zk__in1_cipher_b_count", self.state().borrow()[&["b_count"]]);
                  
                        let (secret2_plain_b_count, zk__in1_cipher_b_count_r) = self.api().dec(zk__data["zk__in1_cipher_b_count"].clone(), convert_type, "elgamal");
                        zk__priv.insert("secret2_plain_b_count",secret0_plain_c_count);
                        zk__priv.insert("zk__in1_cipher_b_count_R",zk__in0_cipher_c_count_r);

                        zk__data.insert("zk__in2_cipher_a_count", self.state().borrow()[&["a_count"]]);
                       
                        let (secret4_plain_a_count, zk__in2_cipher_a_count_r) = self.api().dec(zk__data["zk__in2_cipher_a_count"].clone(), convert_type, "elgamal");
                        zk__priv.insert("secret4_plain_a_count",secret0_plain_c_count);
                        zk__priv.insert("zk__in2_cipher_a_count_R",zk__in2_cipher_a_count_r);

                        zk__data.insert("zk__out0_plain",DataType::Int( zk__priv["secret0_plain_c_count"] << 128 | zk__priv["secret2_plain_b_count"] << 64 | zk__priv["secret4_plain_a_count"]));

                        *self.state().borrow_mut()[&["packed_results"]] = zk__data["zk__out0_plain"].clone();
                        // }

                        // Serialize input values
                        // {
                        zk__in[zk__in_start_idx + 0..zk__in_start_idx + 4].clone_from_slice(&zk__data["zk__in0_cipher_c_count"].try_as_crypto_value_ref().unwrap()[..4]);
                        zk__in[zk__in_start_idx + 4..zk__in_start_idx + 8].clone_from_slice(&zk__data["zk__in1_cipher_b_count"].try_as_crypto_value_ref().unwrap()[..4]);
                        zk__in[zk__in_start_idx + 8..zk__in_start_idx + 12].clone_from_slice(&zk__data["zk__in2_cipher_a_count"].try_as_crypto_value_ref().unwrap()[..4]);
                        // }
                     });
                    // END Simulate body

                    // Serialize circuit outputs and/or secret circuit inputs
                    zk__out[zk__out_start_idx] = self.api().serialize_circuit_outputs(zk__data.into_iter().map(|(k,v)|(k.to_owned(),v.to_string())).collect(), vec![192]);
                    self.api().serialize_private_inputs(zk__priv, vec![32, 0, 32, 0, 32, 0]);
                    });
    }
    // _zk__publish_results._can_be_external = false

    fn check_if_agree_with_majority(&self) -> Vec<String> {
        with_context_block!(var _fc=self._function_ctx(2,0,"check_if_agree_with_majority") =>{
        let (zk__is_ext,_fc)=_fc;
                     with_context_block!(var _tm=time_measure("transaction_full", !zk__is_ext,false)=>{
                        assert! (zk__is_ext);
                        let (msg, block,_tx) = self.api().get_special_variables();
                        let _now = block.borrow().as_ref().unwrap().timestamp;

                        let mut zk__priv =
                             BTreeMap::from([("glob_sk_Ecdh_Chaskey__me", PublicKeyValue::data_type("ecdh-chaskey"))]);


                        let mut actual_params = vec![];
                        let mut zk__out = vec![DataType::Int(0);2];
                        actual_params.push(zk__out);

                        // BEGIN Simulate body
                         with_context_block!(var _sc=self._scope()=>{
                            zk__priv.insert("glob_sk_Ecdh_Chaskey__me",DataType::PrivateKeyValue(self.api().get_my_sk("ecdh-chaskey")));
                            assert!(zk__out.len() == 2,"require(zk__out.length == 2) failed");
                            let mut zk__in = vec![0.to_string();5];

                            // Request static public keys
                            // {
                            // let mut _tmp_key_Ecdh_Chaskey = PublicKeyValue::data_type("ecdh-chaskey");

                            let mut _tmp_key_Ecdh_Chaskey = self.api().get_keystore("ecdh-chaskey").getPk(msg.borrow().as_ref().unwrap().sender);
                            zk__in[0] = _tmp_key_Ecdh_Chaskey[0].clone();
                            // }

                            // Declare return variables
                            let mut zk__ret_0 = CipherValue::data_type("ecdh-chaskey");

                            // Call internal function
                            // zk__ret_0 =
                             self.api().call_fct(1, ||{self._zk__check_if_agree_with_majority( zk__in, 1, zk__out, 0);});
                         });
                        // END Simulate body

                        // Serialize circuit outputs and/or secret circuit inputs
                        self.api().serialize_private_inputs(zk__priv, vec![0]);

                        // Call pure/view function and return value
                        return self.api().call("check_if_agree_with_majority", actual_params, [(true, "ecdh-chaskey".to_owned(), convert_type)])

                    });
                });
    }
    // check_if_agree_with_majority._can_be_external = True

    fn _zk__check_if_agree_with_majority(
        &self,
        mut zk__in: Vec<String>,
        zk__in_start_idx: usize,
        mut zk__out: Vec<String>,
        zk__out_start_idx: usize,
    ) -> String {
        with_context_block!(var _fc=self._function_ctx(1,0,"?") =>{
        let (zk__is_ext,_fc)=_fc;
                    assert! (!zk__is_ext);

                   let (msg, block,_tx) = self.api().get_special_variables();
                   let _now = block.borrow().as_ref().unwrap().timestamp;

                    let zk__priv =
                         BTreeMap::from([("secret0_plain", DataType::Int(0))]);

        // Declare return variables
                        let mut zk__ret_0= CipherValue::data_type("ecdh-chaskey");
          let mut zk__data =
                           BTreeMap::from ([("zk__out0_cipher", CipherValue::data_type("ecdh-chaskey")), 
                        ("zk__in0_plain_c", DataType::Int(0)), ("zk__in1_cipher", CipherValue::data_type("ecdh-chaskey")), 
                    ("zk__in2_key_sender", PublicKeyValue::data_type("ecdh-chaskey"))]);


                    // BEGIN Simulate body
                     with_context_block!(var _sc=self._scope()=>{
                        assert!((zk__out_start_idx + 2) <= zk__out.len(),"require(zk__out_start_idx + 2 <= zk__out.length) failed");
                        assert!((zk__in_start_idx + 4) <= zk__in.len(),"require(zk__in_start_idx + 4 <= zk__in.length) failed");


                        self.locals().borrow_mut().decl("c", self.get_winning_choice());
                        // return (reveal(c, me) == current_votes[me]);
                        // {
                        zk__data.insert("zk__in0_plain_c",self.locals().borrow()["c"]);
                        zk__data.insert("zk__in1_cipher",self.state().borrow()[&["current_votes", &msg.borrow().as_ref().unwrap().sender]]);
                        zk__priv.insert("secret0_plain",self.api().dec(zk__data["zk__in1_cipher"], "Choice", "ecdh-chaskey").0);
                        zk__data.insert("zk__in2_key_sender",DataType::PublicKeyValue(Value::<String,PublicKeyValue>::new(vec![zk__data["zk__in1_cipher"].try_as_crypto_value_ref().unwrap()[2].clone()],None, "ecdh-chaskey")));
                        //msg.borrow().as_ref().unwrap().sender
                        let mut s=self.api().enc(DataType::Bool(zk__data["zk__in0_plain_c"] == zk__priv["secret0_plain"]),convert_type , "ecdh-chaskey").0;
                        s.pop();
                        s.push(self.api().get_my_pk("ecdh-chaskey")[0].clone());
                        zk__data.insert("zk__out0_cipher",DataType::CipherValue(Value::<String,CipherValue>::new(s, None,"ecdh-chaskey")));

                        zk__ret_0 = zk__data["zk__out0_cipher"].clone();
                        // }

                        // Serialize input values
                        // {
                        zk__in[zk__in_start_idx] = zk__data["zk__in0_plain_c"].try_as_int().unwrap().to_string();
                        zk__in[zk__in_start_idx + 1..zk__in_start_idx + 3].clone_from_slice(&zk__data["zk__in1_cipher"][..2]);
                        zk__in[zk__in_start_idx + 3] = zk__data["zk__in2_key_sender"].try_as_public_key_value_ref().unwrap()[0].clone();
                        // }
                     });
                    // END Simulate body

                    // Serialize circuit outputs and/or secret circuit inputs
                    zk__out[zk__out_start_idx..zk__out_start_idx + 2].clone_from_slice(&self.api().serialize_circuit_outputs(zk__data.into_iter().map(|(k,v)|(k.to_owned(),v.to_string())).collect(), vec![0]));
                    self.api().serialize_private_inputs(zk__priv.into_iter().map(|(k,v)|(k.to_owned(),v.to_string())).collect(), vec![256]);

                    return zk__ret_0
                        });
    }
    // _zk__check_if_agree_with_majority._can_be_external = false
}

fn deploy<
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone,
    B: ZkayBlockchainInterface<P> + Web3Blockchain + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
>(
    _min_votes: i32,
    user: &str,
    mut cs: ContractSimulator<C, P, B, K>,
) -> Survey<C, P, B, K> {
    let user = if user.is_empty() {
        "me".to_owned()
    } else {
        user.to_owned()
    };
    Survey::deploy(_min_votes, &user, "", cs)
}

fn connect<
    PS: ProvingScheme,
    C: ZkayCryptoInterface<P, B, K> + ZkayHomomorphicCryptoInterface<P, B, K> + Clone,
    P: ZkayProverInterface + Clone,
    B: ZkayBlockchainInterface<P> + Web3Blockchain + Clone,
    K: ZkayKeystoreInterface<P, B> + Clone,
>(
    address: &str,
    user: &str,
    mut cs: ContractSimulator<C, P, B, K>,
    compile_zkay_file: fn(
        input_file_path: &str,
        output_dir: &str,
        import_keys: bool,
    ) -> anyhow::Result<String>,
    get_verification_contract_names: fn(code_or_ast: String) -> Vec<String>,
) -> Survey<C, P, B, K> {
    let user = if user.is_empty() {
        "me".to_owned()
    } else {
        user.to_owned()
    };
    Survey::connect::<PS>(
        address,
        &user,
        "",
        cs,
        compile_zkay_file,
        get_verification_contract_names,
    )
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
fn convert_type(v: String) -> String {
    v
}
fn main0() {
    // contract_simulator.use_config_from_manifest(file!());
    // os.path.dirname(os.path.realpath(__file__);
    // let me = contract_simulator.default_address();
    // if me.is_some(){
    //     me = me.val;
    // }
    // import code
    // code.interact(local=globals())
}
