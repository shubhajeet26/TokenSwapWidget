#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct TokenSwap;

#[contractimpl]
impl TokenSwap {
    // Initialize contract with admin
    pub fn initialize(env: Env, admin: Address) {
        let key = Symbol::short("ADMIN");
        env.storage().instance().set(&key, &admin);
    }

    // Swap function (fixed rate: 1:1 for simplicity)
    pub fn swap(
        env: Env,
        user: Address,
        token_a: Address,
        token_b: Address,
        amount: i128,
    ) {
        user.require_auth();

        // Transfer token A from user to contract
        let client_a = soroban_sdk::token::Client::new(&env, &token_a);
        client_a.transfer(&user, &env.current_contract_address(), &amount);

        // Transfer token B from contract to user
        let client_b = soroban_sdk::token::Client::new(&env, &token_b);
        client_b.transfer(&env.current_contract_address(), &user, &amount);
    }

    // Deposit liquidity into contract
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
    ) {
        from.require_auth();

        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&from, &env.current_contract_address(), &amount);
    }
}