#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Symbol, Env, Map, Address};

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {

    // Set or update reputation score (only user can update their own score)
    pub fn set_score(env: Env, user: Address, score: u32) {
        // Authentication (important)
        user.require_auth();

        let key = symbol_short!("SCORES");

        let mut scores: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        scores.set(user.clone(), score);

        env.storage().instance().set(&key, &scores);
    }

    // Get reputation score
    pub fn get_score(env: Env, user: Address) -> u32 {
        let key = symbol_short!("SCORES");

        let scores: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Map::new(&env));

        scores.get(user).unwrap_or(0)
    }
}