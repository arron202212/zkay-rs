/** @file
 *****************************************************************************

 Declaration of interfaces for:
 - a TBCS gate,
 - a TBCS variable assignment, and
 - a TBCS circuit.

 Above, TBCS stands for "Two-input Boolean Circuit Satisfiability".

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TBCS_HPP_
// #define TBCS_HPP_

use ffec::common::profiling;

use crate::relations::variable;



/*********************** BACS variable assignment ****************************/

/**
 * A TBCS variable assignment is a vector of bools.
 */
type tbcs_variable_assignment=Vec<bool>;


/**************************** TBCS gate **************************************/

type tbcs_wire_t=usize;

/**
 * Types of TBCS gates (2-input boolean gates).
 *
 * The order and names used below is taken from page 4 of [1].
 *
 * Note that each gate's truth table is encoded in its 4-bit opcode. Namely,
 * if g(X,Y) denotes the output of gate g with inputs X and Y, then
 *            OPCODE(g) = (g(0,0),g(0,1),g(1,0),g(1,1))
 * For example, if g is of type IF_X_THEN_Y, which has opcode 13, then the
 * truth table of g is 1101 (13 in binary).
 *
 * (Note that MSB above is g(0,0) and LSB is g(1,1))
 *
 * References:
 *
 * [1] = https://mitpress.mit.edu/sites/default/files/titles/content/9780262640688_sch_0001.pdf
 */
enum tbcs_gate_type {
    TBCS_GATE_CONSTANT_0 = 0,
    TBCS_GATE_AND = 1,
    TBCS_GATE_X_AND_NOT_Y = 2,
    TBCS_GATE_X = 3,
    TBCS_GATE_NOT_X_AND_Y = 4,
    TBCS_GATE_Y = 5,
    TBCS_GATE_XOR = 6,
    TBCS_GATE_OR = 7,
    TBCS_GATE_NOR = 8,
    TBCS_GATE_EQUIVALENCE = 9,
    TBCS_GATE_NOT_Y = 10,
    TBCS_GATE_IF_Y_THEN_X = 11,
    TBCS_GATE_NOT_X = 12,
    TBCS_GATE_IF_X_THEN_Y = 13,
    TBCS_GATE_NAND = 14,
    TBCS_GATE_CONSTANT_1 = 15
};

static let num_tbcs_gate_types= 16;

/**
 * A TBCS gate is a formal expression of the form
 *
 *                g(left_wire,right_wire) = output ,
 *
 * where 'left_wire' and 'right_wire' are the two input wires, and 'output' is
 * the output wire. In other words, a TBCS gate is a 2-input boolean gate;
 * there are 16 possible such gates (see tbcs_gate_type above).
 *
 * A TBCS gate is used to construct a TBCS circuit (see below).
 */
pub struct tbcs_gate {


    tbcs_wire_t left_wire;
    tbcs_wire_t right_wire;

    tbcs_gate_type type;

    tbcs_wire_t output;

    bool is_circuit_output;

    bool evaluate(input:&tbcs_variable_assignment) const;
    pub fn  print(variable_annotations:&BTreeMap<usize, String> = BTreeMap<usize, String>()) const;
    bool operator==(other:&tbcs_gate) const;

    friend std::ostream& operator<<(std::ostream &out, g:&tbcs_gate);
    friend std::istream& operator>>(std::istream &in, tbcs_gate &g);
};


/****************************** TBCS inputs **********************************/

/**
 * A TBCS primary input is a TBCS variable assignment.
 */
type tbcs_primary_input=tbcs_variable_assignment;

/**
 * A TBCS auxiliary input is a TBCS variable assignment.
 */
type tbcs_auxiliary_input=tbcs_variable_assignment;


/************************** TBCS circuit *************************************/

/**
 * A TBCS circuit is a boolean circuit in which every gate has 2 inputs.
 *
 * A TBCS circuit is satisfied by a TBCS variable assignment if every output
 * evaluates to zero.
 *
 * NOTE:
 * The 0-th variable (i.e., "x_{0}") always represents the constant 1.
 * Thus, the 0-th variable is not included in num_variables.
 */
pub struct tbcs_circuit {

    usize primary_input_size;
    usize auxiliary_input_size;
    Vec<tbcs_gate> gates;

    tbcs_circuit()->Self primary_input_size(0), auxiliary_input_size(0) {}

    usize num_inputs() const;
    usize num_gates() const;
    usize num_wires() const;

    Vec<usize> wire_depths() const;
    usize depth() const;

// #ifdef DEBUG
    BTreeMap<usize, String> gate_annotations;
    BTreeMap<usize, String> variable_annotations;
//#endif

    bool is_valid() const;
    bool is_satisfied(primary_input:&tbcs_primary_input,
                      auxiliary_input:&tbcs_auxiliary_input) const;

    tbcs_variable_assignment get_all_wires(primary_input:&tbcs_primary_input,
                                           auxiliary_input:&tbcs_auxiliary_input) const;
    tbcs_variable_assignment get_all_outputs(primary_input:&tbcs_primary_input,
                                             auxiliary_input:&tbcs_auxiliary_input) const;

    pub fn  add_gate(g:&tbcs_gate);
    pub fn  add_gate(g:&tbcs_gate, annotation:&String);

    bool operator==(other:&tbcs_circuit) const;

    pub fn  print() const;
    pub fn  print_info() const;

    friend std::ostream& operator<<(std::ostream &out, circuit:&tbcs_circuit);
    friend std::istream& operator>>(std::istream &in, tbcs_circuit &circuit);
};



//#endif // TBCS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for:
 - a TBCS gate,
 - a TBCS variable assignment, and
 - a TBCS constraint system.

 See tbcs.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <algorithm>

use ffec::common::utils;

use crate::relations::circuit_satisfaction_problems/tbcs/tbcs;



pub fn evaluate(input:&tbcs_variable_assignment)->bool
{
    /**
     * This function is very tricky.
     * See comment in tbcs.hpp .
     */

    let mut X = if left_wire == 0 {true} else{input[left_wire - 1]};
    let mut Y = if right_wire == 0 {true} else{input[right_wire - 1]};

    let pos = 3 -   ( (if X {2} else{0}) + (if Y {1} else{0})); /* 3 - ... inverts position */

    return (((int)type) & (1u << pos));
}

pub fn  print_tbcs_wire(variable_annotations:&tbcs_wire_t wire, const BTreeMap<usize, String>)
{
    /**
     * The type tbcs_wire_t does not deserve promotion to a class,
     * but still benefits from a dedicated printing mechanism.
     */
    if wire == 0
    {
        print!("  1");
    }
    else
    {
        auto it = variable_annotations.find(wire);
        print!("    x_{} ({})",
               wire,
               (if it == variable_annotations.end()  {"no annotation" }else {it.1}));
    }
}

pub fn print(variable_annotations:&BTreeMap<usize, String>) const
{
    switch (self.type)
    {
    case TBCS_GATE_CONSTANT_0:
        print!("CONSTANT_0");
        break;
    case TBCS_GATE_AND:
        print!("AND");
        break;
    case TBCS_GATE_X_AND_NOT_Y:
        print!("X_AND_NOT_Y");
        break;
    case TBCS_GATE_X:
        print!("X");
        break;
    case TBCS_GATE_NOT_X_AND_Y:
        print!("NOT_X_AND_Y");
        break;
    case TBCS_GATE_Y:
        print!("Y");
        break;
    case TBCS_GATE_XOR:
        print!("XOR");
        break;
    case TBCS_GATE_OR:
        print!("OR");
        break;
    case TBCS_GATE_NOR:
        print!("NOR");
        break;
    case TBCS_GATE_EQUIVALENCE:
        print!("EQUIVALENCE");
        break;
    case TBCS_GATE_NOT_Y:
        print!("NOT_Y");
        break;
    case TBCS_GATE_IF_Y_THEN_X:
        print!("IF_Y_THEN_X");
        break;
    case TBCS_GATE_NOT_X:
        print!("NOT_X");
        break;
    case TBCS_GATE_IF_X_THEN_Y:
        print!("IF_X_THEN_Y");
        break;
    case TBCS_GATE_NAND:
        print!("NAND");
        break;
    case TBCS_GATE_CONSTANT_1:
        print!("CONSTANT_1");
        break;
    default:
        print!("Invalid type");
    }

    print!("\n(\n");
    print_tbcs_wire(left_wire, variable_annotations);
    print!(",\n");
    print_tbcs_wire(right_wire, variable_annotations);
    print!("\n) ->\n");
    print_tbcs_wire(output, variable_annotations);
    print!(" ({})\n", if is_circuit_output  {"circuit output" }else {"internal wire"});
}

bool tbcs_gate::operator==(other:&tbcs_gate) const
{
    return (self.left_wire == other.left_wire &&
            self.right_wire == other.right_wire &&
            self.type == other.type &&
            self.output == other.output &&
            self.is_circuit_output == other.is_circuit_output);
}

std::ostream& operator<<(std::ostream &out, g:&tbcs_gate)
{
    out << g.left_wire << "\n";
    out << g.right_wire << "\n";
    out << (int)g.type << "\n";
    out << g.output << "\n";
    ffec::output_bool(out, g.is_circuit_output);

    return out;
}

std::istream& operator>>(std::istream &in, tbcs_gate &g)
{
    in >> g.left_wire;
    ffec::consume_newline(in);
    in >> g.right_wire;
    ffec::consume_newline(in);
    int tmp;
    in >> tmp;
    g.type = (tbcs_gate_type)tmp;
    ffec::consume_newline(in);
    in >> g.output;
    ffec::input_bool(in, g.is_circuit_output);

    return in;
}

pub fn wire_depths()->Vec<usize>
{
    Vec<usize> depths(num_inputs(), 1);

    for g in &gates
    {
        depths.push(std::max(depths[g.left_wire], depths[g.right_wire]) + 1);
    }

    return depths;
}

pub fn num_inputs()->usize
{
    return primary_input_size + auxiliary_input_size;
}

pub fn num_gates()->usize
{
    return gates.len();
}

pub fn num_wires()->usize
{
    return num_inputs() + num_gates();
}

pub fn depth()->usize
{
    Vec<usize> all_depths = self.wire_depths();
    return *(std::max_element(all_depths.begin(), all_depths.end()));
}

pub fn is_valid()->bool
{
    for i in 0..num_gates()
    {
        /**
         * The output wire of gates[i] must have index 1+num_inputs+i.
         * (The '1+' accounts for the index of the constant wire.)
         */
        if gates[i].output != num_inputs()+i+1
        {
            return false;
        }

        /**
         * Gates must be topologically sorted.
         */
        if gates[i].left_wire >= gates[i].output || gates[i].right_wire >= gates[i].output
        {
            return false;
        }
    }

    return true;
}

tbcs_variable_assignment tbcs_circuit::get_all_wires(primary_input:&tbcs_primary_input,
                                                     auxiliary_input:&tbcs_auxiliary_input) const
{
    assert!(primary_input.len() == primary_input_size);
    assert!(auxiliary_input.len() == auxiliary_input_size);

    tbcs_variable_assignment result;
    result.insert(result.end(), primary_input.begin(), primary_input.end());
    result.insert(result.end(), auxiliary_input.begin(), auxiliary_input.end());

    assert!(result.len() == num_inputs());

    for g in &gates
    {
        let mut gate_output = g.evaluate(result);
        result.push_back(gate_output);
    }

    return result;
}

tbcs_variable_assignment tbcs_circuit::get_all_outputs(primary_input:&tbcs_primary_input,
                                                       auxiliary_input:&tbcs_auxiliary_input) const
{
    let all_wires= get_all_wires(primary_input, auxiliary_input);
    tbcs_variable_assignment all_outputs;

    for g in &gates
    {
        if g.is_circuit_output
        {
            all_outputs.push_back(all_wires[g.output-1]);
        }
    }

    return all_outputs;
}


bool tbcs_circuit::is_satisfied(primary_input:&tbcs_primary_input,
                                auxiliary_input:&tbcs_auxiliary_input) const
{
    let all_outputs= get_all_outputs(primary_input, auxiliary_input);
    for i in 0..all_outputs.len()
    {
        if all_outputs[i]
        {
            return false;
        }
    }

    return true;
}

pub fn add_gate(g:&tbcs_gate)
{
    assert!(g.output == num_wires()+1);
    gates.push(g);
}

pub fn add_gate(g:&tbcs_gate, annotation:&String)
{
    assert!(g.output == num_wires()+1);
    gates.push(g);
// #ifdef DEBUG
    gate_annotations[g.output] = annotation;
#else
    //ffec::UNUSED(annotation);
//#endif
}

bool tbcs_circuit::operator==(other:&tbcs_circuit) const
{
    return (self.primary_input_size == other.primary_input_size &&
            self.auxiliary_input_size == other.auxiliary_input_size &&
            self.gates == other.gates);
}

std::ostream& operator<<(std::ostream &out, circuit:&tbcs_circuit)
{
    out << circuit.primary_input_size << "\n";
    out << circuit.auxiliary_input_size << "\n";
    ffec::operator<<(out, circuit.gates); out << OUTPUT_NEWLINE;

    return out;
}

std::istream& operator>>(std::istream &in, tbcs_circuit &circuit)
{
    in >> circuit.primary_input_size;
    ffec::consume_newline(in);
    in >> circuit.auxiliary_input_size;
    ffec::consume_newline(in);
    ffec::operator>>(in, circuit.gates);
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}

pub fn print() const
{
    ffec::print_indent(); print!("General information about the circuit:\n");
    self.print_info();
    ffec::print_indent(); print!("All gates:\n");
    for i in 0..gates.len()
    {
        String annotation = "no annotation";
// #ifdef DEBUG
        auto it = gate_annotations.find(i);
        if it != gate_annotations.end()
        {
            annotation = it.1;
        }
//#endif
        print!("Gate {} ({}):\n", i, annotation);
// #ifdef DEBUG
        gates[i].print(variable_annotations);
#else
        gates[i].print();
//#endif
    }
}

pub fn print_info() const
{
    ffec::print_indent(); print!("* Number of inputs: {}\n", self.num_inputs());
    ffec::print_indent(); print!("* Number of gates: {}\n", self.num_gates());
    ffec::print_indent(); print!("* Number of wires: {}\n", self.num_wires());
    ffec::print_indent(); print!("* Depth: {}\n", self.depth());
}


