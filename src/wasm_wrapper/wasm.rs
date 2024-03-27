use crate::soroban;
use crate::soroban::FunctionInfo;
use super::wasm_adapter::{InitExpr, LoadError, Module};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TableElement {
    Null,
    Func(u32),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Table {
    pub elements: Vec<TableElement>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Instance {
    module: Module,
    tables: Vec<Table>,
    spec_fn_result: Option<FunctionInfo>
}

impl Instance {
    pub fn from_file<P: AsRef<::std::path::Path>>(path: P) -> Result<Self, LoadError> {
        let module = Module::from_file(&path)?;
        let spec_fns_result = &soroban::read_contract_specs(&path);
        let spec_fn_result = soroban::find_function_specs(&spec_fns_result, "hello");
        Ok(Self {
            tables: init_tables(&module),
            module,
            spec_fn_result: spec_fn_result.cloned()
        })
    }
    pub const fn module(&self) -> &Module {
        &self.module
    }
    pub fn tables(&self) -> &[Table] {
        &self.tables
    }
    pub fn spec_fn_result(&self) -> &Option<FunctionInfo> {
        &self.spec_fn_result
    }
}

fn init_tables(module: &Module) -> Vec<Table> {
    let mut tables: Vec<_> = module
        .tables()
        .iter()
        .map(|table_type| Table {
            elements: vec![TableElement::Null; table_type.limits().initial() as usize],
        })
        .collect();
    for init in module.table_inits() {
        let table = &mut tables[init.index() as usize];
        if let InitExpr::I32Const(offset) = init.offset() {
            for (i, ele) in init.entries().iter().enumerate() {
                let ele = TableElement::Func(*ele);
                let index = i + *offset as usize;
                if index >= table.elements.len() {
                    table.elements.push(ele);
                } else {
                    table.elements[index] = ele;
                }
            }
        }
    }
    tables
}
