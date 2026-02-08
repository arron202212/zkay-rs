//  Declaration of interfaces for basis change routines.

// /**
//  * Compute the Subproduct Tree of degree 2^M and store it in Tree T.
//  * Below we make use of the Subproduct Tree description from
//  * [Bostan and Schost 2005. Polynomial Evaluation and Interpolation on Special Sets of Points], on page 7.
//  */
// //
// // pub fn compute_subproduct_tree(m:usize, Vec<Vec<Vec<FieldT> > > &T);

// /**
//  * Perform the general change of basis from Monomial to Newton Basis with Subproduct Tree T.
//  * Below we make use of the MonomialToNewton and TNewtonToMonomial pseudocode from
//  * [Bostan and Schost 2005. Polynomial Evaluation and Interpolation on Special Sets of Points], on page 12 and 14.
//  */
//
// pub fn monomial_to_newton_basis(a:&Vec<FieldT>, T:&Vec<Vec<Vec<FieldT> > > , n:usize);

// /**
//  * Perform the general change of basis from Newton to Monomial Basis with Subproduct Tree T.
//  * Below we make use of the NewtonToMonomial pseudocode from
//  * [Bostan and Schost 2005. Polynomial Evaluation and Interpolation on Special Sets of Points], on page 11.
//  */
//
// pub fn newton_to_monomial_basis(a:&Vec<FieldT>, T:&Vec<Vec<Vec<FieldT> > > , n:usize);

// /**
//  * Perform the change of basis from Monomial to Newton Basis for geometric sequence.
//  * Below we make use of the psuedocode from
//  * [Bostan & Schost 2005. Polynomial Evaluation and Interpolation on Special Sets of Points] on page 26.
//  */
//
// pub fn monomial_to_newton_basis_geometric(a:&Vec<FieldT>,
//                                          geometric_sequence:&Vec<FieldT>,
//                                          geometric_triangular_sequence:&Vec<FieldT>,
//                                         n:usize);

// /**
//  * Perform the change of basis from Newton to Monomial Basis for geometric sequence
//  * Below we make use of the psuedocode from
//  * [Bostan & Schost 2005. Polynomial Evaluation and Interpolation on Special Sets of Points] on page 26.
//  */
//
// pub fn newton_to_monomial_basis_geometric(a:&Vec<FieldT>,
//                                          geometric_sequence:&Vec<FieldT>,
//                                          geometric_triangular_sequence:&Vec<FieldT>,
//                                         n:usize);

use crate::evaluation_domain::domains::basic_radix2_domain_aux;
use crate::polynomial_arithmetic::basic_operations::*;
use crate::polynomial_arithmetic::xgcd::_polynomial_xgcd;
use ffec::common::utils::log2;

//
pub fn compute_subproduct_tree<
    FieldT: Clone
        + std::convert::From<i32>
        + std::cmp::PartialEq
        + num_traits::Zero
        + num_traits::One
        + std::default::Default,
>(
    m: usize,
    T: &mut Vec<Vec<Vec<FieldT>>>,
) {
    if T.len() != m + 1 {
        T.resize(m + 1, vec![]);
    }

    /*
     * Subproduct tree T is represented as a 2-dimensional array T_{i, j}.
     * T_{i, j} = product_{l = [2^i * j] to [2^i * (j+1) - 1]} (x - x_l)
     * Note: n = 2^m.
     */

    /* Precompute the first row. */
    T[0] = vec![vec![]; 1usize << m];
    for j in 0..m {
        T[0][j] = vec![FieldT::one(); 2];
        T[0][j][0] = FieldT::from(-(j as i32));
    }

    let mut a = vec![];
    let mut b = vec![];

    let mut index = 0;
    for i in 1..=m {
        T[i] = vec![vec![]; 1usize << (m - i)];
        for j in 0..(m - i) {
            a = T[i - 1][index].clone();
            index += 1;

            b = T[i - 1][index].clone();
            index += 1;

            _polynomial_multiplication(&mut T[i][j], &a, &b);
        }
        index = 0;
    }
}

//
pub fn monomial_to_newton_basis<
    FieldT: num_traits::Zero
        + std::default::Default
        + std::ops::Neg<Output = FieldT>
        + std::ops::SubAssign
        + std::cmp::PartialEq
        + std::ops::AddAssign
        + std::ops::Sub<Output = FieldT>
        + Clone
        + num_traits::One,
>(
    a: &mut Vec<FieldT>,
    T: &Vec<Vec<Vec<FieldT>>>,
    n: usize,
) -> eyre::Result<()> {
    let m = log2(n);
    if T.len() != m + 1usize {
        eyre::bail!("expected T.len() == m + 1");
    }

    /* MonomialToNewton */
    let mut I = T[m][0].clone();
    _reverse(&mut I, n);

    let mut mods = vec![FieldT::zero(); n + 1];
    mods[n] = FieldT::one();

    _polynomial_xgcd(
        &mods,
        &I,
        &mut mods.clone(),
        &mut mods.clone(),
        &mut I.clone(),
    );

    I.resize(n, FieldT::zero());

    let mut Q = _polynomial_multiplication_transpose(n - 1, &I, a).unwrap();
    _reverse(&mut Q, n);

    /* TNewtonToMonomial */
    let mut c = vec![vec![]; n];
    c[0] = Q;

    let mut row_length;
    let mut c_vec;
    /* NB: unsigned reverse iteration: cannot do i >= 0, but can do i < m
    because unsigned integers are guaranteed to wrap around */
    for i in m..m {
        row_length = T[i].len() - 1;
        c_vec = 1usize << i;

        /* NB: unsigned reverse iteration */
        let t = 1usize << (m - i - 1);
        for j in (0..t).rev() {
            c[2 * j + 1] = _polynomial_multiplication_transpose(
                (1usize << i) - 1,
                &T[i][row_length - 2 * j],
                &c[j],
            )
            .unwrap();
            c[2 * j] = c[j].clone();
            c[2 * j].resize(c_vec, FieldT::zero());
        }
    }

    /* Store Computed Newton Basis Coefficients */
    let mut j = 0;
    /* NB: unsigned reverse iteration */
    for i in (0..c.len()).rev() {
        a[j] = c[i][0].clone();
        j += 1;
    }
    Ok(())
}

//
pub fn newton_to_monomial_basis<
    FieldT: Clone + num_traits::Zero + num_traits::One + std::cmp::PartialEq + std::default::Default,
>(
    a: &mut Vec<FieldT>,
    T: &Vec<Vec<Vec<FieldT>>>,
    n: usize,
) -> eyre::Result<()> {
    let m = log2(n);
    if T.len() != m + 1usize {
        eyre::bail!("expected T.len() == m + 1");
    }

    let mut f = vec![vec![]; n];
    for i in 0..n {
        f[i] = vec![a[i].clone()];
    }

    /* NewtonToMonomial */
    let mut temp = vec![FieldT::zero()];
    for i in 0..m {
        for j in 0..(m - i - 1) {
            _polynomial_multiplication(
                &mut temp.clone(),
                &T[i][2 * j].clone(),
                &f[2 * j + 1].clone(),
            );
            _polynomial_addition(&mut f[j].clone(), &f[2 * j].clone(), &temp.clone());
        }
    }

    *a = f[0].clone();
    Ok(())
}

//
pub fn monomial_to_newton_basis_geometric<
    FieldT: num_traits::Zero
        + Clone
        + num_traits::One
        + std::default::Default
        + std::cmp::PartialEq
        + std::ops::Neg<Output = FieldT>,
>(
    a: &mut Vec<FieldT>,
    geometric_sequence: &Vec<FieldT>,
    geometric_triangular_sequence: &Vec<FieldT>,
    n: usize,
) {
    let mut u = vec![FieldT::zero(); n];
    let mut w = vec![FieldT::zero(); n];
    let mut z = vec![FieldT::zero(); n];
    let mut f = vec![FieldT::zero(); n];
    u[0] = FieldT::one();
    w[0] = a[0].clone();
    z[0] = FieldT::one();
    f[0] = a[0].clone();

    for i in 1..n {
        // u[i] = u[i-1] * geometric_sequence[i] * (FieldT::one() - geometric_sequence[i]).inverse();
        // w[i] = a[i] * (u[i].inverse());
        // z[i] = u[i] * geometric_triangular_sequence[i].inverse();
        f[i] = w[i].clone() * geometric_triangular_sequence[i].clone();

        if i % 2 == 1 {
            z[i] = -z[i].clone();
            f[i] = -f[i].clone();
        }
    }

    w = _polynomial_multiplication_transpose(n - 1, &z, &f).unwrap();

    // #ifdef MULTICORE
    //#pragma omp parallel for
    //#endif
    for i in 0..n {
        a[i] = w[i].clone() * z[i].clone();
    }
}

//
pub fn newton_to_monomial_basis_geometric<
    FieldT: num_traits::Zero
        + std::default::Default
        + Clone
        + num_traits::One
        + std::ops::Mul
        + std::cmp::PartialEq
        + std::ops::Neg<Output = FieldT>
        + std::ops::Sub,
>(
    a: &mut Vec<FieldT>,
    geometric_sequence: &Vec<FieldT>,
    geometric_triangular_sequence: &Vec<FieldT>,
    n: usize,
) {
    let mut v = vec![FieldT::zero(); n];
    let mut u = vec![FieldT::zero(); n];
    let mut w = vec![FieldT::zero(); n];
    let mut z = vec![FieldT::zero(); n];
    v[0] = a[0].clone();
    u[0] = FieldT::one();
    w[0] = a[0].clone();
    z[0] = FieldT::one();

    for i in 1..n {
        v[i] = a[i].clone() * geometric_triangular_sequence[i].clone();
        if i % 2 == 1 {
            v[i] = -v[i].clone();
        }

        // u[i] = u[i-1] * geometric_sequence[i] * (FieldT::one() - geometric_sequence[i]).inverse();
        // w[i] = v[i] * u[i].inverse();

        // z[i] = u[i] * geometric_triangular_sequence[i].inverse();
        if i % 2 == 1 {
            z[i] = -z[i].clone();
        }
    }

    w = _polynomial_multiplication_transpose(n - 1, &u, &w).unwrap();

    // #ifdef MULTICORE
    //#pragma omp parallel for
    //#endif
    for i in 0..n {
        a[i] = w[i].clone() * z[i].clone();
    }
}
