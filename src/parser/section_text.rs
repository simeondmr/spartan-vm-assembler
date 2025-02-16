use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::instructions::Instructions;
use crate::parser::program::{GrammarProductionParsing, LexerCodeGen};

pub struct SectionText {
    instructions: Instructions
}

impl SectionText {
    pub fn new() -> Self {
        SectionText {
            instructions: Instructions::new(),
        }
    }
}

impl <'a> GrammarProductionParsing<LexerCodeGen<'a>, ()> for SectionText {
    fn parse(&self, lexer_codegen: Rc<RefCell<LexerCodeGen<'a>>>) -> Result<(), AssemblerErrors> {
        let mut lexer_codegen_borrowmut = lexer_codegen.borrow_mut();

        if *lexer_codegen_borrowmut.current_token() != Token::SectionText(0) {
            return Ok(())
        }

        lexer_codegen_borrowmut.match_token(&Token::SectionText(0))?;
        drop(lexer_codegen_borrowmut);

        self.instructions.parse(Rc::clone(&lexer_codegen))
    }
}