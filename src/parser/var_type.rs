use std::cell::RefCell;
use std::num::{NonZero, NonZeroU32};
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::parser::program::{GrammarProductionParsing, LexerCodeGen};
use crate::semantic_analyzer;

pub struct VarType;

impl VarType {
    pub fn new() -> Self {
        VarType {

        }
    }
}

pub struct VarDeclAttribute<'a> {
    lexer_code_gen: Rc<RefCell<LexerCodeGen<'a>>>,
    is_bss: bool
}

impl <'a> VarDeclAttribute<'a> {
    pub fn new(lexer_code_gen: Rc<RefCell<LexerCodeGen<'a>>>, is_bss: bool) -> VarDeclAttribute<'a> {
        VarDeclAttribute {
            lexer_code_gen,
            is_bss,
        }
    }

    pub fn lexer_code_gen(&self) -> &Rc<RefCell<LexerCodeGen<'a>>> {
        &self.lexer_code_gen
    }

    #[allow(dead_code)]
    pub fn is_bss(&self) -> bool {
        self.is_bss
    }

    #[allow(dead_code)]
    pub fn set_is_bss(&mut self, is_bss: bool) {
        self.is_bss = is_bss;
    }
}

impl <'a> GrammarProductionParsing<VarDeclAttribute<'a>, Option<NonZero<u32>>> for VarType {
    fn parse(&self, var_decl_attribute: Rc<RefCell<VarDeclAttribute<'a>>>) -> Result<Option<NonZero<u32>>, AssemblerErrors> {
        let var_decl = var_decl_attribute.borrow_mut();
        let mut lexer_code_gen = var_decl.lexer_code_gen.borrow_mut();
        let allocation_size  = semantic_analyzer::semantic_analyzer::check_var_type(lexer_code_gen.current_token().clone(), &mut lexer_code_gen)?;
        Ok(NonZeroU32::new(allocation_size))
    }
}