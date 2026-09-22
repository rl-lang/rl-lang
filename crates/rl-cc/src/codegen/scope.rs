use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use std::collections::HashMap;

impl<'a> CCodegen<'a> {
    pub fn declare(&mut self, rl_name: &str, c_name: &str) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(rl_name.to_string(), c_name.to_string());
    }

    pub fn lookup(&self, rl_name: &str) -> String {
        for scope in self.scopes.iter().rev() {
            if let Some(c_name) = scope.get(rl_name) {
                return c_name.clone();
            }
        }
        mangle(rl_name)
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        // Flat type maps are scoped too: shadowing declarations must not
        // leak their types past the block end.
        self.type_snapshots.push((
            self.var_types.clone(),
            self.nullable_vars.clone(),
            self.closure_return_types.clone(),
            self.closure_factories.clone(),
        ));
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
        if let Some((var_types, nullable_vars, closure_return_types, closure_factories)) =
            self.type_snapshots.pop()
        {
            self.var_types = var_types;
            self.nullable_vars = nullable_vars;
            self.closure_return_types = closure_return_types;
            self.closure_factories = closure_factories;
        }
    }

    /// Computes the shadowing-safe C name without registering it, for
    /// emission prefixes whose initializer must still see an outer binding
    /// of the same name. Pair with a later `declare`.
    pub fn peek_unique_name(&mut self, name: &str) -> String {
        if self.scopes.iter().any(|s| s.contains_key(name)) {
            let unique = format!("{}_{}", mangle(name), self.shadow_counter);
            self.shadow_counter += 1;
            unique
        } else {
            mangle(name)
        }
    }

    /// Declares `name`, generating a shadowing-safe C name: when an outer
    /// scope already binds the name, a suffixed name is used so later code
    /// sees the new binding. Returns the C name to emit.
    pub fn declare_unique(&mut self, name: &str) -> String {
        let c_name = self.peek_unique_name(name);
        self.declare(name, &c_name);
        c_name
    }

    pub fn temp_var(&mut self) -> String {
        let name = format!("_r_{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }
}
