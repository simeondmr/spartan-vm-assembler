use crate::error::errors::AssemblerErrors;
use crate::parser::program::{GrammarProductionParsing};
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

impl GrammarProductionParsing<(), ()> for VarsDecl {
    fn parse(&self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        self.section_bss.parse(None)?;
        self.section_data.parse(None)
    }
}