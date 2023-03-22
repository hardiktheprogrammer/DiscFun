use crate::{
    constants::{MAX_NAME_SIZE, MAX_SERVER_SUMMARY_SIZE, SEED_USER_PROFILE},
    user::profile::profile_state::ProfileState,
};
pub use anchor_lang::prelude::*;

pub fn create_profile(
    context: Context<ACreateProfile>,
    name: [u8; MAX_NAME_SIZE],
    summary: [u8; MAX_SERVER_SUMMARY_SIZE],
) -> Result<()> {
    let profile_account = &mut context.accounts.profile_account;

    profile_account.name = name;
    profile_account.summary = summary;

    //NOTE: setting current time on creation
    profile_account.created_on = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
pub struct ACreateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [SEED_USER_PROFILE, user.key().as_ref()],
        bump,
        space = ProfileState::MAX_SIZE 
    )]
    pub profile_account: Account<'info, ProfileState>,

    pub system_program: Program<'info, System>,
}
