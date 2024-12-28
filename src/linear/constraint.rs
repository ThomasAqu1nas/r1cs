use super::LinearComb;

#[derive(Debug, Clone)]
pub struct Constraint {
    a: LinearComb,
    b: LinearComb,
    c: LinearComb
}

impl Constraint {
    pub fn new(a: LinearComb, b: LinearComb, c: LinearComb) -> Self {
        Self { a, b, c }
    }
}