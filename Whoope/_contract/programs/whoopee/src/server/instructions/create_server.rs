use crate::prelude::*;
use super::super::server_state::ServerState;

pub fn create_server(
    context: Context<ACreateServer>,
    name: [u8; MAX_NAME_SIZE],
    summary: [u8; MAX_SERVER_SUMMARY_SIZE],
)-> Result<()>{
    let server_account = &mut context.accounts.server_account;

    server_account.name = name;
    server_account.summary = summary;

    //NOTE: setting current time on creation
    server_account.created_on = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
pub struct ACreateServer<'info>{
    #[account(mut)]
    pub admin:Signer<'info>,

    #[account(
        mut,
        mint::authority = admin, 
        mint::decimals = 0,
        constraint = server_token.supply == 1,
    )]
    pub server_token: Account<'info, Mint>,
    
    #[account(
        init,
        payer = admin,
        seeds = [SEED_SERVER_PROFILE, server_token.key().as_ref()],
        bump,
        space= ServerState::MAX_SIZE,
    )]
    pub server_account: Account<'info, ServerState>,

    #[account(
        mut,
        // seeds = [
        //     b"metadata",
        //     // mpl_id.as
        //     server_token.key().as_ref(),
        // ],
        // bump, 
    )]
    pub metadata_account: AccountInfo<'info>,

    pub system_program: Program<'info,System>,
}
