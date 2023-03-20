use crate::constants::*;
use anchor_lang::prelude::*;

///SEED = `SEED_SERVER_PROFILE` + `server_token_id`
#[account]
pub struct ServerState {
    pub name: [u8; MAX_NAME_SIZE],
    pub summary: [u8; MAX_SERVER_SUMMARY_SIZE],
    pub total_members: u64,
    pub total_chats: u64,
    pub created_on: i64,
}

impl ServerState {
    pub const MAX_SIZE: usize = std::mem::size_of::<ServerState>();
}
