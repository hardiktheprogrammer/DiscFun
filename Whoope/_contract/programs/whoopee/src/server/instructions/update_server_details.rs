use super::super::server_state::ServerState;
use crate::{
    _utils::parse_buffer_to_string, main_account::main_account_state::MainAccount, prelude::*,
};
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use mpl_token_metadata::{
    instruction::update_metadata_accounts_v2,
    state::{Creator, DataV2, Metadata},
};

pub fn update_server_details(
    context: Context<AUpdateServerDetails>,
    name: [u8; MAX_NAME_SIZE],
    symbol: [u8; MAX_NFT_SYMBOL_SIZE],
    uri: [u8; MAX_NFT_URI_SIZE],
    summary: [u8; MAX_SERVER_SUMMARY_SIZE],
) -> Result<()> {
    let server_account = &mut context.accounts.server_account;
    let admin = context.accounts.admin.to_account_info();
    let mpl_program = context.accounts.mpl_program.to_account_info();
    let metadata_account = context.accounts.metadata_account.to_account_info();
    // let master_edition_account = context.accounts.master_edition_account.to_account_info();
    let main_account = &mut context.accounts.main_account;
    let system_program = context.accounts.system_program.to_account_info();
    let token_program = context.accounts.token_program.to_account_info();

    server_account.name = name;
    server_account.summary = summary;

    let (_, _bump) = Pubkey::find_program_address(&[SEED_MAIN], &context.program_id);

    //NOTE: setting current time on creation
    server_account.created_on = Clock::get()?.unix_timestamp;

    let mut creators = vec![
        Creator {
            address: main_account.key(),
            share: 30,
            verified: false,
        },
        Creator {
            address: admin.key(),
            share: 30,
            verified: true,
        },
    ];

    let data = DataV2 {
        name: parse_buffer_to_string(&name),
        symbol: parse_buffer_to_string(&symbol),
        uri: parse_buffer_to_string(&uri),
        creators: Some(creators),
        uses: None,
        collection: None,
        seller_fee_basis_points: 10,
    };

    //NOTE: CREATING SERVER_TOKEN as NFT:
    invoke_signed(
        &update_metadata_accounts_v2(
            mpl_program.key(),
            metadata_account.key(),
            admin.key(),
            Some(admin.key()),
            Some(data),
            None,
            Some(true),
        ),
        &[
            mpl_program.to_account_info(),
            metadata_account.to_account_info(),
            admin.to_account_info(),
            system_program.to_account_info(),
        ],
        &[&[SEED_MAIN, &[_bump]]],
    )
    .unwrap();

    Ok(())
}

#[derive(Accounts)]
pub struct AUpdateServerDetails<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    ///CHECK:
    #[account()]
    pub server_token: AccountInfo<'info>,

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
        seeds = [SEED_MAIN],
        bump,
    )]
    pub main_account: Account<'info, MainAccount>,

    ///CHECK:
    #[account(
        address = mpl_token_metadata::id(),
    )]
    pub mpl_program: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut,
        seeds = [
            b"metadata",
            mpl_token_metadata::id().as_ref(),
            server_token.key().as_ref(),
        ],
        bump, 
    )]
    pub metadata_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
