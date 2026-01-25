//  Declaration of interfaces for the FOORAM CPU checker gadget.

//  The gadget checks the correct operation for the CPU of the FOORAM architecture.

//  In FOORAM, the only instruction is FOO(x) and its encoding is x.
//  The instruction FOO(x) has the following semantics:
//  - if x is odd: reg <- [2*x+(pc+1)]
//  - if x is even: [pc+x] <- reg+pc
//  - increment pc by 1

//  Starting from empty memory, FOORAM performs non-trivial pseudo-random computation
//  that exercises both loads, stores, and instruction fetches.

//  E.g. for the first 200 steps on 16 cell machine we get 93 different memory configurations.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::packing_gadget;
use crate::gadgetlib1::gadgets::cpu_checkers::fooram::components::bar_gadget::bar_gadget;
use crate::gadgetlib1::gadgets::cpu_checkers::fooram::components::fooram_protoboard::{
    SubFooRamConfig, fooram_gadget, fooram_protoboard,
};
use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::{protoboard,ProtoboardConfig};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::ram_computations::memory::memory_interface;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::serialization;
use rccell::RcCell;
#[derive(Clone, Default)]
pub struct fooram_cpu_checker<FieldT: FieldTConfig> {
    //  : public fooram_gadget<FieldT>
    pub prev_pc_addr: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub prev_pc_val: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub prev_state: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub guess: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub ls_addr: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub ls_prev_val: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub ls_next_val: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub next_state: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub next_pc_addr: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
    pub next_has_accepted: variable<FieldT, pb_variable>,

    pub zero: variable<FieldT, pb_variable>,
    pub packed_next_pc_addr: variable<FieldT, pb_variable>,
    pub one_as_addr: pb_linear_combination_array<FieldT, fooram_protoboard<FieldT>>,
    pub pack_next_pc_addr: RcCell<
        gadget<
            FieldT,
            fooram_protoboard<FieldT>,
            packing_gadget<FieldT, fooram_protoboard<FieldT>>,
        >,
    >,

    pub packed_load_addr: variable<FieldT, pb_variable>,
    pub packed_store_addr: variable<FieldT, pb_variable>,
    pub packed_store_val: variable<FieldT, pb_variable>,

    pub increment_pc: RcCell<
        gadget<FieldT, fooram_protoboard<FieldT>, bar_gadget<FieldT, fooram_protoboard<FieldT>>>,
    >,
    pub compute_packed_load_addr: RcCell<
        gadget<FieldT, fooram_protoboard<FieldT>, bar_gadget<FieldT, fooram_protoboard<FieldT>>>,
    >,
    pub compute_packed_store_addr: RcCell<
        gadget<FieldT, fooram_protoboard<FieldT>, bar_gadget<FieldT, fooram_protoboard<FieldT>>>,
    >,
    pub compute_packed_store_val: RcCell<
        gadget<FieldT, fooram_protoboard<FieldT>, bar_gadget<FieldT, fooram_protoboard<FieldT>>>,
    >,

    pub packed_ls_addr: variable<FieldT, pb_variable>,
    pub packed_ls_prev_val: variable<FieldT, pb_variable>,
    pub packed_ls_next_val: variable<FieldT, pb_variable>,
    pub packed_prev_state: variable<FieldT, pb_variable>,
    pub packed_next_state: variable<FieldT, pb_variable>,
    pub pack_ls_addr: RcCell<
        gadget<
            FieldT,
            fooram_protoboard<FieldT>,
            packing_gadget<FieldT, fooram_protoboard<FieldT>>,
        >,
    >,
    pub pack_ls_prev_val: RcCell<
        gadget<
            FieldT,
            fooram_protoboard<FieldT>,
            packing_gadget<FieldT, fooram_protoboard<FieldT>>,
        >,
    >,
    pub pack_ls_next_val: RcCell<
        gadget<
            FieldT,
            fooram_protoboard<FieldT>,
            packing_gadget<FieldT, fooram_protoboard<FieldT>>,
        >,
    >,
    pub pack_prev_state: RcCell<
        gadget<
            FieldT,
            fooram_protoboard<FieldT>,
            packing_gadget<FieldT, fooram_protoboard<FieldT>>,
        >,
    >,
    pub pack_next_state: RcCell<
        gadget<
            FieldT,
            fooram_protoboard<FieldT>,
            packing_gadget<FieldT, fooram_protoboard<FieldT>>,
        >,
    >,
    // fooram_cpu_checker(
    // protoboard<FieldT,fooram_protoboard<FieldT>> &pb,
    //                    pb_variable_array<FieldT,fooram_protoboard<FieldT>> &prev_pc_addr,
    //                    pb_variable_array<FieldT,fooram_protoboard<FieldT>> &prev_pc_val,
    //                    pb_variable_array<FieldT,fooram_protoboard<FieldT>> &prev_state,
    //                    pb_variable_array<FieldT,fooram_protoboard<FieldT>> &ls_addr,
    //                    pb_variable_array<FieldT,fooram_protoboard<FieldT>> &ls_prev_val,
    //                    pb_variable_array<FieldT,fooram_protoboard<FieldT>> &ls_next_val,
    //                    pb_variable_array<FieldT,fooram_protoboard<FieldT>> &next_state,
    //                    pb_variable_array<FieldT,fooram_protoboard<FieldT>> &next_pc_addr,
    //                    variable<FieldT,pb_variable> &next_has_accepted,
    //                    annotation_prefix:&String);

    // pub fn  generate_r1cs_constraints();

    // pub fn  generate_r1cs_witness() { assert!(0); }

    // pub fn  generate_r1cs_witness_address();

    // pub fn  generate_r1cs_witness_other(fooram_input_tape_iterator &aux_it,
    //                                  aux_end:&fooram_input_tape_iterator);

    // pub fn  dump() const;
}
impl<FieldT: FieldTConfig> SubFooRamConfig for fooram_cpu_checker<FieldT> {}
impl<FieldT: FieldTConfig> fooram_cpu_checker<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, fooram_protoboard<FieldT>>>,
        prev_pc_addr: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
        prev_pc_val: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
        prev_state: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
        ls_addr: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
        ls_prev_val: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
        ls_next_val: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
        next_state: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
        next_pc_addr: pb_variable_array<FieldT, fooram_protoboard<FieldT>>,
        next_has_accepted: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> gadget<FieldT, fooram_protoboard<FieldT>, fooram_gadget<FieldT, Self>> {
        /* increment PC */
        let mut packed_next_pc_addr = variable::<FieldT, pb_variable>::default();
        packed_next_pc_addr.allocate(&pb, format!("{annotation_prefix} packed_next_pc_addr"));
        let pack_next_pc_addr =
            RcCell::new(packing_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
                pb.clone(),
                next_pc_addr.clone().into(),
                packed_next_pc_addr.clone().into(),
                format!("{annotation_prefix} pack_next_pc_addr"),
            ));
        let mut one_as_addr =
            pb_linear_combination_array::<FieldT, fooram_protoboard<FieldT>>::default();
        one_as_addr
            .contents
            .resize(next_pc_addr.len(), Default::default());
        one_as_addr[0].assign(&pb, &(FieldT::from(1).into()));
        for i in 1..next_pc_addr.len() {
            one_as_addr[i].assign(&pb, &(FieldT::from(0).into()));
        }

        /* packed_next_pc_addr = prev_pc_addr + one_as_addr */
        let increment_pc = RcCell::new(bar_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
            pb.clone(),
            prev_pc_addr.clone().into(),
            FieldT::one(),
            one_as_addr.clone().into(),
            FieldT::one(),
            packed_next_pc_addr.clone().into(),
            format!("{annotation_prefix} increment_pc"),
        ));

        /* packed_store_addr = prev_pc_addr + prev_pc_val */
        let mut packed_store_addr = variable::<FieldT, pb_variable>::default();
        packed_store_addr.allocate(&pb, format!("{annotation_prefix} packed_store_addr"));
        let mut compute_packed_store_addr =
            RcCell::new(bar_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
                pb.clone(),
                prev_pc_addr.clone().into(),
                FieldT::one(),
                prev_pc_val.clone().into(),
                FieldT::one(),
                packed_store_addr.clone().into(),
                format!("{annotation_prefix} compute_packed_store_addr"),
            ));

        /* packed_load_addr = 2 * x + next_pc_addr */
        let mut packed_load_addr = variable::<FieldT, pb_variable>::default();
        packed_load_addr.allocate(&pb, format!("{annotation_prefix} packed_load_addr"));
        let compute_packed_load_addr =
            RcCell::new(bar_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
                pb.clone(),
                prev_pc_val.clone().into(),
                FieldT::from(2),
                next_pc_addr.clone().into(),
                FieldT::one(),
                packed_load_addr.clone().into(),
                format!("{annotation_prefix} compute_packed_load_addr"),
            ));

        /*
          packed_ls_addr = x0 * packed_load_addr + (1-x0) * packed_store_addr
          packed_ls_addr ~ ls_addr
        */
        let mut packed_ls_addr = variable::<FieldT, pb_variable>::default();
        packed_ls_addr.allocate(&pb, format!("{annotation_prefix} packed_ls_addr"));
        let pack_ls_addr = RcCell::new(packing_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
            pb.clone(),
            ls_addr.clone().into(),
            packed_ls_addr.clone().into(),
            " pack_ls_addr".to_owned(),
        ));

        /* packed_store_val = prev_state_bits + prev_pc_addr */
        let mut packed_store_val = variable::<FieldT, pb_variable>::default();
        packed_store_val.allocate(&pb, format!("{annotation_prefix} packed_store_val"));
        let compute_packed_store_val =
            RcCell::new(bar_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
                pb.clone(),
                prev_state.clone().into(),
                FieldT::one(),
                prev_pc_addr.clone().into(),
                FieldT::one(),
                packed_store_val.clone().into(),
                format!("{annotation_prefix} compute_packed_store_val"),
            ));

        /*
          packed_ls_next_val = x0 * packed_ls_prev_val + (1-x0) * packed_store_val
          packed_ls_next_val ~ ls_next_val
        */
        let mut packed_ls_prev_val = variable::<FieldT, pb_variable>::default();
        packed_ls_prev_val.allocate(&pb, format!("{annotation_prefix} packed_ls_prev_val"));
        let pack_ls_prev_val =
            RcCell::new(packing_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
                pb.clone(),
                ls_prev_val.clone().into(),
                packed_ls_prev_val.clone().into(),
                format!("{annotation_prefix} pack_ls_prev_val"),
            ));
        let mut packed_ls_next_val = variable::<FieldT, pb_variable>::default();
        packed_ls_next_val.allocate(&pb, format!("{annotation_prefix} packed_ls_next_val"));
        let pack_ls_next_val =
            RcCell::new(packing_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
                pb.clone(),
                ls_next_val.clone().into(),
                packed_ls_next_val.clone().into(),
                format!("{annotation_prefix} pack_ls_next_val"),
            ));

        /*
          packed_next_state = x0 * packed_ls_prev_val + (1-x0) * packed_prev_state
          packed_next_state ~ next_state
          packed_prev_state ~ prev_state
        */
        let mut packed_prev_state = variable::<FieldT, pb_variable>::default();
        packed_prev_state.allocate(&pb, format!("{annotation_prefix} packed_prev_state"));
        let pack_prev_state =
            RcCell::new(packing_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
                pb.clone(),
                prev_state.clone().into(),
                packed_prev_state.clone().into(),
                " pack_prev_state".to_owned(),
            ));
        let mut packed_next_state = variable::<FieldT, pb_variable>::default();
        packed_next_state.allocate(&pb, format!("{annotation_prefix} packed_next_state"));
        let pack_next_state =
            RcCell::new(packing_gadget::<FieldT, fooram_protoboard<FieldT>>::new(
                pb.clone(),
                next_state.clone().into(),
                packed_next_state.clone().into(),
                " pack_next_state".to_owned(),
            ));

        /* next_has_accepted = 1 */
        fooram_gadget::<FieldT, Self>::new(
            pb,
            annotation_prefix,
            Self {
                prev_pc_addr,
                prev_pc_val,
                prev_state,
                guess: pb_variable_array::<FieldT, fooram_protoboard<FieldT>>::default(),
                ls_addr,
                ls_prev_val,
                ls_next_val,
                next_state,
                next_pc_addr,
                next_has_accepted,
                zero: variable::<FieldT, pb_variable>::default(),
                packed_next_pc_addr,
                one_as_addr,
                pack_next_pc_addr,
                packed_load_addr,
                packed_store_addr,
                packed_store_val,
                increment_pc,
                compute_packed_load_addr,
                compute_packed_store_addr,
                compute_packed_store_val,
                packed_ls_addr,
                packed_ls_prev_val,
                packed_ls_next_val,
                packed_prev_state,
                packed_next_state,
                pack_ls_addr,
                pack_ls_prev_val,
                pack_ls_next_val,
                pack_prev_state,
                pack_next_state,
            },
        )
    }
}

impl<FieldT: FieldTConfig>
    gadget<FieldT, fooram_protoboard<FieldT>, fooram_gadget<FieldT, fooram_cpu_checker<FieldT>>>
{
    pub fn generate_r1cs_constraints(&self) {
        /* packed_next_pc_addr = prev_pc_addr + one_as_addr */
        self.t
            .t
            .pack_next_pc_addr
            .borrow()
            .generate_r1cs_constraints(false);
        self.t.t.increment_pc.borrow().generate_r1cs_constraints();

        /* packed_store_addr = prev_pc_addr + prev_pc_val */
        self.t
            .t
            .compute_packed_store_addr
            .borrow()
            .generate_r1cs_constraints();

        /* packed_load_addr = 2 * x + next_pc_addr */
        self.t
            .t
            .compute_packed_load_addr
            .borrow()
            .generate_r1cs_constraints();

        /*
          packed_ls_addr = x0 * packed_load_addr + (1-x0) * packed_store_addr
          packed_ls_addr - packed_store_addr = x0 * (packed_load_addr - packed_store_addr)
          packed_ls_addr ~ ls_addr
        */
        self.t
            .t
            .pack_ls_addr
            .borrow()
            .generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.prev_pc_val[0].clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.packed_load_addr.clone(),
                ) - self.t.t.packed_store_addr.clone(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.packed_ls_addr.clone(),
                ) - self.t.t.packed_store_addr.clone(),
            ),
            format!("{} compute_ls_addr_packed", self.annotation_prefix),
        );

        /* packed_store_val = prev_state_bits + prev_pc_addr */
        self.t
            .t
            .compute_packed_store_val
            .borrow()
            .generate_r1cs_constraints();

        /*
          packed_ls_next_val = x0 * packed_ls_prev_val + (1-x0) * packed_store_val
          packed_ls_next_val - packed_store_val = x0 * (packed_ls_prev_val - packed_store_val)
          packed_ls_next_val ~ ls_next_val
        */
        self.t
            .t
            .pack_ls_prev_val
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .pack_ls_next_val
            .borrow()
            .generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.prev_pc_val[0].clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.packed_ls_prev_val.clone(),
                ) - self.t.t.packed_store_val.clone(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.packed_ls_next_val.clone(),
                ) - self.t.t.packed_store_val.clone(),
            ),
            format!("{} compute_packed_ls_next_val", self.annotation_prefix),
        );

        /*
          packed_next_state = x0 * packed_ls_prev_val + (1-x0) * packed_prev_state
          packed_next_state - packed_prev_state = x0 * (packed_ls_prev_val - packed_prev_state)
          packed_next_state ~ next_state
          packed_prev_state ~ prev_state
        */
        self.t
            .t
            .pack_prev_state
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .pack_next_state
            .borrow()
            .generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.prev_pc_val[0].clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.packed_ls_prev_val.clone(),
                ) - self.t.t.packed_prev_state.clone(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.packed_next_state.clone(),
                ) - self.t.t.packed_prev_state.clone(),
            ),
            format!("{} compute_packed_next_state", self.annotation_prefix),
        );

        /* next_has_accepted = 1 */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                FieldT::from(1).into(),
                self.t.t.next_has_accepted.clone().into(),
                FieldT::from(1).into(),
            ),
            format!("{} always_accepted", self.annotation_prefix),
        );
    }

    pub fn generate_r1cs_witness_address(&self) {
        self.t.t.one_as_addr.evaluate(&self.pb);

        /* packed_next_pc_addr = prev_pc_addr + one_as_addr */
        self.t.t.increment_pc.borrow().generate_r1cs_witness();
        self.t
            .t
            .pack_next_pc_addr
            .borrow()
            .generate_r1cs_witness_from_packed();

        /* packed_store_addr = prev_pc_addr + prev_pc_val */
        self.t
            .t
            .compute_packed_store_addr
            .borrow()
            .generate_r1cs_witness();

        /* packed_load_addr = 2 * x + next_pc_addr */
        self.t
            .t
            .compute_packed_load_addr
            .borrow()
            .generate_r1cs_witness();

        /*
          packed_ls_addr = x0 * packed_load_addr + (1-x0) * packed_store_addr
          packed_ls_addr - packed_store_addr = x0 * (packed_load_addr - packed_store_addr)
          packed_ls_addr ~ ls_addr
        */
        *self.pb.borrow_mut().val_ref(&self.t.t.packed_ls_addr) =
            (self.pb.borrow().val(&self.t.t.prev_pc_val[0])
                * self.pb.borrow().val(&self.t.t.packed_load_addr)
                + (FieldT::one() - self.pb.borrow().val(&self.t.t.prev_pc_val[0]))
                    * self.pb.borrow().val(&self.t.t.packed_store_addr));
        self.t
            .t
            .pack_ls_addr
            .borrow()
            .generate_r1cs_witness_from_packed();
    }

    pub fn generate_r1cs_witness_other(&self, aux: &[usize]) {
        //_fooram_input_tape
        /* fooram memory contents do not depend on program/input. */
        // //ffec::UNUSED(aux_it, aux_end);
        /* packed_store_val = prev_state_bits + prev_pc_addr */
        self.t
            .t
            .compute_packed_store_val
            .borrow()
            .generate_r1cs_witness();

        /*
          packed_ls_next_val = x0 * packed_ls_prev_val + (1-x0) * packed_store_val
          packed_ls_next_val - packed_store_val = x0 * (packed_ls_prev_val - packed_store_val)
          packed_ls_next_val ~ ls_next_val
        */
        self.t
            .t
            .pack_ls_prev_val
            .borrow()
            .generate_r1cs_witness_from_bits();
        *self.pb.borrow_mut().val_ref(&self.t.t.packed_ls_next_val) =
            (self.pb.borrow().val(&self.t.t.prev_pc_val[0])
                * self.pb.borrow().val(&self.t.t.packed_ls_prev_val)
                + (FieldT::one() - self.pb.borrow().val(&self.t.t.prev_pc_val[0]))
                    * self.pb.borrow().val(&self.t.t.packed_store_val));
        self.t
            .t
            .pack_ls_next_val
            .borrow()
            .generate_r1cs_witness_from_packed();

        /*
          packed_next_state = x0 * packed_ls_prev_val + (1-x0) * packed_prev_state
          packed_next_state - packed_prev_state = x0 * (packed_ls_prev_val - packed_prev_state)
          packed_next_state ~ next_state
          packed_prev_state ~ prev_state
        */
        self.t
            .t
            .pack_prev_state
            .borrow()
            .generate_r1cs_witness_from_bits();
        *self.pb.borrow_mut().val_ref(&self.t.t.packed_next_state) =
            (self.pb.borrow().val(&self.t.t.prev_pc_val[0])
                * self.pb.borrow().val(&self.t.t.packed_ls_prev_val)
                + (FieldT::one() - self.pb.borrow().val(&self.t.t.prev_pc_val[0]))
                    * self.pb.borrow().val(&self.t.t.packed_prev_state));
        self.t
            .t
            .pack_next_state
            .borrow()
            .generate_r1cs_witness_from_packed();

        /* next_has_accepted = 1 */
        *self.pb.borrow_mut().val_ref(&self.t.t.next_has_accepted) = FieldT::one();
    }

    pub fn dump(&self) {
        print!("packed_store_addr: ");
        self.pb.borrow().val(&self.t.t.packed_store_addr).print();
        print!("packed_load_addr: ");
        self.pb.borrow().val(&self.t.t.packed_load_addr).print();
        print!("packed_ls_addr: ");
        self.pb.borrow().val(&self.t.t.packed_ls_addr).print();
        print!("packed_store_val: ");
        self.pb.borrow().val(&self.t.t.packed_store_val).print();
        print!("packed_ls_next_val: ");
        self.pb.borrow().val(&self.t.t.packed_ls_next_val).print();
        print!("packed_next_state: ");
        self.pb.borrow().val(&self.t.t.packed_next_state).print();
    }
}

//#endif // FOORAM_CPU_CHECKER_TCC
