use anchor_lang::prelude::*;

use super::{Vote, VoteType};

#[account]
pub struct Repo {
    pub bump: u8,
    pub approved: bool,
    pub approved_timestamp: u128,
    pub proposed_timestamp: u128,
    pub votes: i128,
    pub total_claimed: u128,
    pub subscribers: u128,
    pub publisher: Pubkey,
    pub name: String,
    pub owner: String,
    pub branch: String,
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
        let weight = match vote.vote_type {
            VoteType::Up => vote.weight as i128,
            VoteType::Down => -(vote.weight as i128),
        };
        self.votes = self.votes.checked_add(weight).unwrap();
        self.check_approve(vote.timestamp);
    }

    pub fn change_vote(&mut self, net_change: i128, timestamp: u128) {
        self.votes = self.votes.checked_add(net_change).unwrap();
        self.check_approve(timestamp);
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
