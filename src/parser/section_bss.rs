use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::decl_bss::DeclBss;
use crate::parser::program::{GrammarProductionParsing};

pub struct SectionBss {
    decl_bss: DeclBss
}


impl SectionBss {
    pub fn new() -> Self {
        SectionBss {
            decl_bss: DeclBss::new()
        }
    }
}

impl GrammarProductionParsing<(), ()> for SectionBss {
    fn parse(&self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        let mut lexer = <SectionBss as GrammarProductionParsing<_, _>>::lexer_lock();
        if lexer.current_token() != Token::SectionBssTok(0) {
            return Ok(())
        }

        <SectionBss as GrammarProductionParsing<_, _>>::match_token(&Token::SectionBssTok(0), &mut lexer)?;
        drop(lexer);
        self.decl_bss.parse(None)?;

        Ok(())
    }
}