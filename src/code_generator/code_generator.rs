use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use crate::error::errors::AssemblerErrors;
use crate::lexer::lexer::Token;
use crate::symbol_table::symbol_table::SymbolTable;

pub static CODEGEN_SINGLETON: OnceLock<Mutex<CodeGen>> = OnceLock::new();

///Machine instructions size
const PUSHB_INSTR_SIZE: u8 = 4;
const PUSHW_INSTR_SIZE: u8 = 5;
const PUSHD_INSTR_SIZE: u8 = 7;
pub const JMPS_INSTR_SIZE: u8  = 6;

///Machine instructions opcodes
const PUSH_INSTR_OPCODE: u16   = 0x0001;
const JMP_INSTR_OPCODE: u16    = 0x0006;

///Note: at the moment the generated code will be copied into a Vec<u8> for simplicity
#[allow(dead_code)]
pub struct CodeGen {
    code_generated: Vec<u8>,
    instruction_info: HashMap<Token, InstructionInfo>,
    label_backpatching: HashMap<String, u32>
}

struct InstructionInfo {
    opcode: u16,
    param_size: u8,
    instruction_size: u8
}

impl InstructionInfo {
    fn new(opcode: u16, param_size: u8, instruction_size: u8) -> Self {
        InstructionInfo {
            opcode,
            param_size,
            instruction_size
        }
    }

    fn opcode(&self) -> u16 { self.opcode }
    fn instruction_size(&self) -> u8 { self.instruction_size }

    fn param_size(&self) -> u8 { self.param_size }

}

impl CodeGen {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut instruction_info = HashMap::new();
        instruction_info.insert(Token::Pushb(0), InstructionInfo::new(PUSH_INSTR_OPCODE, 1, PUSHB_INSTR_SIZE));
        instruction_info.insert(Token::Pushw(0), InstructionInfo::new(PUSH_INSTR_OPCODE, 2, PUSHW_INSTR_SIZE));
        instruction_info.insert(Token::Pushd(0), InstructionInfo::new(PUSH_INSTR_OPCODE, 4, PUSHD_INSTR_SIZE));
        instruction_info.insert(Token::Jmp(0), InstructionInfo::new(JMP_INSTR_OPCODE, 4,JMPS_INSTR_SIZE));

        CodeGen {
            code_generated: Vec::new(),
            instruction_info,
            label_backpatching: HashMap::new(),
        }
    }

    pub fn alloc_space(&mut self, alloc_size: usize) {
        self.code_generated.extend(std::iter::repeat(0).take(alloc_size))
    }

    fn u32_to_bytes_vec(&self, type_size: u32, value: u32) -> Vec<u8>{
       match type_size {
            1 => (value as u8).to_le_bytes().to_vec(),
            2 => (value as u16).to_le_bytes().to_vec(),
            4 => value.to_le_bytes().to_vec(),
            _ => { Vec::new() }
        }
    }

    pub fn init_memory_from_addr(&mut self, type_size: u32, number_cell: u32, value: u32, address: u32) {
        let values = self.u32_to_bytes_vec(type_size, value);
        let mut address = address;
        for _ in 0..number_cell {
            self.code_generated[address as usize..(address as usize + values.len())].copy_from_slice(&values);
            address += type_size;
        }
    }

    pub fn copy_string_from_addr(&mut self, init_value: String, address: u32) {
        init_value.char_indices().for_each(|(index, c)| self.code_generated[address as usize + index] = c as u8)
    }

    pub fn init_single_value_from_address(&mut self, type_size: u32, value: u32, address: u32) {
        let values = self.u32_to_bytes_vec(type_size, value);
        self.code_generated[address as usize..(address as usize + values.len())].copy_from_slice(&values);
    }


    /// For machine istruction like: |opcode: 2 byte|param_len: 1 byte|param: param_len bytes|
    /// Return the machine instruction len
    pub fn instr_format0_codegen(&mut self, instruction: Token, param: Token) -> u8 {
        let instruction_info = self.instruction_info.get(&instruction).unwrap();
        let param_bytes = match instruction_info.param_size() {
            1 => (param.extract_number_character_val().unwrap() as u8).to_le_bytes().to_vec(),
            2 => (param.extract_number_character_val().unwrap() as u16).to_le_bytes().to_vec(),
            4 => param.extract_number_character_val().unwrap().to_le_bytes().to_vec(),
            _ => { Vec::new() }
        };

        self.code_generated.extend_from_slice(&instruction_info.opcode().to_le_bytes());
        self.code_generated.push(instruction_info.param_size());
        self.code_generated.extend_from_slice(&param_bytes);
        instruction_info.instruction_size()
    }

    pub fn jmps_codegen(&mut self, jmp_istr: Token, label_name: String, option_address: Option<&u32>, address_to_backpatch: u32) {
        self.code_generated.extend_from_slice(&self.instruction_info.get(&jmp_istr).unwrap().opcode().to_le_bytes());
        if let Some(address) = option_address {
            self.code_generated.extend_from_slice(&address.to_le_bytes());
        } else {
            self.label_backpatching.insert(label_name, address_to_backpatch);
            self.code_generated.extend_from_slice(&[0, 0, 0, 0]);
        }
    }

    pub fn perform_backpatching(&mut self, symbol_table: &SymbolTable) -> Result<(), AssemblerErrors> {
        let mut error = false;
        self.label_backpatching.iter().for_each(|(label_name, address_to_backpatch)| {
            let label_address_option = symbol_table.label_address(&label_name);
            if let Some(label_address) = label_address_option {
                self.code_generated[(*address_to_backpatch as usize)..(*address_to_backpatch as usize + 4)].copy_from_slice(&label_address.to_le_bytes());
            } else {
                eprintln!("Error: label {} not found, you must declare it", label_name);
                error = true;
            }
        });

        if error {
            return Err(AssemblerErrors::SemanticError);
        }

        Ok(())
    }

    pub fn debug_codegenerated(&self) {
        println!("Generated code: {:?}", self.code_generated);
    }
}