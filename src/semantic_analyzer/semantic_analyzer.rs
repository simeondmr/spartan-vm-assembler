use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::symbol_table::symbol_table::VariableInfo;

pub fn check_var_type(token: Token) -> Result<u32, AssemblerErrors> {
    match token {
        Token::RESB(_) => {
            Ok(1)
        },
        Token::RESW(_) => {
            Ok(2)
        },
        Token::RESD(_) => {
            Ok(4)
        },
        _ => {
            eprintln!("Error a line {}: missing variable type, found: {:?}", token.line(), token);
            Err(AssemblerErrors::SemanticError)
        }
    }
}

pub fn check_array_size(size: u32) -> Result<(), AssemblerErrors> {
    if size <= 0 {
        eprintln!("Error: the current array size is {} but must be greater than 0", size);
        return Err(AssemblerErrors::SemanticError);
    }

    Ok(())
}

pub fn check_string_init(variable_info: &VariableInfo, str_len: u32, line: u32) -> Result<(), AssemblerErrors> {
    if variable_info.type_size() != 1 {
        eprintln!("Error at line {}: expected 'resb' declaration for string initializzation", line);
        return Err(AssemblerErrors::SemanticError);
    }

    let total_size = variable_info.total_size();
    if total_size <= str_len {
        eprintln!("Error at line {}: allocated {} bytes but the init string size is {} bytes", line, total_size, str_len + 1);
        return Err(AssemblerErrors::SemanticError);
    }

    Ok(())
}

pub fn check_list_init_type(list_type: &Token, current_token_type: &Token) -> Result<(), AssemblerErrors> {
    if current_token_type != list_type {
        eprintln!("Error: list type is {:?} but found also element {:?}", list_type, current_token_type);
        return Err(AssemblerErrors::SemanticError)
    }

    Ok(())
}

pub fn check_list_init_first(first_element: &Token) -> Result<(), AssemblerErrors> {
    if *first_element != Token::NumberU32(0, 0) {
        eprintln!("Error at line {}: expected number or charter, found: {:?}", first_element.line(), first_element);
        return  Err(AssemblerErrors::SemanticError)
    }

    Ok(())
}

pub fn check_list_init_size(line: u32, current_size: u32, list_size: u32) -> Result<(), AssemblerErrors> {
    if current_size > list_size {
        eprintln!("Error at line {}: declared a variable with {} elements, but the initialization list contains {} elements", line, list_size, current_size);
        return Err(AssemblerErrors::SemanticError)
    }

    Ok(())
}

pub fn check_list_init_smaller_size(line: u32, current_size: u32, list_size: u32) {
    if current_size < list_size {
        println!("Warning at line {}: declared array with size {}, but the init list have size {}", line, current_size, list_size);
    }
}

pub fn check_var_declaration(line: u32, variable_info: Option<VariableInfo>) -> Result<VariableInfo, AssemblerErrors> {
    variable_info.ok_or_else(|| {
        eprintln!("Error at line {}: variable already declared", line);
        AssemblerErrors::SemanticError
    })
}

pub fn check_literal_var_name(token: Token) -> Result<String, AssemblerErrors> {
    if let Token::Literal(_, name) = token {
        return Ok(name)
    }

    eprintln!("Error at line {}: expected a Literal for variable name but not found {:?}", token.line(), token);
    Err(AssemblerErrors::SemanticError)
}