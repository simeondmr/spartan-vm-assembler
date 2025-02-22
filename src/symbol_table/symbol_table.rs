use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub static SYMBOL_TABLE_SINGLETON: OnceLock<Mutex<SymbolTable>> = OnceLock::new();

pub struct SymbolTable {
    variables: HashMap<String, VariableInfo>,
    current_offset: u32
}

#[derive(Debug)]
pub struct VariableInfo {
    type_size: u32,
    total_size: u32,
    offset: u32
}

impl Clone for VariableInfo {
    fn clone(&self) -> Self {
        VariableInfo {
            type_size: self.type_size,
            total_size: self.total_size,
            offset: self.offset
        }
    }
}

impl VariableInfo {
    pub fn new(type_size: u32, total_size: u32, offset: u32) -> VariableInfo {
        VariableInfo {
            type_size,
            total_size,
            offset
        }
    }

    pub fn type_size(&self) -> u32 {
        self.type_size
    }

    pub fn total_size(&self) -> u32 {
        self.total_size
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
            current_offset: 0,
        }
    }

    #[allow(dead_code)]
    pub fn insert_variable(&mut self, name: String, type_size: u32, total_size: u32) -> Option<VariableInfo> {
        if self.variables.contains_key(&name) {
            return None;
        }

        let variable_info = VariableInfo::new(type_size, total_size, self.current_offset);
        self.variables.insert(name, variable_info.clone());
        self.current_offset += total_size;
        Some(variable_info)
    }

    #[allow(dead_code)]
    pub fn get_variable(&self, name: String) -> Option<&VariableInfo> {
        if let Some(variable_info) = self.variables.get(&name) {
            return Some(variable_info);
        }

        None
    }

    #[allow(dead_code)]
    pub fn get_current_offset(&self) -> u32 {
        self.current_offset
    }
}

