// data.rs
#[repr(C)]
pub struct Fe25519(pub [u64; 5]);

#[repr(C)]
pub struct Ge25519Niels {
    pub y_plus_x: Fe25519,
    pub y_minus_x: Fe25519,
    pub xy2d: Fe25519,
}

// 貼上腳本生成的常量...
// pub const GE25519_BASE_MULTIPLES_NIELS: [Ge25519Niels; 32] = [ ... ];
