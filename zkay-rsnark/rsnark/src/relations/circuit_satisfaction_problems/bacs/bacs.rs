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



use crate::relations::variable;



/*********************** BACS variable assignment ****************************/

/**
 * A BACS variable assignment is a vector of field elements.
 */

using bacs_variable_assignment = Vec<FieldT>;


/**************************** BACS gate **************************************/


struct bacs_gate;


std::ostream& operator<<(std::ostream &out, g:&bacs_gate<FieldT>);


std::istream& operator>>(std::istream &in, bacs_gate<FieldT> &g);

/**
 * A BACS gate is a formal expression of the form lhs * rhs = output ,
 * where lhs and rhs are linear combinations (of variables) and output is a variable.
 *
 * In other words, a BACS gate is an arithmetic gate that is bilinear.
 */

struct bacs_gate {

    linear_combination<FieldT> lhs;
    linear_combination<FieldT> rhs;

    variable<FieldT> output;
    bool is_circuit_output;

    FieldT evaluate(input:&bacs_variable_assignment<FieldT>) const;
    pub fn  print(variable_annotations:&BTreeMap<usize, String> = BTreeMap<usize, String>()) const;

    bool operator==(other:&bacs_gate<FieldT>) const;

    friend std::ostream& operator<< <FieldT>(std::ostream &out, g:&bacs_gate<FieldT>);
    friend std::istream& operator>> <FieldT>(std::istream &in, bacs_gate<FieldT> &g);
};


/****************************** BACS inputs **********************************/

/**
 * A BACS primary input is a BACS variable assignment.
 */

using bacs_primary_input = bacs_variable_assignment<FieldT>;

/**
 * A BACS auxiliary input is a BACS variable assigment.
 */

using bacs_auxiliary_input = bacs_variable_assignment<FieldT>;


/************************** BACS circuit *************************************/


pub struct bacs_circuit;


std::ostream& operator<<(std::ostream &out, circuit:&bacs_circuit<FieldT>);


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

pub struct bacs_circuit {

    usize primary_input_size;
    usize auxiliary_input_size;
    Vec<bacs_gate<FieldT> > gates;

    bacs_circuit()->Self primary_input_size(0), auxiliary_input_size(0) {}

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
    bool is_satisfied(primary_input:&bacs_primary_input<FieldT>,
                      auxiliary_input:&bacs_auxiliary_input<FieldT>) const;

    bacs_variable_assignment<FieldT> get_all_outputs(primary_input:&bacs_primary_input<FieldT>,
                                                     auxiliary_input:&bacs_auxiliary_input<FieldT>) const;
    bacs_variable_assignment<FieldT> get_all_wires(primary_input:&bacs_primary_input<FieldT>,
                                                   auxiliary_input:&bacs_auxiliary_input<FieldT>) const;

    pub fn  add_gate(g:&bacs_gate<FieldT>);
    pub fn  add_gate(g:&bacs_gate<FieldT>, annotation:&String);

    bool operator==(other:&bacs_circuit<FieldT>) const;

    pub fn  print() const;
    pub fn  print_info() const;

    friend std::ostream& operator<< <FieldT>(std::ostream &out, circuit:&bacs_circuit<FieldT>);
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




pub fn evaluate(input:&bacs_variable_assignment<FieldT>)->FieldT
{
    return lhs.evaluate(input) * rhs.evaluate(input);
}


pub fn print(variable_annotations:&BTreeMap<usize, String>) const
{
    print!("(\n");
    lhs.print(variable_annotations);
    print!(")\n *\n(\n");
    rhs.print(variable_annotations);
    print!(")\n -> \n");
    auto it = variable_annotations.find(output.index);
    print!("    x_{} ({}) ({})\n",
           output.index,
           (if it == variable_annotations.end()  {"no annotation"} else{ it.1}),
           (if is_circuit_output  {"circuit output"} else {"internal wire"}));
}


bool bacs_gate<FieldT>::operator==(other:&bacs_gate<FieldT>) const
{
    return (self.lhs == other.lhs &&
            self.rhs == other.rhs &&
            self.output == other.output &&
            self.is_circuit_output == other.is_circuit_output);
}


std::ostream& operator<<(std::ostream &out, g:&bacs_gate<FieldT>)
{
    out <<  (if g.is_circuit_output {1} else{0}) << "\n";
    out << g.lhs << OUTPUT_NEWLINE;
    out << g.rhs << OUTPUT_NEWLINE;
    out << g.output.index << "\n";

    return out;
}


std::istream& operator>>(std::istream &in, bacs_gate<FieldT> &g)
{
    usize tmp;
    in >> tmp;
    ffec::consume_newline(in);
    g.is_circuit_output = if tmp != 0 {true} else{false};
    in >> g.lhs;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> g.rhs;
    ffec::consume_OUTPUT_NEWLINE(in);
    in >> g.output.index;
    ffec::consume_newline(in);

    return in;
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


pub fn wire_depths()->Vec<usize>
{
    Vec<usize> depths;
    depths.push(0);
    depths.resize(num_inputs() + 1, 1);

    for g in &gates
    {
        usize max_depth = 0;
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


bacs_variable_assignment<FieldT> bacs_circuit<FieldT>::get_all_wires(primary_input:&bacs_primary_input<FieldT>,
                                                                     auxiliary_input:&bacs_auxiliary_input<FieldT>) const
{
    assert!(primary_input.len() == primary_input_size);
    assert!(auxiliary_input.len() == auxiliary_input_size);

    bacs_variable_assignment<FieldT> result;
    result.insert(result.end(), primary_input.begin(), primary_input.end());
    result.insert(result.end(), auxiliary_input.begin(), auxiliary_input.end());

    assert!(result.len() == num_inputs());

    for g in &gates
    {
        let gate_output= g.evaluate(result);
        result.push(gate_output);
    }

    return result;
}


bacs_variable_assignment<FieldT> bacs_circuit<FieldT>::get_all_outputs(primary_input:&bacs_primary_input<FieldT>,
                                                                       auxiliary_input:&bacs_auxiliary_input<FieldT>) const
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


bool bacs_circuit<FieldT>::is_satisfied(primary_input:&bacs_primary_input<FieldT>,
                                        auxiliary_input:&bacs_auxiliary_input<FieldT>) const
{
    const bacs_variable_assignment<FieldT> all_outputs = get_all_outputs(primary_input, auxiliary_input);

    for i in 0..all_outputs.len()
    {
        if !all_outputs[i].is_zero()
        {
            return false;
        }
    }

    return true;
}


pub fn add_gate(g:&bacs_gate<FieldT>)
{
    assert!(g.output.index == num_wires()+1);
    gates.push(g);
}


pub fn add_gate(g:&bacs_gate<FieldT>, annotation:&String)
{
    assert!(g.output.index == num_wires()+1);
    gates.push(g);
// #ifdef DEBUG
    gate_annotations[g.output.index] = annotation;
//#endif
}


bool bacs_circuit<FieldT>::operator==(other:&bacs_circuit<FieldT>) const
{
    return (self.primary_input_size == other.primary_input_size &&
            self.auxiliary_input_size == other.auxiliary_input_size &&
            self.gates == other.gates);
}


std::ostream& operator<<(std::ostream &out, circuit:&bacs_circuit<FieldT>)
{
    out << circuit.primary_input_size << "\n";
    out << circuit.auxiliary_input_size << "\n";
    ffec::operator<<(out, circuit.gates); out << OUTPUT_NEWLINE;

    return out;
}


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



//#endif // BACS_TCC_
