#[derive(Debug, Clone, Copy)]
pub enum Typ {
    Int,
    Bool,
}

pub type Var = String;

pub type Program = Vec<Stmt>;

#[derive(Clone, Debug)]
pub enum Stmt {
    Decl(Typ, Var, Option<Expr>),
    Asgn(Expr, AsnOp, Expr),

    Expr(Expr),

    // block
    Block(Vec<Stmt>),

    // control
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(Option<Box<Stmt>>, Expr, Option<Box<Stmt>>, Box<Stmt>),
    Ret(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Variable(Var),
    Binop(Box<Expr>, BinOp, Box<Expr>),
    Unop(UnOp, Box<Expr>),
    True,
    False,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnOp {
    Neg,    // -
    Not,    // !
    BitNot, // ~
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,         // +
    Sub,         // -
    Mul,         // *
    Div,         // /
    LessThan,    // <
    GreaterThan, // >
    EqEq,        // ==
    Uneq,        // !=
    LogicalAnd,  // &&
    LogicalOr,   // ||
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AsnOp {
    Eq,      // =
    PlusEq,  // +=
    MinusEq, // -=
    TimesEq, // *=
    DivEq,   // /=
}
