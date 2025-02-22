use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::instructions::Instructions;
use crate::parser::program::GrammarProductionParsing;

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

impl GrammarProductionParsing<(), ()> for SectionText {
    fn parse(&self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        let mut lexer = <SectionText as GrammarProductionParsing<_, _>>::lexer_lock();
        if lexer.current_token() != Token::SectionText(0) {
            return Ok(())
        }

        <SectionText as GrammarProductionParsing<_, _>>::match_token(&Token::SectionText(0), &mut lexer)?;
        drop(lexer);
        self.instructions.parse(None)
    }
}