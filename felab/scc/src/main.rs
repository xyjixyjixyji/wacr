use clap::{Parser, ValueEnum};
use compiler::SimpCCompiler;

mod compiler;

/// The type for telling the compiler what to emit
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Emission {
    Ast,
    Ir,
    Mach,
}

/// The passed in commandline arguments
#[derive(Parser, Debug)]
pub struct SccArguments {
    #[arg(short, long)]
    src: String,
    #[arg(short, long)]
    emit: Emission,
}

fn main() {
    let args = SccArguments::parse();
    let compiler = SimpCCompiler::new(args);
    compiler.emit();
}
