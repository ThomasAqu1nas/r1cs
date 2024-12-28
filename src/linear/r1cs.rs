use std::collections::HashSet;

use super::constraint::Constraint;

#[derive(Debug)]
pub struct R1CS {
    constraints: Vec<Constraint>,
    indexes: Vec<usize>
}

impl R1CS {
    pub fn new() -> Self {
        R1CS {
            constraints: Vec::new(),
            indexes: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, index: usize) {
        if !self.indexes.contains(&index) {
            self.indexes.push(index);
        }
    }

    pub fn extend_variables(&mut self, indexes: Vec<usize>) {
        let mut hs_indexes = self.indexes.iter().copied().collect::<HashSet<_>>();
        hs_indexes.extend(indexes);
        self.indexes = hs_indexes.iter().copied().collect();
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
}