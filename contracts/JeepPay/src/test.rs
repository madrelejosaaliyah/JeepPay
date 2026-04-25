#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Address};

    #[test]
    fn test_payment_success() {
        let env = Env::default();
        let contract_id = env.register_contract(None, JeepPay);
        let client = JeepPayClient::new(&env, &contract_id);

        let passenger = Address::random(&env);
        let driver = Address::random(&env);

        client.pay_fare(&passenger, &driver, &13);
        assert_eq!(client.get_driver_earnings(&driver), 13);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized() {
        let env = Env::default();
        let contract_id = env.register_contract(None, JeepPay);
        let client = JeepPayClient::new(&env, &contract_id);

        let passenger = Address::random(&env);
        let driver = Address::random(&env);

        // No auth simulation → should panic
        client.pay_fare(&passenger, &driver, &13);
    }

    #[test]
    fn test_state_update() {
        let env = Env::default();
        let contract_id = env.register_contract(None, JeepPay);
        let client = JeepPayClient::new(&env, &contract_id);

        let passenger = Address::random(&env);
        let driver = Address::random(&env);

        client.pay_fare(&passenger, &driver, &10);
        client.pay_fare(&passenger, &driver, &5);

        assert_eq!(client.get_driver_earnings(&driver), 15);
    }

    #[test]
    fn test_zero_initial() {
        let env = Env::default();
        let contract_id = env.register_contract(None, JeepPay);
        let client = JeepPayClient::new(&env, &contract_id);

        let driver = Address::random(&env);
        assert_eq!(client.get_driver_earnings(&driver), 0);
    }

    #[test]
    fn test_multiple_drivers() {
        let env = Env::default();
        let contract_id = env.register_contract(None, JeepPay);
        let client = JeepPayClient::new(&env, &contract_id);

        let passenger = Address::random(&env);
        let d1 = Address::random(&env);
        let d2 = Address::random(&env);

        client.pay_fare(&passenger, &d1, &10);
        client.pay_fare(&passenger, &d2, &20);

        assert_eq!(client.get_driver_earnings(&d1), 10);
        assert_eq!(client.get_driver_earnings(&d2), 20);
    }
}

