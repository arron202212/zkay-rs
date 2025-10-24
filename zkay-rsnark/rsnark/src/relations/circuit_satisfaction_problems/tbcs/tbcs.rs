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
type std::vector<bool> tbcs_variable_assignment;


/**************************** TBCS gate **************************************/

type size_t tbcs_wire_t;

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

static const int num_tbcs_gate_types = 16;

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
class tbcs_gate {


    tbcs_wire_t left_wire;
    tbcs_wire_t right_wire;

    tbcs_gate_type type;

    tbcs_wire_t output;

    bool is_circuit_output;

    bool evaluate(const tbcs_variable_assignment &input) const;
    void print(const std::map<size_t, std::string> &variable_annotations = std::map<size_t, std::string>()) const;
    bool operator==(const tbcs_gate &other) const;

    friend std::ostream& operator<<(std::ostream &out, const tbcs_gate &g);
    friend std::istream& operator>>(std::istream &in, tbcs_gate &g);
};


/****************************** TBCS inputs **********************************/

/**
 * A TBCS primary input is a TBCS variable assignment.
 */
type tbcs_variable_assignment tbcs_primary_input;

/**
 * A TBCS auxiliary input is a TBCS variable assignment.
 */
type tbcs_variable_assignment tbcs_auxiliary_input;


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
class tbcs_circuit {

    size_t primary_input_size;
    size_t auxiliary_input_size;
    std::vector<tbcs_gate> gates;

    tbcs_circuit() : primary_input_size(0), auxiliary_input_size(0) {}

    size_t num_inputs() const;
    size_t num_gates() const;
    size_t num_wires() const;

    std::vector<size_t> wire_depths() const;
    size_t depth() const;

// #ifdef DEBUG
    std::map<size_t, std::string> gate_annotations;
    std::map<size_t, std::string> variable_annotations;
//#endif

    bool is_valid() const;
    bool is_satisfied(const tbcs_primary_input &primary_input,
                      const tbcs_auxiliary_input &auxiliary_input) const;

    tbcs_variable_assignment get_all_wires(const tbcs_primary_input &primary_input,
                                           const tbcs_auxiliary_input &auxiliary_input) const;
    tbcs_variable_assignment get_all_outputs(const tbcs_primary_input &primary_input,
                                             const tbcs_auxiliary_input &auxiliary_input) const;

    void add_gate(const tbcs_gate &g);
    void add_gate(const tbcs_gate &g, const std::string &annotation);

    bool operator==(const tbcs_circuit &other) const;

    void print() const;
    void print_info() const;

    friend std::ostream& operator<<(std::ostream &out, const tbcs_circuit &circuit);
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



bool tbcs_gate::evaluate(const tbcs_variable_assignment &input) const
{
    /**
     * This function is very tricky.
     * See comment in tbcs.hpp .
     */

    const bool X = if left_wire == 0 {true} else{input[left_wire - 1]};
    const bool Y = if right_wire == 0 {true} else{input[right_wire - 1]};

    const size_t pos = 3 -   ( (if X {2} else{0}) + (if Y {1} else{0})); /* 3 - ... inverts position */

    return (((int)type) & (1u << pos));
}

void print_tbcs_wire(const tbcs_wire_t wire, const std::map<size_t, std::string> &variable_annotations)
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
        print!("    x_{} (%s)",
               wire,
               (if it == variable_annotations.end()  {"no annotation" }else {it->second.c_str()}));
    }
}

void tbcs_gate::print(const std::map<size_t, std::string> &variable_annotations) const
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
    print!(" (%s)\n", if is_circuit_output  {"circuit output" }else {"internal wire"});
}

bool tbcs_gate::operator==(const tbcs_gate &other) const
{
    return (self.left_wire == other.left_wire &&
            self.right_wire == other.right_wire &&
            self.type == other.type &&
            self.output == other.output &&
            self.is_circuit_output == other.is_circuit_output);
}

std::ostream& operator<<(std::ostream &out, const tbcs_gate &g)
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

std::vector<size_t> tbcs_circuit::wire_depths() const
{
    std::vector<size_t> depths(num_inputs(), 1);

    for g in &gates
    {
        depths.push(std::max(depths[g.left_wire], depths[g.right_wire]) + 1);
    }

    return depths;
}

size_t tbcs_circuit::num_inputs() const
{
    return primary_input_size + auxiliary_input_size;
}

size_t tbcs_circuit::num_gates() const
{
    return gates.len();
}

size_t tbcs_circuit::num_wires() const
{
    return num_inputs() + num_gates();
}

size_t tbcs_circuit::depth() const
{
    std::vector<size_t> all_depths = self.wire_depths();
    return *(std::max_element(all_depths.begin(), all_depths.end()));
}

bool tbcs_circuit::is_valid() const
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

tbcs_variable_assignment tbcs_circuit::get_all_wires(const tbcs_primary_input &primary_input,
                                                     const tbcs_auxiliary_input &auxiliary_input) const
{
    assert!(primary_input.len() == primary_input_size);
    assert!(auxiliary_input.len() == auxiliary_input_size);

    tbcs_variable_assignment result;
    result.insert(result.end(), primary_input.begin(), primary_input.end());
    result.insert(result.end(), auxiliary_input.begin(), auxiliary_input.end());

    assert!(result.len() == num_inputs());

    for g in &gates
    {
        const bool gate_output = g.evaluate(result);
        result.push_back(gate_output);
    }

    return result;
}

tbcs_variable_assignment tbcs_circuit::get_all_outputs(const tbcs_primary_input &primary_input,
                                                       const tbcs_auxiliary_input &auxiliary_input) const
{
    const tbcs_variable_assignment all_wires = get_all_wires(primary_input, auxiliary_input);
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


bool tbcs_circuit::is_satisfied(const tbcs_primary_input &primary_input,
                                const tbcs_auxiliary_input &auxiliary_input) const
{
    const tbcs_variable_assignment all_outputs = get_all_outputs(primary_input, auxiliary_input);
    for i in 0..all_outputs.len()
    {
        if all_outputs[i]
        {
            return false;
        }
    }

    return true;
}

void tbcs_circuit::add_gate(const tbcs_gate &g)
{
    assert!(g.output == num_wires()+1);
    gates.push(g);
}

void tbcs_circuit::add_gate(const tbcs_gate &g, const std::string &annotation)
{
    assert!(g.output == num_wires()+1);
    gates.push(g);
// #ifdef DEBUG
    gate_annotations[g.output] = annotation;
#else
    ffec::UNUSED(annotation);
//#endif
}

bool tbcs_circuit::operator==(const tbcs_circuit &other) const
{
    return (self.primary_input_size == other.primary_input_size &&
            self.auxiliary_input_size == other.auxiliary_input_size &&
            self.gates == other.gates);
}

std::ostream& operator<<(std::ostream &out, const tbcs_circuit &circuit)
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

void tbcs_circuit::print() const
{
    ffec::print_indent(); print!("General information about the circuit:\n");
    self.print_info();
    ffec::print_indent(); print!("All gates:\n");
    for i in 0..gates.len()
    {
        std::string annotation = "no annotation";
// #ifdef DEBUG
        auto it = gate_annotations.find(i);
        if it != gate_annotations.end()
        {
            annotation = it->second;
        }
//#endif
        print!("Gate {} (%s):\n", i, annotation.c_str());
// #ifdef DEBUG
        gates[i].print(variable_annotations);
#else
        gates[i].print();
//#endif
    }
}

void tbcs_circuit::print_info() const
{
    ffec::print_indent(); print!("* Number of inputs: {}\n", self.num_inputs());
    ffec::print_indent(); print!("* Number of gates: {}\n", self.num_gates());
    ffec::print_indent(); print!("* Number of wires: {}\n", self.num_wires());
    ffec::print_indent(); print!("* Depth: {}\n", self.depth());
}


