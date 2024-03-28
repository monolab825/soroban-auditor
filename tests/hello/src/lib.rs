#![no_std]
use soroban_sdk::{contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}
