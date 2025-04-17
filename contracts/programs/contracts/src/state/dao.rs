use anchor_lang::prelude::*;

pub const MAX_DAO_NAME_LENGTH: usize = 20;

#[account]
pub struct DaoAccount {
    pub owner: Pubkey,
    pub name: String,
    pub merkle_root: [u8; 32],
}

