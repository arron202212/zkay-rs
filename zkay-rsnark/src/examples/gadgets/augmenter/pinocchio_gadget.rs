

use circuit::operations::gadget;
use circuit::structure::wire;

pub struct PinocchioGadget extends Gadget {

	 Vec<Wire> inputWires;
	 Vec<Wire> proverWitnessWires;
	 Vec<Wire> outputWires;

	pub  PinocchioGadget(inputWires:Vec<Wire>, String pathToArithFile, desc:Vec<String>) {
		super(desc);
		self.inputWires = inputWires;
		try {
			buildCircuit(pathToArithFile);
		} catch (Exception e) {
			e.printStackTrace();
		}
	}

	  buildCircuit(String path) throws FileNotFoundException {

		ArrayList<Wire> proverWitnessWires = new ArrayList<Wire>();
		ArrayList<Wire> outputWires = new ArrayList<Wire>();

		Vec<Wire> wireMapping;
		Scanner scanner = Scanner::new(File::new(path));

		if !scanner.next().equals("total") {
			scanner.close();
			panic!("Expected total %d in the first line");
		}
		i32 numWires = scanner.nextInt();
		scanner.nextLine();
		wireMapping = vec![Wire::default();numWires];

		i32 inputCount = 0;
		while (scanner.hasNext()) {
			String line = scanner.nextLine();
			// remove comments
			if line.contains("#") {
				line = line.substring(0, line.indexOf("#"));
			}
			if line.equals("") {
				continue;
			} else if line.startsWith("input") {
				Vec<String> tokens = line.split("\\s+");
				i32 wireIndex = Integer.parseInt(tokens[1]);
				if wireMapping[wireIndex] != null {
					throwParsingError(scanner, "Wire assigned twice! " + wireIndex);
				}
				if inputCount < inputWires.length {
					wireMapping[wireIndex] = inputWires[inputCount];
				} else {
					// the last input wire is assumed to be the one wire
					wireMapping[wireIndex] = generator.getOneWire();
				}
				inputCount+=1;
			} else if line.startsWith("output") {
				Vec<String> tokens = line.split("\\s+");
				i32 wireIndex = Integer.parseInt(tokens[1]);
				outputWires.add(wireMapping[wireIndex]);
			} else if line.startsWith("nizk") {
				Vec<String> tokens = line.split("\\s+");
				i32 wireIndex = Integer.parseInt(tokens[1]);
				if wireMapping[wireIndex] != null {
					throwParsingError(scanner, "Wire assigned twice! " + wireIndex);
				}
				Wire w = generator.createProverWitnessWire();
				proverWitnessWires.add(w);
				wireMapping[wireIndex] = w;
			} else {
				ArrayList<Integer> ins = getInputs(line);
				for i in  ins {
					if wireMapping[i] == null {
						throwParsingError(scanner, "Undefined input wire " + i + " at line " + line);
					}
				}
				ArrayList<Integer> outs = getOutputs(line);
				if line.startsWith("mul ") {
					wireMapping[outs.get(0)] = wireMapping[ins.get(0)].mul(wireMapping[ins.get(1)]);
				} else if line.startsWith("add ") {
					Wire result = wireMapping[ins.get(0)];
					for i in 1..ins.size() {
						result = result.add(wireMapping[ins.get(i)]);
					}
					wireMapping[outs.get(0)] = result;
				} else if line.startsWith("zerop ") {
					wireMapping[outs.get(1)] = wireMapping[ins.get(0)].checkNonZero();
				} else if line.startsWith("split ") {
					Vec<Wire> bits = wireMapping[ins.get(0)].getBitWires(outs.size()).asArray();
					for i in 0..outs.size() {
						wireMapping[outs.get(i)] = bits[i];
					}
				} else if line.startsWith("const-mul-neg-") {
					String constantStr = line.substring("const-mul-neg-".length(), line.indexOf(" "));
					BigInteger constant = BigInteger::new(constantStr, 16);
					wireMapping[outs.get(0)] = wireMapping[ins.get(0)].mul(constant.negate());
				} else if line.startsWith("const-mul-") {
					String constantStr = line.substring("const-mul-".length(), line.indexOf(" "));
					BigInteger constant = BigInteger::new(constantStr, 16);
					wireMapping[outs.get(0)] = wireMapping[ins.get(0)].mul(constant);
				} else {
					throwParsingError(scanner, "Unsupport Circuit Line " + line);
				}

			}
		}

		scanner.close();

		self.proverWitnessWires = vec![Wire::default();proverWitnessWires.size()];
		proverWitnessWires.toArray(self.proverWitnessWires);
		self.outputWires = vec![Wire::default();outputWires.size()];
		outputWires.toArray(self.outputWires);
	}

	 ArrayList<Integer> getOutputs(String line) {
		Scanner scanner = Scanner::new(line.substring(line.lastIndexOf("<") + 1, line.lastIndexOf(">")));
		ArrayList<Integer> outs = new ArrayList<>();
		while (scanner.hasNextInt()) {
			i32 v = scanner.nextInt();
			outs.add(v);
		}
		scanner.close();
		return outs;
	}

	 ArrayList<Integer> getInputs(String line) {
		Scanner scanner = Scanner::new(line.substring(line.indexOf("<") + 1, line.indexOf(">")));
		ArrayList<Integer> ins = new ArrayList<>();
		while (scanner.hasNextInt()) {
			ins.add(scanner.nextInt());
		}
		scanner.close();
		return ins;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return outputWires;
	}

	pub  Vec<Wire> getProverWitnessWires() {
		return proverWitnessWires;
	}

	  throwParsingError(Scanner s, String m) {
		s.close();
		panic!(m);
	}
}
