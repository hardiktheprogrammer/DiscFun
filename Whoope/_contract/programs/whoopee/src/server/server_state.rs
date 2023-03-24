use crate::constants::*;
use anchor_lang::prelude::*;

///SEED = `SEED_SERVER_PROFILE` + `server_token_id`
#[account]
pub struct ServerState {
    pub admin: Pubkey,
    pub server_token: Pubkey,
    pub name: [u8; MAX_NAME_SIZE],
    pub summary: [u8; MAX_SERVER_SUMMARY_SIZE],
    // pub total_members: u64,
    pub total_chats: u64,
    pub created_on: i64,
}

impl ServerState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}

///SEED = `SEED_USER_SERVER_PROFILE` + `UserId` + `ServerToken`
///It have to be init before joining to the server.
#[account]
pub struct UserServerProfile {
    pub xq: i64,
    pub is_joined: bool,
}

impl UserServerProfile {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}

///SEED = `SEED_SERVER_SECTION` + `ServerToken` + `$section_name`
#[account]
pub struct ServerSection {
    pub section_name: [u8; MAX_SERVER_SECTION_NAME_SIZE],
    pub section_details: [u8; 256],
}

impl ServerSection {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}
