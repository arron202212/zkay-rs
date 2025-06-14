/**
 * The input to a homomorphic operation which can either be a ciphertext wire array or a plaintext wire.
 *
 * This class exists because some homomorphic operations require plaintext operands, such as when performing
 * multiplication on additively homomorphic ciphertexts encrypted with Paillier or Dummy-Hom, and having
 * arguments of this type is preferable to having dozens of overloads with different combinations of Vec<TypedWire>
 * and TypedWire or having to tell apart plaintext and ciphertext inputs from the length of the Vec<TypedWire> input.
 */
pub struct HomomorphicInput {
    array: Vec<TypedWire>,
    isCipher: bool,
}
impl HomomorphicInput {
    pub fn new(array: Vec<TypedWire>, isCipher: bool) -> Self {
        Self { array, isCipher }
    }

    pub fn of(cipher: Vec<TypedWire>) -> HomomorphicInput {
        HomomorphicInput::new(cipher, true)
    }

    pub fn of(plain: TypedWire) -> HomomorphicInput {
        HomomorphicInput::new(vec![plain], false)
    }

    pub fn isCipher() -> bool {
        isCipher
    }

    pub fn isPlain() -> bool {
        !isCipher
    }

    pub fn getCipher() -> Vec<TypedWire> {
        assert!(self.isCipher(), "Homomorphic input was not a ciphertext");
        array
    }

    pub fn getPlain() -> TypedWire {
        assert!(!self.isCipher(), "Homomorphic input was not a plaintext");
        array[0]
    }

    pub fn getLength() -> i32 {
        array.len()
    }

    pub fn getName() -> String {
        array[0].name
    }
}
