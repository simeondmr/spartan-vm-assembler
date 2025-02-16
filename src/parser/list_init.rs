use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::decl_data::DataInitAttribute;
use crate::parser::program::{GrammarProductionParsing};
use crate::semantic_analyzer;

pub struct ListInit;

impl ListInit {
    pub fn new() -> Self {
        ListInit {

        }
    }
}

impl <'a> GrammarProductionParsing<DataInitAttribute<'a>, ()> for ListInit {
    fn parse(&self, param: Rc<RefCell<DataInitAttribute<'a>>>) -> Result<(), AssemblerErrors> {
        let data_init_attribute = param.borrow_mut();
        let mut lexer_codegen = data_init_attribute.lexer_code_gen().borrow_mut();
        let list_type = lexer_codegen.current_token().clone();
        let mut current_size = 1;
        let list_size = data_init_attribute.variable_info().total_size() / data_init_attribute.variable_info().type_size();

        semantic_analyzer::semantic_analyzer::check_list_init_first(&list_type)?;
        lexer_codegen.match_token(&list_type)?;

        let mut current_token = lexer_codegen.current_token().clone();
        while current_token == Token::SingleElem(0, ',') {
            lexer_codegen.match_token(&Token::SingleElem(0, ','))?;
            current_token = lexer_codegen.current_token().clone();
            semantic_analyzer::semantic_analyzer::check_list_init_type(&list_type, &current_token)?;
            lexer_codegen.match_token(&current_token)?;
            current_size += 1;
            semantic_analyzer::semantic_analyzer::check_list_init_size(current_token.line(), current_size, list_size)?;
            current_token = lexer_codegen.current_token().clone();
        }

        semantic_analyzer::semantic_analyzer::check_list_init_smaller_size(current_token.line(), current_size, list_size);

        Ok(())
    }
}