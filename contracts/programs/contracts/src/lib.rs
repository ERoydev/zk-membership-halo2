use anchor_lang::prelude::*;

declare_id!("4RmcfUfBuX78VABySpG7p5nzCAbqWLLJdbJ6DiYPZyFS");

pub mod instructions;
pub mod state;
pub mod dao_mock;
pub mod dao_error;
pub mod constants;

pub use dao_mock::*;
pub use instructions::*;
pub use state::*;


// anchor build --arch sbf

#[program]
pub mod contracts {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn dao_initialize(ctx: Context<DaoInitialize>, name: String) -> ProgramResult {
        _dao_initialize(ctx, name)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
