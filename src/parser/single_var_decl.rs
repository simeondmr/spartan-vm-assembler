use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};
use crate::parser::var_type::{VarType};
use crate::semantic_analyzer;
use crate::symbol_table::symbol_table::VariableInfo;

pub struct SingleVarDecl {
    var_type: VarType
}

impl SingleVarDecl {
    pub fn new() -> Self {
        SingleVarDecl {
            var_type: VarType::new()
        }
    }
}

impl GrammarProductionParsing<(), VariableInfo> for SingleVarDecl {
    fn parse(&self, _param: Option<()>) -> Result<VariableInfo, AssemblerErrors> {
        let variable_name = {
            let mut lexer = Self::lexer_lock();
            let current_token = lexer.current_token();
            Self::match_token(&Token::Literal(0, "".to_string()), &mut lexer)?;
            current_token.extract_literal_value().ok_or_else(|| AssemblerErrors::SemanticError)
        }?;

        let type_size = self.var_type.parse(None)?.ok_or_else(|| AssemblerErrors::SyntaxError)?.get();

        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();
        Self::match_token(&Token::NumberU32(0, 0), &mut lexer)?;
        let array_size = current_token.extract_number_character_val().ok_or_else(|| AssemblerErrors::SyntaxError)?;
        semantic_analyzer::semantic_analyzer::check_array_size(array_size)?;
        let variable_info =  semantic_analyzer::semantic_analyzer::check_var_declaration(current_token.line(), Self::symbol_table().lock().unwrap().insert_variable(variable_name, type_size, array_size, array_size * type_size))?;
        Self::codegen().lock().unwrap().alloc_space(variable_info.total_size() as usize);

        return Ok(variable_info);
    }
}