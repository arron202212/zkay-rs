//::os
//::sys
// use abc::ABCMeta

// use Crypto.PublicKey::RSA

// use zkay::config::cfg;
// use zkay::transaction::interface::{PrivateKeyValue, PublicKeyValue, KeyPair, ZkayBlockchainInterface};
// use zkay::transaction::interface::ZkayCryptoInterface;


// # persistent_locals2 has been co-authored with Andrea Maffezzoli
// # http://code.activestate.com/recipes/577283-decorator-to-expose-local-variables-of-a-function-/
// # (hack to get local variables of called library function, used to extract randomness)
// class PersistentLocals(object):
//     def __init__(self, func):
//         self._locals = {}
//         self.func = func

//     def __call__(self, *args, **kwargs):
//         def tracer(frame, event, arg):
//             if event == 'return':
//                 self._locals = frame.f_locals.copy()

//         # tracer is activated on next call, return or exception
//         sys.setprofile(tracer)
//         try:
//             # trace the function call
//             res = self.func(*args, **kwargs)
//         finally:
//             # disable tracer and replace with old one
//             sys.setprofile(None)
//         return res

//     def clear_locals(self):
//         self._locals = {}

//     @property
//     def locals(self):
//         return self._locals


// class RSACrypto(ZkayCryptoInterface, metaclass=ABCMeta):
//     default_exponent = 65537 # == 0x10001

//     def _generate_or_load_key_pair(self, address: str) -> KeyPair:
//         key_file = os.path.join(cfg.data_dir, 'keys', f'rsa_{self.params.key_bits}_{address}.bin')
//         os.makedirs(os.path.dirname(key_file), exist_ok=True)
//         if not os.path.exists(key_file):
//             print(f'Key pair not found, generating new {self.params.key_bits} bit rsa key pair...')
//             key = RSA.generate(self.params.key_bits, e=self.default_exponent)
//             with open(key_file, 'wb') as f:
//                 f.write(key.export_key())
//             print('done')
//         else:
//             print(f'Key pair found, loading from file {key_file}')
//             with open(key_file, 'rb') as f:
//                 key = RSA.import_key(f.read())

//         modulus = key.publickey().n
//         return KeyPair(PublicKeyValue(self.serialize_pk(modulus, self.params.key_bytes), params=self.params),
//                        PrivateKeyValue(key))
