use std::sync::{Mutex, MutexGuard};
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::{Lexer, LEXER_SINGLETON, Token};
use crate::parser::section_text::SectionText;
use crate::parser::vars_decl::VarsDecl;
use crate::symbol_table::symbol_table::{SYMBOL_TABLE_SINGLETON, SymbolTable};

/// Every grammar production must implement this trait
/// #Generics type: A, B
/// A is the structure that will contain the inherited attributes that we want to pass to the production that implement this trait
/// B is the structure that will contain the synthetized attributes that we want to pass to the root
pub trait GrammarProductionParsing<A, B> {
    fn parse(&self, param: Option<A>) -> Result<B, AssemblerErrors>;

    fn match_token(expected_token: &Token, lexer: &mut Lexer) -> Result<(), AssemblerErrors> {
        if *expected_token == lexer.current_token() {
            lexer.next_token();
            return Ok(())
        }

        eprintln!("Error at line {}: expected {:?}, but found {:?}", lexer.current_line(), expected_token, lexer.current_token());
        Err(AssemblerErrors::SyntaxError)
    }

    ///Use this method if you wanna lock the lexer
    fn lexer_lock() -> MutexGuard<'static, Lexer> {
        LEXER_SINGLETON.get().unwrap().lock().unwrap()
    }

    ///Use this methods if you don't want to lock the lexer
    fn lexer() -> &'static Mutex<Lexer> {
        LEXER_SINGLETON.get().unwrap()
    }

    ///Use this method if you wanna lock the symbol table
    //fn symbol_table_lock() -> MutexGuard<'static, SymbolTable> {
    //    SymbolTableSingleton.get().unwrap().lock().unwrap()
    //}

    ///Use this methods if you don't want to lock the symbol table
    fn symbol_table() -> &'static Mutex<SymbolTable> {
        SYMBOL_TABLE_SINGLETON.get().unwrap()
    }
}

pub struct Program {
    vars_decl: VarsDecl,
    section_text: SectionText,
    file_name: String
}

impl GrammarProductionParsing<(), ()> for Program {
    fn parse(&self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        let lexer = LEXER_SINGLETON.get_or_init(|| Mutex::new(Lexer::new(self.file_name.clone())));
        SYMBOL_TABLE_SINGLETON.get_or_init(|| Mutex::new(SymbolTable::new()));
        lexer.lock().unwrap().next_token();
        self.vars_decl.parse(None)?;
        self.section_text.parse(None)?;
        let mut lexer = lexer.lock().unwrap();
        <Program as GrammarProductionParsing<_, _>>::match_token(&Token::EOF(0), &mut lexer)
    }
}

impl Program {
    pub fn new(file_name: String) -> Option<Program> {
        Some(Program {
            vars_decl: VarsDecl::new(),
            section_text: SectionText::new(),
            file_name,
        })
    }
}