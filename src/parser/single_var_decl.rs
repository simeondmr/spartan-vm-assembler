use std::cell::RefCell;
use std::rc::Rc;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};
use crate::parser::var_type::{VarDeclAttribute, VarType};
use crate::semantic_analyzer;
use crate::symbol_table::symbol_table::VariableInfo;

pub struct SingleVarDecl {
    var_type: VarType
}

impl SingleVarDecl {
    pub fn new() -> Self {
        SingleVarDecl {
            var_type: VarType::new()
        }
    }
}

impl <'a> GrammarProductionParsing<VarDeclAttribute<'a>, VariableInfo> for SingleVarDecl {
    fn parse(&self, var_decl_attribute: Rc<RefCell<VarDeclAttribute<'a>>>) -> Result<VariableInfo, AssemblerErrors> {
        let variable_name=
        {
            let var_decl = var_decl_attribute.borrow_mut();
            let mut lexer_codegen = var_decl.lexer_code_gen().borrow_mut();
            let current_token = lexer_codegen.current_token().clone();
            lexer_codegen.match_token(&Token::Literal(0, "".to_string()))?;
            semantic_analyzer::semantic_analyzer::check_literal_var_name(current_token)?
        };
        let type_size = self.var_type.parse(Rc::clone(&var_decl_attribute))?.unwrap().get();
        let var_decl = var_decl_attribute.borrow_mut();
        let mut lexer_code_gen = var_decl.lexer_code_gen().borrow_mut();
        let current_token = lexer_code_gen.current_token().clone();

        if let Token::NumberU32(_, array_size) = current_token {
            semantic_analyzer::semantic_analyzer::check_array_size(array_size)?;
            let variable_info =  semantic_analyzer::semantic_analyzer::check_var_declaration(lexer_code_gen.symbol_table().insert_bss_var(variable_name, type_size, array_size * type_size))?;
            lexer_code_gen.match_token(&Token::NumberU32(0, array_size))?;
            return Ok(variable_info);
        }

        eprintln!("Error at line {}: expected Number token for variable type, but found: {:?}", current_token.line(), current_token);
        return Err(AssemblerErrors::SyntaxError);
    }
}