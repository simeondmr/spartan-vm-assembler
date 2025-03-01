use crate::code_generator::code_generator::JMPS_INSTR_SIZE;
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::{ Token};
use crate::parser::program::{GrammarProductionParsing};

pub struct Instructions;

impl Instructions {
    pub fn new() -> Self {
        Instructions {

        }
    }
}



impl GrammarProductionParsing<(), ()> for Instructions {
    fn parse(&self, _param: Option<()>) -> Result<(), AssemblerErrors> {
        let mut lexer = <Instructions as GrammarProductionParsing<_,_>>::lexer_lock();
        let mut symbol_table = <Instructions as GrammarProductionParsing<_,_>>::symbol_table_lock();

        while lexer.is_instruction() {
            let instruction_token = lexer.current_token();
            match instruction_token {
                Token::Pushb(_) | Token::Pushw(_) | Token::Pushd(_) => {
                    let param = lexer.next_token();
                    <Instructions as GrammarProductionParsing<_,_>>::expected(&Token::NumberU32(0, 0), &mut lexer)?;
                    let instruction_size = <Instructions as GrammarProductionParsing<_,_>>::codegen().lock().unwrap().instr_format0_codegen(instruction_token, param);
                    symbol_table.update_current_instruction_address(instruction_size);
                }
                Token::Label(_, ref name) => {
                    symbol_table.insert_label(name.to_string());
                }
                Token::Jmp(_) => {
                    let label= lexer.next_token();
                    <Instructions as GrammarProductionParsing<_,_>>::expected(&Token::Literal(0, "".to_string()), &mut lexer)?;
                    let label_name = label.extract_literal_value().unwrap();
                    let label_address = symbol_table.label_address(&label_name);
                    let current_instruction_address = symbol_table.current_instruction_address();
                    <Instructions as GrammarProductionParsing<_,_>>::codegen().lock().unwrap().jmps_codegen(instruction_token, label_name, label_address, current_instruction_address + 2);// + 2 to point after the jmp opcode
                    symbol_table.update_current_instruction_address(JMPS_INSTR_SIZE);
                }
                _ =>  {
                    println!("Error at line {}: not recognized instruction", instruction_token.line());
                    return Err(AssemblerErrors::SyntaxError)
                }
            }

            lexer.next_token();
        }

        Ok(())
    }
}
