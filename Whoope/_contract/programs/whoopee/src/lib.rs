#![allow(unused, dead_code)]
use anchor_lang::prelude::*;

pub mod _utils;
pub mod constants;
pub mod error;
pub mod main_account;
pub mod prelude;
pub mod server;
pub mod user;

pub use constants::*;
pub use main_account::*;
pub use server::*;
pub use user::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod whoopee {
    use super::*;

    //NOTE: Adming side
    pub fn init_main_account(context: Context<AInitMainAccount>) -> Result<()> {
        main_account::init_main_account(context)?;
        Ok(())
    }
    // pub fn upate_main_account(context: Context<AUpdateMainAccount>) -> Result<()>{
    //     Ok(())
    // }

    //NOTE: Uesr side (for server admin)
    pub fn create_server(
        context: Context<ACreateServer>,
        name: [u8; MAX_NAME_SIZE],
        symbol: [u8; MAX_NFT_SYMBOL_SIZE],
        uri: [u8; MAX_NFT_URI_SIZE],
        summary: [u8; MAX_SERVER_SUMMARY_SIZE],
    ) -> Result<()> {
        server::create_server(context, name, symbol, uri, summary)?;
        Ok(())
    }

    pub fn update_server_details(
        context: Context<AUpdateServerDetails>,
        name: [u8; MAX_NAME_SIZE],
        symbol: [u8; MAX_NFT_SYMBOL_SIZE],
        uri: [u8; MAX_NFT_URI_SIZE],
        summary: [u8; MAX_SERVER_SUMMARY_SIZE],
    ) -> Result<()> {
        server::update_server_details(context, name, symbol, uri, summary)?;
        Ok(())
    }

    pub fn create_server_scection(
        context: Context<ACreateServerSection>,
        section_name: [u8; MAX_SERVER_SECTION_NAME_SIZE],
        section_details: [u8; MAX_SERVER_SECTION_DETAILS_SIZE],
    ) -> Result<()> {
        server::create_server_section(context, section_name, section_details)?;
        Ok(())
    }

    pub fn init_user_server_profile(context: Context<AInitUserServerProfile>) -> Result<()> {
        server::init_user_server_profile(context)?;
        Ok(())
    }

    pub fn join_server(context: Context<AJoinServer>) -> Result<()> {
        server::join_server(context)?;
        Ok(())
    }

    pub fn leave_server(context: Context<ALeaveServer>) -> Result<()> {
        server::leave_server(context)?;
        Ok(())
    }

    pub fn send_chat(context: Context<ASendChat>, chat: Vec<u8>) -> Result<()> {
        server::send_chat(context, chat)?;
        Ok(())
    }

    //NOTE: Uesr side (for user profile)
    pub fn create_profile(
        context: Context<ACreateProfile>,
        name: [u8; MAX_NAME_SIZE],
        summary: [u8; MAX_SERVER_SUMMARY_SIZE],
    ) -> Result<()> {
        user::create_profile(context, name, summary)?;
        Ok(())
    }

    pub fn update_profile(
        context: Context<AUpdateProfile>,
        name: [u8; MAX_NAME_SIZE],
        summary: [u8; MAX_SERVER_SUMMARY_SIZE],
    ) -> Result<()> {
        user::update_profile(context, name, summary)?;
        Ok(())
    }
}
