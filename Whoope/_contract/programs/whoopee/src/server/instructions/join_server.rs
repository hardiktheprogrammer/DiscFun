use crate::{
    main_account_state::MainAccount,
    prelude::*,
    server_state::{ServerState, UserServerProfile},
};
use anchor_spl::token::FreezeAccount;

pub fn join_server(context: Context<AJoinServer>) -> Result<()> {
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

    let cpi_accounts = MintTo {
        authority: server_account.to_account_info(),
        mint: server_token.to_account_info(),
        to: user_token_account.to_account_info(),
    };

    token::mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            cpi_accounts,
            &[&[SEED_SERVER_PROFILE, server_token.key().as_ref(), &[_bump]]],
        ),
        1,
    )?;

    //NOTE: freezing the user token account so that user can't transfer the server token.
    let cpi_accounts = FreezeAccount {
        authority: server_account,
        mint: server_token.to_account_info(),
        account: user_token_account,
    };

    token::freeze_account(CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts,
        &[&[SEED_SERVER_PROFILE, server_token.key().as_ref(), &[_bump]]],
    ))?;

    Ok(())
}

#[derive(Accounts)]
pub struct AJoinServer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token::authority = user,
        token::mint = server_token,
        constraint = user_token_account.amount == 0,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    ///NOTE: To join the server user have to init first their `USER_SERVER_PROFILE_ACCOUNT` first:
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
