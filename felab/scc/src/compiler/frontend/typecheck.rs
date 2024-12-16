use anyhow::Result;

use super::{ElabProgram, ElabStmt, Expr, Typ, Var};

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

    /// Type check the statements
    ///
    /// Returns true if type check passes, false if not
    /// Also returns the new variables that are _defined_ in the statements
    fn typecheck_stmts(&mut self, stmt: &ElabStmt) -> Result<(bool, Vec<Var>)> {
        unimplemented!("Implement stmts typechecker!")
    }

    /// Type check the statement
    ///
    /// Returns true if type check passes, false if not
    /// Also returns the new variables that are _defined_ in the statement
    fn typecheck_stmt(&mut self, stmt: &ElabStmt) -> Result<(bool, Vec<Var>)> {
        unimplemented!("Implement stmt typechecker!")
    }

    fn typecheck_expr(&mut self, expr: &Expr) -> Result<Typ> {
        unimplemented!("Implement expr typechecker!")
    }
}

// End your solution
