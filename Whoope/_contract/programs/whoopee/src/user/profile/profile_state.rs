use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProfileState {
    name: [u8; MAX_NAME_SIZE],
    summary: [u8; MAX_PROFILE_SUMMARY_SIZE],
}
