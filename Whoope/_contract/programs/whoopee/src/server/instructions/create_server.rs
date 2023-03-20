use crate::prelude::*;
use super::super::server_state::ServerState;

pub fn create_server(context: Context<ACreateServer>)-> Result<()>{
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
