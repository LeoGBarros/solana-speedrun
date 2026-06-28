pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DBfznUhc47xnK5B1wC6zWcqtZzWqu2MTNZckWBi6w5fX");

#[program]
pub mod message_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, text: String) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx, text)
    }

    pub fn update_message(ctx: Context<UpdateMessage>, new_text: String) -> Result<()> {
        crate::instructions::update_message::handle_update_message(ctx, new_text)
    }
}
