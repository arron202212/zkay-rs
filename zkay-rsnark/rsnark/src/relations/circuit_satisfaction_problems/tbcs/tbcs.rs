// Declaration of interfaces for:
// - a TBCS gate,
// - a TBCS variable assignment, and
// - a TBCS circuit.

// Above, TBCS stands for "Two-input Boolean Circuit Satisfiability".

use crate::relations::variable;
use ffec::FieldTConfig;

use ffec::common::profiling::print_indent;
use std::collections::BTreeMap;

/**
 * A TBCS variable assignment is a vector of bools.
 */
pub type tbcs_variable_assignment = Vec<bool>;

pub type tbcs_wire_t = usize;

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
use num_enum::{FromPrimitive, IntoPrimitive};
use strum::Display;
#[derive(Display, Debug, Default, Clone, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum tbcs_gate_type {
    #[default]
    // #[num_enum(default)]
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
    TBCS_GATE_CONSTANT_1 = 15,
}

pub const num_tbcs_gate_types: u8 = 16;

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
#[derive(Default, Clone)]
pub struct tbcs_gate {
    pub left_wire: tbcs_wire_t,
    pub right_wire: tbcs_wire_t,

    pub types: tbcs_gate_type,

    pub output: tbcs_wire_t,

    pub is_circuit_output: bool,
    // bool evaluate(input:&tbcs_variable_assignment);
    // pub fn  print(variable_annotations:&BTreeMap<usize, String> = BTreeMap<usize, String>());
    // bool operator==(other:&tbcs_gate);

    // friend std::ostream& operator<<(std::ostream &out, g:&tbcs_gate);
    // friend std::istream& operator>>(std::istream &in, tbcs_gate &g);
}

/**
 * A TBCS primary input is a TBCS variable assignment.
 */
pub type tbcs_primary_input = tbcs_variable_assignment;

/**
 * A TBCS auxiliary input is a TBCS variable assignment.
 */
pub type tbcs_auxiliary_input = tbcs_variable_assignment;

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
#[derive(Default, Clone)]
pub struct tbcs_circuit {
    pub primary_input_size: usize,
    pub auxiliary_input_size: usize,
    pub gates: Vec<tbcs_gate>,

    // tbcs_circuit()->Self primary_input_size(0), auxiliary_input_size(0) {}

    // usize num_inputs();
    // usize num_gates();
    // usize num_wires();

    // Vec<usize> wire_depths();
    // usize depth();

    // #ifdef DEBUG
    pub gate_annotations: BTreeMap<usize, String>,
    pub variable_annotations: BTreeMap<usize, String>,
    // bool is_valid();
    // bool is_satisfied(primary_input:&tbcs_primary_input,
    //                   auxiliary_input:&tbcs_auxiliary_input);

    // tbcs_variable_assignment get_all_wires(primary_input:&tbcs_primary_input,
    //                                        auxiliary_input:&tbcs_auxiliary_input);
    // tbcs_variable_assignment get_all_outputs(primary_input:&tbcs_primary_input,
    //                                          auxiliary_input:&tbcs_auxiliary_input);

    // pub fn  add_gate(g:&tbcs_gate);
    // pub fn  add_gate(g:&tbcs_gate, annotation:&String);

    // bool operator==(other:&tbcs_circuit);

    // pub fn  print();
    // pub fn  print_info();

    // friend std::ostream& operator<<(std::ostream &out, circuit:&tbcs_circuit);
    // friend std::istream& operator>>(std::istream &in, tbcs_circuit &circuit);
}

use ffec::common::utils;

use crate::relations::circuit_satisfaction_problems::tbcs::tbcs;

impl tbcs_gate {
    pub fn evaluate(&self, input: &tbcs_variable_assignment) -> bool {
        /**
         * This function is very tricky.
         * See comment in tbcs.hpp .
         */
        let mut X = if self.left_wire == 0 {
            true
        } else {
            input[self.left_wire - 1]
        };
        let mut Y = if self.right_wire == 0 {
            true
        } else {
            input[self.right_wire - 1]
        };

        let pos = 3 - ((if X { 2 } else { 0 }) + (if Y { 1 } else { 0 })); /* 3 - ... inverts position */
        let t: usize = self.types.clone() as usize;
        (t & (1usize << pos)) != 0
    }

    pub fn print_tbcs_wire(
        &self,
        wire: tbcs_wire_t,
        variable_annotations: &BTreeMap<usize, String>,
    ) {
        /**
         * The type tbcs_wire_t does not deserve promotion to a class,
         * but still benefits from a dedicated printing mechanism.
         */
        if wire == 0 {
            print!("  1");
        } else {
            print!(
                "    x_{} ({})",
                wire,
                (if let Some(it) = variable_annotations.get(&wire) {
                    it
                } else {
                    "no annotation"
                })
            );
        }
    }

    pub fn print(&self, variable_annotations: &BTreeMap<usize, String>) {
        match self.types {
            tbcs_gate_type::TBCS_GATE_CONSTANT_0 => print!("CONSTANT_0"),
            tbcs_gate_type::TBCS_GATE_AND => print!("AND"),
            tbcs_gate_type::TBCS_GATE_X_AND_NOT_Y => print!("X_AND_NOT_Y"),
            tbcs_gate_type::TBCS_GATE_X => print!("X"),
            tbcs_gate_type::TBCS_GATE_NOT_X_AND_Y => print!("NOT_X_AND_Y"),
            tbcs_gate_type::TBCS_GATE_Y => print!("Y"),
            tbcs_gate_type::TBCS_GATE_XOR => print!("XOR"),
            tbcs_gate_type::TBCS_GATE_OR => print!("OR"),
            tbcs_gate_type::TBCS_GATE_NOR => print!("NOR"),
            tbcs_gate_type::TBCS_GATE_EQUIVALENCE => print!("EQUIVALENCE"),
            tbcs_gate_type::TBCS_GATE_NOT_Y => print!("NOT_Y"),
            tbcs_gate_type::TBCS_GATE_IF_Y_THEN_X => print!("IF_Y_THEN_X"),
            tbcs_gate_type::TBCS_GATE_NOT_X => print!("NOT_X"),
            tbcs_gate_type::TBCS_GATE_IF_X_THEN_Y => print!("IF_X_THEN_Y"),
            tbcs_gate_type::TBCS_GATE_NAND => print!("NAND"),
            tbcs_gate_type::TBCS_GATE_CONSTANT_1 => print!("CONSTANT_1"),
            _ => print!("Invalid type"),
        }

        print!("\n(\n");
        self.print_tbcs_wire(self.left_wire, variable_annotations);
        print!(",\n");
        self.print_tbcs_wire(self.right_wire, variable_annotations);
        print!("\n) ->\n");
        self.print_tbcs_wire(self.output, variable_annotations);
        print!(
            " ({})\n",
            if self.is_circuit_output {
                "circuit output"
            } else {
                "internal wire"
            }
        );
    }

    // bool tbcs_gate::operator==(other:&tbcs_gate)
    // {
    //     return (self.left_wire == other.left_wire &&
    //             self.right_wire == other.right_wire &&
    //             self.types == other.types &&
    //             self.output == other.output &&
    //             self.is_circuit_output == other.is_circuit_output);
    // }

    // std::ostream& operator<<(std::ostream &out, g:&tbcs_gate)
    // {
    //     out << g.left_wire << "\n";
    //     out << g.right_wire << "\n";
    //     out << (int)g.types << "\n";
    //     out << g.output << "\n";
    //     output_bool(out, g.is_circuit_output);

    //     return out;
    // }

    // std::istream& operator>>(std::istream &in, tbcs_gate &g)
    // {
    //     in >> g.left_wire;
    //     consume_newline(in);
    //     in >> g.right_wire;
    //     consume_newline(in);
    //     int tmp;
    //     in >> tmp;
    //     g.types = (tbcs_gate_type)tmp;
    //     consume_newline(in);
    //     in >> g.output;
    //     input_bool(in, g.is_circuit_output);

    //     return in;
    // }
}

impl tbcs_circuit {
    pub fn wire_depths(&self) -> Vec<usize> {
        let mut depths = vec![1; self.num_inputs()];

        for g in &self.gates {
            depths.push(std::cmp::max(depths[g.left_wire], depths[g.right_wire]) + 1);
        }

        return depths;
    }

    pub fn num_inputs(&self) -> usize {
        return self.primary_input_size + self.auxiliary_input_size;
    }

    pub fn num_gates(&self) -> usize {
        return self.gates.len();
    }

    pub fn num_wires(&self) -> usize {
        return self.num_inputs() + self.num_gates();
    }

    pub fn depth(&self) -> usize {
        let all_depths = self.wire_depths();
        *all_depths.iter().max().unwrap()
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.num_gates() {
            /**
             * The output wire of gates[i] must have index 1+num_inputs+i.
             * (The '1+' accounts for the index of the constant wire.)
             */
            if self.gates[i].output != self.num_inputs() + i + 1 {
                return false;
            }

            /**
             * Gates must be topologically sorted.
             */
            if self.gates[i].left_wire >= self.gates[i].output
                || self.gates[i].right_wire >= self.gates[i].output
            {
                return false;
            }
        }

        return true;
    }

    pub fn get_all_wires(
        &self,
        primary_input: &tbcs_primary_input,
        auxiliary_input: &tbcs_auxiliary_input,
    ) -> tbcs_variable_assignment {
        assert!(primary_input.len() == self.primary_input_size);
        assert!(auxiliary_input.len() == self.auxiliary_input_size);

        let mut result = tbcs_variable_assignment::new();
        result.extend(primary_input.iter());
        result.extend(auxiliary_input.iter());

        assert!(result.len() == self.num_inputs());

        for g in &self.gates {
            let mut gate_output = g.evaluate(&result);
            result.push(gate_output);
        }

        return result;
    }

    pub fn get_all_outputs(
        &self,
        primary_input: &tbcs_primary_input,
        auxiliary_input: &tbcs_auxiliary_input,
    ) -> tbcs_variable_assignment {
        let all_wires = self.get_all_wires(primary_input, auxiliary_input);
        let mut all_outputs = tbcs_variable_assignment::new();

        for g in &self.gates {
            if g.is_circuit_output {
                all_outputs.push(all_wires[g.output - 1]);
            }
        }

        return all_outputs;
    }

    pub fn is_satisfied(
        &self,
        primary_input: &tbcs_primary_input,
        auxiliary_input: &tbcs_auxiliary_input,
    ) -> bool {
        let all_outputs = self.get_all_outputs(primary_input, auxiliary_input);
        for i in 0..all_outputs.len() {
            if all_outputs[i] {
                return false;
            }
        }

        return true;
    }

    pub fn add_gate(&mut self, g: tbcs_gate) {
        assert!(g.output == self.num_wires() + 1);
        self.gates.push(g);
    }

    pub fn add_gate2(&mut self, g: tbcs_gate, annotation: String) {
        assert!(g.output == self.num_wires() + 1);
        self.gates.push(g.clone());
        // #ifdef DEBUG
        self.gate_annotations.insert(g.output.clone(), annotation);
        // #else
        //UNUSED(annotation);
    }

    // bool tbcs_circuit::operator==(other:&tbcs_circuit)
    // {
    //     return (self.primary_input_size == other.primary_input_size &&
    //             self.auxiliary_input_size == other.auxiliary_input_size &&
    //             self.gates == other.gates);
    // }

    // std::ostream& operator<<(std::ostream &out, circuit:&tbcs_circuit)
    // {
    //     out << circuit.primary_input_size << "\n";
    //     out << circuit.auxiliary_input_size << "\n";
    //     operator<<(out, circuit.gates); out << OUTPUT_NEWLINE;

    //     return out;
    // }

    // std::istream& operator>>(std::istream &in, tbcs_circuit &circuit)
    // {
    //     in >> circuit.primary_input_size;
    //     consume_newline(in);
    //     in >> circuit.auxiliary_input_size;
    //     consume_newline(in);
    //     operator>>(in, circuit.gates);
    //     consume_OUTPUT_NEWLINE(in);

    //     return in;
    // }

    pub fn print(&self) {
        print_indent();
        print!("General information about the circuit:\n");
        self.print_info();
        print_indent();
        print!("All gates:\n");
        for i in 0..self.gates.len() {
            let mut annotation = "no annotation";
            // #ifdef DEBUG
            if let Some(it) = self.gate_annotations.get(&i) {
                annotation = it;
            }

            print!("Gate {} ({}):\n", i, annotation);
            // #ifdef DEBUG
            self.gates[i].print(&self.variable_annotations);
            // #else
            // self.gates[i].print();
        }
    }

    pub fn print_info(&self) {
        print_indent();
        print!("* Number of inputs: {}\n", self.num_inputs());
        print_indent();
        print!("* Number of gates: {}\n", self.num_gates());
        print_indent();
        print!("* Number of wires: {}\n", self.num_wires());
        print_indent();
        print!("* Depth: {}\n", self.depth());
    }
}
