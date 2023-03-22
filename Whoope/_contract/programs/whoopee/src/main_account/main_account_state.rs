use anchor_lang::prelude::*;

///seed = MAX_SIZE
#[account]
pub struct MainAccount {
    total_users: u64,
    total_servers: u64,
}

impl MainAccount {
    pub const MAX_SIZE: usize = std::mem::size_of::<MainAccount>();
}
