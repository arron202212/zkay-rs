// use random::Random
// use typing::Tuple, List, Union

// use Crypto::Math::Primality::generate_probable_prime

// use zkay::compiler::privacy::library_contracts::bn128_scalar_field;
// use zkay::transaction::crypto::params::CryptoParams;
// use zkay::transaction::interface::{PrivateKeyValue, PublicKeyValue, KeyPair, RandomnessValue, \
//     ZkayHomomorphicCryptoInterface};
// use zkay::transaction::types::CipherValue;


// class DummyHomCrypto(ZkayHomomorphicCryptoInterface):
//     params = CryptoParams('dummy-hom')

//     def _generate_or_load_key_pair(self, address: str) -> KeyPair:
//         seed = int(address, 16)
//         rng = Random(seed)
//         def rand_bytes(n: int) -> bytes:
//             return bytes([rng.randrange(256) for _ in range(n)])

//         pk = int(generate_probable_prime(exact_bits=self.params.key_bits, randfunc=rand_bytes))
//         return KeyPair(PublicKeyValue(self.serialize_pk(pk, self.params.key_bytes), params=self.params),
//                        PrivateKeyValue(pk))

//     def _enc(self, plain: int, _: int, target_pk: int):
//         plain = plain % bn128_scalar_field  # handle negative values
//         cipher = (plain * target_pk + 1) % bn128_scalar_field
//         return [cipher], list(RandomnessValue(params=self.params)[:])

//     def _dec(self, cipher: Tuple[int, ...], sk: int) -> Tuple[int, List[int]]:
//         key_inv = pow(sk, -1, bn128_scalar_field)
//         plain = ((cipher[0] - 1) * key_inv) % bn128_scalar_field
//         if plain > bn128_scalar_field // 2:
//             plain = plain - bn128_scalar_field
//         return plain, list(RandomnessValue(params=self.params)[:])

//     def do_op(self, op: str, public_key: Union[List[int], int], *args: Union[CipherValue, int]) -> List[int]:
//         def deserialize(operand: Union[CipherValue, int]) -> int:
//             if isinstance(operand, CipherValue):
//                 val = operand[0]
//                 return val - 1 if val != 0 else 0
//             else:
//                 return operand

//         operands = [deserialize(arg) for arg in args]
//         if op == 'sign-':
//             result = -operands[0]
//         elif op == '+':
//             result = operands[0] + operands[1]
//         elif op == '-':
//             result = operands[0] - operands[1]
//         elif op == '*':
//             result = operands[0] * operands[1]
//         else:
//             raise ValueError(f'Unsupported operation {op}')
//         return [(result + 1) % bn128_scalar_field]

//     def do_rerand(self, arg: CipherValue, public_key: List[int]) -> Tuple[List[int], List[int]]:
//         return arg, [0]
