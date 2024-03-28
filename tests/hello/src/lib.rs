#![no_std]
use soroban_sdk::{contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contractimpl]
impl HelloContract {
    pub fn hello() -> Symbol {
      symbol_short!("Hello")
    }

    pub fn hello2(env: Env, to: Symbol) -> Vec<Symbol> {
      vec![&env, symbol_short!("Hello"), to]
    }
}
