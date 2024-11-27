use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Label,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType) -> Self {
        Self { name, symbol_type }
    }
}

#[derive(Clone)]
pub struct SymbolTable {
    symbols: HashMap<Symbol, u32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, s: Symbol, v: u32) {
        self.symbols.insert(s, v);
    }

    pub fn symbol_value(&self, s: &Symbol) -> Option<&u32> {
        self.symbols.get(s)
    }
}
