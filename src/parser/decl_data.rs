use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::data_init::DataInit;
use crate::parser::program::GrammarProductionParsing;
use crate::parser::single_var_decl::SingleVarDecl;

pub struct DeclData {
    single_var_decl: SingleVarDecl,
    data_init: DataInit,
}

impl DeclData {
    pub fn new() -> Self {
        DeclData {
            single_var_decl: SingleVarDecl::new(),
            data_init: DataInit::new()
        }
    }
}

impl GrammarProductionParsing<(), ()> for DeclData {
    fn parse(&self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        while let Token::Literal(_, _) = {
            let lexer = <DeclData as GrammarProductionParsing<_, _>>::lexer_lock();
            lexer.current_token()
        } {
            let variable_info = self.single_var_decl.parse(None)?;
            self.data_init.parse(Some(variable_info))?;
        }

        Ok(())
    }
}