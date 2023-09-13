use crate::constant::EIGHTFISH_SEED;
use crate::events::DisableUpgradeWasm;
use crate::state::EightfishStorage;
use anchor_lang::prelude::*;
use std::ops::DerefMut;

#[derive(Accounts)]
pub struct DisableWasmUpgradeFlagInstruction<'info> {
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

pub fn disable_wasm_upgrade_flag_instruction(
    ctx: Context<DisableWasmUpgradeFlagInstruction>,
) -> Result<()> {
    let block_time = Clock::get()?.unix_timestamp;
    let eight_fish = ctx.accounts.eight_fish.deref_mut();
    eight_fish.wasm_file_new_flag = false;

    // In this call function, we do nothing now, excepting emitting the event back
    // This trick is to record the original requests from users to the blocks,
    // but not record it to the on-chain storage.
    emit!(DisableUpgradeWasm {
        block_time,
        wasm_file_new_flag: eight_fish.wasm_file_new_flag
    });

    Ok(())
}
