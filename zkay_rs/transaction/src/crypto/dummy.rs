// use typing::Tuple, List

// use zkay::compiler::privacy::library_contracts::bn128_scalar_field;
// use zkay::transaction::crypto::params::CryptoParams;
// use zkay::transaction::interface::{PrivateKeyValue, PublicKeyValue, KeyPair, RandomnessValue};
// use zkay::transaction::interface::ZkayCryptoInterface;


// class DummyCrypto(ZkayCryptoInterface):
//     params = CryptoParams('dummy')

//     def _generate_or_load_key_pair(self, address: str) -> KeyPair:
//         aint = int(address, 16)
//         return KeyPair(PublicKeyValue(self.serialize_pk(aint, self.params.key_bytes), params=self.params),
//                        PrivateKeyValue(aint))

//     def _enc(self, plain: int, _: int, target_pk: int):
//         cipher = (plain + target_pk) % bn128_scalar_field
//         return [cipher] * self.params.cipher_payload_len, list(RandomnessValue(params=self.params)[:])

//     def _dec(self, cipher: Tuple[int, ...], sk: int) -> Tuple[int, List[int]]:
//         plain = (cipher[0] - sk) % bn128_scalar_field
//         return plain, list(RandomnessValue(params=self.params)[:])
