use soroban_sdk::{contractimpl, Env, Address};

pub struct RewardsContract;

#[contractimpl]
impl RewardsContract {
    pub fn claim(env: Env, addr: Address) {
        // TODO: trigger XLM transfer via Stellar SDK
    }
}
