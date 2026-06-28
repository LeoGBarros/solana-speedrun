use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, state::MessageAccount};

#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    #[account(
        mut,
        seeds = [MESSAGE_SEED, authority.key().as_ref()],
        bump,
        has_one = authority @ ErrorCode::Unauthorized
    )]
    pub message_account: Account<'info, MessageAccount>,
    pub authority: Signer<'info>,
}

pub fn handle_update_message(ctx: Context<UpdateMessage>, new_text: String) -> Result<()> {
    require!(new_text.len() <= MAX_LEN, ErrorCode::MessageTooLong);

    let message_account = &mut ctx.accounts.message_account;
    message_account.text = new_text;

    msg!("Message updated: {}", message_account.text);
    Ok(())
}
