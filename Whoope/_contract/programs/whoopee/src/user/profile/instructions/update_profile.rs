use crate::{
    constants::{MAX_NAME_SIZE, MAX_SERVER_SUMMARY_SIZE, SEED_USER_PROFILE},
    user::profile::profile_state::ProfileState,
};
use anchor_lang::prelude::*;

pub fn update_profile(
    context: Context<AUpdateProfile>,
    name: [u8; MAX_NAME_SIZE],
    summary: [u8; MAX_SERVER_SUMMARY_SIZE],
) -> Result<()> {
    let profile_account = &mut context.accounts.profile_account;
    profile_account.name = name;
    profile_account.summary = summary;

    Ok(())
}

#[derive(Accounts)]
pub struct AUpdateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_USER_PROFILE, user.key().as_ref()],
        bump,
    )]
    pub profile_account: Account<'info, ProfileState>,

    pub system_program: Program<'info, System>,
}
