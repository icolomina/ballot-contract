#![cfg(test)]

use super::{Ballot, BallotClient};
use soroban_sdk::{Env, Symbol, testutils::Address as _, Address};


#[test]
fn vote_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Ballot);
    let client = BallotClient::new(&env, &contract_id);

    let addr_admin = Address::random(&env);

    assert_eq!(client.vote(&addr1, &Symbol::short("hyyt76"), &Symbol::short("Laborist")), true);
    assert_eq!(client.vote(&addr2, &Symbol::short("htft76"), &Symbol::short("Conserv")), true);

}

#[test]
#[should_panic(expected = "Error(1)")]
fn vote_test_already_voted() {

}