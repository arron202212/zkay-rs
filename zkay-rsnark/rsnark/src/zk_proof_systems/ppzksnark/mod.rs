pub mod bacs_ppzksnark;
pub mod r1cs_gg_ppzksnark;
pub mod r1cs_ppzksnark;
pub mod r1cs_se_ppzksnark;
pub mod ram_ppzksnark;
pub mod tbcs_ppzksnark;
pub mod uscs_ppzksnark;

pub trait KeyPairTConfig: Default + Clone {
    type VK;
    type PK;
    fn vk(&self) -> &Self::VK;
    fn pk(&self) -> &Self::PK;
}

pub trait ProofTConfig: Default + Clone {
    //     type ppT: PublicParams;
    //   pub g_A: knowledge_commitment<G1<ppT>, G1<ppT>>,
    //     pub g_B: knowledge_commitment<G2<ppT>, G1<ppT>>,
    //     pub g_C: knowledge_commitment<G1<ppT>, G1<ppT>>,
    //     pub g_H: G1<ppT>,
    //     pub g_K: G1<ppT>,

    //     pub g_A: G1<ppT>,
    //     pub g_B: G2<ppT>,
    //     pub g_C: G1<ppT>,

    //  pub A: G1<ppT>,
    //     pub B: G2<ppT>,
    //     pub C: G1<ppT>,
}

pub trait ProvingKeyTConfig: Default + Clone {
    //     pub A_query: knowledge_commitment_vector<G1<ppT>, G1<ppT>>,
    //     pub B_query: knowledge_commitment_vector<G2<ppT>, G1<ppT>>,
    //     pub C_query: knowledge_commitment_vector<G1<ppT>, G1<ppT>>,
    //     pub H_query: G1_vector<ppT>,
    //     pub K_query: G1_vector<ppT>,

    //     pub constraint_system: r1cs_ppzksnark_constraint_system<ppT>,

    //     pub alpha_g1: G1<ppT>,
    //     pub beta_g1: G1<ppT>,
    //     pub beta_g2: G2<ppT>,
    //     pub delta_g1: G1<ppT>,
    //     pub delta_g2: G2<ppT>,

    //     pub A_query: G1_vector<ppT>, // this could be a sparse vector if we had multiexp for those
    //     pub B_query: knowledge_commitment_vector<G2<ppT>, G1<ppT>>,
    //     pub H_query: G1_vector<ppT>,
    //     pub L_query: G1_vector<ppT>,

    //     pub constraint_system: r1cs_gg_ppzksnark_constraint_system<ppT>,

    //  // G^{gamma * A_i(t)} for 0 <= i <= sap.num_variables()
    //     pub A_query: G1_vector<ppT>,

    //     // H^{gamma * A_i(t)} for 0 <= i <= sap.num_variables()
    //     pub B_query: G2_vector<ppT>,

    //     // G^{gamma^2 * C_i(t) + (alpha + beta) * gamma * A_i(t)}
    //     // for sap.num_inputs() + 1 < i <= sap.num_variables()
    //     pub C_query_1: G1_vector<ppT>,

    //     // G^{2 * gamma^2 * Z(t) * A_i(t)} for 0 <= i <= sap.num_variables()
    //     pub C_query_2: G1_vector<ppT>,

    //     // G^{gamma * Z(t)}
    //     pub G_gamma_Z: G1<ppT>,

    //     // H^{gamma * Z(t)}
    //     pub H_gamma_Z: G2<ppT>,

    //     // G^{(alpha + beta) * gamma * Z(t)}
    //     pub G_ab_gamma_Z: G1<ppT>,

    //     // G^{gamma^2 * Z(t)^2}
    //     pub G_gamma2_Z2: G1<ppT>,

    //     // G^{gamma^2 * Z(t) * t^i} for 0 <= i < sap.degree
    //     pub G_gamma2_Z_t: G1_vector<ppT>,

    //     pub constraint_system: r1cs_se_ppzksnark_constraint_system<ppT>,
}

pub trait VerificationKeyTConfig: Default + Clone {
    //  pub alphaA_g2: G2<ppT>,
    //     pub alphaB_g1: G1<ppT>,
    //     pub alphaC_g2: G2<ppT>,
    //     pub gamma_g2: G2<ppT>,
    //     pub gamma_beta_g1: G1<ppT>,
    //     pub gamma_beta_g2: G2<ppT>,
    //     pub rC_Z_g2: G2<ppT>,

    //     pub encoded_IC_query: accumulation_vector<G1<ppT>>,

    //  pub alpha_g1_beta_g2: GT<ppT>,
    //     pub gamma_g2: G2<ppT>,
    //     pub delta_g2: G2<ppT>,

    //     pub gamma_ABC_g1: accumulation_vector<G1<ppT>>,

    //  // H
    //     pub H: G2<ppT>,

    //     // G^{alpha}
    //     pub G_alpha: G1<ppT>,

    //     // H^{beta}
    //     pub H_beta: G2<ppT>,

    //     // G^{gamma}
    //     pub G_gamma: G1<ppT>,

    //     // H^{gamma}
    //     pub H_gamma: G2<ppT>,

    //     // G^{gamma * A_i(t) + (alpha + beta) * A_i(t)}
    //     // for 0 <= i <= sap.num_inputs()
    //     pub query: G1_vector<ppT>,
}
