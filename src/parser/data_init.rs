use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::decl_data::DataInitAttribute;
use crate::parser::list_init::ListInit;
use crate::parser::program::{GrammarProductionParsing};
use crate::semantic_analyzer;

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

impl <'a> GrammarProductionParsing<DataInitAttribute<'a>, ()> for DataInit {
    fn parse(&self, data_init_attribute: Rc<RefCell<DataInitAttribute<'a>>>) -> Result<(), AssemblerErrors> {
        let var_attribute = data_init_attribute.borrow_mut();
        let variable_info = var_attribute.variable_info();
        let mut lexer = var_attribute.lexer_code_gen().borrow_mut();

        match lexer.current_token() {
            Token::NumberU32(_, init_value) => {
                let init_value = *init_value;
                lexer.match_token(&Token::NumberU32(init_value, 0))?;
                //TODO: call code generator in order to copy init_value, n° ariable_info.total_size() / variable_info.type_size() times, from address variable_info.offset()
                return Ok(())
            },
            Token::StringTok(line, init_value) => {
                semantic_analyzer::semantic_analyzer::check_string_init(&variable_info, init_value.len() as u32, *line)?;
                //TODO: call code generator in order to copy init_value, from address variable_info.offset()
                lexer.match_token(&Token::StringTok(0, String::new()))?;
                return Ok(())
            },
            Token::SingleElem(_, '[') => {
                lexer.match_token(&Token::SingleElem(0, '['))?;
                drop(lexer);
                drop(var_attribute);
                self.list_init.parse(Rc::clone(&data_init_attribute))?;
                data_init_attribute.borrow_mut().lexer_code_gen().borrow_mut().match_token(&Token::SingleElem(0, ']'))?;
                return Ok(())
            },
            _ => {
                println!("Error at line {} : in 'section_data', data must be initialized", lexer.current_line());
                Err(AssemblerErrors::SyntaxError)
            }
        }
    }
}