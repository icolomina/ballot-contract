#![cfg(test)]

use super::{Ballot, BallotClient};
use chrono::Utc;
use soroban_sdk::{Env, Symbol, testutils::Address as _, Address};

#[test]
fn configure_test() {
    let env = Env::default();
    let client = create_client(&env);
    let addr_admin = Address::random(&env);

    let from: i64 = 5985244457;
    let to: i64 = 69985745587;

    let result_conf = client.configure(&addr_admin, &from, &to);
    assert_eq!(result_conf, true);
}

#[test]
fn vote_test() {
    let env = Env::default();
    let client = create_client(&env);
    let addr_admin = Address::random(&env);

    configure_valid_ballot(&addr_admin, &client);

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
#[should_panic(expected = "Error(9)")]
fn vote_ballot_not_configured() {
    let env = Env::default();
    let client = create_client(&env);

    let addr_admin = Address::random(&env);
    client.vote(&addr_admin, &Symbol::short("hyyt76"), &Symbol::short("Laborist"));
}

#[test]
#[should_panic(expected = "Error(2)")]
fn vote_test_already_voted() {
    let env = Env::default();
    let client = create_client(&env);
    let addr_admin = Address::random(&env);
    configure_valid_ballot(&addr_admin, &client);

    client.vote(&addr_admin, &Symbol::short("hyyt76"), &Symbol::short("Laborist"));
    client.vote(&addr_admin, &Symbol::short("hyyt76"), &Symbol::short("Laborist"));
}

#[test]
#[should_panic(expected = "Error(1)")]
fn vote_test_delegated_vote() {
    let env = Env::default();
    let client = create_client(&env);
    let addr_admin = Address::random(&env);
    configure_valid_ballot(&addr_admin, &client);

    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("oonvv5"));
    client.vote(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("Laborist"));
}

#[test]
#[should_panic(expected = "Error(7)")]
fn vote_before_from() {
    let env = Env::default();
    let client = create_client(&env);
    let addr_admin = Address::random(&env);
    configure_invalid_ballot_before(&addr_admin, &client);
    
    client.vote(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("Laborist"));
}

#[test]
#[should_panic(expected = "Error(7)")]
fn vote_after_to() {
    let env = Env::default();
    let client = create_client(&env);
    let addr_admin = Address::random(&env);

    configure_invalid_ballot_after(&addr_admin, &client);
    client.vote(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("Laborist"));
}

#[test]
#[should_panic(expected = "Error(3)")]
fn delegate_test_has_delegated_votes() {
    let env = Env::default();
    let client = create_client(&env);

    let addr_admin = Address::random(&env);
    configure_valid_ballot(&addr_admin, &client);
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("oonvv5"));
    client.delegate(&addr_admin, &Symbol::short("oonvv5"), &Symbol::short("ppky55"));
}

#[test]
#[should_panic(expected = "Error(1)")]
fn delegate_test_has_delegated_his_vote() {
    let env = Env::default();
    let client = create_client(&env);

    let addr_admin = Address::random(&env);
    configure_valid_ballot(&addr_admin, &client);
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("oonvv5"));
    client.delegate(&addr_admin, &Symbol::short("hhvftp"), &Symbol::short("ippcxs"));
}

#[test]
#[should_panic(expected = "Error(4)")]
fn delegate_test_o_voter_has_voted() {
    let env = Env::default();
    let client = create_client(&env);

    let addr_admin = Address::random(&env);
    configure_valid_ballot(&addr_admin, &client);
    client.vote(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("Laborist"));
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("hhcfrp"));
}

#[test]
#[should_panic(expected = "Error(5)")]
fn delegate_test_d_voter_has_voted() {
    let env = Env::default();
    let client = create_client(&env);

    let addr_admin = Address::random(&env);
    configure_valid_ballot(&addr_admin, &client);
    client.vote(&addr_admin, &Symbol::short("hhcfrp"), &Symbol::short("Laborist"));
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("hhcfrp"));
}

#[test]
#[should_panic(expected = "Error(9)")]
fn delegate_ballot_not_configured() {
    let env = Env::default();
    let client = create_client(&env);

    let addr_admin = Address::random(&env);
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("hhcfrp"));
}

#[test]
#[should_panic(expected = "Error(8)")]
fn delegate_before_from() {
    let env = Env::default();
    let client = create_client(&env);

    let addr_admin = Address::random(&env);
    configure_invalid_ballot_before(&addr_admin, &client);
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("hhcfrp"));
}

#[test]
#[should_panic(expected = "Error(8)")]
fn delegate_after_to() {
    let env = Env::default();
    let client = create_client(&env);

    let addr_admin = Address::random(&env);
    configure_invalid_ballot_after(&addr_admin, &client);
    client.delegate(&addr_admin, &Symbol::short("ippcxs"), &Symbol::short("hhcfrp"));
}

fn create_client(env: &Env) -> BallotClient{
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Ballot);
    let client = BallotClient::new(&env, &contract_id);
    client
}

fn configure_valid_ballot(admin: &Address, client: &BallotClient) {
    let dt: chrono::DateTime<Utc> = Utc::now();
    let now: i64 = dt.timestamp();
    let from = now - 50000;
    let to = now + 50000;
    client.configure(&admin, &from, &to);
}

fn configure_invalid_ballot_before(admin: &Address, client: &BallotClient) {
    let dt: chrono::DateTime<Utc> = Utc::now();
    let now: i64 = dt.timestamp();
    let from = now + 10000;
    let to = from + 50000;

    client.configure(&admin, &from, &to);
}

fn configure_invalid_ballot_after(admin: &Address, client: &BallotClient) {
    let dt: chrono::DateTime<Utc> = Utc::now();
    let now: i64 = dt.timestamp();
    let from = now - 100000;
    let to = from - 50000;

    client.configure(&admin, &from, &to);
}