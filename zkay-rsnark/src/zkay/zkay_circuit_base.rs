use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::operations::gadget;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use zkay::crypto::crypto_backend;
use zkay::crypto::homomorphic_backend;

use zkay::zkay_type::check_type;
use zkay::zkay_type::zk_bool;
use zkay::zkay_type::zk_uint;

pub struct ZkayCircuitBase {
    /**
     * Whether to include comments for the more complex operations in the circuit.arith file
     */
    ADD_OP_LABELS: bool,
    LEGACY_CRYPTO_BACKEND: Object,

    realCircuitName: String,

    cryptoBackends: Map<Object, CryptoBackend>,

    currentPubInIdx: i32,
    currentPubOutIdx: i32,
    allPubIOWires: Vec<Option<WireType>>,

    currentPrivInIdx: i32,
    allPrivInWires: Vec<Option<WireType>>,

    pubInNames: List<String>,
    pubOutNames: List<String>,
    privInNames: List<String>,

    pubInCount: i32,
    useInputHashing: bool,

    vars: Map<String, Vec<TypedWire>>,

    currentGuardCondition: VecDeque<TypedWire>,
    serializedArguments: Vec<BigInteger>,

    namePrefixIndices: HashMap<String, Integer>,
    namePrefix: VecDeque<String>,

    guardPrefixes: VecDeque<VecDeque<String>>,
    guardPrefixIndices: VecDeque<HashMap<String, Integer>>,
}
impl ZkayCircuitBase {
    pub fn new(
        name: String,
        cryptoBackend: String,
        keyBits: i32,
        pubInSize: i32,
        pubOutSize: i32,
        privSize: i32,
        useInputHashing: bool,
    ) {
        this(name, pubInSize, pubOutSize, privSize, useInputHashing);

        // Legacy handling: add default "main" crypto backend
        self.cryptoBackends.put(
            LEGACY_CRYPTO_BACKEND,
            CryptoBackend.create(cryptoBackend, keyBits),
        );
    }

    pub fn new(
        name: String,
        pubInSize: i32,
        pubOutSize: i32,
        privSize: i32,
        useInputHashing: bool,
    ) {
        super("circuit");
        self.realCircuitName = name;

        self.pubInCount = pubInSize;
        self.currentPubOutIdx = pubInSize;
        self.allPubIOWires = vec![None; pubInSize + pubOutSize];
        self.allPrivInWires = vec![None; privSize];

        self.useInputHashing = useInputHashing;

        clearPrefix(self.namePrefix, self.namePrefixIndices);
        pushGuardPrefix(self.guardPrefixes, self.guardPrefixIndices);
    }

    pub fn run(args: Vec<String>) {
        match args[0] {
            "compile" => compileCircuit(),

            "prove" => {
                compileCircuit();
                parseInputs(Arrays.asList(args).subList(1, args.len()));
                println!("Evaluating circuit '" + realCircuitName + "'");
                evalCircuit();
            }

            _ => panic!("invalid command"),
        }
        prepFiles();
    }

    fn parseInputs(args: Vec<String>) {
        let totCount = allPubIOWires.len() + allPrivInWires.len();
        assert!(
            args.size() == totCount,
            "Input count mismatch, expected " + totCount + ", was " + args.size()
        );
        let mut serializedArguments = vec![BigInteger::default(); totCount];
        for i in 0..totCount {
            let v = BigInteger::new(args.get(i), 16);
            assert!(
                v.sign() != Sign::Minus,
                "No signed inputs (signed must be converted to unsigned beforehand)"
            );
            serializedArguments[i] = v;
        }
    }
}
impl CircuitGenerator for ZkayCircuitBase {
    fn compileCircuit() {
        println!("Compiling circuit '{realCircuitName}'");
        generateCircuit();
        assert!(
            currentPubInIdx == pubInCount && currentPubOutIdx == allPubIOWires.len(),
            "Not all pub  inputs assigned"
        );
        assert!(
            currentPrivInIdx == allPrivInWires.len(),
            "Not all  inputs assigned"
        );
        if useInputHashing {
            makeOutputArray(
                ZkaySHA256Gadget::new(allPubIOWires, 253).getOutputWires(),
                "digest",
            );
        }
        println!("Done with generateCircuit, preparing dummy files...");
    }

    fn buildCircuit() {
        // Create IO wires
        let pubOutCount = allPubIOWires.len() - pubInCount;
        let (mut inArray, mut outArray, mut privInArray);
        if useInputHashing {
            inArray = createProverWitnessWireArray(pubInCount, "in_");
            outArray = createProverWitnessWireArray(pubOutCount, "out_");
        } else {
            inArray = createInputWireArray(pubInCount, "in_");
            outArray = createInputWireArray(pubOutCount, "out_");
        }
        privInArray = createProverWitnessWireArray(allPrivInWires.len(), "priv_");

        // Legacy handling
        let legacyCryptoBackend = cryptoBackends.get(LEGACY_CRYPTO_BACKEND);
        if legacyCryptoBackend != None && legacyCryptoBackend.isSymmetric() {
            let myPk = inArray[0];
            let mySk = privInArray[0];
            setKeyPair(LEGACY_CRYPTO_BACKEND, myPk, mySk);
        }

        System.arraycopy(inArray, 0, allPubIOWires, 0, pubInCount);
        System.arraycopy(outArray, 0, allPubIOWires, pubInCount, pubOutCount);
        System.arraycopy(privInArray, 0, allPrivInWires, 0, allPrivInWires.len());
    }

    fn addIO(
        typeName: String,
        name: String,
        nameList: List<String>,
        src: Vec<Option<WireType>>,
        startIdx: i32,
        size: i32,
        t: ZkayType,
        restrict: bool,
    ) -> Vec<Option<WireType>> {
        name = getQualifiedName(name);
        println!(
            "Adding '{name}' = {typeName}[{startIdx}:{}]",
            startIdx + size
        );
        let input = src[startIdx..startIdx + size].to_vec();
        let tInput = vec![TypedWire::default(); input.len()];
        for i in 0..input.len() {
            // Enforce size and associate wire with type (technically restrict is only required for  inputs)
            tInput[i] = TypedWire::new(input[i], t, name, restrict);
        }
        vars.put(name, tInput);
        nameList.add(name);
        return input;
    }

    /* CRYPTO BACKENDS */
    fn addCryptoBackend(cryptoBackendId: Object, cryptoBackendName: String, keyBits: i32) {
        assert!(
            !self.cryptoBackends.containsKey(cryptoBackendId),
            "Crypto backend " + cryptoBackendId + " already registered"
        );

        self.cryptoBackends.put(
            cryptoBackendId,
            CryptoBackend.create(cryptoBackendName, keyBits),
        );
    }

    fn setKeyPair(cryptoBackendId: Object, pkName: String, skName: String) {
        setKeyPair(cryptoBackendId, get(pkName).wire, get(skName).wire);
    }

    fn setKeyPair(cryptoBackendId: Object, myPk: WireType, mySk: WireType) {
        let cryptoBackend = getCryptoBackend(cryptoBackendId);
        assert!(
            cryptoBackend.isSymmetric(),
            "Crypto backend is not symmetric"
        );

        let symmetricCrypto = cryptoBackend;
        symmetricCrypto.setKeyPair(myPk, mySk);
    }

    fn getCryptoBackend(cryptoBackendId: Object) -> CryptoBackend {
        let backend = cryptoBackends.get(cryptoBackendId);
        assert!(
            backend.is_some(),
            "Unknown crypto backend: " + cryptoBackendId
        );
        return backend;
    }

    fn getHomomorphicCryptoBackend(cryptoBackendId: Object) -> HomomorphicBackend {
        let cryptoBackend = getCryptoBackend(cryptoBackendId);
        if cryptoBackend.instance_of(HomomorphicBackend) {
            return cryptoBackend;
        } else {
            panic!("Crypto backend {cryptoBackendId} is not homomorphic");
        }
    }

    /* CIRCUIT IO */

    fn addIn(name: String, size: i32, t: ZkayType) {
        addIO(
            "in",
            name,
            pubInNames,
            allPubIOWires,
            currentPubInIdx,
            size,
            t,
            false,
        );
        currentPubInIdx += size;
    }

    fn addK(cryptoBackendId: Object, name: String, size: i32) {
        let cryptoBackend = getCryptoBackend(cryptoBackendId);
        let chunkSize = cryptoBackend.getKeyChunkSize();
        let input = addIO(
            "in",
            name,
            pubInNames,
            allPubIOWires,
            currentPubInIdx,
            size,
            ZkUint(chunkSize),
            false,
        );
        currentPubInIdx += size;
        cryptoBackend.addKey(getQualifiedName(name), input);
    }

    fn addK(name: String, size: i32) {
        addK(LEGACY_CRYPTO_BACKEND, name, size);
    }

    fn addOut(name: String, size: i32, t: ZkayType) {
        addIO(
            "out",
            name,
            pubOutNames,
            allPubIOWires,
            currentPubOutIdx,
            size,
            t,
            false,
        );
        currentPubOutIdx += size;
    }

    fn addS(name: String, size: i32, t: ZkayType) {
        addIO(
            "priv",
            name,
            privInNames,
            allPrivInWires,
            currentPrivInIdx,
            size,
            t,
            true,
        );
        currentPrivInIdx += size;
    }

    /* CONTROL FLOW */

    fn stepIn(fct: String) {
        pushPrefix(
            namePrefix,
            namePrefixIndices,
            guardPrefixes.element().element() + fct,
        );
        pushGuardPrefix(guardPrefixes, guardPrefixIndices);
    }

    fn stepOut() {
        popPrefix(namePrefix);
        guardPrefixes.pop();
        guardPrefixIndices.pop();
    }

    fn addGuard(name: String, isTrue: bool) {
        let newWire = get(name).wire;

        pushPrefix(
            guardPrefixes.element(),
            guardPrefixIndices.element(),
            name + "_" + isTrue,
        );

        if !isTrue {
            newWire = newWire.invAsBit();
        }

        if !currentGuardCondition.isEmpty() {
            newWire = currentGuardCondition.element().wire.and(newWire);
        }
        currentGuardCondition.push(TypedWire::new(
            newWire,
            ZkBool,
            "guard_cond_top_" + name + "_" + isTrue,
        ));
    }

    fn popGuard() {
        currentGuardCondition.pop();
        popPrefix(guardPrefixes.element());
    }

    fn ite(condition: TypedWire, trueVal: TypedWire, falseVal: TypedWire) -> TypedWire {
        checkType(ZkBool, condition.zkay_type);
        checkType(trueVal.zkay_type, falseVal.zkay_type);
        return TypedWire::new(
            condExpr(condition.wire, trueVal.wire, falseVal.wire),
            trueVal.zkay_type,
            format!(
                "if %s  {%s}  {%s}",
                condition.name, trueVal.name, falseVal.name
            ),
        );
    }

    /* UNARY OPS */

    pub fn negate(val: TypedWire) -> TypedWire {
        let bits = val.zkay_type.bitwidth;
        if bits < 256 {
            // Take two's complement
            let invBits = TypedWire::new(
                val.wire.invBits(val.zkay_type.bitwidth),
                val.zkay_type,
                "~" + val.name,
            );
            return invBits.plus((getActiveCircuitGenerator()).val(1, val.zkay_type));
        } else {
            return TypedWire::new(
                val.wire.mul(-1, "-" + val.name),
                val.zkay_type,
                "-" + val.name,
            );
        }
    }

    pub fn bitInv(val: TypedWire) -> TypedWire {
        let resultType = checkType(val.zkay_type, val.zkay_type, false);
        let res = val.wire.invBits(resultType.bitwidth, "~" + val.name);
        return TypedWire::new(res, resultType, "~" + val.name);
    }

    pub fn not(val: TypedWire) -> TypedWire {
        checkType(ZkBool, val.zkay_type);
        return TypedWire::new(val.wire.invAsBit("!" + val.name), ZkBool, "!" + val.name);
    }

    /* String op interface */

    pub fn o_(op: char, wire: TypedWire) -> TypedWire {
        match op {
            '-' => negate(wire),
            '~' => bitInv(wire),
            '!' => not(wire),
            _ => panic!(),
        }
    }

    pub fn o_(lhs: TypedWire, op: char, rhs: TypedWire) -> TypedWire {
        match op {
            '+' => lhs.plus(rhs),
            '-' => lhs.minus(rhs),
            '*' => lhs.times(rhs),
            '/' => lhs.divideBy(rhs),
            '%' => lhs.modulo(rhs),
            '|' => lhs.bitOr(rhs),
            '&' => lhs.bitAnd(rhs),
            '^' => lhs.bitXor(rhs),
            '<' => lhs.isLessThan(rhs),
            '>' => lhs.isGreaterThan(rhs),
            _ => panic!(),
        }
    }

    pub fn o_(
        cond: TypedWire,
        condChar: char,
        trueVal: TypedWire,
        altChar: char,
        falseVal: TypedWire,
    ) -> TypedWire {
        assert!(condChar == '?' && altChar == ':');
        return ite(cond, trueVal, falseVal);
    }

    pub fn o_(lhs: TypedWire, op: String, rhs: i32) -> TypedWire {
        match op {
            "<<" => lhs.shiftLeftBy(rhs),
            ">>" => lhs.shiftRightBy(rhs),
            _ => panic!(),
        }
    }

    pub fn o_(lhs: TypedWire, op: String, rhs: TypedWire) -> TypedWire {
        match op {
            "==" => lhs.isEqualTo(rhs),
            "!=" => lhs.isNotEqualTo(rhs),
            "<=" => lhs.isLessThanOrEqual(rhs),
            ">=" => lhs.isGreaterThanOrEqual(rhs),
            "&&" => lhs.and(rhs),
            "||" => lhs.or(rhs),
            _ => panic!(),
        }
    }

    /* Homomorphic operations */

    pub fn o_hom(
        cryptoBackendId: String,
        key: String,
        op: char,
        arg: HomomorphicInput,
    ) -> Vec<TypedWire> {
        let backend = getHomomorphicCryptoBackend(cryptoBackendId);
        return backend.doHomomorphicOp(op, arg, getQualifiedName(key));
    }

    pub fn o_hom(
        cryptoBackendId: String,
        key: String,
        lhs: HomomorphicInput,
        op: char,
        rhs: HomomorphicInput,
    ) -> Vec<TypedWire> {
        let backend = getHomomorphicCryptoBackend(cryptoBackendId);
        return backend.doHomomorphicOp(lhs, op, rhs, getQualifiedName(key));
    }

    pub fn o_hom(
        cryptoBackendId: String,
        key: String,
        cond: HomomorphicInput,
        condChar: char,
        trueVal: HomomorphicInput,
        altChar: char,
        falseVal: HomomorphicInput,
    ) -> Vec<TypedWire> {
        assert!(condChar == '?' && altChar == ':');
        let backend = getHomomorphicCryptoBackend(cryptoBackendId);
        return backend.doHomomorphicCond(cond, trueVal, falseVal, getQualifiedName(key));
    }

    pub fn o_hom(
        cryptoBackendId: String,
        key: String,
        lhs: HomomorphicInput,
        op: String,
        rhs: HomomorphicInput,
    ) -> Vec<TypedWire> {
        let backend = getHomomorphicCryptoBackend(cryptoBackendId);
        return backend.doHomomorphicOp(lhs, op, rhs, getQualifiedName(key));
    }

    pub fn o_rerand(
        arg: Vec<TypedWire>,
        cryptoBackendId: String,
        key: String,
        randomness: TypedWire,
    ) -> Vec<TypedWire> {
        let backend = getHomomorphicCryptoBackend(cryptoBackendId);
        return backend.doHomomorphicRerand(arg, getQualifiedName(key), randomness);
    }

    /* TYPE CASTING */

    fn cast(w: TypedWire, targetType: ZkayType) -> TypedWire {
        return convertTo(w, targetType);
    }

    /* SOURCE */

    fn get(name: String) -> TypedWire {
        let w = getTypedArr(name);
        assert!(w.len() == 1, "Tried to treat array as a single wire");
        return w[0];
    }

    fn getCipher(name: String) -> Vec<TypedWire> {
        return getTypedArr(name);
    }

    pub fn val(val: bool) -> TypedWire {
        return TypedWire::new(
            if val { getOneWire() } else { getZeroWire() },
            ZkBool,
            "const_" + val,
        );
    }

    pub fn val(val: i32, t: ZkayType) -> TypedWire {
        let mut w;
        if val == 0 {
            w = getZeroWire();
        } else if val == 1 {
            w = getOneWire();
        } else {
            return val(String.valueOf(val), t);
        }
        return TypedWire::new(w, t, "const_" + val);
    }

    pub fn val(val: String, t: ZkayType) -> TypedWire {
        let v = BigInteger::new(val, 10);
        let w;
        if v.sign() == Sign::Minus {
            assert!(!t.signed, "Cannot store negative constant in unsigned wire");

            let vNeg = ZkayType.GetNegativeConstant(v.neg(), t.bitwidth);
            assert!(vNeg.sign() != Sign::Minus, "Constant is still negative");
            w = createConstantWire(vNeg, "const_" + v.toString(10));
        } else {
            w = createConstantWire(v, "const_" + v.toString(10));
        }
        return TypedWire::new(w, t, "const_" + v.toString(10));
    }

    /* SINK */

    fn decl(lhs: String, val: TypedWire) {
        assert!(val.zkay_type.is_some(), "Tried to use untyped wires");

        // Get old value and check type
        let mut oldVal;
        if vars.containsKey(lhs) {
            oldVal = get(lhs);
            checkType(oldVal.zkay_type, val.zkay_type);
        } else {
            oldVal = val(0, val.zkay_type);
        }

        // Only assign value if guard condition is met
        if currentGuardCondition.isEmpty() {
            set(lhs, TypedWire::new(val.wire, val.zkay_type, lhs));
        } else {
            set(
                lhs,
                TypedWire::new(
                    condExpr(currentGuardCondition.element().wire, val.wire, oldVal.wire),
                    val.zkay_type,
                    lhs,
                ),
            );
        }
    }

    fn decl(lhs: String, val: Vec<TypedWire>) {
        assert!(val.is_some() && !val.empty(), "val");
        assert!(val[0].zkay_type.is_some(), "Tried to use untyped wires");
        // Check that all types match; else this gets really strange
        for i in 0..val.len() - 1 {
            checkType(val[i].zkay_type, val[i + 1].zkay_type);
        }

        // Get old value and check type and length
        let mut oldVal;
        if vars.containsKey(lhs) {
            oldVal = getTypedArr(lhs);
            checkType(oldVal[0].zkay_type, val[0].zkay_type);
            assert!(
                val.len() == oldVal.len(),
                "WireType amounts differ - old ={}, new = {}",
                oldVal.len(),
                val.len()
            );
        } else {
            oldVal = vec![val(0, val[0].zkay_type); val.len()];
        }

        // Only assign value if guard condition is met
        let resVal = vec![TypedWire::default(); val.len()];
        let guard = currentGuardCondition.peek(); // Null if empty
        for i in 0..val.len() {
            if guard == null {
                resVal[i] = TypedWire::new(val[i].wire, val[i].zkay_type, lhs); // No guard, just rename
            } else {
                resVal[i] = TypedWire::new(
                    condExpr(guard.wire, val[i].wire, oldVal[i].wire),
                    val[i].zkay_type,
                    lhs,
                );
            }
        }
        set(lhs, resVal);
    }

    fn condExpr(cond: WireType, trueVal: WireType, falseVal: WireType) -> WireType {
        if ZkayUtil.ZKAY_RESTRICT_EVERYTHING {
            addBinaryAssertion(cond);
        }
        return cond
            .mul(trueVal, "ite_true")
            .add(cond.invAsBit().mul(falseVal, "ite_false"), "ite_res");
    }

    fn convertTo(w: TypedWire, targetType: ZkayType) -> TypedWire {
        let fromType = w.zkay_type;

        let fromBitWidth = fromType.bitwidth;
        let wasSigned = fromType.signed;
        let toBitWidth = targetType.bitwidth;

        let mut newWire;
        if fromBitWidth < toBitWidth {
            // Upcast -> sign/zero extend
            if !wasSigned && w.wire.getBitWiresIfExistAlready() == None {
                // If this wire was not yet split we can return it without splitting as an optimization
                // -> upcasts from an unsigned type (most common ) are for free this way
                newWire = w.wire;
            } else {
                let bitWires = w.wire.getBitWires(fromBitWidth);
                if wasSigned && toBitWidth == 256 {
                    // Special  -> sign extension not possible since not enough bits,
                    // want -1 to be field_prime - 1
                    let signBit = bitWires.get(fromBitWidth - 1);
                    newWire = signBit.mux(negate(w).wire.mul(-1), w.wire);
                } else {
                    let extendBit = if wasSigned {
                        bitWires.get(fromBitWidth - 1)
                    } else {
                        getZeroWire()
                    };
                    let newWs = vec![None; toBitWidth];
                    newWs[..fromBitWidth].clone_from_slice(&bitWires.asArray());
                    for i in fromBitWidth..toBitWidth {
                        newWs[i] = extendBit;
                    }
                    newWire = WireArray::new(newWs).packAsBitsi(toBitWidth);
                }
            }
        } else if fromBitWidth > toBitWidth {
            // Downcast -> only keep lower bits
            newWire = w
                .wire
                .getBitWires(fromBitWidth, "downcast1 " + w.name)
                .packAsBits(toBitWidth, "downcast2 " + w.name);
        } else {
            // Type stays the same -> no expensive bitwise operations necessary
            newWire = w.wire;
        }
        return TypedWire::new(newWire, targetType, format!("(%s) %s", targetType, w.name));
    }

    fn cryptoEnc(
        cryptoBackend: CryptoBackend,
        plain: String,
        key: String,
        rnd: String,
        isDec: bool,
    ) -> Vec<Option<WireType>> {
        assert!(
            !cryptoBackend.isSymmetric(),
            "Crypto backend is not asymmetric"
        );

        let desc = if ADD_OP_LABELS {
            format!(
                "enc%s(%s, %s, %s)",
                if isDec { "[dec]" } else { "" },
                getQualifiedName(plain),
                getQualifiedName(key),
                getQualifiedName(rnd)
            )
        } else {
            ""
        };
        let enc = cryptoBackend.createEncryptionGadget(
            get(plain),
            getQualifiedName(key),
            getArr(rnd),
            desc,
        );
        return enc.getOutputWires();
    }

    fn cryptoDec(
        cryptoBackend: CryptoBackend,
        cipher: String,
        pkey: String,
        skey: String,
        expPlain: String,
    ) -> WireType {
        let desc = if ADD_OP_LABELS {
            format!(
                "dec(%s, %s, %s)",
                getQualifiedName(cipher),
                getQualifiedName(pkey),
                getQualifiedName(skey)
            )
        } else {
            ""
        };
        let dec = cryptoBackend.createDecryptionGadget(
            get(expPlain),
            getArr(cipher),
            getQualifiedName(pkey),
            getArr(skey),
            desc,
        );
        return dec.getOutputWires()[0];
    }

    fn cryptoSymmEnc(
        cryptoBackend: CryptoBackend,
        plain: String,
        otherPk: String,
        ivCipher: String,
        isDec: bool,
    ) -> Vec<Option<WireType>> {
        assert!(
            cryptoBackend.isSymmetric(),
            "Crypto backend is not symmetric"
        );

        let desc = if ADD_OP_LABELS {
            format!("enc%s(%s, k, iv)", if isDec { "[dec]" } else { "" }, plain)
        } else {
            ""
        };
        let enc = cryptoBackend.createEncryptionGadget(
            get(plain),
            getQualifiedName(otherPk),
            getArr(ivCipher),
            desc,
        );
        return enc.getOutputWires();
    }

    fn addGuardedEncryptionAssertion(expectedCipher: String, computedCipher: Vec<Option<WireType>>) {
        let expCipher = getArr(expectedCipher);
        let compStr = if ADD_OP_LABELS {
            format!("%s == cipher", getQualifiedName(expectedCipher))
        } else {
            ""
        };
        addGuardedOneAssertion(
            isEqual(expCipher, expectedCipher, computedCipher, "cipher"),
            compStr,
        );
    }

    fn addGuardedNonZeroAssertion(value: Vec<Option<WireType>>, name: String) {
        addGuardedOneAssertion(
            isNonZero(value, name),
            format!("assert %s != 0", getQualifiedName(name)),
        );
    }

    /**
     * Asymmetric Encryption
     */
    fn checkEnc(
        cryptoBackendId: Object,
        plain: String,
        key: String,
        rnd: String,
        expectedCipher: String,
    ) {
        let cryptoBackend = getCryptoBackend(cryptoBackendId);

        // 1. Check that expected cipher != 0 (since 0 is reserved for default initialization)
        addGuardedNonZeroAssertion(getArr(expectedCipher), expectedCipher);

        // 2. Encrypt
        let computedCipher = cryptoEnc(cryptoBackend, plain, key, rnd, false);

        // 3. Check encryption == expected cipher
        addGuardedEncryptionAssertion(expectedCipher, computedCipher);
    }

    /**
     * Symmetric Encryption
     */
    fn checkSymmEnc(cryptoBackendId: Object, plain: String, otherPk: String, ivCipher: String) {
        let cryptoBackend = getCryptoBackend(cryptoBackendId);

        // 1. Check that expected cipher != 0 (since 0 is reserved for default initialization)
        addGuardedNonZeroAssertion(getArr(ivCipher), ivCipher);

        // 2. Encrypt
        let computedCipher = cryptoSymmEnc(cryptoBackend, plain, otherPk, ivCipher, false);

        // 3. Check encryption == expected cipher
        addGuardedEncryptionAssertion(ivCipher, computedCipher);
    }

    /**
     * Asymmetric Decryption
     */
    fn checkDec(cryptoBackendId: Object, plain: String, key: String, rnd: String, cipher: String) {
        let cryptoBackend = getCryptoBackend(cryptoBackendId);

        if cryptoBackend.usesDecryptionGadget() {
            // TODO we're misusing the randomness wire for the secret key, which is extremely ugly...
            let msgOk = cryptoDec(cryptoBackend, cipher, key, rnd, plain);

            let expCipher = getArr(cipher);
            let expCipherIsNonZero = isNonZero(expCipher, cipher); // "!= 0"
            let expCipherIsZero = expCipherIsNonZero.invAsBit(cipher + " == 0");
            let plainZero = isZero(getArr(plain), plain);

            // handle uninitialized ciphertext: cipher == 0 => plain == 0
            addGuardedOneAssertion(expCipherIsNonZero.or(plainZero));

            // else: cipher != 0 ==> msgOk == 1
            addGuardedOneAssertion(expCipherIsZero.or(msgOk));
        } else {
            // 1. Decrypt [dec(cipher, rnd, sk) -> enc(plain, rnd, pk)] (compute inverse op)
            let computedCipher = cryptoEnc(cryptoBackend, plain, key, rnd, true);

            let expCipher = getArr(cipher);
            let expCipherIsNonZero = isNonZero(expCipher, cipher); // "!= 0"
            let expCipherIsZero = expCipherIsNonZero.invAsBit(cipher + " == 0");
            let plainZero = isZero(getArr(plain), plain);
            let rndZero = isZero(getArr(rnd), rnd);

            // 2. Check that: expectedCipher == 0 => plain == 0 && rnd == 0
            addGuardedOneAssertion(expCipherIsNonZero.or(plainZero.and(rndZero)));

            // 3. Check that expectedCipher != 0 => expectedCipher == computedCipher
            addGuardedOneAssertion(expCipherIsZero.or(isEqual(
                expCipher,
                cipher,
                computedCipher,
                "cipher",
            )));
        }
    }

    /**
     * Symmetric Decryption
     */
    fn checkSymmDec(cryptoBackendId: Object, plain: String, otherPk: String, ivCipher: String) {
        let cryptoBackend = getCryptoBackend(cryptoBackendId);

        // 1. Decrypt [dec(cipher, rnd, sk) -> encSymm(plain, ecdh(mySk, otherPk), iv)] (compute inverse op)
        let computedCipher = cryptoSymmEnc(cryptoBackend, plain, otherPk, ivCipher, true);

        let expIvCipher = getArr(ivCipher);
        let expCipherNonZero = isNonZero(expIvCipher, ivCipher);
        let expCipherZero = expCipherNonZero.invAsBit(ivCipher + " == 0");
        let otherPkNonZero = get(otherPk).wire.checkNonZero(otherPk + "!= 0");
        let otherPkZero = otherPkNonZero.invAsBit(otherPk + " == 0");
        let plainZero = isZero(getArr(plain), plain);

        // Some of these checks are probably not necessary, as zkay should already enforce that
        // otherPk == 0 <=> expCipher == 0

        // 2. Check that: ivCipher == 0 => plain == 0 && otherPk == 0
        addGuardedOneAssertion(
            expCipherNonZero.or(plainZero.and(otherPkZero)),
            if ADD_OP_LABELS {
                format!("%s == 0 => %s == 0 && %s == 0", ivCipher, plain, otherPk)
            } else {
                ""
            },
        );

        // 3. Check that: otherPk == 0 => plain == 0 && ivCipher == 0
        addGuardedOneAssertion(
            otherPkNonZero.or(plainZero.and(expCipherZero)),
            if ADD_OP_LABELS {
                format!("%s == 0 => %s == 0 && %s == 0", otherPk, plain, ivCipher)
            } else {
                ""
            },
        );

        // 4. Check that: (ivCipher != 0 && otherPk != 0) => ivCipher == computedCipher
        let cipherZeroOrPkZero = expCipherZero.or(otherPkZero);
        addGuardedOneAssertion(
            cipherZeroOrPkZero.or(isEqual(expIvCipher, ivCipher, computedCipher, "cipher")),
            if ADD_OP_LABELS {
                format!(
                    "(%s != 0 && %s != 0) => %s == %s",
                    ivCipher, otherPk, ivCipher, "cipher"
                )
            } else {
                ""
            },
        );
    }

    // Legacy handling

    fn checkEnc(plain: String, key: String, rnd: String, expectedCipher: String) {
        checkEnc(LEGACY_CRYPTO_BACKEND, plain, key, rnd, expectedCipher);
    }

    fn checkEnc(plain: String, otherPk: String, ivCipher: String) {
        checkSymmEnc(LEGACY_CRYPTO_BACKEND, plain, otherPk, ivCipher);
    }

    fn checkDec(plain: String, key: String, rnd: String, expectedCipher: String) {
        checkDec(LEGACY_CRYPTO_BACKEND, plain, key, rnd, expectedCipher);
    }

    fn checkDec(plain: String, otherPk: String, ivCipher: String) {
        checkSymmDec(LEGACY_CRYPTO_BACKEND, plain, otherPk, ivCipher);
    }

    fn checkEq(lhs: String, rhs: String) {
        let (l, r) = (getArr(lhs), getArr(rhs));
        let len = l.len();
        assert!(r.len() == len, "Size mismatch for equality check");

        for i in 0..len {
            let compStr = if ADD_OP_LABELS {
                format!(
                    "%s[%d] == %s[%d]",
                    getQualifiedName(lhs),
                    i,
                    getQualifiedName(rhs),
                    i
                )
            } else {
                ""
            };
            addGuardedEqualityAssertion(l[i], r[i], compStr);
        }
    }

    fn isNonZero(value: Vec<Option<WireType>>, name: String) -> WireType {
        let res = value[0].checkNonZero(name + "[0] != 0");
        for i in 1..value.len() {
            res = res.add(
                value[i].checkNonZero(format!("%s[%d] != 0", name, i)),
                format!("or %s[%d] != 0", name, i),
            );
        }
        return res.checkNonZero(name + " != 0");
    }

    fn isZero(value: Vec<Option<WireType>>, name: String) -> WireType {
        return isNonZero(value, name).invAsBit(name + " == 0");
    }

    fn isEqual(wires1: Vec<Option<WireType>>, name1: String, wires2: Vec<Option<WireType>>, name2: String) -> WireType {
        assert!(wires1.length == wires2.length, "WireType array size mismatch");
        let res = getOneWire();
        for i in 0..wires1.length {
            res = res.and(
                wires1[i].isEqualTo(wires2[i], format!("%s[%d] == %s[%d]", name1, i, name2, i)),
            );
        }
        return res;
    }

    fn clearPrefix(prefix: VecDeque<String>, indices: HashMap<String, Integer>) {
        prefix.clear();
        prefix.push("");
        indices.clear();
    }

    fn pushPrefix(
        prefix: VecDeque<String>,
        prefixIndices: HashMap<String, Integer>,
        newStr: String,
    ) {
        let newPrefix = prefix.peek() + newStr + ".";
        let count = prefixIndices.getOrDefault(newPrefix, 0);
        prefixIndices.put(newPrefix, count + 1);
        prefix.push(newPrefix + count + ".");
    }

    fn pushGuardPrefix(
        guardPrefixes: VecDeque<VecDeque<String>>,
        guardPrefixIndices: VecDeque<HashMap<String, Integer>>,
    ) {
        let newPrefix = VecDeque::new();
        let newPrefixIndices = HashMap::new();
        clearPrefix(newPrefix, newPrefixIndices);
        guardPrefixes.push(newPrefix);
        guardPrefixIndices.push(newPrefixIndices);
    }

    fn popPrefix(prefix: VecDeque<String>) {
        prefix.pop();
    }

    fn getQualifiedName(name: String) -> String {
        if name.startsWith("glob_") {
            return name;
        } else {
            return namePrefix.element() + name;
        }
    }

    fn addGuardedEqualityAssertion(lhs: WireType, rhs: WireType, desc: &String) {
        if currentGuardCondition.isEmpty() {
            addEqualityAssertion(lhs, rhs, desc);
        } else {
            let eq = lhs.isEqualTo(rhs);
            addOneAssertion(currentGuardCondition.element().wire.invAsBit().or(eq), desc); // guard => lhs == rhs
        }
    }

    fn addGuardedOneAssertion(val: WireType, desc: &String) {
        if currentGuardCondition.isEmpty() {
            addOneAssertion(val, desc);
        } else {
            addOneAssertion(
                currentGuardCondition.element().wire.invAsBit().or(val),
                desc,
            ); // guard => val
        }
    }

    fn getTypedArr(name: String) -> Vec<TypedWire> {
        name = getQualifiedName(name);
        let w = vars.get(name);
        assert!(
            w.is_some(),
            "Variable " + name + " is not associated with a wire"
        );
        return w;
    }

    fn getArr(name: String) -> Vec<Option<WireType>> {
        let w = getTypedArr(name);
        let wa = vec![None; w.len()];
        for i in 0..w.len() {
            wa[i] = w[i].wire;
        }
        return wa;
    }

    fn set(name: String, val: TypedWire) {
        set(name, vec![val]);
    }

    fn set(name: String, val: Vec<TypedWire>) {
        name = getQualifiedName(name);
        assert!(val.is_some(), "Tried to set value " + name + " to null");
        let oldVal = vars.get(name);
        assert!(
            oldVal.is_none(),
            "SSA violation when trying to write to {name}"
        );
        vars.put(name, val);
    }

    pub fn generateSampleInput(evaluator: CircuitEvaluator) {
        assert!(
            serializedArguments.is_some(),
            "No inputs specified, this should not have been called"
        );

        assert!(
            serializedArguments.len() == allPubIOWires.len() + allPrivInWires.len(),
            "Invalid serialized argument count, expected {} was {}",
            allPubIOWires.len(),
            serializedArguments.len()
        );

        let idx = 0;
        for ioNameList in Arrays.asList(pubInNames, pubOutNames, privInNames) {
            for name in ioNameList {
                let wires = vars.get(name);
                let sb = StringBuilder::new("Setting '" + name + "' to [");
                for w in wires {
                    let val = serializedArguments[idx += 1];
                    evaluator.setWireValue(w.wire, val);
                    sb.append("wid ")
                        .append(w.wire.getWireId())
                        .append("=")
                        .append(val)
                        .append(", ");
                }
                sb.setLength(sb.len()() - 2);
                sb.append("]");
                println!(sb);
            }
        }

        assert!(
            idx == allPubIOWires.len() + allPrivInWires.len(),
            "Not all inputs consumed"
        );
    }

    pub fn prepFiles() {
        if serializedArguments != None {
            super.prepFiles();
        } else {
            writeCircuitFile();
            writeDummyInputFile();
        }
    }

    fn writeDummyInputFile() {
        let printWriter = File::create(getName() + ".in");
        write!(printWriter, "0 1");
        let allIOWires =
            vec![0; getInWires().size() + getOutWires().size() + getProverWitnessWires().size()];
        allIOWires.addAll(getInWires().subList(1, getInWires().size()));
        allIOWires.addAll(getOutWires());
        allIOWires.addAll(getProverWitnessWires());
        for w in allIOWires {
            write!(printWriter, w.getWireId() + " " + "0");
        }
    }
}
