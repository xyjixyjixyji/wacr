use anyhow::Result;

use super::{ElabProgram, ElabStmt, Expr};

// Begin your solution
pub(crate) struct TypeChecker {}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Type check the elaborated AST
    ///
    /// Returns true if type check passes, false if not
    pub fn typecheck(&mut self, prog: &ElabProgram) -> Result<bool> {
        unimplemented!("Implement typechecker!")
    }

    fn typecheck_stmt(&mut self, stmt: &ElabStmt) -> Result<bool> {
        unimplemented!("Implement stmt typechecker!")
    }

    fn typecheck_expr(&mut self, expr: &Expr) -> Result<bool> {
        unimplemented!("Implement expr typechecker!")
    }
}

// End your solution
