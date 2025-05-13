

/**
 * The input to a homomorphic operation which can either be a ciphertext wire array or a plaintext wire.
 *
 * This class exists because some homomorphic operations require plaintext operands, such as when performing
 * multiplication on additively homomorphic ciphertexts encrypted with Paillier or Dummy-Hom, and having
 * arguments of this type is preferable to having dozens of overloads with different combinations of Vec<TypedWire>
 * and TypedWire or having to tell apart plaintext and ciphertext inputs from the length of the Vec<TypedWire> input.
 */
pub struct HomomorphicInput {

	 array:Vec<TypedWire>,
	 isCipher:bool,
}
impl HomomorphicInput{
	 pub fn new(array:Vec<TypedWire>, isCipher:bool )->Self {
		Self{array,
		 isCipher,
    }
	}

	pub  fn of(cipher:Vec<TypedWire>)->   HomomorphicInput {
		return HomomorphicInput::new(cipher, true);
	}

	pub  fn of(plain:TypedWire )->   HomomorphicInput {
		return HomomorphicInput::new(vec![TypedWire::default();] {plain}, false);
	}

	pub  fn isCipher()->  bool {
		return isCipher;
	}

	pub  fn isPlain()->  bool {
		return !isCipher;
	}

	pub  fn getCipher()->  Vec<TypedWire> {
		assert!(self.isCipher(),"Homomorphic input was not a ciphertext");
		return array;
	}

	pub  fn getPlain()->  TypedWire {
		assert!(!self.isCipher(),"Homomorphic input was not a plaintext");
		return array[0];
	}

	pub  fn getLength()->  i32 {
		return array.length;
	}

	pub  fn getName()->  String {
		return array[0].name;
	}
}
