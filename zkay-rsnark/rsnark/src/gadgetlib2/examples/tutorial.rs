// Tutorial and usage examples of the gadgetlib2 library and ppzkSNARK integration.
// This file is meant to be read top-down as a tutorial for gadget writing.

use crate::gadgetlib2::examples::simple_example::gen_r1cs_example_from_gadgetlib2_protoboard;
use crate::gadgetlib2::gadget::{
    AND_Gadget, DualWord_Gadget, Gadget, GadgetConfig, GadgetPtr, GadgetType, PackingMode,
    R1P_Gadget,
};
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;
// use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::examples::run_r1cs_ppzksnark;
use crate::gadgetlib2::constraint::PrintOptions;
use crate::gadgetlib2::pp::{FrConfig, initPublicParamsFromDefaultPp};
use crate::gadgetlib2::protoboard::Protoboard;
use crate::gadgetlib2::variable::{
    DualWord, FElem, FieldType, FlagVariable, FlagVariableArray, LinearCombination,
    MultiPackedWord, ProtoboardPtr, UnpackedWord, Variable, VariableArray, VariableArrayBase,
    VariableArrayConfig, VariableArrayType, sum,
};
use crate::relations::FieldTConfig;
use crate::relations::variable::{SubLinearCombinationConfig, SubVariableConfig};
use rccell::RcCell;

/*
    This test gives the first example of a construction of a constraint system. We use the terms
    'Constraint System' and 'Circuit' interchangeably rather loosely. It is easy to
    visualize a circuit with inputs and outputs. Each gate imposes some logic on the inputs and
    output wires. For instance, AND(inp1, inp2) will impose the 'constraint' (inp1 & inp2 = out)
    Thus, we can also think of this circuit as a system of constraints. Each gate is a mathematical
    constraint and each wire is a variable. In the AND example over a boolean field {0,1} we would
    write the constraint as (inp1 * inp2 == out). This constraint is 'satisfied' relative to an
    assignment if we assign values to {inp1, inp2, out} such that the constraint evaluates to TRUE.
    All following examples will be either field agnostic or of a specific form of prime fields:
    (1) Field agnostic case: In these examples we create high level circuits by using lower level
        circuits. This way we can ignore the specifics of a field and assume the lower level takes
        care of this. If we must explicitly write constraints in these circuits, they will always
        be very basic constraints which are defined over every field (e.g. x + y = 0).
    (2) All field specific examples in this library are for a prime characteristic field with the
        special form of 'quadratic rank 1 polynomials', or R1P. This is the only form used with the
        current implementation of SNARKs. The form for these constraints is
        (Linear Combination) * (Linear Combination) == (Linear Combination).
        The library has been designed to allow future addition of other characteristics/forms in
        the future by implementing only low level circuits for these forms.
*/

pub fn test_ProtoboardUsage<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
    Fr: FrConfig,
>() {
    // Initialize prime field parameters. This is always needed for R1P.
    initPublicParamsFromDefaultPp::<Fr>();
    // The protoboard is the 'memory manager' which holds all constraints (when creating the
    // verifying circuit) and variable assignments (when creating the proof witness). We specify
    // the type as R1P, this can be augmented in the future to allow for BOOLEAN or GF2_EXTENSION
    // fields in the future.
    let pb = Protoboard::create(FieldType::R1P, None);
    // We now create 3 input variables and one output
    let mut input =
        VariableArray::<VariableArrayBase>::new(3, "input".to_owned(), VariableArrayBase);
    let mut output = Variable::new("output");
    // We can now add some constraints. The string is for debugging purposes and can be a textual
    // description of the constraint
    pb.as_ref().unwrap().borrow_mut().addRank1Constraint(
        input[0].clone().into(),
        LinearCombination::from(5) + &input[2],
        output.clone().into(),
        "Constraint 1: input[0] * (5 + input[2]) == output".to_owned(),
    );
    // The second form addUnaryConstraint(LinearCombination) means (LinearCombination == 0).
    pb.as_ref().unwrap().borrow_mut().addUnaryConstraint(
        input[1].clone() - &output,
        "Constraint 2: input[1] - output == 0".to_owned(),
    );
    // Notice this could also have been written:
    // pb.addRank1Constraint(1, input[1] - input[2], 0, "");
    //
    // For fields with more general forms, once implemented, we could use
    // addGeneralConstraint(Polynomial1, Polynomial2, string) which translates to the constraint
    // (Polynomial1 == Polynomial2).  Example:
    // pb.addGeneralConstraint(input[0] * (3 + input[1]) * input[2], output + 5,
    //                          "input[0] * (3 + input[1]) * input[2] == output + 5");
    //
    // Now we can assign values to the variables and see if the constraints are satisfied.
    // Later, when we will run a SNARK (or any other proof system), the constraints will be
    // used by the verifier, and the assigned values will be used by the prover.
    // Notice the protoboard stores the assignment values.
    for i in 0..3 {
        *pb.as_ref().unwrap().borrow_mut().val(&input[i]) = 42.into();
    }
    *pb.as_ref().unwrap().borrow_mut().val(&output) = 42.into();
    assert!(
        !pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
    // The constraint system is not satisfied. Now let's try values which satisfy the two equations
    // above:
    *pb.as_ref().unwrap().borrow_mut().val(&input[0]) = 1.into();
    *pb.as_ref().unwrap().borrow_mut().val(&input[1]) = 42.into();
    *pb.as_ref().unwrap().borrow_mut().val(&output) = 42.into(); // input[1] - output == 0
    *pb.as_ref().unwrap().borrow_mut().val(&input[2]) = 37.into(); // 1 * (5 + 37) == 42
    assert!(
        pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
}

// /*
//     In the above example we explicitly wrote all constraints and assignments.

//     In this example we will construct a very simple gadget, one that implements a NAND gate. This
//     gadget is field-agnostic as it only uses lower level gadgets and the field elements '0' & '1'.

//     Gadgets are the framework which allow us to delegate construction of sophisticated circuitry
//     to lower levels. Each gadget can construct a constraint system or a witness or both, by
//     defining constraints and assignments as well as by utilizing sub-gadgets.
// */
#[derive(Clone, Default)]
pub struct NAND_Gadget {
    //Gadget

    // This is a convention we use to always create gadgets as if from a factory class. This will
    // be needed later for gadgets which have different implementations in different fields.
    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         inputs:&FlagVariableArray,
    //                         output:&FlagVariable);
    // generateConstraints() is the method which creates all constraints on the protoboard
    // pub fn  generateConstraints();
    // generateWitness() is the method which generates the witness by assigning a valid value to
    // each wire in the circuit (variable) and putting this on the protoboard
    // pub fn  generateWitness();

    // constructor is private in order to stick to the convention that gadgets are created using a
    // create() method. This may not make sense now, but when dealing with non-field agnostic
    // gadgets it is very convenient to have a factory pub struct with this convention.
    // Notice the protoboard. This can be thought of as a 'memory manager' which holds the circuit
    // as the constraints are being built, and the 'wire values' as the witness is being built
    // NAND_Gadget(pb:ProtoboardPtr, inputs:&FlagVariableArray, output:&FlagVariable);
    // init() does any non trivial work which we don't want in the constructor. This is where we
    // will 'wire' the sub-gadgets into the circuit. Each sub-gadget can be thought of as a
    // circuit gate with some specific functionality.
    // pub fn  init();
    // we want every gadget to be explicitly constructed
    // DISALLOW_COPY_AND_ASSIGN(NAND_Gadget);

    // This is an internal gadget. Once a gadget is created it can be used as a black box gate. We
    // will initialize this pointer to be an AND_Gadget in the init() method.
    andGadget_: GadgetPtr,
    // These are internal variables used by the class. They will always include the variables from
    // the constructor, but can include many more as well. Notice that almost always the variables
    // can be declared 'const', as these are local copies of formal variables, and do not change
    // over the span of the class' lifetime.
    inputs_: VariableArrayType,
    output_: FlagVariable,
    andResult_: FlagVariable,
}

// IMPLEMENTATION
impl NAND_Gadget {
    // Most constructors are trivial and only initialize and assert values.
    pub fn new(pb: ProtoboardPtr, inputs: FlagVariableArray, output: FlagVariable) -> Gadget<Self> {
        // : Gadget(pb), inputs_(inputs), output_(output), andResult_("andResult")
        Gadget::<Self>::new(
            pb,
            Self {
                andGadget_: RcCell::new(GadgetType::default()),
                inputs_: inputs.into(),
                output_: output.into(),
                andResult_: FlagVariable::new("andResult"),
            },
        )
    }

    // The create() method will usually look like this, for field-agnostic gadgets:
    pub fn create(pb: ProtoboardPtr, inputs: FlagVariableArray, output: FlagVariable) -> GadgetPtr {
        let mut pGadget = NAND_Gadget::new(pb, inputs, output);
        pGadget.init();
        RcCell::new(GadgetType::NAND(pGadget))
    }
}

impl GadgetConfig for Gadget<NAND_Gadget> {
    fn init(&mut self) {
        // we 'wire' the AND gate.
        self.t.andGadget_ = AND_Gadget::create_r1p(
            self.pb_.clone(),
            self.t.inputs_.clone(),
            self.t.andResult_.clone(),
        );
    }

    fn generateConstraints(&self) {
        // we will invoke the AND gate constraint generator
        self.t.andGadget_.borrow().generateConstraints();
        // and add our out negation constraint in order to make this a NAND gate
        self.addRank1Constraint(
            1.into(),
            LinearCombination::from(1) - &self.t.andResult_,
            self.t.output_.clone().into(),
            "1 * (1 - andResult) = output",
        );
        // Another way to write the same constraint is:
        // addUnaryConstraint(1 - andResult_ - output_, "1 - andResult == output");
        //
        // At first look, it would seem that this is enough. However, the AND_Gadget expects all of its
        // inputs to be boolean, a dishonest prover could put non-boolean inputs, so we must check this
        // here. Notice 'FlagVariable' means a variable which we intend to hold only '0' or '1', but
        // this is just a convention (it is a type for Variable) and we must enforce it.
        // Look into the internals of the R1P implementation of AND_Gadget and see that
        // {2, 1, 0} as inputs with {1} as output would satisfy all constraints, even though this is
        // clearly not our intent!
        for input in self.t.inputs_.iter() {
            self.enforceBooleanity(input); // This adds a constraint of the form: input * (1 - input) == 0
        }
    }

    fn generateWitness(&self) {
        // First we can assert that all input values are indeed boolean. The purpose of this assertion
        // is simply to print a clear error message, it is not security critical.
        // Notice the method val() which returns a reference to the current assignment for a variable
        for input in self.t.inputs_.iter() {
            assert!(
                self.val(&input) == 0 || self.val(&input) == 1,
                "NAND input is not boolean"
            );
        }
        // we will invoke the AND gate witness generator, this will set andResult_ correctly
        self.t.andGadget_.borrow().generateWitness();
        // and now we set the value of output_
        self.val_set_v(
            &self.t.output_,
            FElem::from(1) - &self.val(&self.t.andResult_),
        );
        // notice the use of 'val()' to tell the protoboard to assign this new value to the
        // variable 'output_'. The variable itself is only a formal variable and never changes.
    }
}

// // And now for a test which will exemplify the usage:
pub fn test_NAND_Gadget<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
    Fr: FrConfig,
>() {
    // initialize the field
    initPublicParamsFromDefaultPp::<Fr>();
    // create a protoboard for a system of rank 1 constraints over a prime field.
    let mut pb = Protoboard::create(FieldType::R1P, None);
    // create 5 variables inputs[0]...inputs[4]. The string "inputs" is used for debug messages
    let mut inputs = FlagVariableArray::new(5, "inputs".to_owned(), VariableArrayBase);
    let mut output = FlagVariable::new("output");
    let nandGadget = NAND_Gadget::create(pb.clone(), inputs.clone(), output.clone());
    // now we can generate a constraint system (or circuit)
    nandGadget.borrow().generateConstraints();
    // if we try to evaluate the circuit now, an exception will be thrown, because we will
    // be attempting to evaluate unassigned variables.
    assert!(
        pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
    // so let's assign the input variables for NAND and try again after creating the witness
    for input in inputs.iter() {
        *pb.as_ref().unwrap().borrow_mut().val(&input) = 1.into();
    }
    nandGadget.borrow().generateWitness();
    assert!(
        pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
    assert!(*pb.as_ref().unwrap().borrow_mut().val(&output) == 0);
    // now let's try to ruin something and see what happens
    *pb.as_ref().unwrap().borrow_mut().val(&inputs[2]) = 0.into();
    assert!(
        !pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
    // now let's try to cheat. If we hadn't enforced booleanity, this would have worked!
    *pb.as_ref().unwrap().borrow_mut().val(&inputs[1]) = 2.into();
    assert!(
        !pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
    // now let's reset inputs[1] to a valid value
    *pb.as_ref().unwrap().borrow_mut().val(&inputs[1]) = 1.into();
    // before, we set both the inputs and the output. Notice the output is still set to '0'
    assert!(*pb.as_ref().unwrap().borrow_mut().val(&output) == 0);
    // Now we will let the gadget compute the result using generateWitness() and see what happens
    nandGadget.borrow().generateWitness();
    assert!(*pb.as_ref().unwrap().borrow_mut().val(&output) == 1);
    assert!(
        pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
}

// /*
//     Another example showing the use of DualVariable. A DualVariable is a variable which holds both
//     a bitwise representation of a word and a packed representation (e.g. both the packed value {42}
//     and the unpacked value {1,0,1,0,1,0}). If the word is short enough
//     (for example any integer smaller than the prime characteristic) then the packed representation
//     will be stored in 1 field element. 'word' in this context means a set of bits, it is a
//     convention which means we expect some semantic ability to decompose the packed value into its
//     bits.
//     The use of DualVariables is for efficiency reasons. More on this at the end of this example.
//     In this example we will construct a gadget which receives as input a packed integer value
//     called 'hash', and a 'difficulty' level in bits, and constructs a circuit validating that the
//     first 'difficulty' bits of 'hash' are '0'. For simplicity we will assume 'hash' is always 64
//     bits long.
// */
#[derive(Clone, Default)]
pub struct HashDifficultyEnforcer_Gadget {
    //Gadget

    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         hashValue:MultiPackedWord&,
    //                         difficultyBits:usize);
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    hashSizeInBits_: usize,
    difficultyBits_: usize,
    hashValue_: DualWord,
    // This GadgetPtr will be a gadget to unpack hashValue_ from packed representation to bit
    // representation. Recall 'DualWord' holds both values, but only the packed version will be
    // received as input to the constructor.
    hashValueUnpacker_: GadgetPtr,
    // HashDifficultyEnforcer_Gadget(pb:ProtoboardPtr,
    //                               hashValue:MultiPackedWord&,
    //                               difficultyBits:usize);
    // pub fn  init();
    // DISALLOW_COPY_AND_ASSIGN(HashDifficultyEnforcer_Gadget);
}

// IMPLEMENTATION
impl HashDifficultyEnforcer_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        hashValue: VariableArray<MultiPackedWord>,
        difficultyBits: usize,
    ) -> Gadget<Self> {
        //  : Gadget(pb), hashSizeInBits_(64), difficultyBits_(difficultyBits),
        //       hashValue_(hashValue, UnpackedWord(64, "hashValue_u"))
        Gadget::<Self>::new(
            pb,
            Self {
                hashSizeInBits_: 64,
                difficultyBits_: difficultyBits,
                hashValue_: DualWord::new2(hashValue, UnpackedWord::new(64, "hashValue_u")),
                hashValueUnpacker_: RcCell::new(GadgetType::default()),
            },
        )
    }
    pub fn create(
        pb: ProtoboardPtr,
        hashValue: VariableArray<MultiPackedWord>,
        difficultyBits: usize,
    ) -> GadgetPtr {
        let mut pGadget = HashDifficultyEnforcer_Gadget::new(pb, hashValue, difficultyBits);
        pGadget.init();
        RcCell::new(GadgetType::HashDifficultyEnforcer(pGadget))
    }
}

impl GadgetConfig for Gadget<HashDifficultyEnforcer_Gadget> {
    fn init(&mut self) {
        // because we are using a prime field with large characteristic, we can assume a 64 bit value
        // fits in the first element of a multipacked variable.
        assert!(
            self.t.hashValue_.multipacked().len() == 1,
            "multipacked word size too large"
        );
        // A DualWord_Gadget's constraints assert that the unpacked and packed values represent the
        // same integer element. The generateWitness() method has two modes, one for packing (taking the
        // bit representation as input) and one for unpacking (creating the bit representation from
        // the packed representation)
        self.t.hashValueUnpacker_ = DualWord_Gadget::create(
            self.pb_.clone(),
            self.t.hashValue_.clone(),
            PackingMode::UNPACK,
        );
    }

    fn generateConstraints(&self) {
        // enforce that both representations are equal
        self.t.hashValueUnpacker_.borrow().generateConstraints();
        // add constraints asserting that the first 'difficultyBits' bits of 'hashValue' equal 0. Note
        // endianness, unpacked()[0] is LSB and unpacked()[63] is MSB
        for i in 0..self.t.difficultyBits_ {
            self.addUnaryConstraint(
                self.t.hashValue_.unpacked()[63 - i].clone().into(),
                &format!("hashValue[{}] == 0", 63 - i),
            );
        }
    }

    fn generateWitness(&self) {
        // Take the packed representation and unpack to bits.
        self.t.hashValueUnpacker_.borrow().generateWitness();
        // In a real setting we would add an assertion that the value will indeed satisfy the
        // difficulty constraint, and notify the user with an error otherwise. As this is a tutorial,
        // we'll let invalid values pass through so that we can see how isSatisfied() returns false.
    }
}
// // Remember we pointed out that DualVariables are used for efficiency reasons. Now is the time to
// // elaborate on this. As you've seen, we needed a bit representation in order to check the first
// // bits of hashValue. But hashValue may be used in many other places, for instance we may want to
// // check equality with another value. Checking equality on a packed representation will 'cost' us
// // 1 constraint, while checking equality on the unpacked value will 'cost' us 64 constraints. This
// // translates heavily into proof construction time and memory in the ppzkSNARK proof system.

pub fn TEST_HashDifficultyEnforcer_Gadget<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
    Fr: FrConfig,
>() {
    initPublicParamsFromDefaultPp::<Fr>();
    let mut pb = Protoboard::create(FieldType::R1P, None);
    let mut hashValue = MultiPackedWord::new(64, &FieldType::R1P, "hashValue");
    let difficulty = 10;
    let mut difficultyEnforcer =
        HashDifficultyEnforcer_Gadget::create(pb.clone(), hashValue.clone(), difficulty);
    difficultyEnforcer.borrow().generateConstraints();
    // constraints are created but no assignment yet. Will throw error on evaluation
    assert!(
        pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
    *pb.as_ref().unwrap().borrow_mut().val(&hashValue[0]) = 42.into();
    difficultyEnforcer.borrow().generateWitness();
    // First 10 bits of 42 (when represented as a 64 bit number) are '0' so this should work
    assert!(
        pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::DBG_PRINT_IF_NOT_SATISFIED)
    );
    *pb.as_ref().unwrap().borrow_mut().val(&hashValue[0]) = 1000000000000000000.into();
    // This is a value > 2^54 so we expect constraint system not to be satisfied.
    difficultyEnforcer.borrow().generateWitness(); // This would have failed had we put an assertion
    assert!(
        !pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
}

// /*
//     In this example we will construct a gadget which builds a circuit for proof (witness) and
//     validation (constraints) that a bitcoin transaction's sum of inputs equals the sum of
//     outputs + miners fee. Construction of the proof will include finding the miners'
//     fee. This fee can be thought of as an output of the circuit.

//     This is a field specific gadget, as we will use the '+' operator freely. The addition
//     operation works as expected over integers while in prime characteristic fields but not so in
//     extension fields. If you are not familiar with extension fields, don't worry about it. Simply
//     be aware that + and * behave differently in different fields and don't necessarily give the
//     integer values you would expect.

//     The library design supports multiple field constructs due to different applied use cases. Some
//     cryptographic applications may need extension fields while others may need prime fields,
//     but with constraints which are not rank-1, and yet others may need boolean circuits. The library
//     was designed so that high level gadgets can be reused by implementing only the low level for
//     a new field or constraint structure.

//     Later we will supply a recipe for creation of such field specific gadgets with agnostic
//     interfaces. We use a few conventions here in order to ease the process by using macros.
// */
// // This is a macro which creates an interface pub struct for all field specific derived gadgets
// // Convention is: pub struct {GadgetName}_GadgetBase
// CREATE_GADGET_BASE_CLASS(VerifyTransactionAmounts_GadgetBase);
pub trait VerifyTransactionAmounts_GadgetBase {}
impl VerifyTransactionAmounts_GadgetBase for R1P_VerifyTransactionAmounts_Gadget {}
impl R1P_Gadget for R1P_VerifyTransactionAmounts_Gadget {}

// // Notice the multiple inheritance. We must specify the interface as well as the field specific
// // base gadget. This is what allows the factory pub struct to decide at compile time which field
// // specific pub struct to instantiate for every protoboard. See design notes in "gadget.hpp"
// // Convention is: pub struct {FieldType}_{GadgetName}_Gadget
#[derive(Clone, Default)]
pub struct R1P_VerifyTransactionAmounts_Gadget {
    // pub fn  generateConstraints();
    // pub fn  generateWitness();

    // We give the factory pub struct friend access in order to instantiate via private constructor.
    // friend pub struct VerifyTransactionAmounts_Gadget;

    // R1P_VerifyTransactionAmounts_Gadget(pb:ProtoboardPtr,
    //                                     txInputAmounts:VariableArray&,
    //                                     txOutputAmounts:VariableArray&,
    //                                     minersFee:&Variable);
    // pub fn  init();
    txInputAmounts_: VariableArrayType,
    txOutputAmounts_: VariableArrayType,
    minersFee_: Variable,
    // DISALLOW_COPY_AND_ASSIGN(R1P_VerifyTransactionAmounts_Gadget);
}

// // create factory pub struct using CREATE_GADGET_FACTORY_CLASS_XX macro (substitute XX with the number
// // of arguments to the constructor, excluding the protoboard). Sometimes we want multiple
// // constructors, see AND_Gadget for example. In this case we will have to manually write the
// // factory class.
// CREATE_GADGET_FACTORY_CLASS_3(VerifyTransactionAmounts_Gadget,
//                               VariableArray, txInputAmounts,
//                               VariableArray, txOutputAmounts,
//                               Variable, minersFee);

#[derive(Clone, Default)]
pub struct VerifyTransactionAmounts_Gadget;
impl VerifyTransactionAmounts_Gadget {
    pub fn create(
        pb: ProtoboardPtr,
        txInputAmounts: VariableArrayType,
        txOutputAmounts: VariableArrayType,
        minersFee: Variable,
    ) -> GadgetPtr {
        assert!(
            pb.as_ref().unwrap().borrow().fieldType_ == FieldType::R1P,
            "Attempted to create gadget of undefined Protoboard type."
        );
        let mut pGadget = R1P_VerifyTransactionAmounts_Gadget::new(
            pb,
            txInputAmounts,
            txOutputAmounts,
            minersFee,
        );

        pGadget.init();
        RcCell::new(GadgetType::R1PVerifyTransactionAmounts(pGadget))
    }
}

// // IMPLEMENTATION

// // Destructor for the Base class
// VerifyTransactionAmounts_GadgetBase::~VerifyTransactionAmounts_GadgetBase() {}
impl R1P_VerifyTransactionAmounts_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        txInputAmounts: VariableArrayType,
        txOutputAmounts: VariableArrayType,
        minersFee: Variable,
    ) -> Gadget<Self> {
        // Notice we must initialize 3 base classes (diamond inheritance):
        // : Gadget(pb), VerifyTransactionAmounts_GadgetBase(pb), R1P_Gadget(pb),
        // txInputAmounts_(txInputAmounts), txOutputAmounts_(txOutputAmounts),
        // minersFee_(minersFee)
        Gadget::<Self>::new(
            pb,
            Self {
                txInputAmounts_: txInputAmounts,
                txOutputAmounts_: txOutputAmounts,
                minersFee_: minersFee,
            },
        )
    }
}
impl GadgetConfig for Gadget<R1P_VerifyTransactionAmounts_Gadget> {
    fn generateConstraints(&self) {
        self.addUnaryConstraint(
            sum(&self.t.txInputAmounts_) - &sum(&self.t.txOutputAmounts_) - &self.t.minersFee_,
            "sum(txInputAmounts) == sum(txOutputAmounts) + minersFee",
        );
        // It would seem this is enough, but an adversary could cause an overflow of one side of the
        // equation over the field modulus. In fact, for every input/output sum we will always find a
        // miners' fee which will satisfy this constraint!
        // It is left as an exercise for the reader to implement additional constraints (and witness)
        // to check that each of the amounts (inputs, outputs, fee) are between 0 and 21,000,000 * 1E8
        // satoshis. Combine this with a maximum amount of inputs/outputs to disallow field overflow.
        //
        // Hint: use Comparison_Gadget to create a gadget which compares a variable's assigned value
        // to a constant. Use a vector of these new gadgets to check each amount.
        // Don't forget to:
        // (1) Wire these gadgets in init()
        // (2) Invoke the gadgets' constraints in generateConstraints()
        // (3) Invoke the gadgets' witnesses in generateWitness()
    }

    fn generateWitness(&self) {
        let mut sumInputs = FElem::from(0);
        let mut sumOutputs = FElem::from(0);
        for inputAmount in self.t.txInputAmounts_.iter() {
            sumInputs += &self.val(&inputAmount);
        }
        for outputAmount in self.t.txOutputAmounts_.iter() {
            sumOutputs += &self.val(&outputAmount);
        }
        self.val_set_v(&self.t.minersFee_, sumInputs - &sumOutputs);
    }

    fn init(&mut self) {}
}

// /*
//     As promised, recipe for creating field specific gadgets with agnostic interfaces:

//     (1) Create the Base pub struct using macro:
//         CREATE_GADGET_BASE_CLASS({GadgetName}_GadgetBase);
//     (2) Create the destructor for the base class:
//         {GadgetName_Gadget}Base::~{GadgetName}_GadgetBase() {}
//     (3) Create any field specific gadgets with multiple inheritance:
//         pub struct {FieldType}_{GadgetName}_Gadget : public {GadgetName}_GadgetBase,
//                                                 public {FieldType_Gadget}
//         Notice all arguments to the constructors must be const& in order to use the factory class
//         macro. Constructor arguments must be the same for all field specific implementations.
//     (4) Give the factory pub struct {GadgetName}_Gadget public friend access to the field specific
//         classes.
//     (5) Create the factory pub struct using the macro:
//         CREATE_GADGET_FACTORY_CLASS_XX({GadgetName}_Gadget, type1, input1, type2, input2, ... ,
//                                                                                   typeXX, inputXX);
// */
pub fn TEST_R1P_VerifyTransactionAmounts_Gadget<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
    Fr: FrConfig,
>() {
    initPublicParamsFromDefaultPp::<Fr>();
    let mut pb = Protoboard::create(FieldType::R1P, None);
    let inputAmounts =
        VariableArray::<VariableArrayBase>::new(2, "inputAmounts".to_owned(), VariableArrayBase);
    let outputAmounts =
        VariableArray::<VariableArrayBase>::new(3, "outputAmounts".to_owned(), VariableArrayBase);
    let minersFee = Variable::new("minersFee");
    let mut verifyTx = VerifyTransactionAmounts_Gadget::create(
        pb.clone(),
        inputAmounts.clone().into(),
        outputAmounts.clone().into(),
        minersFee.clone(),
    );
    verifyTx.borrow().generateConstraints();
    for i in 0..2 {
        *pb.as_ref().unwrap().borrow_mut().val(&inputAmounts[i]) = 2.into();
    }
    for i in 0..3 {
        *pb.as_ref().unwrap().borrow_mut().val(&outputAmounts[i]) = 1.into();
    }

    verifyTx.borrow().generateWitness();
    assert!(
        pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
    assert_eq!(*pb.as_ref().unwrap().borrow_mut().val(&minersFee), 1, "");
    *pb.as_ref().unwrap().borrow_mut().val(&minersFee) = 3.into();
    assert!(
        !pb.as_ref()
            .unwrap()
            .borrow()
            .isSatisfied(&PrintOptions::NO_DBG_PRINT)
    );
}

/*
    Below is an example of integrating gadgetlib2 constructed constraint systems with the
    ppzkSNARK.
*/

pub fn TEST_Integration<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
    Fr: FrConfig,
>() {
    initPublicParamsFromDefaultPp::<Fr>();
    // Create an example constraint system and translate to libsnark format
    let example = gen_r1cs_example_from_gadgetlib2_protoboard::<FieldT, SV, SLC>(100);
    let mut test_serialization = false;
    // Run ppzksnark. Jump into function for breakdown
    // let mut bit = run_r1cs_ppzksnark::<FieldT>(example, test_serialization);
    // assert!(bit);
}

pub fn main(argc: i32, argv: &[&str]) {}
