use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};
use crate::parser::single_var_decl::SingleVarDecl;

pub struct DeclBss {
    single_var_decl: SingleVarDecl
}

impl DeclBss {
    pub fn new() -> Self {
        DeclBss {
            single_var_decl: SingleVarDecl::new(),
        }
    }
}

impl GrammarProductionParsing<(), ()> for DeclBss {
    fn parse(&self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        let lexer = <DeclBss as GrammarProductionParsing<_, _>>::lexer();

        while lexer.lock().unwrap().current_token() == Token::Literal(0, "".to_string()) {
            self.single_var_decl.parse(None)?;
        }

        Ok(())
    }
}