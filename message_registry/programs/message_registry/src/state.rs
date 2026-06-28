use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MessageAccount {
    pub authority: Pubkey,
    #[max_len(200)]
    pub text: String,
}
