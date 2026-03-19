// =============================
// Simple DAO Smart Contract (Soroban - Stellar)
// =============================
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Address};

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub description: Symbol,
    pub votes_for: u32,
    pub votes_against: u32,
}

#[contracttype]
pub enum DataKey {
    Proposal(u32),
    ProposalCount,
}

#[contract]
pub struct SimpleDAO;

#[contractimpl]
impl SimpleDAO {
    // Create a proposal
    pub fn create_proposal(env: Env, description: Symbol) -> u32 {
        let mut count: u32 = env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0);
        count += 1;

        let proposal = Proposal {
            id: count,
            description,
            votes_for: 0,
            votes_against: 0,
        };

        env.storage().instance().set(&DataKey::Proposal(count), &proposal);
        env.storage().instance().set(&DataKey::ProposalCount, &count);

        count
    }

    // Vote on proposal
    pub fn vote(env: Env, voter: Address, proposal_id: u32, support: bool) {
        voter.require_auth();

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found");

        if support {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }

        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
    }

    // Get proposal
    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        env.storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found")
    }
}