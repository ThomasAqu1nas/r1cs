use linear::{ops::{ConstraintOp, Linear, NestingDepth}, LinearComb};
use modular_math::mod_math;
use primitive_types::U256;

pub mod linear;

fn main() {
    let a = LinearComb::new(U256::from_dec_str(
        "17"
    ).unwrap(), vec![
        1,
        2,
        3,
        ], vec![        
        U256::from_dec_str("3").unwrap(), 
        U256::from_dec_str("2").unwrap(), 
        U256::from_dec_str("1").unwrap()
        ]
    );

    let b = LinearComb::new(U256::from_dec_str(
        "17"
    ).unwrap(), vec![
        1,
        2,
        3,
        ], vec![        
        U256::from_dec_str("9").unwrap(), 
        U256::from_dec_str("5").unwrap(), 
        U256::from_dec_str("3").unwrap()
        ]
    );

    let c = a.ladd(&b);
    let d = c.wmul(&U256::from_dec_str("9").unwrap());
    let e = d.lsub(&c);
    let f = e.ldiv(&U256::from_dec_str("123").unwrap());
    let g = f.lpow(&U256::from(13));
    let nesting = g.depth();
    let linear = g.linear_comb();
    let r1cs = g.r1cs();
    println!("{nesting}");
    println!("lienar comb: {:#?}", linear);
    println!("r1cs: {:#?}", r1cs);

}

