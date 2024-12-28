use std::collections::HashMap;

use primitive_types::U256;
use super::{constraint::Constraint, r1cs::R1CS, LinearComb};


// Structs
////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct AddLC<Op: ConstraintOp> {
    inner_op: Op,
    linear_comb: LinearComb,
    constraint: Constraint
}

#[derive(Clone)]
pub struct DivLC<Op: ConstraintOp> {
    inner_op: Op,
    linear_comb: LinearComb,
    constraint: Constraint
}

#[derive(Clone)]
pub struct SmulLC<Op: ConstraintOp> {
    inner_op: Op,
    linear_comb: LinearComb,
    constraint: Constraint
}

#[derive(Clone)]
pub struct SubLC<Op: ConstraintOp> {
    inner_op: Op,
    linear_comb: LinearComb,
    constraint: Constraint
}

#[derive(Clone)]
pub struct PowLC<Op: ConstraintOp> {
    inner_op: Op,
    linear_comb: LinearComb,
    constraint: Constraint
}


// NestingDepth
////////////////////////////////////////////////////////////////
pub trait NestingDepth {
    const DEPTH: usize;
    fn depth(&self) -> usize {
        Self::DEPTH
    }
}

impl<T: NestingDepth + ConstraintOp + Clone> NestingDepth for AddLC<T> {
    const DEPTH: usize = T::DEPTH + 1;
}

impl<T: NestingDepth + ConstraintOp + Clone> NestingDepth for DivLC<T> {
    const DEPTH: usize = T::DEPTH + 1;
}

impl<T: NestingDepth + ConstraintOp + Clone> NestingDepth for SubLC<T> {
    const DEPTH: usize = T::DEPTH + 1;
}

impl<T: NestingDepth + ConstraintOp + Clone> NestingDepth for SmulLC<T> {
    const DEPTH: usize = T::DEPTH + 1;
}

impl<T: NestingDepth + ConstraintOp + Clone> NestingDepth for PowLC<T> {
    const DEPTH: usize = T::DEPTH + 1;
}

impl NestingDepth for LinearComb {
    const DEPTH: usize = 0;
} 


// Linear
////////////////////////////////////////////////////////////////
pub trait Linear {
    type Inner: ConstraintOp;
    fn linear_comb(&self) -> LinearComb;
    fn constraint(&self) -> Constraint;
    fn wise_mul_linear_comb(&mut self, scalar: &U256);
    fn wise_div_linear_comb(&mut self, scalar: &U256);
    fn modulus(&self) -> U256;
    fn indexes(&self) -> Vec<usize>;
    fn scalars(&self) -> Vec<U256>;
    fn terms(&self) -> HashMap<usize, U256>;
    fn inner(&self) -> Self::Inner;
}

macro_rules! impl_linear {
    ($type:ident) => {
        impl<Op: ConstraintOp> Linear for $type<Op> {
            type Inner = Op;

            fn linear_comb(&self) -> LinearComb {
                self.linear_comb.clone()
            }

            fn constraint(&self) -> Constraint {
                self.constraint.clone()
            }

            fn wise_mul_linear_comb(&mut self, scalar: &U256) {
                self.linear_comb = self.linear_comb.elem_wise_mul(*scalar);
            }

            fn wise_div_linear_comb(&mut self, scalar: &U256) {
                self.linear_comb = self.linear_comb.elem_wise_div(*scalar);
            }

            fn modulus(&self) -> U256 {
                self.linear_comb.modulus.clone()
            }

            fn indexes(&self) -> Vec<usize> {
                self.linear_comb.terms.keys().cloned().collect()
            }

            fn scalars(&self) -> Vec<U256> {
                self.linear_comb.terms.values().cloned().collect()
            }

            fn terms(&self) -> HashMap<usize, U256> {
                self.linear_comb.terms.clone()
            }

            fn inner(&self) -> Self::Inner {
                self.inner_op.clone()
            }
        }
    };
}

impl_linear!(AddLC);
impl_linear!(SmulLC);
impl_linear!(SubLC);
impl_linear!(DivLC);
impl_linear!(PowLC);


// ConstraintOp
////////////////////////////////////////////////////////////////
pub trait ConstraintOp: Linear + Sized + Clone + NestingDepth {

    fn ladd(&self, rhs: &impl ConstraintOp) -> AddLC<Self> {
        assert_eq!(self.linear_comb().modulus, rhs.linear_comb().modulus);
        let result = self.linear_comb() + rhs.linear_comb();
        let a = result.linear_comb();
        let b = LinearComb::one(self.modulus());
        let c = LinearComb::new(
            self.modulus(), 
            vec![self.depth()], 
            vec![U256::one()]
        );
        let constraint = Constraint::new(a, b, c);
        AddLC { inner_op: self.clone(), linear_comb: result, constraint }
    }
    fn wmul(&self, rhs: &U256) -> Self {
        let mut res = self.clone();
        res.wise_mul_linear_comb(rhs);
        res
    }
    fn ldiv(&self, rhs: &U256) -> Self {
        let mut res = self.clone();
        res.wise_div_linear_comb(rhs);
        res
    }
    fn lsub(&self, rhs: &impl ConstraintOp) -> SubLC<Self> {
        assert_eq!(self.modulus(), rhs.modulus());
        let result = self.linear_comb() - rhs.linear_comb();
        let a = result.linear_comb();
        let b = LinearComb::one(self.modulus());
        let c = LinearComb::new(
            self.modulus(), 
            vec![self.depth()], 
            vec![U256::one()]
        );
        let constraint = Constraint::new(a, b, c);
        SubLC { inner_op: self.clone(), linear_comb: result, constraint }
    }
    fn scalar_mul(&self, rhs: &impl ConstraintOp) -> SmulLC<Self> {
        assert_eq!(self.modulus(), rhs.modulus());
        assert_eq!(self.indexes(), rhs.indexes());
        let result = self.linear_comb().scalars()
            .into_iter().zip(rhs.linear_comb().scalars())
            .map(|(a, b)| a * b)
            .collect::<Vec<_>>();
        let a = self.linear_comb();
        let b = rhs.linear_comb();
        let c = LinearComb::new(
            self.modulus(), 
            vec![self.depth()], 
            vec![U256::one()]
        );
        let constraint = Constraint::new(a, b, c);
        let new_linear_comb = super::LinearComb::new(
            self.modulus(),
            self.indexes(),
            result
        );
        SmulLC { inner_op: self.clone(), linear_comb: new_linear_comb, constraint }
    }
    fn lpow(&self, exp: &U256) -> PowLC<Self> {
        let mut result = super::LinearComb::one(self.modulus());
        let one = U256::one();
        let mut i = U256::zero();

        while i < *exp {
            let tmp_lc = SmulLC {
                inner_op: self.clone(),
                linear_comb: result.clone(),
                constraint: Constraint::new(
                    result.clone(),
                    LinearComb::one(self.modulus()),
                    LinearComb::one(self.modulus()),
                ),
            };

            let mul_res = ConstraintOp::scalar_mul(&tmp_lc, self);
            result = mul_res.linear_comb().clone();

            i = i + one;
        }

        let a = result.clone();
        let b = LinearComb::one(self.modulus());
        let c = LinearComb::one(self.modulus());
        let constraint = Constraint::new(a, b, c);

        PowLC {
            inner_op: self.clone(),
            linear_comb: result,
            constraint,
        }
    }
    fn r1cs(&self) -> R1CS 
        where Self: ConstraintOp
    {
        let mut r1cs = R1CS::new();
        // НУО вложенность >= 1;
        let mut depth = self.depth();

        r1cs.add_constraint(self.constraint());
        r1cs.extend_variables(self.indexes());
        
        depth -= 1;

        loop {
            let _self = self.inner();
            if depth == 0 {
                break;
            } else {
                r1cs.add_constraint(_self.constraint());
                r1cs.extend_variables(_self.indexes());
                depth -= 1;
            }
        }
        r1cs
    }
}

impl ConstraintOp for LinearComb {}
impl<Op: ConstraintOp> ConstraintOp for AddLC<Op> {}
impl<Op: ConstraintOp> ConstraintOp for SubLC<Op> {}
impl<Op: ConstraintOp> ConstraintOp for SmulLC<Op> {}
impl<Op: ConstraintOp> ConstraintOp for DivLC<Op> {}
impl<Op: ConstraintOp> ConstraintOp for PowLC<Op> {}

////////////////////////
