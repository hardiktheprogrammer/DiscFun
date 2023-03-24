use crate::{main_account_state::MainAccount, prelude::*, server_state::{ServerState, UserServerProfile}};
use anchor_spl::token::{Burn, Revoke};

pub fn leave_server(context: Context<ALeaveServer>) -> Result<()> {
    let user = context.accounts.user.to_account_info();
    let user_token_account = context.accounts.user_token_account.to_account_info();
    let server_token = context.accounts.server_token.to_account_info();
    let server_account = context.accounts.server_account.to_account_info();
    let token_program = context.accounts.token_program.to_account_info();
    // let main_account = context.accounts.main_account.to_account_info();

    let (_, _bump) = Pubkey::find_program_address(
        &[SEED_SERVER_PROFILE, server_token.key().as_ref()],
        context.program_id,
    );

    //NOTE: unfreezing the user token account so that user can't transfer the server token.
    let cpi_accounts = Revoke {
        authority: server_account,
        source: user_token_account.to_account_info(),
    };

    token::revoke(CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts,
        &[&[SEED_SERVER_PROFILE, server_token.key().as_ref(), &[_bump]]],
    ))?;

    //NOTE: Burn a token which represnat you don't have server token you can't chat.
    let cpi_accounts = Burn {
        authority: user,
        mint: server_token.to_account_info(),
        from: user_token_account.to_account_info(),
    };

    token::burn(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            cpi_accounts,
            &[&[SEED_SERVER_PROFILE, server_token.key().as_ref(), &[_bump]]],
        ),
        1,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct ALeaveServer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token::authority = user,
        token::mint = server_token,
        constraint = user_token_account.amount == 0,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [SEED_USER_SERVER_PROFILE, user.key().as_ref(), server_token.key().as_ref()],
        bump,
    )]
    pub user_server_profile_account: Account<'info, UserServerProfile>,

    ///CHECK:
    #[account(mut)]
    pub server_token: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_SERVER_PROFILE, server_token.key().as_ref()],
        bump,
    )]
    pub server_account: Account<'info, ServerState>,

    // #[account(
    //     mut,
    //     seeds = [SEED_MAIN],
    //     bump,
    // )]
    // pub main_account: Account<'info, MainAccount>,

    pub token_program: Program<'info, Token>,
}
