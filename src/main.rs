use std::collections::HashMap;

use linear::{ops::{ConstraintOp, Linear, NestingDepth}, LinearComb};
use modular_math::mod_math;
use primitive_types::U256;

pub mod linear;

fn main() {
    let a: LinearComb = LinearComb::new_terms(U256::from_dec_str(
        "17"
    ).unwrap(),
    HashMap::from([
        (1usize, U256::from_dec_str("214").unwrap()),
        (2usize, U256::from_dec_str("12412").unwrap()),
        (3usize, U256::from_dec_str("5645").unwrap())
    ]));

    let b: LinearComb = LinearComb::new_terms(U256::from_dec_str(
        "17"
    ).unwrap(),
    HashMap::from([
        (1usize, U256::from_dec_str("5674").unwrap()),
        (2usize, U256::from_dec_str("345347").unwrap()),
        (3usize, U256::from_dec_str("524").unwrap())
    ]));

    let pow = a.lpow(&U256::from(3));
    println!("\n\n\n");
    println!("pow constraints: {:#?}", pow.constraints());
    println!("pow constraints len: {:}", pow.constraints().len());
    println!("{}", pow.linear_comb());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use primitive_types::U256;

    use crate::linear::LinearComb;

    #[test]
    fn check_display() {
        let a: LinearComb = LinearComb::new_terms(U256::from_dec_str(
            "17"
        ).unwrap(),
        HashMap::from([
            (1usize, U256::from_dec_str("214").unwrap()),
            (2usize, U256::from_dec_str("12412").unwrap()),
            (3usize, U256::from_dec_str("5645").unwrap())
        ]));
        println!("{a}");
    }
}
// a ^ 3
// a * a = t1
// a * t1 = t2
// variables (a, t1, t2)
// indexes: {1, 2, 3}: {x1, x2, x3}


// a ^ 5
// a * a = t1
// a * t1 = t2
// a * t2 = t3
// a * t3 = t4
// variables (a, t1, t2, t3, t4)
// indexes: {1, 2, 3, 4, 5}: {x1, x2, x3, x4, x5}
