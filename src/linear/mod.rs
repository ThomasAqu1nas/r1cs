pub mod ops;
pub mod constraint;
pub mod r1cs;
use std::{collections::HashMap, ops::{Add, Mul, Sub}};
use constraint::Constraint;
use modular_math::mod_math;
use ops::Linear;
use primitive_types::U256;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinearComb {
    modulus: U256,
    terms: HashMap<usize, U256>,
}

impl Linear for LinearComb {
    fn linear_comb(&self) -> LinearComb {
        self.clone()
    }

    fn modulus(&self) -> U256 {
        self.modulus.clone()
    }

    fn wise_mul_linear_comb(&mut self, scalar: &U256) {
        self.scalars().iter_mut().for_each(|x| { let _ = x.mul(scalar); });
    }

    fn wise_div_linear_comb(&mut self, scalar: &U256) {
        self.scalars().iter_mut().for_each(|x| { x.div_mod(*scalar); });
    }

    fn indexes(&self) -> Vec<usize> {
        LinearComb::indexes(&self)
    }

    fn scalars(&self) -> Vec<U256> {
        self.scalars()
    }

    fn terms(&self) -> HashMap<usize, U256> {
        self.terms.clone()
    }
    
    fn constraint(&self) -> constraint::Constraint {
        Constraint::new(
            LinearComb::new(self.modulus, self.indexes(), self.scalars()), 
            LinearComb::new(self.modulus, self.indexes(), self.scalars()), 
            LinearComb::new(self.modulus, self.indexes(), self.scalars()), 
        )
    }

    fn inner(&self) -> Self::Inner {
        self.clone()
    }
    
    type Inner = Self;
}

impl LinearComb {
    pub fn new(modulus: U256, indexes: Vec<usize>, scalars: Vec<U256>) -> Self {
        assert_eq!(indexes.len(), scalars.len());
        let indexes = [0usize..indexes.len()]
            .iter().enumerate().map(|(i, _)| i)
            .collect::<Vec<_>>();
        let result_terms = indexes.iter().zip(&scalars)
            .map(|(&index, &scalar)| {
                (index, scalar)
            })
            .collect::<HashMap<_, _>>();
        Self { modulus, terms: result_terms }
    }

    pub fn new_terms(modulus: U256, terms: HashMap<usize, U256>) -> Self {
        assert_eq!(terms.keys().len(), terms.values().len());
        Self { modulus, terms }
    }

    fn scalars(&self) -> Vec<U256> {
        self.terms.iter()
            .map(|(_index, scalar)| {
                *scalar
            })
            .collect()
    } 

    fn indexes(&self) -> Vec<usize> {
        self.terms.iter()
            .map(|(index, _scalar)| {
                *index
            })
            .collect()
    } 

    pub fn get(&self, values: &[U256]) -> U256 {
        assert_eq!(self.scalars().len(), values.len());
        let math = self.math();
        self.scalars().iter()
            .enumerate()
            .fold(U256::zero(), |acc, (i, &scalar)| {
                let value = values[i];
                math.add(acc, math.mul(value, scalar))
            })
    }

    fn math(&self) -> mod_math::ModMath {
        mod_math::ModMath::new(self.modulus)
    }

    pub fn elem_wise_mul(&self, scalar: U256) -> Self {

        let result = self.indexes().iter()
            .zip(self.scalars()) 
            .map(|(&index, sc)| {
                (index, self.math().mul(sc, scalar))
            })
            .collect();
        
        Self { modulus: self.modulus, terms: result }
    }

    pub fn elem_wise_div(&self, scalar: U256) -> Self {
        let result = self.indexes().iter()
            .zip(self.scalars()) 
            .map(|(&index, sc)| {
                (index, self.math().div(sc, scalar))
            })
            .collect();
        Self { modulus: self.modulus, terms: result }
    }

    pub fn one(modulus: U256) -> Self {
        let mut result = HashMap::<usize, U256>::new();
        result.insert(0, U256::one());
        Self { modulus, terms: result }
    }
}

impl Add for LinearComb {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.scalars().len(), rhs.scalars().len());
        assert_eq!(self.indexes(), rhs.indexes());
        assert_eq!(self.modulus, rhs.modulus);
        let math = self.math();
        let result = self.scalars().iter().zip(rhs.scalars())
            .zip(self.indexes())
            .map(|((&s1, s2), i)| {
                (i, math.add(s1, s2))
            }).collect();
        Self::Output {
            modulus: self.modulus,
            terms: result
        }
    }
}

impl Sub for LinearComb {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.scalars().len(), rhs.scalars().len());
        assert_eq!(self.indexes(), rhs.indexes());
        assert_eq!(self.modulus, rhs.modulus);
        let math = self.math();
        let result = self.scalars().iter().zip(rhs.scalars())
            .zip(self.indexes())
            .map(|((&s1, s2), i)| {
                (i, math.sub(s1, s2))
            }).collect();
        Self::Output {
            modulus: self.modulus,
            terms: result
        }
    }
}


