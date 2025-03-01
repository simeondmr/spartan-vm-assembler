use std::num::{NonZero, NonZeroU32};
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::GrammarProductionParsing;

pub struct VarType;

impl VarType {
    pub fn new() -> Self {
        VarType {

        }
    }
}

static RESB_SIZE: u32 = 1;
static RESW_SIZE: u32 = 2;
static RESD_SIZE: u32 = 4;

impl GrammarProductionParsing<(), Option<NonZero<u32>>> for VarType {
    fn parse(&self, _param: Option<()>) -> Result<Option<NonZero<u32>>, AssemblerErrors> {
        let mut lexer = <VarType as GrammarProductionParsing<_, _>>::lexer_lock();
        let current_token = lexer.current_token();
        let allocation_size = match current_token {
            Token::RESB(_) => Ok(NonZeroU32::new(RESB_SIZE)),
            Token::RESW(_) => Ok(NonZeroU32::new(RESW_SIZE)),
            Token::RESD(_) => Ok(NonZeroU32::new(RESD_SIZE)),
            _ => {
                eprintln!("Error a line {}: missing variable type, found: {:?}", current_token.line(), current_token);
                return Err(AssemblerErrors::SyntaxError)
            }
        };

        lexer.next_token();
        allocation_size
    }
}