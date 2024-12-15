use super::ast::{Expr, Typ, Var};

pub type ElabProgram = Vec<ElabStmt>;

#[derive(Debug, Clone)]
pub enum ElabStmt {
    Decl(Typ, Var, Option<Expr>),

    // asnop should always be "=" now
    Asgn(Var, Expr),

    Expr(Expr),

    // block
    Block(ElabStmts),

    // control
    If(Expr, ElabStmts, Option<ElabStmts>),
    While(Expr, ElabStmts),
    Ret(Expr),
}

#[derive(Debug, Clone)]
pub enum ElabStmts {
    // Stmts that introduce a new scope, make it easier for typechecking
    NewScopeStmts(Vec<ElabStmt>),
    // Normal stmts that do not introduce a new scope
    NormalStmts(Vec<ElabStmt>),
}
