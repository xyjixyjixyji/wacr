use std::cell::RefCell;

use anyhow::{anyhow, Result};
use logos::{Logos, Span};

use crate::{Emission, SccArguments};

use super::{
    frontend::{elab_program, ElabProgram, Program, ProgramParser, Token, TypeChecker},
    Compiler,
};

/// The SimpC Compiler, with passed in commandline arguments,
/// all compiling pipeline starts here
pub struct SimpCCompiler {
    args: SccArguments,
    typechecker: RefCell<TypeChecker>,
}

impl Compiler for SimpCCompiler {
    type NotElaboratedProgram = Program;
    type ElaboratedProgram = ElabProgram;

    fn generate_simple_ast(&self) -> Result<Program> {
        self.generate_ast()
    }

    fn elaborate_ast(&self, program: Program) -> Result<ElabProgram> {
        self.elaborate_program(program)
    }

    fn typecheck(&self, program: &ElabProgram) -> Result<bool> {
        let mut typechecker = self.typechecker.borrow_mut();
        typechecker.typecheck(program)
    }
}

impl SimpCCompiler {
    /// Constuct a new SimpCCompiler instance
    pub fn new(args: SccArguments) -> Self {
        Self {
            args,
            typechecker: RefCell::new(TypeChecker::new()),
        }
    }

    /// Emit the target, could be one of Abstract Syntax Tree,
    /// Intermediate Representation or Machine Code
    pub fn emit(&self) {
        match self.args.emit {
            Emission::Ast => self.emit_ast(),
            Emission::Ir => self.emit_ir(),
            Emission::Mach => self.emit_mach(),
        }
    }

    /// Emit the **elaborated** Abstract Syntax Tree
    fn emit_ast(&self) {
        let ast = self
            .generate_simple_ast()
            .unwrap_or_else(|e| panic!("Failed to generate AST: {}", e));

        let elab_ast = self
            .elaborate_ast(ast)
            .unwrap_or_else(|e| panic!("Failed to elaborate AST: {}", e));

        let pass_typecheck = self.typecheck(&elab_ast).unwrap_or_else(|e| {
            panic!("Failed to typecheck AST: {}", e);
        });

        if pass_typecheck {
            println!("Typecheck passed!");
        } else {
            println!("Typecheck failed!");
        }

        println!("Ast: {:?}", elab_ast);
    }

    /// Emit the Intermediate Representation
    fn emit_ir(&self) {
        todo!("This lab does not require you to emit IR.")
    }

    /// Emit the final machine code
    fn emit_mach(&self) {
        todo!("This lab does not require you to emit machine code.")
    }

    /// Parse the tokens to generate the AST
    fn generate_ast(&self) -> Result<Program> {
        let source = std::fs::read_to_string(self.args.src.clone())?;
        let lex = Token::lexer(&source)
            .spanned()
            .map(|(t, y): (Token, Span)| (y.start, t, y.end));
        let prog = ProgramParser::new()
            .parse(lex)
            .map_err(|e| anyhow!("Parsing failed {:?}", e))?;

        Ok(prog)
    }

    /// Elaborate the AST to its final stage
    fn elaborate_program(&self, prog: Program) -> Result<ElabProgram> {
        elab_program(prog)
    }
}
