mod backend;
mod compiler;
mod frontend;
mod middleend;

use anyhow::Result;
pub use compiler::SimpCCompiler;

pub trait Compiler {
    type NotElaboratedProgram;
    type ElaboratedProgram;

    // frontend
    fn generate_simple_ast(&self) -> Result<Self::NotElaboratedProgram>;
    fn elaborate_ast(&self, program: Self::NotElaboratedProgram)
        -> Result<Self::ElaboratedProgram>;
    fn typecheck(&self, program: &Self::ElaboratedProgram) -> Result<bool>;
}
