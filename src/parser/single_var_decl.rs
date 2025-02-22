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

        let variable_name =
        {
            let mut lexer = <SingleVarDecl as GrammarProductionParsing<_, _>>::lexer_lock();

            let current_token = lexer.current_token().clone();
            <SingleVarDecl as GrammarProductionParsing<_, _>>::match_token(&Token::Literal(0, "".to_string()), &mut lexer)?;
            semantic_analyzer::semantic_analyzer::check_literal_var_name(current_token)?
        };
        let type_size = self.var_type.parse(None)?
            .ok_or_else(|| AssemblerErrors::SyntaxError)?
            .get();

        let mut lexer = <SingleVarDecl as GrammarProductionParsing<_, _>>::lexer_lock();

        let current_token = lexer.current_token().clone();

        if let Token::NumberU32(line, array_size) = current_token {
            semantic_analyzer::semantic_analyzer::check_array_size(array_size)?;
            let variable_info =  semantic_analyzer::semantic_analyzer::check_var_declaration(line,
                <SingleVarDecl as GrammarProductionParsing<_, _>>::symbol_table().lock().unwrap().insert_variable(variable_name, type_size, array_size * type_size)
            )?;

            <SingleVarDecl as GrammarProductionParsing<_, _>>::match_token(&Token::NumberU32(0, array_size), &mut lexer)?;

            return Ok(variable_info);
        }

        eprintln!("Error at line {}: expected Number token for variable type, but found: {:?}", current_token.line(), current_token);
        return Err(AssemblerErrors::SyntaxError);
    }
}