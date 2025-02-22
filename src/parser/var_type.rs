use std::num::{NonZero, NonZeroU32};
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::GrammarProductionParsing;
use crate::semantic_analyzer::semantic_analyzer;

pub struct VarType;

impl VarType {
    pub fn new() -> Self {
        VarType {

        }
    }
}

impl GrammarProductionParsing<(), Option<NonZero<u32>>> for VarType {
    fn parse(&self, _param: Option<()>) -> Result<Option<NonZero<u32>>, AssemblerErrors> {
        let mut lexer = <VarType as GrammarProductionParsing<_, _>>::lexer_lock();
        let allocation_size  = semantic_analyzer::check_var_type(lexer.current_token().clone())?;

        match lexer.current_token().clone() {
            Token::RESB(_) => <VarType as GrammarProductionParsing<_, _>>::match_token(&Token::RESB(0), &mut lexer)?,
            Token::RESW(_) => <VarType as GrammarProductionParsing<_, _>>::match_token(&Token::RESW(0), &mut lexer)?,
            Token::RESD(_) => <VarType as GrammarProductionParsing<_, _>>::match_token(&Token::RESD(0), &mut lexer)?,
            _ => {
                println!("Expected size");
                return Err(AssemblerErrors::SyntaxError)
            }
        }

        Ok(NonZeroU32::new(allocation_size))
    }
}