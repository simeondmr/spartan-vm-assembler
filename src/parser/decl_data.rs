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
        let lexer = <DeclData as GrammarProductionParsing<_, _>>::lexer();

        while lexer.lock().unwrap().current_token() == Token::Literal(0, "".to_string()) {
            let variable_info = self.single_var_decl.parse(None)?;
            self.data_init.parse(Some(variable_info))?;
        }

        Ok(())
    }
}