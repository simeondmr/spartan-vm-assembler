use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::decl_bss::DeclBss;
use crate::parser::program::{GrammarProductionParsing, LexerCodeGen};
use crate::parser::var_type::VarDeclAttribute;

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

impl <'a> GrammarProductionParsing<LexerCodeGen<'a>, ()> for SectionBss {
    fn parse(&self, lexer_code_gen: Rc<RefCell<LexerCodeGen<'a>>>) -> Result<(), AssemblerErrors> {
        let var_decl_attribute = VarDeclAttribute::new(Rc::clone(&lexer_code_gen), true);
        let mut lexer_borrow_mut = var_decl_attribute.lexer_code_gen().borrow_mut();

        if *lexer_borrow_mut.current_token() != Token::SectionBss(0) {
            return Ok(())
        }

        lexer_borrow_mut.match_token(&Token::SectionBss(0))?;
        drop(lexer_borrow_mut);
        self.decl_bss.parse(Rc::new(RefCell::new(var_decl_attribute)))?;

        Ok(())
    }
}