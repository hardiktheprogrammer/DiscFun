use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ServerState {
    name: [u8; MAX_NAME_SIZE],
    summary: [u8; MAX_SERVER_SUMMARY_SIZE],
}
