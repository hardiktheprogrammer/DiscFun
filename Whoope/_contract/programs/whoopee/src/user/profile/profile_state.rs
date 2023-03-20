use crate::constants::*;
use anchor_lang::prelude::*;

///SEED = `SEED_USER_PROFILE` + `user_ID`
#[account]
pub struct ProfileState {
    pub name: [u8; MAX_NAME_SIZE],
    pub summary: [u8; MAX_PROFILE_SUMMARY_SIZE],
    pub total_chats: u64,
    pub created_on: i64,
}

impl ProfileState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}
