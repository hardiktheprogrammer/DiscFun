use crate::{prelude::*, server_state::ServerState};

pub fn create_server_section(
    context: Context<ACreateServerSection>,
    section_name: [u8; MAX_SERVER_SECTION_NAME_SIZE],
    section_details: [u8; MAX_SERVER_SECTION_DETAILS_SIZE],
) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    section_name: [u8; MAX_SERVER_SECTION_NAME_SIZE],
    section_details: [u8; 256],
)]
pub struct ACreateServerSection<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account()]
    pub server_token: Account<'info, Mint>,

    #[account(
        // mut, 
        seeds = [SEED_SERVER_PROFILE, server_token.key().as_ref()],
        bump,
    )]
    pub server_account: Account<'info, ServerState>,
}
