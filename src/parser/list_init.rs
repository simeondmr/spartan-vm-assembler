use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};
use crate::semantic_analyzer::semantic_analyzer;
use crate::symbol_table::symbol_table::VariableInfo;

pub struct ListInit;

impl ListInit {
    pub fn new() -> Self {
        ListInit {

        }
    }
}

impl GrammarProductionParsing<VariableInfo, ()> for ListInit {
    fn parse(&self, variable_info: Option<VariableInfo>) -> Result<(), AssemblerErrors> {
        let mut lexer = <ListInit as GrammarProductionParsing<_, _>>::lexer_lock();
        let list_type = lexer.current_token().clone();
        let mut current_size = 1;
        let variable_info = variable_info.unwrap();
        let list_size = variable_info.total_size() / variable_info.type_size();

        semantic_analyzer::check_list_init_first(&list_type)?;
        <ListInit as GrammarProductionParsing<_, _>>::match_token(&list_type, &mut lexer)?;

        let mut current_token = lexer.current_token().clone();
        while current_token == Token::SingleElem(0, ',') {
            <ListInit as GrammarProductionParsing<_, _>>::match_token(&current_token, &mut lexer)?;
            current_token = lexer.current_token().clone();
            semantic_analyzer::check_list_init_type(&list_type, &current_token)?;
            <ListInit as GrammarProductionParsing<_, _>>::match_token(&current_token, &mut lexer)?;
            current_size += 1;
            semantic_analyzer::check_list_init_size(current_token.line(), current_size, list_size)?;
            current_token = lexer.current_token().clone();
        }

        semantic_analyzer::check_list_init_smaller_size(current_token.line(), current_size, list_size);

        Ok(())
    }
}