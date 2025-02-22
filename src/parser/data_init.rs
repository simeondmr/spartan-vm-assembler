use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::list_init::ListInit;
use crate::parser::program::{GrammarProductionParsing};
use crate::semantic_analyzer;
use crate::symbol_table::symbol_table::VariableInfo;

pub struct DataInit {
    list_init: ListInit
}

impl DataInit {
    pub fn new() -> Self {
        DataInit {
            list_init: ListInit::new()
        }
    }
}

impl GrammarProductionParsing<VariableInfo, ()> for DataInit {
    fn parse(&self, variable_info_option: Option<VariableInfo>) -> Result<(), AssemblerErrors> {
        let lexer = <DataInit as GrammarProductionParsing<_, _>>::lexer();
        let variable_info = variable_info_option.unwrap();
        let current_token = lexer.lock().unwrap().current_token().clone();

        match current_token {
            Token::NumberU32(_, init_value) => {
                <DataInit as GrammarProductionParsing<_, _>>::match_token(&Token::NumberU32(init_value, 0), &mut lexer.lock().unwrap())?;
                //TODO: call code generator in order to copy init_value, n° ariable_info.total_size() / variable_info.type_size() times, from address variable_info.offset()
                return Ok(())
            },
            Token::StringTok(line, init_value) => {
                semantic_analyzer::semantic_analyzer::check_string_init(&variable_info, init_value.len() as u32, line)?;
                //TODO: call code generator in order to copy init_value, from address variable_info.offset()
                <DataInit as GrammarProductionParsing<_, _>>::match_token(&Token::StringTok(0, String::new()), &mut lexer.lock().unwrap())?;
                return Ok(())
            },
            Token::SingleElem(_, '[') => {
                <DataInit as GrammarProductionParsing<_, _>>::match_token(&Token::SingleElem(0, '['), &mut lexer.lock().unwrap())?;
                self.list_init.parse(Some(variable_info))?;
                <DataInit as GrammarProductionParsing<_, _>>::match_token(&Token::SingleElem(0, ']'), &mut lexer.lock().unwrap())?;
                return Ok(())
            },
            _ => {
                println!("Error at line {} : in 'section_data', data must be initialized", lexer.lock().unwrap().current_line());
                Err(AssemblerErrors::SyntaxError)
            }
        }
    }
}