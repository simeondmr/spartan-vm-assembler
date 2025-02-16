use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::data_init::DataInit;
use crate::parser::program::{GrammarProductionParsing, LexerCodeGen};
use crate::parser::single_var_decl::SingleVarDecl;
use crate::parser::var_type::VarDeclAttribute;
use crate::symbol_table::symbol_table::VariableInfo;

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

pub struct DataInitAttribute<'a> {
    lexer_code_gen: Rc<RefCell<LexerCodeGen<'a>>>,
    variable_info: VariableInfo
}

impl <'a> DataInitAttribute<'a> {
    pub fn new(lexer_code_gen: Rc<RefCell<LexerCodeGen<'a>>>, variable_info: VariableInfo) -> DataInitAttribute<'a> {
        DataInitAttribute {
            lexer_code_gen,
            variable_info,
        }
    }

    pub fn lexer_code_gen(&self) -> &Rc<RefCell<LexerCodeGen<'a>>> {
        &self.lexer_code_gen
    }

    pub fn variable_info(&self) -> VariableInfo {
        self.variable_info.clone()
    }
}

impl <'a> GrammarProductionParsing<VarDeclAttribute<'a>, ()> for DeclData {
    fn parse(&self, var_decl_attribute: Rc<RefCell<VarDeclAttribute<'a>>>) -> Result<(), AssemblerErrors> {
        while *var_decl_attribute.borrow().lexer_code_gen().borrow().current_token() == Token::Literal(0, "".to_string()) {
            let variable_info = self.single_var_decl.parse(Rc::clone(&var_decl_attribute))?;
            let data_init_attribute = Rc::new(RefCell::new(DataInitAttribute::new(Rc::clone(&var_decl_attribute.borrow().lexer_code_gen()), variable_info)));
            self.data_init.parse(Rc::clone(&data_init_attribute))?;
        }

        Ok(())
    }
}