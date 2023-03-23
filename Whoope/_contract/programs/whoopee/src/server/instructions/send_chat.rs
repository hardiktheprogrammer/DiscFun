use crate::{prelude::*, server::server_state::ServerState};

pub fn send_chat(context: Context<ASendChat>, chat: Vec<u8>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ASendChat<'info> {
    #[account(mut)]
    sender: Signer<'info>,

    #[account(
        token::authority = sender,
        //NOTE: you have a token (which represant you have joined to the server) 
        constraint = sender_token_account.amount == 1, 
    )]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [SEED_SERVER_PROFILE, sender_token_account.mint.as_ref()],
        bump,
    )]
    pub server_account: Account<'info, ServerState>,
}
