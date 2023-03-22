#![allow(unused, dead_code)]
use anchor_lang::prelude::*;

pub mod _utils;
pub mod constants;
pub mod error;
pub mod main_account;
pub mod prelude;
pub mod server;
pub mod user;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod whoopee {
    use super::*;
}
