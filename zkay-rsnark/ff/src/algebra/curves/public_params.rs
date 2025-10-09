

/*
  for every curve the user should define corresponding
  public_params with the following typedefs:

  Fp_type
  G1_type
  G2_type
  G1_precomp_type
  G2_precomp_type
  affine_ate_G1_precomp_type
  affine_ate_G2_precomp_type
  Fq_type
  Fqe_type
  Fqk_type
  GT_type

  one should also define the following static methods:

  void init_public_params();

  GT<EC_ppT> final_exponentiation(const Fqk<EC_ppT> &elt);

  G1_precomp<EC_ppT> precompute_G1(const G1<EC_ppT> &P);
  G2_precomp<EC_ppT> precompute_G2(const G2<EC_ppT> &Q);

  Fqk<EC_ppT> miller_loop(const G1_precomp<EC_ppT> &prec_P,
                          const G2_precomp<EC_ppT> &prec_Q);

  affine_ate_G1_precomp<EC_ppT> affine_ate_precompute_G1(const G1<EC_ppT> &P);
  affine_ate_G2_precomp<EC_ppT> affine_ate_precompute_G2(const G2<EC_ppT> &Q);


  Fqk<EC_ppT> affine_ate_miller_loop(const affine_ate_G1_precomp<EC_ppT> &prec_P,
                                     const affine_ate_G2_precomp<EC_ppT> &prec_Q);
  Fqk<EC_ppT> affine_ate_e_over_e_miller_loop(const affine_ate_G1_precomp<EC_ppT> &prec_P1,
                                              const affine_ate_G2_precomp<EC_ppT> &prec_Q1,
                                              const affine_ate_G1_precomp<EC_ppT> &prec_P2,
                                              const affine_ate_G2_precomp<EC_ppT> &prec_Q2);
  Fqk<EC_ppT> affine_ate_e_times_e_over_e_miller_loop(const affine_ate_G1_precomp<EC_ppT> &prec_P1,
                                                      const affine_ate_G2_precomp<EC_ppT> &prec_Q1,
                                                      const affine_ate_G1_precomp<EC_ppT> &prec_P2,
                                                      const affine_ate_G2_precomp<EC_ppT> &prec_Q2,
                                                      const affine_ate_G1_precomp<EC_ppT> &prec_P3,
                                                      const affine_ate_G2_precomp<EC_ppT> &prec_Q3);
  Fqk<EC_ppT> double_miller_loop(const G1_precomp<EC_ppT> &prec_P1,
                                 const G2_precomp<EC_ppT> &prec_Q1,
                                 const G1_precomp<EC_ppT> &prec_P2,
                                 const G2_precomp<EC_ppT> &prec_Q2);

  Fqk<EC_ppT> pairing(const G1<EC_ppT> &P,
                      const G2<EC_ppT> &Q);
  GT<EC_ppT> reduced_pairing(const G1<EC_ppT> &P,
                             const G2<EC_ppT> &Q);
  GT<EC_ppT> affine_reduced_pairing(const G1<EC_ppT> &P,
                                    const G2<EC_ppT> &Q);
*/


// type Fr<EC_ppT> =  EC_ppT::Fp_type;

// type G1<EC_ppT> =  EC_ppT::G1_type;

// type G2<EC_ppT> =  EC_ppT::G2_type;

// type G1_precomp<EC_ppT> =  EC_ppT::G1_precomp_type;

// type G2_precomp<EC_ppT> =  EC_ppT::G2_precomp_type;

// type affine_ate_G1_precomp<EC_ppT> =  EC_ppT::affine_ate_G1_precomp_type;

// type affine_ate_G2_precomp<EC_ppT> =  EC_ppT::affine_ate_G2_precomp_type;

// type Fq<EC_ppT> =  EC_ppT::Fq_type;

// type Fqe<EC_ppT> =  EC_ppT::Fqe_type;

// type Fqk<EC_ppT> =  EC_ppT::Fqk_type;

// type GT<EC_ppT> =  EC_ppT::GT_type;


// type Fr_vector<EC_ppT> = Vec<Fr<EC_ppT> >;

// type G1_vector<EC_ppT> = Vec<G1<EC_ppT> >;

// type G2_vector<EC_ppT> = Vec<G2<EC_ppT> >;


