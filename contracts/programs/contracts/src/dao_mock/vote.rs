use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};

use crate::dao_error::DaoError;
use crate::constants::NULLIFIER_RECORD_SEED;

// zk_proof -> pp -> proves i am in the merkle tree
// nullifier_hash -> pp -> prevent reuse of the same proof
// merkle_root -> pp

pub fn _vote(ctx: Context<Vote>, zk_proof: Vec<u8>, merkle_root: [u8; 32], nullifier_hash: [u8; 32]) -> Result<()> {

    // Verify ZK proof
    // TODO: Verify code here ...
    
    // Check if this nullifier is already used
    let record = &mut ctx.accounts.nullifier_record;
    require!(!record.used, DaoError::AlreadyVoted);

    // If not mark it as used from now on
    record.nullifier_hash = nullifier_hash;
    record.used = true;

    Ok(())
}

#[derive(Accounts)]
#[instruction(nullifier_hash: [u8; 32])]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer=signer,
        space = 8 + 32 + 1,
        seeds = [NULLIFIER_RECORD_SEED.as_bytes(), nullifier_hash.as_ref()],
        bump,
    )]
    pub nullifier_record: Account<'info, NullifierRecord>, // Save this as a PDA Account
    pub system_program: Program<'info, System>
}

#[account]
pub struct NullifierRecord {
    pub nullifier_hash: [u8; 32],
    pub used: bool,
}