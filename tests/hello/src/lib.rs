#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) -> Env {
      let var1 = &env;
      symbol_short!("Hello")
    }

    pub fn hello2(env: Env, to: Symbol) -> Vec<Symbol> {
      let virtual_env = &env;
      vec![virtual_env, symbol_short!("Hello"), to]
    }

    pub fn test_internal(env:Env) {

    }
}
