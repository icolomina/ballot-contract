use soroban_sdk::{ Env, Symbol, Vec, contracttype};

pub const VOTES: Symbol = Symbol::short("votes");
pub const PARTIES: Symbol = Symbol::short("parties");
pub const DVOTES: Symbol = Symbol::short("dvotes");
pub const CONFIG: Symbol = Symbol::short("config");

#[derive(Debug)]
#[contracttype]
pub struct Config {
    pub from: i64,
    pub to: i64
}

#[derive(Debug)]
#[contracttype]
pub enum ConfigTypes {
    Str(Symbol),
    Int(u32)
}

#[contracttype]
pub enum VCounter {
    Counter(Symbol)
}

pub fn get_candidates(env: &Env) -> Vec<Symbol> {
    let pts: Vec<Symbol> = env
        .storage()
        .get(&PARTIES)
        .unwrap_or(Ok(Vec::new(env)))
        .unwrap()
    ;

    pts
}

pub fn store_party(env: &Env, p: &Symbol) -> bool {
    let mut pts: Vec<Symbol> = get_candidates(env);
    if !pts.contains(p) {
        pts.push_back(p.clone());
        env.storage().set(&PARTIES, &pts);
        return true;
    }

    false
}

pub fn get_votes(env: &Env) -> Vec<Symbol>{
    let vts: Vec<Symbol> = env
        .storage()
        .get(&VOTES)
        .unwrap_or(Ok(Vec::new(env)))
        .unwrap()
    ;

    vts
}

pub fn get_voter_delegated_votes(env: &Env, d_voter: &Symbol) -> Vec<Symbol> {
    let v_dvts: Vec<Symbol> = env
        .storage()
        .get(d_voter)
        .unwrap_or(Ok(Vec::new(env)))
        .unwrap()
    ;

    v_dvts
}

pub fn get_delegated_votes(env: &Env) -> Vec<Symbol> {
    let dvts: Vec<Symbol> = env
        .storage()
        .get(&DVOTES)
        .unwrap_or(Ok(Vec::new(env)))
        .unwrap()
    ;

    dvts
}

pub fn update_config(env: &Env, from: i64, to: i64) {
    let config: Config = Config { from, to };
    env.storage().set(&CONFIG, &config);
}

pub fn get_config(env: &Env) -> Option<Config>{
    let config: Option<Config> = env
        .storage()
        .get(&CONFIG)
        .unwrap_or(Ok(None))
        .unwrap();

    config
}

pub fn get_candidate_votes_count(env: &Env, candidate: &VCounter) -> u32 {
    let total_votes = env.storage().get(candidate).unwrap_or(Ok(0)).unwrap();
    total_votes
}

pub fn update_candidate_count(env: &Env, candidate: VCounter, count: u32) {
    env.storage().set(&candidate, &count);
}

pub fn update_votes(env: &Env, votes: Vec<Symbol>) {
    env.storage().set(&VOTES, &votes);
}

pub fn update_delegated_votes(env: &Env, d_votes: Vec<Symbol>) {
    env.storage().set(&DVOTES, &d_votes);
}

pub fn update_voter_delegated_votes(env: &Env, d_voter: Symbol, d_vot_delegs: Vec<Symbol>) {
    env.storage().set(&d_voter, &d_vot_delegs);
}