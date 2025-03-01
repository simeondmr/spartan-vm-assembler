#[derive(Debug)]
pub enum AssemblerErrors {
    SyntaxError,
    SemanticError,
    WrongArgument
}