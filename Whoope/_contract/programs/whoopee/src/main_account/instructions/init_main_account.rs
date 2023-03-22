use crate::{constants::SEED_MAIN, main_account::main_account_state::MainAccount};
use anchor_lang::prelude::*;

pub fn init_main_account(context: Context<AInitMainAccount>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct AInitMainAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer= owner,
        seeds = [SEED_MAIN],
        bump,
        space = MainAccount::MAX_SIZE,
    )]
    pub main_account: Account<'info, MainAccount>,

    pub system_program: Program<'info, System>,
}
