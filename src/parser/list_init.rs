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
        let mut codegen = <ListInit as GrammarProductionParsing<_, _>>::codegen_lock();
        let list_type = lexer.current_token();
        let mut current_list_size = 1;
        let variable_info = variable_info.ok_or_else(|| AssemblerErrors::SemanticError)?;
        let var_number_cell = variable_info.number_cell();

        semantic_analyzer::check_list_init_first(&list_type)?;
        codegen.init_single_value_from_address(variable_info.type_size(), list_type.extract_number_character_val().ok_or_else(||AssemblerErrors::SyntaxError)?, variable_info.offset());
        <ListInit as GrammarProductionParsing<_, _>>::match_token(&list_type, &mut lexer)?;

        let mut current_token = lexer.current_token();
        while let Token::SingleElem(_, ',') = current_token {
            lexer.next_token();
            let init_val = lexer.current_token();
            semantic_analyzer::check_list_init_type(&list_type, &init_val)?;
            <ListInit as GrammarProductionParsing<_, _>>::match_token(&init_val, &mut lexer)?;
            semantic_analyzer::check_list_init_size(init_val.line(), current_list_size + 1, var_number_cell)?;
            codegen.init_single_value_from_address(variable_info.type_size(), init_val.extract_number_character_val().ok_or_else(||AssemblerErrors::SyntaxError)?, variable_info.offset() + (variable_info.type_size() * current_list_size));
            current_list_size += 1;
            current_token = lexer.current_token();
        }

        semantic_analyzer::check_list_init_smaller_size(current_token.line(), current_list_size, var_number_cell);

        Ok(())
    }
}