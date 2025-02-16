use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing, LexerCodeGen};

pub struct Instructions {

}

impl Instructions {
    pub fn new() -> Self {
        Instructions {

        }
    }
}

impl <'a> GrammarProductionParsing<LexerCodeGen<'a>, ()> for Instructions {
    fn parse(&self, lexer_codegen: Rc<RefCell<LexerCodeGen>>) -> Result<(), AssemblerErrors> {
        let mut lexer_codegen = lexer_codegen.borrow_mut();

        while lexer_codegen.is_instruction() {
            let current_token = lexer_codegen.current_token().clone();

            match current_token {
                Token::Iadd32(_) => {
                    lexer_codegen.match_token(&current_token)?;
                    lexer_codegen.match_token(&Token::NumberU32(0, 0))?;
                }
                Token::Pushb(_) => {
                    lexer_codegen.match_token(&current_token)?;
                    lexer_codegen.match_token(&Token::NumberU32(0, 0))?;
                }
                Token::Jmp(_) => {}
                _ => return Err(AssemblerErrors::SyntaxError)
            }
        }

        Ok(())
    }
}
