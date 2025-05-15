use circuit::operations::gadget;
use circuit::structure::wire;

pub struct PinocchioGadget {
    inputWires: Vec<Wire>,
    proverWitnessWires: Vec<Wire>,
    outputWires: Vec<Wire>,
}
impl PinocchioGadget {
    pub fn new(inputWires: Vec<Wire>, pathToArithFile: String, desc: Vec<String>) -> Self {
        super(desc);
        self.inputWires = inputWires;
        buildCircuit(pathToArithFile);
    }
}
impl Gadget for PinocchioGadget {
    fn buildCircuit(path: String) {
        let proverWitnessWires = Vec::new();
        let outputWires = Vec::new();

        let mut wireMapping;
        let scanner = Scanner::new(File::new(path));

        if !scanner.next().equals("total") {
            scanner.close();
            panic!("Expected total %d in the first line");
        }
        let numWires = scanner.nextInt();
        scanner.nextLine();
        wireMapping = vec![Wire::default(); numWires];

        let inputCount = 0;
        while (scanner.hasNext()) {
            let line = scanner.nextLine();
            // remove comments
            if line.contains("#") {
                line = line.substring(0, line.indexOf("#"));
            }
            if line.equals("") {
                continue;
            } else if line.startsWith("input") {
                let tokens = line.split("\\s+");
                let wireIndex = Integer.parseInt(tokens[1]);
                if wireMapping[wireIndex] != null {
                    throwParsingError(scanner, "Wire assigned twice! " + wireIndex);
                }
                if inputCount < inputWires.length {
                    wireMapping[wireIndex] = inputWires[inputCount];
                } else {
                    // the last input wire is assumed to be the one wire
                    wireMapping[wireIndex] = generator.getOneWire();
                }
                inputCount += 1;
            } else if line.startsWith("output") {
                let tokens = line.split("\\s+");
                let wireIndex = Integer.parseInt(tokens[1]);
                outputWires.add(wireMapping[wireIndex]);
            } else if line.startsWith("nizk") {
                let tokens = line.split("\\s+");
                let wireIndex = Integer.parseInt(tokens[1]);
                if wireMapping[wireIndex] != null {
                    throwParsingError(scanner, "Wire assigned twice! " + wireIndex);
                }
                let w = generator.createProverWitnessWire();
                proverWitnessWires.add(w);
                wireMapping[wireIndex] = w;
            } else {
                let ins = getInputs(line);
                for i in ins {
                    if wireMapping[i] == null {
                        throwParsingError(
                            scanner,
                            "Undefined input wire " + i + " at line " + line,
                        );
                    }
                }
                let outs = getOutputs(line);
                if line.startsWith("mul ") {
                    wireMapping[outs.get(0)] = wireMapping[ins.get(0)].mul(wireMapping[ins.get(1)]);
                } else if line.startsWith("add ") {
                    let result = wireMapping[ins.get(0)];
                    for i in 1..ins.size() {
                        result = result.add(wireMapping[ins.get(i)]);
                    }
                    wireMapping[outs.get(0)] = result;
                } else if line.startsWith("zerop ") {
                    wireMapping[outs.get(1)] = wireMapping[ins.get(0)].checkNonZero();
                } else if line.startsWith("split ") {
                    let bits = wireMapping[ins.get(0)].getBitWires(outs.size()).asArray();
                    for i in 0..outs.size() {
                        wireMapping[outs.get(i)] = bits[i];
                    }
                } else if line.startsWith("const-mul-neg-") {
                    let constantStr = line.substring("const-mul-neg-".length(), line.indexOf(" "));
                    let constant = BigInteger::new(constantStr, 16);
                    wireMapping[outs.get(0)] = wireMapping[ins.get(0)].mul(constant.negate());
                } else if line.startsWith("const-mul-") {
                    let constantStr = line.substring("const-mul-".length(), line.indexOf(" "));
                    let constant = BigInteger::new(constantStr, 16);
                    wireMapping[outs.get(0)] = wireMapping[ins.get(0)].mul(constant);
                } else {
                    throwParsingError(scanner, "Unsupport Circuit Line " + line);
                }
            }
        }

        scanner.close();

        self.proverWitnessWires = vec![Wire::default(); proverWitnessWires.size()];
        proverWitnessWires.toArray(self.proverWitnessWires);
        self.outputWires = vec![Wire::default(); outputWires.size()];
        outputWires.toArray(self.outputWires);
    }

    fn getOutputs(line: String) -> Vec<Integer> {
        let scanner =
            Scanner::new(line.substring(line.lastIndexOf("<") + 1, line.lastIndexOf(">")));
        let outs = Vec::new()();
        while (scanner.hasNextInt()) {
            let v = scanner.nextInt();
            outs.add(v);
        }
        scanner.close();
        return outs;
    }

    fn getInputs(line: String) -> Vec<Integer> {
        let scanner = Scanner::new(line.substring(line.indexOf("<") + 1, line.indexOf(">")));
        let ins = Vec::new()();
        while (scanner.hasNextInt()) {
            ins.add(scanner.nextInt());
        }
        scanner.close();
        return ins;
    }

    pub fn getOutputWires() -> Vec<Wire> {
        return outputWires;
    }

    pub fn getProverWitnessWires() -> Vec<Wire> {
        return proverWitnessWires;
    }

    fn throwParsingError(s: Scanner, m: String) {
        s.close();
        panic!(m);
    }
}
