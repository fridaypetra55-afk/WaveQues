use soroban_sdk::{contractimpl, Env, Map, Address};

pub struct LeaderboardContract;

#[contractimpl]
impl LeaderboardContract {
    pub fn get_rank(env: Env, addr: Address) -> i32 {
        0
    }
}
