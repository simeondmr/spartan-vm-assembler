use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};
use crate::parser::single_var_decl::SingleVarDecl;
use crate::parser::var_type::VarDeclAttribute;

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

impl <'a> GrammarProductionParsing<VarDeclAttribute<'a>, ()> for DeclBss {
    fn parse(&self, var_decl_attribute: Rc<RefCell<VarDeclAttribute<'a>>>) -> Result<(), AssemblerErrors> {
        while *var_decl_attribute.borrow().lexer_code_gen().borrow().current_token() == Token::Literal(0, "".to_string()) {
            self.single_var_decl.parse(Rc::clone(&var_decl_attribute))?;
        }

        Ok(())
    }
}