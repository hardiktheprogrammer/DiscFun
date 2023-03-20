use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ServerState {
    name: [u8; MAX_NAME_SIZE],
    summary: [u8; MAX_SERVER_SUMMARY_SIZE],
    total_members: u64,
}

impl ServerState {
    pub const MAX_SIZE: usize = std::mem::size_of::<ServerState>();
}
