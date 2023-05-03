#![cfg(test)]

use super::{Ballot, BallotClient};
use soroban_sdk::{Env, Symbol, testutils::Address as _, Address};


#[test]
fn vote_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Ballot);
    let client = BallotClient::new(&env, &contract_id);

    let addr_admin = Address::random(&env);

    assert_eq!(client.vote(&addr_admin, &Symbol::short("hyyt76"), &Symbol::short("Laborist")), true);
    assert_eq!(client.vote(&addr_admin, &Symbol::short("ptft37"), &Symbol::short("Conserv")), true);
    assert_eq!(client.vote(&addr_admin, &Symbol::short("oo9gt6"), &Symbol::short("Conserv")), true);

    let count = client.count(&addr_admin);

    assert_eq!(count.get(Symbol::short("Laborist")).unwrap(), Ok(1));
    assert_eq!(count.get(Symbol::short("Conserv")).unwrap(), Ok(2));

    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("oonvv5"));
    assert_eq!(client.vote(&addr_admin, &Symbol::short("oonvv5"), &Symbol::short("Conserv")), true);

    let count = client.count(&addr_admin);

    assert_eq!(count.get(Symbol::short("Laborist")).unwrap(), Ok(1));
    assert_eq!(count.get(Symbol::short("Conserv")).unwrap(), Ok(4));

}

#[test]
#[should_panic(expected = "Error(2)")]
fn vote_test_already_voted() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Ballot);
    let client = BallotClient::new(&env, &contract_id);

    let addr_admin = Address::random(&env);

    client.vote(&addr_admin, &Symbol::short("hyyt76"), &Symbol::short("Laborist"));
    client.vote(&addr_admin, &Symbol::short("hyyt76"), &Symbol::short("Laborist"));
}

#[test]
#[should_panic(expected = "Error(1)")]
fn vote_test_delegated_vote() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Ballot);
    let client = BallotClient::new(&env, &contract_id);

    let addr_admin = Address::random(&env);
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("oonvv5"));
    client.vote(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("Laborist"));
}

#[test]
#[should_panic(expected = "Error(3)")]
fn delegate_test_has_delegated_votes() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Ballot);
    let client = BallotClient::new(&env, &contract_id);

    let addr_admin = Address::random(&env);
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("oonvv5"));
    client.delegate(&addr_admin, &Symbol::short("oonvv5"), &Symbol::short("ppky55"));
}

#[test]
#[should_panic(expected = "Error(1)")]
fn delegate_test_has_delegated_his_vote() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Ballot);
    let client = BallotClient::new(&env, &contract_id);

    let addr_admin = Address::random(&env);
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("oonvv5"));
    client.delegate(&addr_admin, &Symbol::short("hhvftp"), &Symbol::short("ippcxs"));
}