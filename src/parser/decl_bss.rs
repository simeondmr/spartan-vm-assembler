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
        while let Token::Literal(_, _) = {
            let lexer = Self::lexer_lock();
            lexer.current_token()
        } {
            self.single_var_decl.parse(None)?;
        }

        Ok(())
    }
}