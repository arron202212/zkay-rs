#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// use typing::Optional, Collection, Any, Dict, Tuple, List, Union, Callable

use std::marker::PhantomData;
use std::path::PathBuf;
use zkay_ast::homomorphism::Homomorphism;
use zkay_config::{
    config::{zk_print_banner, CFG},
    config_user::UserConfig,
    zk_print,
};
use zkay_transaction_crypto_params::params::CryptoParams;
#[derive(Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Value<T: Clone + Default, V: Clone + Default> {
    pub contents: Vec<T>,
    pub value: V,
    pub params: Option<CryptoParams>,
    pub crypto_backend: Option<String>,
}
// class Value(tuple):
impl<T: Clone + Default, V: Clone + Default> Value<T, V> {
    pub fn new(
        contents: Vec<T>,
        params: Option<CryptoParams>,
        crypto_backend: Option<String>,
    ) -> Self {
        Self {
            contents,
            value: V::default(),
            params,
            crypto_backend,
        }
    }
    //     fn __str__(self):
    //         return f'{type(self).__name__}({super().__str__()})'

    //     fn __eq__(self, other):
    //         return isinstance(other, type(self)) and super().__eq__(other)

    //     fn __hash__(self):
    //         return self[:].__hash__()
    pub fn lens(&self) -> usize {
        0
    }
    //     @staticmethod
    pub fn unwrap_values(v: Self) -> Self {
        //         if isinstance(v, List):
        //             return list(map(Value.unwrap_values, v))
        //         elif isinstance(v, AddressValue):
        //             return v.val
        //         elif isinstance(v, Dict):
        //             return {key: Value.unwrap_values(vals) for key, vals in v.items()}
        //         else:
        //             return list(v[:]) if isinstance(v, Value) else v
        v
    }
    //     @staticmethod
    pub fn flatten(_v: Self) -> Vec<Self> {
        //         out = []
        //         for elem in v:
        //             if isinstance(elem, Collection):
        //                 out += Value.flatten(elem)
        //             else:
        //                 out.append(elem)
        //         return out
        vec![]
    }
    //     @staticmethod
    pub fn collection_to_string(_v: Self) -> String {
        //         if isinstance(v, List):
        //             return f"[{', '.join(map(Value.collection_to_string, v))}]"
        //         elif isinstance(v, Tuple):
        //             return f"({', '.join(map(Value.collection_to_string, v))})"
        //         elif isinstance(v, Dict):
        //             return f"{{{', '.join([f'{key}: {Value.collection_to_string(val)}' for key, val in v.items()])}}}"
        //         else:
        //             return str(v)
        String::new()
    }

    //     @staticmethod
}
pub trait ValueContent<T> {
    fn get_params(params: Option<CryptoParams>, crypto_backend: Option<String>) -> CryptoParams {
        // from zkay.config::cfg
        if let Some(params) = params {
            return params;
        }
        if let Some(crypto_backend) = crypto_backend {
            return CryptoParams::new(crypto_backend);
        }

        CryptoParams::new(
            CFG.lock()
                .unwrap()
                .get_crypto_params(&Homomorphism::non_homomorphic()),
        )
    }
    fn create_content(
        contents: Option<Vec<T>>,
        params: Option<CryptoParams>,
        crypto_backend: Option<String>,
    ) -> Vec<T>;
}

pub trait ParamLength {
    fn len(params: &CryptoParams) -> usize;
}

#[derive(Default, Clone, PartialEq)]
pub struct CipherValue;
// <T>{
// base_value:Value<T>,
// params:Option<CryptoParams>,
// value:Option<i32>,
// }
// class CipherValue(Value):
impl<T: Default + Clone + Copy, V: ParamLength + Clone + Default> ValueContent<T> for Value<T, V> {
    fn create_content(
        contents: Option<Vec<T>>,
        params: Option<CryptoParams>,
        crypto_backend: Option<String>,
    ) -> Vec<T> {
        let params = Self::get_params(params, crypto_backend);
        let mut content = vec![T::default(); V::len(&params)];
        if let Some(contents) = contents {
            content[..contents.len()].copy_from_slice(&contents[..]);
        }

        content
    }
}
impl ParamLength for CipherValue {
    fn len(params: &CryptoParams) -> usize {
        params.cipher_len() as _
    }
}
#[derive(Clone, Default)]
pub struct PrivateKeyValue;
#[derive(Clone, Default)]
pub struct PublicKeyValue;

impl ParamLength for PublicKeyValue {
    fn len(params: &CryptoParams) -> usize {
        params.key_len() as _
    }
}

use std::clone::Clone;
#[derive(Clone, Default)]
pub struct RandomnessValue;

impl ParamLength for RandomnessValue {
    fn len(params: &CryptoParams) -> usize {
        params.randomness_len().unwrap_or(0) as _
    }
}
type Callable = fn(&AddressValue) -> i32;

#[derive(Clone, Default, Ord, PartialEq, PartialOrd, Eq)]
pub struct AddressValue {
    get_balance: Option<Callable>,
}
impl AddressValue {
    //     get_balance: Optional[Callable[['AddressValue'], int]] = None

    // fn new(val: T) -> Self {
    // if isinstance(val, AddressValue):
    //     val = val.val
    // if not isinstance(val, bytes):
    //     if isinstance(val, str):
    //         val = int(val, 16)
    //     val = val.to_bytes(20, byteorder='big')
    // return super(AddressValue, cls).__new__(cls, [val])

    //
    // fn val(&self) -> T {
    //     self.base_value.contents[0].clone()
    // }

    fn transfer(&self, _amount: i32) {}

    fn send(&self, _amount: i32) -> bool {
        true
    }

    //
    fn balance(&self) -> i32 {
        self.get_balance.unwrap()(self)
    }
}
use std::fmt;
impl<T: Clone + Default, V: Clone + Default> std::fmt::Display for Value<T, V>
where
    Vec<T>: AsRef<[u8]>+std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.contents.clone())
    }
}
#[derive(Clone, Default)]
pub struct KeyPair {
    pub pk: String,
    pub sk: String,
}
// class KeyPair:
impl KeyPair {
    pub fn new(pk: String, sk: String) -> Self {
        Self { pk, sk }
    }
}
#[derive(Clone)]
pub struct MsgStruct {
    sender: String,
    value: i32,
}
impl MsgStruct {
   pub  fn new(sender: String, value: i32) -> Self {
        Self { sender, value }
    }

    //
    fn sender(&self) -> String {
        self.sender.clone()
    }

    //
    fn value(&self) -> i32 {
        self.value
    }
}
#[derive(Clone)]
pub struct BlockStruct {
    coinbase: String,
    difficulty: i32,
    gaslimit: i32,
    number: i32,
    timestamp: i32,
}
impl BlockStruct {
   pub  fn new(
        coinbase: String,
        difficulty: i32,
        gaslimit: i32,
        number: i32,
        timestamp: i32,
    ) -> Self {
        Self {
            coinbase,
            difficulty,
            gaslimit,
            number,
            timestamp,
        }
    }

    fn coinbase(&self) -> String {
        self.coinbase.clone()
    }

    fn difficulty(self) -> i32 {
        self.difficulty
    }

    fn gaslimit(self) -> i32 {
        self.gaslimit
    }

    fn number(&self) -> i32 {
        self.number
    }

    fn timestamp(&self) -> i32 {
        self.timestamp
    }
}
#[derive(Clone)]
pub struct TxStruct {
    gasprice: i32,
    origin: String,
}
impl TxStruct{
    pub fn new(gasprice: i32, origin: String) -> Self {
        Self { gasprice, origin }
    }

    pub fn gasprice(&self) -> i32 {
        self.gasprice
    }

    pub fn origin(&self) -> String {
        self.origin.clone()
    }
}
