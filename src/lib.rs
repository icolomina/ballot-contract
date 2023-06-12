#![no_std]

use soroban_sdk::{contractimpl, contracterror, panic_with_error, Env, Symbol, Map, Address, Vec};

mod storage;
use storage::VCounter;

struct Voter<'a> {
    id: &'a Symbol
}

impl<'a> Voter<'a> {
    
    fn has_voted(&self, env: &Env) -> bool {
        let vts: Vec<Symbol> = storage::get_votes(env);
        vts.contains(self.id)
    }

    fn is_delegated(&self, env: &Env) -> bool {
        let dvts: Vec<Symbol> = storage::get_delegated_votes(env);
        dvts.contains(self.id)
    }

    fn has_delegated_votes(&self, env: &Env) -> bool {
        let dvotes = storage::get_voter_delegated_votes(env, self.id);
        if dvotes.len() > 0 {
            return true;
        }
    
        false
    }
}


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    VoterHasHisVoteDelegated = 1,
    VoterHasAlreadyVoted = 2,
    VoterHasDelegatedVotes = 3,
    VoterOriginHasAlreadyVotedAndCannotDelegate = 4,
    VoterTargetHasAlreadyVotedAndCannotDelegate = 5,
    SetConfigurationOutOfTime = 6,
    TryingToVoteOutOfBallotTime = 7,
    TryingToDelegateVoteOutOfBallotTime = 8,
    BallotNotConfigured = 9

}

pub struct Ballot;

#[contractimpl]
impl Ballot {

    pub fn vote(env: Env, admin: Address, voter: Symbol, candidate: Symbol) -> bool {
        admin.require_auth();

        let v: Voter = Voter { id: &voter };

        if v.is_delegated(&env) {
            panic_with_error!(&env, Error::VoterHasHisVoteDelegated);
        }
        if v.has_voted(&env) {
            panic_with_error!(&env, Error::VoterHasAlreadyVoted);
        }
        
        storage::store_party(&env, &candidate);

        let mut votes: Vec<Symbol> = storage::get_votes(&env);
        let candidate_key = VCounter::Counter(candidate);
        let d_votes: Vec<Symbol> = storage::get_voter_delegated_votes(&env, v.id);
        let count = 1 + d_votes.len() + storage::get_candidate_votes_count(&env, &candidate_key);
        votes.push_back(voter);
 
        storage::update_candidate_count(&env, candidate_key, count);
        storage::update_votes(&env, votes);

        true
    }

    pub fn delegate(env: Env,  admin: Address, o_voter: Symbol, d_voter: Symbol) -> bool {
        admin.require_auth();

        let ov: Voter = Voter { id: &o_voter };
        let dv: Voter = Voter { id: &d_voter };
    
        
        if ov.has_voted(&env) {
            panic_with_error!(&env, Error::VoterOriginHasAlreadyVotedAndCannotDelegate);
        }

        if dv.has_voted(&env) {
            panic_with_error!(&env, Error::VoterTargetHasAlreadyVotedAndCannotDelegate);
        }

        if ov.is_delegated(&env) {
            panic_with_error!(&env, Error::VoterHasHisVoteDelegated);
        }

        if dv.is_delegated(&env) {
            panic_with_error!(&env, Error::VoterHasHisVoteDelegated);
        }

        if ov.has_delegated_votes(&env) {
            panic_with_error!(&env, Error::VoterHasDelegatedVotes);
        }


        let mut d_votes = storage::get_delegated_votes(&env);
        let mut d_vot_delegs: Vec<Symbol> = storage::get_voter_delegated_votes(&env, &d_voter);
        d_votes.push_back(o_voter.clone());
        d_vot_delegs.push_back(o_voter.clone());

        storage::update_delegated_votes(&env, d_votes);
        storage::update_voter_delegated_votes(&env, d_voter, d_vot_delegs);

        true

    }

    pub fn count(env: Env,  admin: Address) -> Map<Symbol, u32> {
        
        admin.require_auth();
        let pts = storage::get_candidates(&env);
        let mut count_map: Map<Symbol, u32>= Map::new(&env);
        for party in pts.iter() {
            match party {
                Ok(p) => {
                    let candidate_key = VCounter::Counter(p.clone());
                    let candidate_count: u32 = storage::get_candidate_votes_count(&env, &candidate_key);
                    count_map.set(p, candidate_count);
                },
                _ => ()
            }
        }

        count_map
    }
}

mod test;
