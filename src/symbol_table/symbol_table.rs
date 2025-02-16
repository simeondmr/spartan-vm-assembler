use std::collections::HashMap;

pub struct SymbolTable {
    section_bss_variables: HashMap<String, VariableInfo>,
    section_data_variables: HashMap<String, VariableInfo>,
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
            section_bss_variables: HashMap::new(),
            section_data_variables: HashMap::new(),
            current_offset: 0,
        }
    }

    pub fn insert_bss_var(&mut self, name: String, type_size: u32, total_size: u32) -> Option<VariableInfo> {
        if self.section_data_variables.contains_key(&name) || self.section_bss_variables.contains_key(&name) {
            return None;
        }

        let variable_info = VariableInfo::new(type_size, total_size, self.current_offset);
        self.section_bss_variables.insert(name, variable_info.clone());
        self.current_offset += total_size;
        Some(variable_info)
    }

    #[allow(dead_code)]
    pub fn insert_data_var(&mut self, name: String, type_size: u32, total_size: u32) -> bool {
        if self.section_data_variables.contains_key(&name) || self.section_bss_variables.contains_key(&name) {
            return false;
        }

        self.section_data_variables.insert(name, VariableInfo::new(type_size, total_size, self.current_offset));
        self.current_offset += total_size;
        true
    }

    #[allow(dead_code)]
    pub fn get_variable(&self, name: String) -> Option<&VariableInfo> {
        if let Some(variable_info) = self.section_data_variables.get(&name) {
            return Some(variable_info);
        }

        if let Some(variable_info) = self.section_bss_variables.get(&name) {
            return Some(variable_info);
        }

        None
    }

    #[allow(dead_code)]
    pub fn get_bss_section_size(&self) -> u32 {
        self.section_bss_variables.values().map(|element| element.total_size()).sum()
    }

    #[allow(dead_code)]
    pub fn get_data_section_size(&self) -> u32 {
        self.section_data_variables.values().map(|element| element.total_size()).sum()
    }

    #[allow(dead_code)]
    pub fn get_current_offset(&self) -> u32 {
        self.current_offset
    }
}

