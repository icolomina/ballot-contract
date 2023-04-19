#![no_std]

use soroban_sdk::{contractimpl, contracttype, panic_with_error, Env, Symbol, Map, Address, Vec};

const VOTES: Symbol = Symbol::short("votes");
const PARTIES: Symbol = Symbol::short("parties");
const DVOTES: Symbol = Symbol::short("dvotes");

fn get_parties(env: &Env) -> Vec<Symbol> {
    let pts: Vec<Symbol> = env
        .storage()
        .get(&PARTIES)
        .unwrap_or(Ok(Vec::new(env)))
        .unwrap()
    ;

    pts
}

fn store_party(env: &Env, p: &Symbol) -> bool {
    let mut pts: Vec<Symbol> = get_parties(env);
    if !pts.contains(p) {
        pts.push_back(p.clone());
        env.storage().set(&PARTIES, &pts);
        return true;
    }

    false
}

fn get_votes(env: &Env) -> Vec<Symbol>{
    let vts: Vec<Symbol> = env
        .storage()
        .get(&VOTES)
        .unwrap_or(Ok(Vec::new(env)))
        .unwrap()
    ;

    vts
}

fn voter_has_voted(env: &Env, voter: &Symbol) -> bool {
    let vts: Vec<Symbol> = get_votes(env);
    vts.contains(voter)
}

fn voter_is_delegated(env: &Env, voter: &Symbol) -> bool {
    let dvts: Vec<Symbol> = env
        .storage()
        .get(&DVOTES)
        .unwrap_or(Ok(Vec::new(env)))
        .unwrap()
    ;

    dvts.contains(voter)
}

fn count_delegated_votes(env: &Env, d_voter_ckey: DCounter) -> u32 {
    let v_dvts: u32 = env
        .storage()
        .get(&d_voter_ckey)
        .unwrap_or(Ok(0))
        .unwrap()
    ;

    v_dvts
}

#[contracttype]
pub enum VCounter {
    Counter(Symbol)
}

#[contracttype]
pub enum DCounter {
    Counter(Symbol)
}

pub struct Ballot;

#[contractimpl]
impl Ballot {

    pub fn vote(env: Env, admin: Address, voter: Symbol, party: Symbol) -> bool {
        admin.require_auth();

        if voter_is_delegated(&env, &voter) {

        }
        if voter_has_voted(&env, &voter) {
            
        }
        
        store_party(&env, &party);

        let mut votes: Vec<Symbol> = get_votes(&env);
        let pckey = VCounter::Counter(party);
        let dvot_del_counter = DCounter::Counter(voter.clone());
        let count = 1 + count_delegated_votes(&env, dvot_del_counter) + env.storage().get(&pckey).unwrap_or(Ok(0)).unwrap();
        votes.push_back(voter);
 
        env.storage().set(&pckey, &count);
        env.storage().set(&VOTES, &votes);

        true
    }

    pub fn delegate(env: Env,  admin: Address, o_voter: Symbol, d_voter: Symbol) -> bool {
        admin.require_auth();
        if voter_is_delegated(&env, &d_voter) || voter_is_delegated(&env, &o_voter){

        }

        let dvot_del_counter = DCounter::Counter(d_voter);
        let mut d_votes = env.storage().get(&DVOTES).unwrap_or(Ok(Vec::new(&env))).unwrap();
        let mut d_vot_delegs: u32 = env.storage().get(&dvot_del_counter).unwrap_or(Ok(0)).unwrap();
        d_votes.push_back(o_voter.clone());
        
        d_vot_delegs += 1;

        env.storage().set(&DVOTES, &d_votes);
        env.storage().set(&dvot_del_counter, &d_vot_delegs);

        true

    }

    pub fn count(env: Env,  admin: Address) -> Map<Symbol, u32> {
        
        admin.require_auth();
        let pts = get_parties(&env);
        let mut count_map: Map<Symbol, u32>= Map::new(&env);
        for party in pts.iter() {
            match party {
                Ok(p) => {
                    let pckey = VCounter::Counter(p.clone());
                    let pcount: u32 = env.storage().get(&pckey).unwrap_or(Ok(0)).unwrap();
                    count_map.set(p, pcount);
                },
                _ => ()
            }
        }

        count_map
    }
}
