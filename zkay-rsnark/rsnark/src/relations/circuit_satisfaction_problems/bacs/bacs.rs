/** @file
 *****************************************************************************

 Declaration of interfaces for:
 - a BACS variable assignment,
 - a BACS gate,
 - a BACS primary input,
 - a BACS auxiliary input,
 - a BACS circuit.

 Above, BACS stands for "Bilinear Arithmetic Circuit Satisfiability".

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BACS_HPP_
// #define BACS_HPP_

use  <vector>

use crate::relations::variable;



/*********************** BACS variable assignment ****************************/

/**
 * A BACS variable assignment is a vector of field elements.
 */
template<typename FieldT>
using bacs_variable_assignment = std::vector<FieldT>;


/**************************** BACS gate **************************************/

template<typename FieldT>
struct bacs_gate;

template<typename FieldT>
std::ostream& operator<<(std::ostream &out, const bacs_gate<FieldT> &g);

template<typename FieldT>
std::istream& operator>>(std::istream &in, bacs_gate<FieldT> &g);

/**
 * A BACS gate is a formal expression of the form lhs * rhs = output ,
 * where lhs and rhs are linear combinations (of variables) and output is a variable.
 *
 * In other words, a BACS gate is an arithmetic gate that is bilinear.
 */
template<typename FieldT>
struct bacs_gate {

    linear_combination<FieldT> lhs;
    linear_combination<FieldT> rhs;

    variable<FieldT> output;
    bool is_circuit_output;

    FieldT evaluate(const bacs_variable_assignment<FieldT> &input) const;
    void print(const std::map<size_t, std::string> &variable_annotations = std::map<size_t, std::string>()) const;

    bool operator==(const bacs_gate<FieldT> &other) const;

    friend std::ostream& operator<< <FieldT>(std::ostream &out, const bacs_gate<FieldT> &g);
    friend std::istream& operator>> <FieldT>(std::istream &in, bacs_gate<FieldT> &g);
};


/****************************** BACS inputs **********************************/

/**
 * A BACS primary input is a BACS variable assignment.
 */
template<typename FieldT>
using bacs_primary_input = bacs_variable_assignment<FieldT>;

/**
 * A BACS auxiliary input is a BACS variable assigment.
 */
template<typename FieldT>
using bacs_auxiliary_input = bacs_variable_assignment<FieldT>;


/************************** BACS circuit *************************************/

template<typename FieldT>
class bacs_circuit;

template<typename FieldT>
std::ostream& operator<<(std::ostream &out, const bacs_circuit<FieldT> &circuit);

template<typename FieldT>
std::istream& operator>>(std::istream &in, bacs_circuit<FieldT> &circuit);

/**
 * A BACS circuit is an arithmetic circuit in which every gate is a BACS gate.
 *
 * Given a BACS primary input and a BACS auxiliary input, the circuit can be evaluated.
 * If every output evaluates to zero, then the circuit is satisfied.
 *
 * NOTE:
 * The 0-th variable (i.e., "x_{0}") always represents the constant 1.
 * Thus, the 0-th variable is not included in num_variables.
 */
template<typename FieldT>
class bacs_circuit {
public:
    size_t primary_input_size;
    size_t auxiliary_input_size;
    std::vector<bacs_gate<FieldT> > gates;

    bacs_circuit() : primary_input_size(0), auxiliary_input_size(0) {}

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
    bool is_satisfied(const bacs_primary_input<FieldT> &primary_input,
                      const bacs_auxiliary_input<FieldT> &auxiliary_input) const;

    bacs_variable_assignment<FieldT> get_all_outputs(const bacs_primary_input<FieldT> &primary_input,
                                                     const bacs_auxiliary_input<FieldT> &auxiliary_input) const;
    bacs_variable_assignment<FieldT> get_all_wires(const bacs_primary_input<FieldT> &primary_input,
                                                   const bacs_auxiliary_input<FieldT> &auxiliary_input) const;

    void add_gate(const bacs_gate<FieldT> &g);
    void add_gate(const bacs_gate<FieldT> &g, const std::string &annotation);

    bool operator==(const bacs_circuit<FieldT> &other) const;

    void print() const;
    void print_info() const;

    friend std::ostream& operator<< <FieldT>(std::ostream &out, const bacs_circuit<FieldT> &circuit);
    friend std::istream& operator>> <FieldT>(std::istream &in, bacs_circuit<FieldT> &circuit);
};



use crate::relations::circuit_satisfaction_problems/bacs/bacs;

//#endif // BACS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for:
 - a BACS variable assignment,
 - a BACS gate,
 - a BACS primary input,
 - a BACS auxiliary input,
 - a BACS circuit.

 See bacs.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BACS_TCC_
// #define BACS_TCC_

use  <algorithm>

use ffec::common::profiling;
use ffec::common::utils;



template<typename FieldT>
FieldT bacs_gate<FieldT>::evaluate(const bacs_variable_assignment<FieldT> &input) const
{
    return lhs.evaluate(input) * rhs.evaluate(input);
}

template<typename FieldT>
void bacs_gate<FieldT>::print(const std::map<size_t, std::string> &variable_annotations) const
{
    print!("(\n");
    lhs.print(variable_annotations);
    print!(")\n *\n(\n");
    rhs.print(variable_annotations);
    print!(")\n -> \n");
    auto it = variable_annotations.find(output.index);
    print!("    x_{} (%s) (%s)\n",
           output.index,
           (it == variable_annotations.end() ? "no annotation" : it->second.c_str()),
           (is_circuit_output ? "circuit output" : "internal wire"));
}

template<typename FieldT>
bool bacs_gate<FieldT>::operator==(const bacs_gate<FieldT> &other) const
{
    return (self.lhs == other.lhs &&
            self.rhs == other.rhs &&
            self.output == other.output &&
            self.is_circuit_output == other.is_circuit_output);
}

template<typename FieldT>
std::ostream& operator<<(std::ostream &out, const bacs_gate<FieldT> &g)
{
    out << (g.is_circuit_output ? 1 : 0) << "\n";
    out << g.lhs << OUTPUT_NEWLINE;
    out << g.rhs << OUTPUT_NEWLINE;
    out << g.output.index << "\n";

    return out;
}

template<typename FieldT>
std::istream& operator>>(std::istream &in, bacs_gate<FieldT> &g)
{
    size_t tmp;
    in >> tmp;
    ffec::consume_newline(in);
    g.is_circuit_output = (tmp != 0 ? true : false);
    in >> g.lhs;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> g.rhs;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> g.output.index;
    ffec::consume_newline(in);

    return in;
}

template<typename FieldT>
size_t bacs_circuit<FieldT>::num_inputs() const
{
    return primary_input_size + auxiliary_input_size;
}

template<typename FieldT>
size_t bacs_circuit<FieldT>::num_gates() const
{
    return gates.size();
}

template<typename FieldT>
size_t bacs_circuit<FieldT>::num_wires() const
{
    return num_inputs() + num_gates();
}

template<typename FieldT>
std::vector<size_t> bacs_circuit<FieldT>::wire_depths() const
{
    std::vector<size_t> depths;
    depths.push(0);
    depths.resize(num_inputs() + 1, 1);

    for g in &gates
    {
        size_t max_depth = 0;
        for t in &g.lhs
        {
            max_depth = std::max(max_depth, depths[t.index]);
        }

        for t in &g.rhs
        {
            max_depth = std::max(max_depth, depths[t.index]);
        }

        depths.push(max_depth + 1);
    }

    return depths;
}

template<typename FieldT>
size_t bacs_circuit<FieldT>::depth() const
{
    std::vector<size_t> all_depths = self.wire_depths();
    return *(std::max_element(all_depths.begin(), all_depths.end()));
}

template<typename FieldT>
bool bacs_circuit<FieldT>::is_valid() const
{
    for i in 0..num_gates()
    {
        /**
         * The output wire of gates[i] must have index 1+num_inputs+i.
         * (The '1+' accounts for the the index of the constant wire.)
         */
        if gates[i].output.index != 1+num_inputs()+i
        {
            return false;
        }

        /**
         * Gates must be topologically sorted.
         */
        if !gates[i].lhs.is_valid(gates[i].output.index) || !gates[i].rhs.is_valid(gates[i].output.index)
        {
            return false;
        }
    }

    return true;
}

template<typename FieldT>
bacs_variable_assignment<FieldT> bacs_circuit<FieldT>::get_all_wires(const bacs_primary_input<FieldT> &primary_input,
                                                                     const bacs_auxiliary_input<FieldT> &auxiliary_input) const
{
    assert!(primary_input.size() == primary_input_size);
    assert!(auxiliary_input.size() == auxiliary_input_size);

    bacs_variable_assignment<FieldT> result;
    result.insert(result.end(), primary_input.begin(), primary_input.end());
    result.insert(result.end(), auxiliary_input.begin(), auxiliary_input.end());

    assert!(result.size() == num_inputs());

    for g in &gates
    {
        const FieldT gate_output = g.evaluate(result);
        result.push(gate_output);
    }

    return result;
}

template<typename FieldT>
bacs_variable_assignment<FieldT> bacs_circuit<FieldT>::get_all_outputs(const bacs_primary_input<FieldT> &primary_input,
                                                                       const bacs_auxiliary_input<FieldT> &auxiliary_input) const
{
    const bacs_variable_assignment<FieldT> all_wires = get_all_wires(primary_input, auxiliary_input);

    bacs_variable_assignment<FieldT> all_outputs;

    for g in &gates
    {
        if g.is_circuit_output
        {
            all_outputs.push(all_wires[g.output.index-1]);
        }
    }

    return all_outputs;
}

template<typename FieldT>
bool bacs_circuit<FieldT>::is_satisfied(const bacs_primary_input<FieldT> &primary_input,
                                        const bacs_auxiliary_input<FieldT> &auxiliary_input) const
{
    const bacs_variable_assignment<FieldT> all_outputs = get_all_outputs(primary_input, auxiliary_input);

    for i in 0..all_outputs.size()
    {
        if !all_outputs[i].is_zero()
        {
            return false;
        }
    }

    return true;
}

template<typename FieldT>
void bacs_circuit<FieldT>::add_gate(const bacs_gate<FieldT> &g)
{
    assert!(g.output.index == num_wires()+1);
    gates.push(g);
}

template<typename FieldT>
void bacs_circuit<FieldT>::add_gate(const bacs_gate<FieldT> &g, const std::string &annotation)
{
    assert!(g.output.index == num_wires()+1);
    gates.push(g);
// #ifdef DEBUG
    gate_annotations[g.output.index] = annotation;
//#endif
}

template<typename FieldT>
bool bacs_circuit<FieldT>::operator==(const bacs_circuit<FieldT> &other) const
{
    return (self.primary_input_size == other.primary_input_size &&
            self.auxiliary_input_size == other.auxiliary_input_size &&
            self.gates == other.gates);
}

template<typename FieldT>
std::ostream& operator<<(std::ostream &out, const bacs_circuit<FieldT> &circuit)
{
    out << circuit.primary_input_size << "\n";
    out << circuit.auxiliary_input_size << "\n";
    ffec::operator<<(out, circuit.gates); out << OUTPUT_NEWLINE;

    return out;
}

template<typename FieldT>
std::istream& operator>>(std::istream &in, bacs_circuit<FieldT> &circuit)
{
    in >> circuit.primary_input_size;
    ffec::consume_newline(in);
    in >> circuit.auxiliary_input_size;
    ffec::consume_newline(in);
    ffec::operator>>(in, circuit.gates);
    ffec::consume_OUTPUT_NEWLINE(in);

    return in;
}

template<typename FieldT>
void bacs_circuit<FieldT>::print() const
{
    ffec::print_indent(); print!("General information about the circuit:\n");
    self.print_info();
    ffec::print_indent(); print!("All gates:\n");
    for i in 0..gates.size()
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

template<typename FieldT>
void bacs_circuit<FieldT>::print_info() const
{
    ffec::print_indent(); print!("* Number of inputs: {}\n", self.num_inputs());
    ffec::print_indent(); print!("* Number of gates: {}\n", self.num_gates());
    ffec::print_indent(); print!("* Number of wires: {}\n", self.num_wires());
    ffec::print_indent(); print!("* Depth: {}\n", self.depth());
}



//#endif // BACS_TCC_
