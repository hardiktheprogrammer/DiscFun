use crate::constants::*;
use crate::server_state::{ServerState, UserServerProfile};
use anchor_lang::prelude::*;

pub fn init_user_server_profile(context: Context<AInitUserServerProfile>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct AInitUserServerProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [SEED_USER_SERVER_PROFILE, user.key().as_ref(), server_token.key().as_ref()],
        bump,
        space = UserServerProfile::MAX_SIZE,
    )]
    pub user_server_profile_account: Account<'info, UserServerProfile>,

    ///CHECK:
    #[account()]
    pub server_token: AccountInfo<'info>,

    ///NOTE: We are make suring that the `SERVER` should be initialized
    #[account(
        seeds = [SEED_SERVER_PROFILE, server_token.key().as_ref()],
        bump,
    )]
    pub server_account: Account<'info, ServerState>,

    pub system_program: Program<'info, System>,
}
