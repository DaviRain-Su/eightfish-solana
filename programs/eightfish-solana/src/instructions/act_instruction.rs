use crate::constant::EIGHTFISH_SEED;
use crate::events::Action;
use crate::generate_random;
use crate::state::EightfishStorage;
use crate::types::{ActionName, ModelName, Payload};
use anchor_lang::prelude::*;
use std::ops::DerefMut;

///
#[derive(Accounts)]
pub struct ActInstruction<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [EIGHTFISH_SEED],
        bump,
    )]
    pub eight_fish: Account<'info, EightfishStorage>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn act_instruction(
    ctx: Context<ActInstruction>,
    model: ModelName,
    action: ActionName,
    payload: Payload,
) -> Result<()> {
    let block_time = Clock::get()?.unix_timestamp;

    // random value
    let eight_fish = ctx.accounts.eight_fish.deref_mut();
    eight_fish.nonce = eight_fish.nonce.wrapping_add(1);
    let random_value = generate_random(eight_fish.nonce);

    // In this call function, we do nothing now, excepting emitting the event back
    // This trick is to record the original requests from users to the blocks,
    // but not record it to the on-chain state storage.
    //
    emit!(Action {
        model_name: model,
        action_name: action,
        payload,
        block_time,
        random_output: random_value,
        nonce: eight_fish.nonce,
    });

    Ok(())
}
