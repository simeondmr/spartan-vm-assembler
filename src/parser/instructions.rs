use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};

pub struct Instructions;

impl Instructions {
    pub fn new() -> Self {
        Instructions {

        }
    }
}

impl GrammarProductionParsing<(), ()> for Instructions {
    fn parse(&self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        let mut lexer = <Instructions as GrammarProductionParsing<_,_>>::lexer_lock();

        while lexer.is_instruction() {
            let current_token = lexer.current_token().clone();

            match current_token {
                Token::Iaddb(_) => {
                    <Instructions as GrammarProductionParsing<_,_>>::match_token(&current_token, &mut lexer)?;
                }
                Token::Pushb(_) => {
                    <Instructions as GrammarProductionParsing<_,_>>::match_token(&current_token, &mut lexer)?;
                    <Instructions as GrammarProductionParsing<_,_>>::match_token(&Token::NumberU32(0, 0), &mut lexer)?;
                }
                Token::Jmp(_) => {}
                _ =>  {
                    println!("Error at line {}: not recognized instruction", current_token.line());
                    return Err(AssemblerErrors::SyntaxError)
                }
            }
        }

        Ok(())
    }
}
