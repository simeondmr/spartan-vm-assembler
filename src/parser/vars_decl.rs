use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::parser::program::{GrammarProductionParsing, LexerCodeGen};
use crate::parser::section_bss::SectionBss;
use crate::parser::section_data::SectionData;

pub struct VarsDecl {
    section_bss: SectionBss,
    section_data: SectionData
}

impl VarsDecl {
    pub fn new() -> Self {
        VarsDecl {
            section_bss: SectionBss::new(),
            section_data: SectionData::new()
        }
    }
}

impl <'a> GrammarProductionParsing<LexerCodeGen<'a> ,()> for VarsDecl {
    fn parse(&self, lexer_code_gen: Rc<RefCell<LexerCodeGen<'a>>>) -> Result<(), AssemblerErrors> {
        self.section_bss.parse(Rc::clone(&lexer_code_gen))?;
        self.section_data.parse(Rc::clone(&lexer_code_gen))
    }
}