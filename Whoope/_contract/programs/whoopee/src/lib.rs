#![allow(unused, dead_code)]
use anchor_lang::prelude::*;

pub mod prelude;
pub mod server;  
pub mod user;  
pub mod _utils;  
pub mod error;
pub mod constants;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod whoopee {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}