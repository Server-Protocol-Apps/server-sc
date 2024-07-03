use anchor_lang::prelude::*;

use super::{Vote, VoteType};

#[account]
pub struct Repo {
    pub bump: u8,                 // 1
    pub approved: bool,           // 1
    pub approved_timestamp: u128, // 16
    pub proposed_timestamp: u128, // 16
    pub votes: i128,              // 16
    pub total_claimed: u128,      // 16
    pub subscribers: u128,        // 16
    pub publisher: Pubkey,        // 32
    pub name: String,             // 4 + len()
    pub owner: String,            // 4 + len()
    pub branch: String,           // 4 + len()
}

impl Repo {
    pub fn size(name: &String, owner: &String, branch: &String) -> usize {
        8 + 1
            + 1
            + 16
            + 16
            + 16
            + 16
            + 16
            + 32
            + 4
            + 4
            + 4
            + name.len()
            + owner.len()
            + branch.len()
    }

    pub fn check_approve(&mut self, timestamp: u128) {
        if self.votes >= 1 && !self.approved {
            self.approved_timestamp = timestamp;
            self.approved = true;
        }

        if self.votes < 1 && self.approved {
            self.approved = false;
            self.approved_timestamp = 0;
        }
    }

    pub fn vote(&mut self, vote: &Vote) {
        match vote.vote_type {
            VoteType::Up => self.votes = self.votes.checked_add(1).unwrap(),
            VoteType::Down => self.votes = self.votes.checked_sub(1).unwrap(),
        }
        self.check_approve(vote.timestamp);
    }

    pub fn change_vote(&mut self, vote: &Vote) {
        match vote.vote_type {
            VoteType::Up => self.votes = self.votes.checked_add(2).unwrap(),
            VoteType::Down => self.votes = self.votes.checked_sub(2).unwrap(),
        }
        self.check_approve(vote.timestamp);
    }

    pub fn update_total_claimed(&mut self, rewards: u128) {
        self.total_claimed = self.total_claimed.checked_add(rewards).unwrap();
    }

    pub fn add_subscriber(&mut self) {
        self.subscribers = self.subscribers.checked_add(1).unwrap();
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct RepoPayload {
    pub owner: String,
    pub name: String,
    pub branch: String,
}

impl RepoPayload {
    pub fn serialize(&self) -> Vec<u8> {
        self.try_to_vec().unwrap()
    }
}
