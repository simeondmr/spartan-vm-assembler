use std::cell::RefCell;
use std::rc::Rc;
use crate::code_generator::code_generator::CodeGen;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::section_text::SectionText;
use crate::parser::vars_decl::VarsDecl;
use crate::symbol_table::symbol_table::SymbolTable;

/// Every grammar production must implement this trait
/// #Generics type: A, B
/// A is the structure that will contain the inherited attributes that we want to pass to the production that implement this trait
/// B is the structure that will contain the synthetized attributes that we want to pass to the root
pub trait GrammarProductionParsing<A, B> {
    fn parse(&self, param: Rc<RefCell<A>>) -> Result<B, AssemblerErrors>;
}

#[allow(dead_code)]
pub struct LexerCodeGen<'a> {
    lexer: &'a mut Lexer,
    symbol_table: &'a mut SymbolTable,
    code_gen: &'a mut CodeGen,
    current_token: Token,
}

#[allow(dead_code)]
impl <'a> LexerCodeGen <'a> {
    pub fn new(lexer: &'a mut  Lexer, symbol_table: &'a mut SymbolTable, code_gen: &'a mut CodeGen) -> Self {
        LexerCodeGen {
            lexer,
            symbol_table,
            code_gen,
            current_token: Token::INIT(1)
        }
    }

    pub fn match_token(&mut self, token: &Token) -> Result<(), AssemblerErrors> {
        if *token == self.current_token {
            self.current_token = self.lexer.next_token();
            return Ok(())
        }

        eprintln!("Error at line {}: expected {:?}, but found {:?}", self.lexer.current_line(), token, self.current_token());
        Err(AssemblerErrors::SyntaxError)
    }

    pub fn current_line(&self) -> u32  {
        self.lexer.current_line()
    }

    pub fn is_instruction(&self) -> bool {
        match self.current_token {
            Token::Iadd32(_) => true,
            Token::Pushb(_) => true,
            Token::Jmp(_) => true,
            _ => false
        }
    }

    pub fn current_token(&self) -> &Token {
        &self.current_token
    }

    pub fn set_current_token(&mut self, token: Token) {
        self.current_token = token;
    }

    pub fn symbol_table(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    pub fn lexer_mut(&mut self) -> &mut Lexer {
        &mut self.lexer
    }
}

pub struct Program {
    vars_decl: VarsDecl,
    section_text: SectionText,
    lexer: Lexer,
    symbol_table: SymbolTable,
    code_gen: CodeGen
}

impl Program {
    pub fn new(file_name: String) -> Option<Program> {
        Some(Program {
            vars_decl: VarsDecl::new(),
            section_text: SectionText::new(),
            lexer: Lexer::new(file_name)?,
            symbol_table: SymbolTable::new(),
            code_gen: CodeGen::new() ,
        })
    }

    pub fn start(mut self) -> Result<(), AssemblerErrors> {
        let token = self.lexer.next_token();
        let mut lexer_codegen = LexerCodeGen::new(&mut self.lexer, &mut self.symbol_table, &mut self.code_gen);
        lexer_codegen.set_current_token(token);
        let lexer_codegen = Rc::new(RefCell::new(lexer_codegen));
        self.vars_decl.parse(Rc::clone(&lexer_codegen))?;
        self.section_text.parse(Rc::clone(&lexer_codegen))?;
        let parsing_result = lexer_codegen.borrow_mut().match_token(&Token::EOF(0));
        parsing_result
    }
}
