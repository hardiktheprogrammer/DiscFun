use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProfileState {
    pub name: [u8; MAX_NAME_SIZE],
    pub summary: [u8; MAX_PROFILE_SUMMARY_SIZE],
}

impl ProfileState {
    pub const MAX_SIZE: usize = std::mem::size_of::<Self>();
}
