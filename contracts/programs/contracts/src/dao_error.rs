use anchor_lang::prelude::*;


#[error_code]
pub enum DaoError {
    #[msg("This user already voted with this proof.")]
    AlreadyVoted,
}