use anchor_lang::{prelude::*, solana_program::entrypoint_deprecated::ProgramResult};

use crate::dao::*;
use crate::constants::DAO_ACCOUNT_SEED;

use super::merkle_mock::generate_merkle_tree_mock;

pub fn _dao_initialize(ctx: Context<DaoInitialize>, name: String) -> ProgramResult {
    let dao_account = &mut ctx.accounts.dao_account;
    let formatted_name = DaoInitialize::truncate_string(&name);
    let merkle_root: [u8; 32] = generate_merkle_tree_mock();

    dao_account.name = formatted_name;
    dao_account.owner = *ctx.accounts.signer.key;
    dao_account.merkle_root = merkle_root;


    Ok(())
}

#[derive(Accounts)]
pub struct DaoInitialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        space = 8 + 32 + 4 + MAX_DAO_NAME_LENGTH + 4 + 32,
        seeds = [DAO_ACCOUNT_SEED.as_bytes(), signer.key().as_ref()],
        bump
    )]
    pub dao_account: Account<'info, DaoAccount>,
    pub system_program: Program<'info, System>
}

impl DaoInitialize<'_> {
    fn truncate_string(s: &str) -> String {
        // If name longer than limit just slice it to fit
        let mut end = MAX_DAO_NAME_LENGTH;

        if s.len() <= MAX_DAO_NAME_LENGTH {
            return s.to_string();
        }
        
        while !s.is_char_boundary(end) {
            end -= 1;
        }

        s[..end].to_string()
    }
}
