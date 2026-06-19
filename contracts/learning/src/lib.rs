use soroban_sdk::{contractimpl, symbol, Env, Map, Vec, Address};

pub struct LearningContract;

#[derive(Clone)]
pub struct Learner {
    pub xp: i128,
    pub streak: i64,
    pub last_active: i64,
}

#[contractimpl]
impl LearningContract {
    pub fn register(env: Env, addr: Address) {
        let key = symbol!("learner:") ;
        let mut m: Map<Address, Learner> = Env::from(env.clone()).get_contract_data().unwrap_or_default();
        let learner = Learner { xp: 0, streak: 0, last_active: 0 };
        m.set(addr, learner);
        // store back
    }

    pub fn add_xp(env: Env, addr: Address, amount: i128) {
        // TODO: implement
    }
}

mod test {
    // tests would go here
}
