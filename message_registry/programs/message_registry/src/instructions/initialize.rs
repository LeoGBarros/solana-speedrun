use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, state::MessageAccount};

#[derive(Accounts)]
#[instruction(text: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + MessageAccount::INIT_SPACE,
        seeds = [MESSAGE_SEED, authority.key().as_ref()],
        bump
    )]
    pub message_account: Account<'info, MessageAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Initialize>, text: String) -> Result<()> {
    require!(text.len() <= MAX_LEN, ErrorCode::MessageTooLong);

    let message_account = &mut ctx.accounts.message_account;
    message_account.authority = ctx.accounts.authority.key();
    message_account.text = text;

    msg!("Message initialized: {}", message_account.text);
    Ok(())
}
