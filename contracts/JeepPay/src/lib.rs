#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, Map};

#[contract]
pub struct JeepPay;

#[contractimpl]
impl JeepPay {

    // Record a fare payment from commuter to driver
    pub fn pay_fare(env: Env, passenger: Address, driver: Address, amount: i128) {
        // Require passenger authorization
        passenger.require_auth();

        // Load payment history
        let mut payments: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&Symbol::short("pay"))
            .unwrap_or(Map::new(&env));

        // Update total received by driver
        let current = payments.get(driver.clone()).unwrap_or(0);
        payments.set(driver.clone(), current + amount);

        // Save updated state
        env.storage().instance().set(&Symbol::short("pay"), &payments);

        // Emit event (for frontend confirmation)
        env.events().publish(
            (Symbol::short("fare_paid"), passenger),
            (driver, amount),
        );
    }

    // Get total earnings of a driver
    pub fn get_driver_earnings(env: Env, driver: Address) -> i128 {
        let payments: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&Symbol::short("pay"))
            .unwrap_or(Map::new(&env));

        payments.get(driver).unwrap_or(0)
    }
}