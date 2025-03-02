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
        let lexer = Self::lexer();
        let variable_info = variable_info_option.unwrap();
        let current_token = lexer.lock().unwrap().current_token();

        match current_token {
            Token::NumberU32(_, init_value) => {
                lexer.lock().unwrap().next_token();
                Self::codegen().lock().unwrap().init_memory_from_addr(variable_info.type_size(), variable_info.number_cell(), init_value, variable_info.offset());
            },
            Token::StringTok(line, init_value) => {
                semantic_analyzer::semantic_analyzer::check_string_init(&variable_info, init_value.len() as u32, line)?;
                Self::codegen().lock().unwrap().copy_string_from_addr(init_value, variable_info.offset());
                lexer.lock().unwrap().next_token();
            },
            Token::SingleElem(_, '[') => {
                lexer.lock().unwrap().next_token();
                self.list_init.parse(Some(variable_info))?;
                Self::match_token(&Token::SingleElem(0, ']'), &mut lexer.lock().unwrap())?;
            },
            _ => {
                println!("Error at line {} : in 'section_data', data must be initialized", lexer.lock().unwrap().current_line());
                return Err(AssemblerErrors::SyntaxError)
            }
        }

        Ok(())
    }
}