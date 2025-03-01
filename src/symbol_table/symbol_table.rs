use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub static SYMBOL_TABLE_SINGLETON: OnceLock<Mutex<SymbolTable>> = OnceLock::new();

pub struct SymbolTable {
    variables: HashMap<String, VariableInfo>,
    labels: HashMap<String, u32>,
    current_variable_offset: u32,
    current_instruction_address: u32
}

#[derive(Debug)]
pub struct VariableInfo {
    type_size: u32,
    total_size: u32,
    number_cell: u32,
    offset: u32
}

impl Clone for VariableInfo {
    fn clone(&self) -> Self {
        VariableInfo {
            type_size: self.type_size,
            total_size: self.total_size,
            number_cell: self.number_cell,
            offset: self.offset
        }
    }
}

impl VariableInfo {
    pub fn new(type_size: u32, total_size: u32, number_cell: u32, offset: u32) -> VariableInfo {
        VariableInfo {
            type_size,
            total_size,
            number_cell,
            offset
        }
    }

    pub fn type_size(&self) -> u32 {
        self.type_size
    }

    pub fn total_size(&self) -> u32 {
        self.total_size
    }

    pub fn number_cell(&self) -> u32 {
        self.number_cell
    }

    #[allow(dead_code)]
    pub fn offset(&self) -> u32 {
        self.offset
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            labels: HashMap::new(),
            current_variable_offset: 0,
            current_instruction_address: 0
        }
    }

    #[allow(dead_code)]
    pub fn insert_variable(&mut self, name: String, type_size: u32, number_cell: u32, total_size: u32) -> Option<VariableInfo> {
        if self.variables.contains_key(&name) {
            return None;
        }

        let variable_info = VariableInfo::new(type_size, total_size, number_cell, self.current_variable_offset);
        self.variables.insert(name, variable_info.clone());
        self.current_variable_offset += total_size;
        self.current_instruction_address += total_size;
        Some(variable_info)
    }

    pub fn insert_label(&mut self, name: String) {
        self.labels.insert(name, self.current_instruction_address);
    }

    pub fn label_address(&self, name: &String) -> Option<&u32> {
        self.labels.get(name)
    }

    #[allow(dead_code)]
    pub fn update_current_instruction_address(&mut self, instruction_size: u8) {
        self.current_instruction_address += instruction_size as u32;
    }

    pub fn current_instruction_address(&self) -> u32 {
        self.current_instruction_address
    }

    #[allow(dead_code)]
    pub fn get_variable(&self, name: String) -> Option<&VariableInfo> {
        if let Some(variable_info) = self.variables.get(&name) {
            return Some(variable_info);
        }

        None
    }

    #[allow(dead_code)]
    pub fn get_current_variable_offset(&self) -> u32 {
        self.current_variable_offset
    }
}

