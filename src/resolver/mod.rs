use crate::ast::statements::{Param, Statement};

mod expressions;
mod statements;

pub struct Resolver {
    scopes: Vec<Vec<String>>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![vec![]],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(vec![]);
    }
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: String) -> usize {
        let frame = self.scopes.last_mut().unwrap();
        let slot = frame.len();
        frame.push(name);
        slot
    }

    pub fn resolve_name(&self, name: &str) -> Option<(usize, usize)> {
        for (depth, frame) in self.scopes.iter().rev().enumerate() {
            if let Some(slot) = frame.iter().position(|n| n == name) {
                return Some((depth, slot));
            }
        }
        None
    }

    pub fn collect_captures(&self, params: &[Param], body: &[Statement]) -> Vec<(usize, usize)> {
        let mut captures = vec![];
        let param_names: Vec<&str> = params.iter().map(|p| p.param_name.as_str()).collect();
        self.collect_captures_statements(body, &param_names, &mut captures);
        captures.sort();
        captures.dedup();
        captures
    }
}
