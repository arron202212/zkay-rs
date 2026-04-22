// Declaration of interfaces for:
// - a BACS variable assignment,
// - a BACS gate,
// - a BACS primary input,
// - a BACS auxiliary input,
// - a BACS circuit.

// Above, BACS stands for "Bilinear Arithmetic Circuit Satisfiability".

use crate::relations::variable::SubLinearCombinationConfig;
use crate::relations::variable::SubVariableConfig;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use std::collections::BTreeMap;

// /**
//  * A BACS variable assignment is a vector of field elements.
//  */

pub type bacs_variable_assignment<FieldT> = Vec<FieldT>;


// /**
//  * A BACS gate is a formal expression of the form lhs * rhs = output ,
//  * where lhs and rhs are linear combinations (of variables) and output is a variable.
//  *
//  * In other words, a BACS gate is an arithmetic gate that is bilinear.
//  */
#[derive(Clone, Default)]
pub struct bacs_gate<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> {
    pub lhs: linear_combination<FieldT, SV, SLC>,
    pub rhs: linear_combination<FieldT, SV, SLC>,

    pub output: variable<FieldT, SV>,
    pub is_circuit_output: bool,
    
}

// /**
//  * A BACS primary input is a BACS variable assignment.
//  */

pub type bacs_primary_input<FieldT> = bacs_variable_assignment<FieldT>;

// /**
//  * A BACS auxiliary input is a BACS variable assigment.
//  */

pub type bacs_auxiliary_input<FieldT> = bacs_variable_assignment<FieldT>;


// /**
//  * A BACS circuit is an arithmetic circuit in which every gate is a BACS gate.
//  *
//  * Given a BACS primary input and a BACS auxiliary input, the circuit can be evaluated.
//  * If every output evaluates to zero, then the circuit is satisfied.
//  *
//  * NOTE:
//  * The 0-th variable (i.e., "x_{0}") always represents the constant 1.
//  * Thus, the 0-th variable is not included in num_variables.
//  */
#[derive(Default, Clone)]
pub struct bacs_circuit<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
> {
    pub primary_input_size: usize,
    pub auxiliary_input_size: usize,
    pub gates: Vec<bacs_gate<FieldT, SV, SLC>>,

    pub gate_annotations: BTreeMap<usize, String>,
    pub variable_annotations: BTreeMap<usize, String>,
   
}


use ffec::common::profiling::print_indent;
use ffec::common::utils;

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    bacs_gate<FieldT, SV, SLC>
{
    pub fn evaluate(&self, input: &bacs_variable_assignment<FieldT>) -> FieldT {
        return self.lhs.evaluate(input) * self.rhs.evaluate(input);
    }

    pub fn print(&self, variable_annotations: &BTreeMap<usize, String>) {
        print!("(\n");
        self.lhs.print(variable_annotations);
        print!(")\n *\n(\n");
        self.rhs.print(variable_annotations);
        print!(")\n -> \n");
        let it = variable_annotations.get(&self.output.index);
        print!(
            "    x_{} ({}) ({})\n",
            self.output.index,
            (if let Some(v) = it { v } else { "no annotation" }),
            (if self.is_circuit_output {
                "circuit output"
            } else {
                "internal wire"
            })
        );
    }

}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    bacs_circuit<FieldT, SV, SLC>
{
    pub fn num_inputs(&self) -> usize {
        return self.primary_input_size + self.auxiliary_input_size;
    }

    pub fn num_gates(&self) -> usize {
        return self.gates.len();
    }

    pub fn num_wires(&self) -> usize {
        return self.num_inputs() + self.num_gates();
    }

    pub fn wire_depths(&self) -> Vec<usize> {
        let mut depths = vec![0];
        depths.resize(self.num_inputs() + 1, 1);

        for g in &self.gates {
            let mut max_depth = 0;
            for t in &g.lhs {
                max_depth = std::cmp::max(max_depth, depths[t.index.index]);
            }

            for t in &g.rhs {
                max_depth = std::cmp::max(max_depth, depths[t.index.index]);
            }

            depths.push(max_depth + 1);
        }

        return depths;
    }

    pub fn depth(&self) -> usize {
        let all_depths = self.wire_depths();
        *all_depths.iter().max().unwrap()
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.num_gates() {
            // /**
            //  * The output wire of gates[i] must have index 1+num_inputs+i.
            //  * (The '1+' accounts for the the index of the constant wire.)
            //  */
            if self.gates[i].output.index != 1 + self.num_inputs() + i {
                return false;
            }

            // /**
            //  * Gates must be topologically sorted.
            //  */
            if !self.gates[i].lhs.is_valid(self.gates[i].output.index)
                || !self.gates[i].rhs.is_valid(self.gates[i].output.index)
            {
                return false;
            }
        }

        return true;
    }

    pub fn get_all_wires(
        &self,
        primary_input: &bacs_primary_input<FieldT>,
        auxiliary_input: &bacs_auxiliary_input<FieldT>,
    ) -> bacs_variable_assignment<FieldT> {
        assert!(primary_input.len() == self.primary_input_size);
        assert!(auxiliary_input.len() == self.auxiliary_input_size);

        let mut result = bacs_variable_assignment::<FieldT>::new();
        result.extend(primary_input.iter().cloned());
        result.extend(auxiliary_input.iter().cloned());

        assert!(result.len() == self.num_inputs());

        for g in &self.gates {
            let gate_output = g.evaluate(&result);
            result.push(gate_output);
        }

        return result;
    }

    pub fn get_all_outputs(
        &self,
        primary_input: &bacs_primary_input<FieldT>,
        auxiliary_input: &bacs_auxiliary_input<FieldT>,
    ) -> bacs_variable_assignment<FieldT> {
        let all_wires = self.get_all_wires(primary_input, auxiliary_input);

        let mut all_outputs = bacs_variable_assignment::<FieldT>::new();

        for g in &self.gates {
            if g.is_circuit_output {
                all_outputs.push(all_wires[g.output.index - 1].clone());
            }
        }

        return all_outputs;
    }

    pub fn is_satisfied(
        &self,
        primary_input: &bacs_primary_input<FieldT>,
        auxiliary_input: &bacs_auxiliary_input<FieldT>,
    ) -> bool {
        let all_outputs = self.get_all_outputs(primary_input, auxiliary_input);

        for i in 0..all_outputs.len() {
            if !all_outputs[i].is_zero() {
                return false;
            }
        }

        return true;
    }

    pub fn add_gate(&mut self, g: bacs_gate<FieldT, SV, SLC>) {
        assert!(g.output.index == self.num_wires() + 1);
        self.gates.push(g);
    }

    pub fn add_gate2(&mut self, g: bacs_gate<FieldT, SV, SLC>, annotation: String) {
        assert!(g.output.index == self.num_wires() + 1);
        self.gates.push(g.clone());
        self.gate_annotations.insert(g.output.index, annotation);
    }

    pub fn print(&self) {
        print_indent();
        print!("General information about the circuit:\n");
        self.print_info();
        print_indent();
        print!("All gates:\n");
        for i in 0..self.gates.len() {
            let mut annotation = "no annotation";
            if let Some(v) = self.gate_annotations.get(&i) {
                annotation = v;
            }

            print!("Gate {} ({}):\n", i, annotation);
            self.gates[i].print(&self.variable_annotations);

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
