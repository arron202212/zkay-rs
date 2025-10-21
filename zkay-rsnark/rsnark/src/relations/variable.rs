/** @file
 *****************************************************************************

 Declaration of interfaces for:
 - a variable (i.e., x_i),
 - a linear term (i.e., a_i * x_i), and
 - a linear combination (i.e., sum_i a_i * x_i).

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef VARIABLE_HPP_
// #define VARIABLE_HPP_

// use  <cstddef>
// use  <map>
// use  <string>
// use  <vector>



/**
 * Mnemonic typedefs.
 */
type var_index_t=size_t ;
type integer_coeff_t=long ;

// /**
//  * Forward declaration.
//  */
// 
// pub structlinear_term;

// /**
//  * Forward declaration.
//  */
// 
// pub structlinear_combination;

/********************************* Variable **********************************/

/**
 * A variable represents a formal expression of the form "x_{index}".
 */
// 
pub struct variable<FieldT> {
     index:var_index_t,
}
//     variable(index(index:var_index_t index = 0) :) {};

//     linear_term<FieldT> operator*(int_coeff:integer_coeff_t) const;
//     linear_term<FieldT> operator*(&field_coeff:FieldT) const;

//     linear_combination<FieldT> operator+(&other:linear_combination<FieldT>) const;
//     linear_combination<FieldT> operator-(&other:linear_combination<FieldT>) const;

//     linear_term<FieldT> operator-() const;

//     bool operator==(&other:variable<FieldT>) const;
// };

// 
// linear_term<FieldT> operator*(int_coeff:integer_coeff_t &var:variable<FieldT>);

// 
// linear_term<FieldT> operator*(field_coeff:&FieldT &var:variable<FieldT>);

// 
// linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &var:variable<FieldT>);

// 
// linear_combination<FieldT> operator+(field_coeff:&FieldT &var:variable<FieldT>);

// 
// linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &var:variable<FieldT>);

// 
// linear_combination<FieldT> operator-(field_coeff:&FieldT &var:variable<FieldT>);


/****************************** Linear term **********************************/

/**
 * A linear term represents a formal expression of the form "coeff * x_{index}".
 */
// 
pub struct linear_term<FieldT> {
     index:var_index_t,
     coeff:FieldT,
}
//     linear_term() {};
//     linear_term(&var:variable<FieldT>);
//     linear_term(var:&variable<FieldT> int_coeff:integer_coeff_t);
//     linear_term(var:&variable<FieldT> &field_coeff:FieldT);

//     linear_term<FieldT> operator*(int_coeff:integer_coeff_t) const;
//     linear_term<FieldT> operator*(&field_coeff:FieldT) const;

//     linear_combination<FieldT> operator+(&other:linear_combination<FieldT>) const;
//     linear_combination<FieldT> operator-(&other:linear_combination<FieldT>) const;

//     linear_term<FieldT> operator-() const;

//     bool operator==(&other:linear_term<FieldT>) const;
// };

// 
// linear_term<FieldT> operator*(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

// 
// linear_term<FieldT> operator*(field_coeff:&FieldT &lt:linear_term<FieldT>);

// 
// linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

// 
// linear_combination<FieldT> operator+(field_coeff:&FieldT &lt:linear_term<FieldT>);

// 
// linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

// 
// linear_combination<FieldT> operator-(field_coeff:&FieldT &lt:linear_term<FieldT>);


/***************************** Linear combination ****************************/


/**
 * A linear combination represents a formal expression of the form "sum_i coeff_i * x_{index_i}".
 */
// 
pub struct linear_combination<FieldT> {
    terms:std::vector<linear_term<FieldT> > ,
}

//     linear_combination() {};
//     linear_combination(int_coeff:integer_coeff_t);
//     linear_combination(&field_coeff:FieldT);
//     linear_combination(&var:variable<FieldT>);
//     linear_combination(&lt:linear_term<FieldT>);
//     linear_combination(&all_terms:std::vector<linear_term<FieldT> >);

//     /* for supporting range-based for loops over linear_combination */
//     typename std::vector<linear_term<FieldT> >::const_iterator begin() const;
//     typename std::vector<linear_term<FieldT> >::const_iterator end() const;

//     void add_term(&var:variable<FieldT>);
//     void add_term(var:&variable<FieldT> int_coeff:integer_coeff_t);
//     void add_term(var:&variable<FieldT> &field_coeff:FieldT);

//     void add_term(&lt:linear_term<FieldT>);

//     FieldT evaluate(&assignment:std::vector<FieldT>) const;

//     linear_combination<FieldT> operator*(int_coeff:integer_coeff_t) const;
//     linear_combination<FieldT> operator*(&field_coeff:FieldT) const;

//     linear_combination<FieldT> operator+(&other:linear_combination<FieldT>) const;

//     linear_combination<FieldT> operator-(&other:linear_combination<FieldT>) const;
//     linear_combination<FieldT> operator-() const;

//     bool operator==(&other:linear_combination<FieldT>) const;

//     bool is_valid(num_variables:size_t) const;

//     void print(variable_annotations = std::map<size_t:&std::map<size_t, std::string> std::string>()) const;
//     void print_with_assignment(variable_annotations = std::map<size_t:&std::vector<FieldT> &full_assignment, std::string>():std::map<size_t, std::string>) const;

//     friend std::ostream& operator<< <FieldT>(std::ostream &out, &lc:linear_combination<FieldT>);
//     friend std::istream& operator>> <FieldT>(std::istream &in, linear_combination<FieldT> &lc);
// };

// 
// linear_combination<FieldT> operator*(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>);

// 
// linear_combination<FieldT> operator*(field_coeff:&FieldT &lc:linear_combination<FieldT>);

// 
// linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>);

// 
// linear_combination<FieldT> operator+(field_coeff:&FieldT &lc:linear_combination<FieldT>);

// 
// linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>);

// 
// linear_combination<FieldT> operator-(field_coeff:&FieldT &lc:linear_combination<FieldT>);



// use crate::relations::variable;

//#endif // VARIABLE_HPP_



/** @file
 *****************************************************************************

 Implementation of interfaces for:
 - a variable (i.e., x_i),
 - a linear term (i.e., a_i * x_i), and
 - a linear combination (i.e., sum_i a_i * x_i).

 See variabe.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef VARIABLE_TCC_
// #define VARIABLE_TCC_

// use  <algorithm>
// use  <cassert>

use ffec::algebra::field_utils::bigint::bigint;



// 
// linear_term<FieldT> variable<FieldT>::operator*(int_coeff:integer_coeff_t) const
// {
//     return linear_term::<FieldT>(*this, int_coeff);
// }

// 
// linear_term<FieldT> variable<FieldT>::operator*(&field_coeff:FieldT) const
// {
//     return linear_term::<FieldT>(*this, field_coeff);
// }

// 
// linear_combination<FieldT> variable<FieldT>::operator+(&other:linear_combination<FieldT>) const
// {
//     linear_combination<FieldT> result;

//     result.add_term(*this);
//     result.terms.insert(result.terms.begin(), other.terms.begin(), other.terms.end());

//     return result;
// }

// 
// linear_combination<FieldT> variable<FieldT>::operator-(&other:linear_combination<FieldT>) const
// {
//     return (*this) + (-other);
// }

// 
// linear_term<FieldT> variable<FieldT>::operator-() const
// {
//     return linear_term::<FieldT>(*this, -FieldT::one());
// }

impl<ppT> PartialEq for r1cs_se_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
       self.index == other.index
    }
}

// 
// bool variable<FieldT>::operator==(&other:variable<FieldT>) const
// {
//     return (self.index == other.index);
// }

// 
// linear_term<FieldT> operator*(int_coeff:integer_coeff_t &var:variable<FieldT>)
// {
//     return linear_term::<FieldT>(var, int_coeff);
// }

// 
// linear_term<FieldT> operator*(field_coeff:&FieldT &var:variable<FieldT>)
// {
//     return linear_term::<FieldT>(var, field_coeff);
// }

// 
// linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &var:variable<FieldT>)
// {
//     return linear_combination<FieldT>(int_coeff) + var;
// }

// 
// linear_combination<FieldT> operator+(field_coeff:&FieldT &var:variable<FieldT>)
// {
//     return linear_combination<FieldT>(field_coeff) + var;
// }

// 
// linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &var:variable<FieldT>)
// {
//     return linear_combination<FieldT>(int_coeff) - var;
// }

// 
// linear_combination<FieldT> operator-(field_coeff:&FieldT &var:variable<FieldT>)
// {
//     return linear_combination<FieldT>(field_coeff) - var;
// }

impl linear_term<FieldT>{

pub fn new(&var:variable<FieldT>) ->Self
    
{
Self{index:var.index, coeff:FieldT::one()}
}


pub fn new1(var:&variable<FieldT> ,int_coeff:integer_coeff_t)  ->Self
{
Self{index:var.index, coeff:FieldT::from(int_coeff)}
}


pub fn new2(var:&variable<FieldT> ,coeff:FieldT)  ->Self
{
Self{index:var.index, coeff}
}
}

// 
// linear_term<FieldT> linear_term<FieldT>::operator*(int_coeff:integer_coeff_t) const
// {
//     return (self.operator*(FieldT(int_coeff)));
// }

// 
// linear_term<FieldT> linear_term<FieldT>::operator*(&field_coeff:FieldT) const
// {
//     return linear_term::<FieldT>(self.index, field_coeff * self.coeff);
// }

// 
// linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
// {
//     return linear_combination<FieldT>(int_coeff) + lt;
// }

// 
// linear_combination<FieldT> operator+(field_coeff:&FieldT &lt:linear_term<FieldT>)
// {
//     return linear_combination<FieldT>(field_coeff) + lt;
// }

// 
// linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
// {
//     return linear_combination<FieldT>(int_coeff) - lt;
// }

// 
// linear_combination<FieldT> operator-(field_coeff:&FieldT &lt:linear_term<FieldT>)
// {
//     return linear_combination<FieldT>(field_coeff) - lt;
// }

// 
// linear_combination<FieldT> linear_term<FieldT>::operator+(&other:linear_combination<FieldT>) const
// {
//     return linear_combination<FieldT>(*this) + other;
// }

// 
// linear_combination<FieldT> linear_term<FieldT>::operator-(&other:linear_combination<FieldT>) const
// {
//     return (*this) + (-other);
// }

// 
// linear_term<FieldT> linear_term<FieldT>::operator-() const
// {
//     return linear_term::<FieldT>(self.index, -self.coeff);
// }

impl<FieldT> PartialEq for linear_term<FieldT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
       self.index == other.index &&
            self.coeff == other.coeff
    }
}

// 
// bool linear_term<FieldT>::operator==(&other:linear_term<FieldT>) const
// {
//     return (self.index == other.index &&
//             self.coeff == other.coeff);
// }

// 
// linear_term<FieldT> operator*(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
// {
//     return FieldT(int_coeff) * lt;
// }

// 
// linear_term<FieldT> operator*(field_coeff:&FieldT &lt:linear_term<FieldT>)
// {
//     return linear_term::<FieldT>(lt.index, field_coeff * lt.coeff);
// }

impl linear_combination<FieldT>{

pub fn new(int_coeff:integer_coeff_t)
{
    self.add_term(linear_term::<FieldT>(0, int_coeff));
}


pub fn new2(field_coeff:&FieldT)
{
    self.add_term(linear_term::<FieldT>(0, field_coeff));
}


pub fn new3(var:&variable<FieldT>)
{
    self.add_term(var);
}


pub fn new4(lt:&linear_term<FieldT>)
{
    self.add_term(lt);
}


// typename std::vector<linear_term<FieldT> >::const_iterator linear_combination<FieldT>::begin() const
// {
//     return terms.begin();
// }


// typename std::vector<linear_term<FieldT> >::const_iterator linear_combination<FieldT>::end() const
// {
//     return terms.end();
// }


pub fn add_term(var:&variable<FieldT>)
{
    self.terms.push(linear_term::<FieldT>::new(var.index, FieldT::one()));
}


pub fn add_term2(var:&variable<FieldT>, int_coeff:integer_coeff_t)
{
    self.terms.push(linear_term::<FieldT>::new(var.index, int_coeff));
}


pub fn add_term3(var:&variable<FieldT>, coeff:FieldT)
{
    self.terms.push(linear_term::<FieldT>::new(var.index, coeff));
}


pub fn add_term4(other:linear_term<FieldT>)
{
    self.terms.push(other);
}


// linear_combination<FieldT> linear_combination<FieldT>::operator*(int_coeff:integer_coeff_t) const
// {
//     return (*this) * FieldT(int_coeff);
// }


 pub fn evaluate(assignment:std::vector<FieldT>) ->FieldT
{
    let mut  acc = FieldT::zero();
    for lt in &terms
    {
        acc += if lt.index == 0 {FieldT::one()} else{assignment[lt.index-1]} * lt.coeff;
    }
    return acc;
}
}


 pub fn is_valid(num_variables:size_t) ->bool
{
    /* check that all terms in linear combination are sorted */
    for i in 1..terms.size()
    {
        if terms[i-1].index >= terms[i].index
        {
            return false;
        }
    }

    /* check that the variables are in proper range. as the variables
       are sorted, it suffices to check the last term */
   self.terms.last().unwrap().index< num_variables
}


pub fn print(&variable_annotations:std::map<size_t, std::string>) 
{
    for lt in &terms
    {
        if lt.index == 0
        {
            print!("    1 * ");
            lt.coeff.print();
        }
        else
        {
            print!("    x_{} ({}) * ", lt.index, (if let Some(it) = variable_annotations.find(lt.index){it.1.to_string()} else{"no annotation" }));
            lt.coeff.print();
        }
    }
}


pub fn print_with_assignment(full_assignment:&std::vector<FieldT>,variable_annotations:&std::map<size_t, std::string>) 
{
    for lt in &terms
    {
        if lt.index == 0
        {
            print!("    1 * ");
            lt.coeff.print();
        }
        else
        {
            print!("    x_{} * ", lt.index);
            lt.coeff.print();

            
            print!("    where x_{} ({}) was assigned value ", lt.index,
                   (if let Some(it) = variable_annotations.find(lt.index){it.1.to_string()} else {"no annotation"}));
            full_assignment[lt.index-1].print();
            print!("      i.e. negative of ");
            (-full_assignment[lt.index-1]).print();
        }
    }
}




impl<ppT> fmt::Display for r1cs_se_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}",  
lc.terms.size(),
lc.terms.iter().map(|lt| format!("{}\n{}{OUTPUT_NEWLINE}",lt.index,lt.coeff)).collect::<String>(),
)
    }
}


// 
// std::ostream& operator<<(std::ostream &out, &lc:linear_combination<FieldT>)
// {
//     out << lc.terms.size() << "\n";
//     for lt in &lc.terms
//     {
//         out << lt.index << "\n";
//         out << lt.coeff << OUTPUT_NEWLINE;
//     }

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, linear_combination<FieldT> &lc)
// {
//     lc.terms.clear();

//     size_t s;
//     in >> s;

//     ffec::consume_newline(in);

//     lc.terms.reserve(s);

//     for i in 0..s
//     {
//         linear_term<FieldT> lt;
//         in >> lt.index;
//         ffec::consume_newline(in);
//         in >> lt.coeff;
//         ffec::consume_OUTPUT_NEWLINE(in);
//         lc.terms.push(lt);
//     }

//     return in;
// }

// 
// linear_combination<FieldT> operator*(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>)
// {
//     return lc * int_coeff;
// }

// 
// linear_combination<FieldT> operator*(field_coeff:&FieldT &lc:linear_combination<FieldT>)
// {
//     return lc * field_coeff;
// }

// 
// linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>)
// {
//     return linear_combination<FieldT>(int_coeff) + lc;
// }

// 
// linear_combination<FieldT> operator+(field_coeff:&FieldT &lc:linear_combination<FieldT>)
// {
//     return linear_combination<FieldT>(field_coeff) + lc;
// }

// 
// linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>)
// {
//     return linear_combination<FieldT>(int_coeff) - lc;
// }

// 
// linear_combination<FieldT> operator-(field_coeff:&FieldT &lc:linear_combination<FieldT>)
// {
//     return linear_combination<FieldT>(field_coeff) - lc;
// }
impl linear_combination<FieldT>{

pub fn new(all_terms:&std::vector<linear_term<FieldT> >)
{
    if all_terms.empty()
    {
        return;
    }

    let mut terms = all_terms.clone();
    terms.sort_unstable_by_key(|v| v.index);

    // auto result_it = terms.begin();
    let mut j=0;
    for i in 1.terms.len()
    {
        if terms[i].index == terms[j].index
        {
            terms[j].coeff += terms[i].coeff;
        }
        else
        {
            j+=1;
            terms[j]=terms[i].clone();
        }
    }
    terms.resize(j + 1,FieldT::zero());
}

}

//#endif // VARIABLE_TCC

// 
// linear_combination<FieldT> linear_combination<FieldT>::operator*(&field_coeff:FieldT) const
// {
//     linear_combination<FieldT> result;
//     result.terms.reserve(self.terms.size());
//     for lt in &self.terms
//     {
//         result.terms.push(lt * field_coeff);
//     }
//     return result;
// }

// 
// linear_combination<FieldT> linear_combination<FieldT>::operator+(&other:linear_combination<FieldT>) const
// {
//     linear_combination<FieldT> result;
//     result.terms.reserve(self.terms.size() + other.terms.size());

//     auto it1 = self.terms.begin();
//     auto it2 = other.terms.begin();

//     /* invariant: it1 and it2 always point to unprocessed items in the corresponding linear combinations */
//     while (it1 != self.terms.end() && it2 != other.terms.end())
//     {
//         if it1->index < it2->index
//         {
//             result.terms.push(*it1);
//             it1+=1;
//         }
//         else if it1->index > it2->index
//         {
//             result.terms.push(*it2);
//             it2+=1;
//         }
//         else
//         {
//             /* it1->index == it2->index */
//             result.terms.push(variable<FieldT>(it1->index), it1->coeff + it2->coeff);
//             it1+=1;
//             it2+=1;
//         }
//     }

//     if it1 != self.terms.end()
//     {
//         result.terms.insert(result.terms.end(), it1, self.terms.end());
//     }
//     else
//     {
//         result.terms.insert(result.terms.end(), it2, other.terms.end());
//     }

//     return result;
// }

// 
// linear_combination<FieldT> linear_combination<FieldT>::operator-(&other:linear_combination<FieldT>) const
// {
//     return (*this) + (-other);
// }

// 
// linear_combination<FieldT> linear_combination<FieldT>::operator-() const
// {
//     return (*this) * (-FieldT::one());
// }

// 
// bool linear_combination<FieldT>::operator==(&other:linear_combination<FieldT>) const
// {
//     return (self.terms == other.terms);
// }
