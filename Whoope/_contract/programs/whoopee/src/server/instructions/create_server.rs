use crate::{prelude::*, main_account::main_account_state::MainAccount, _utils::parse_buffer_to_string};
use super::super::server_state::ServerState;
use anchor_lang::solana_program::program::invoke;
use mpl_token_metadata::{instruction::{create_metadata_accounts_v3, create_master_edition_v3}, state::{Metadata, Creator}};


pub fn create_server(
    context: Context<ACreateServer>,
    name: [u8; MAX_NAME_SIZE],
    symbol: [u8; MAX_NFT_SYMBOL_SIZE],
    uri: [u8; MAX_NFT_URI_SIZE],
    summary: [u8; MAX_SERVER_SUMMARY_SIZE],
)-> Result<()>{
    let server_account = &mut context.accounts.server_account;
    let admin = context.accounts.admin.to_account_info();
    let mpl_program = context.accounts.mpl_program.to_account_info();
    let metadata_account= context.accounts.metadata_account.to_account_info();
    let master_edition_account= context.accounts.master_edition_account.to_account_info();
    let server_token = context.accounts.server_token.to_account_info();
    let main_account= &mut context.accounts.main_account;
    let system_program= context.accounts.system_program.to_account_info();
    let token_program = context.accounts.token_program.to_account_info();

    server_account.name = name;
    server_account.summary = summary;

    //NOTE: setting current time on creation
    server_account.created_on = Clock::get()?.unix_timestamp;

    let mut creators = vec![
        Creator{
            address: main_account.key(),
            share: 30,
            verified: false,
        },
        Creator{
            address: admin.key(),
            share: 30,
            verified: true,
        },
    ];

    //NOTE: CREATING SERVER_TOKEN as NFT:
    invoke( 
        &create_metadata_accounts_v3(
            mpl_program.key(), 
            metadata_account.key(), 
            server_token.key(), 
            admin.key(), 
            admin.key(), 
            // server_account.key(), 
            admin.key(), 
            parse_buffer_to_string(&name), 
            parse_buffer_to_string(&symbol), 
            parse_buffer_to_string(&uri), 
            Some(creators),
            10, //TODO: IDK about its (need to on it reserach)
            true, 
            true, 
            None, 
            None, 
            None
        ),
        &[
            mpl_program.to_account_info(),
            metadata_account.to_account_info(),
            server_token.to_account_info(),
            // server_account.to_account_info(),
            admin.to_account_info(),
            system_program.to_account_info(),
        ]
    ).unwrap();

    invoke(
        &create_master_edition_v3(
            mpl_program.key(), 
            master_edition_account.key(), 
            server_token.key(), 
            admin.key(), 
            admin.key(), 
            metadata_account.key(), 
            admin.key(), 
            None, 
        ),
        &[
            mpl_program.to_account_info(),
            master_edition_account.to_account_info(),
            server_token.to_account_info(),
            admin.to_account_info(),
            token_program.to_account_info(),
            system_program.to_account_info(),
        ]
    ).unwrap();

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

    ///CHECK:
    #[account(
        mut,
    )]
    pub master_edition_account: AccountInfo<'info>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info, Token>,
}
