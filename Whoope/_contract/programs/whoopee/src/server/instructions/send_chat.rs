use crate::{prelude::*, server::server_state::ServerState};

pub fn send_chat(context: Context<ASendChat>, chat: Vec<u8>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ASendChat<'info> {
    #[account(mut)]
    sender: Signer<'info>,

    ///CHECK:
    #[account()]
    server_token: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_SERVER_PROFILE, server_token.key().as_ref()],
        bump,
    )]
    pub server_account: Account<'info, ServerState>,
}
