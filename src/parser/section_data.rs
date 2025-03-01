use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::decl_data::DeclData;
use crate::parser::program::GrammarProductionParsing;

pub struct SectionData {
    decl_data: DeclData
}

impl SectionData {
    pub fn new() -> Self {
        SectionData {
            decl_data: DeclData::new()
        }
    }
}

impl GrammarProductionParsing<(), ()> for SectionData {
    fn parse(& self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        let mut lexer = <SectionData as GrammarProductionParsing<_, _>>::lexer_lock();
        if lexer.current_token() != Token::SectionData(0) {
            return Ok(())
        }

        lexer.next_token();
        drop(lexer);
        self.decl_data.parse(None)?;

        Ok(())
    }
}

