use crate::constant::EIGHTFISH_SEED;
use crate::events::WasmUpgrade;
use crate::state::EightfishStorage;
use anchor_lang::prelude::*;
use std::ops::DerefMut;

#[derive(Accounts)]
pub struct WasmUpgradeInstruction<'info> {
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

pub fn wasm_upgrade_instruction(
    ctx: Context<WasmUpgradeInstruction>,
    wasm_file: Vec<u8>,
) -> Result<()> {
    let block_time = Clock::get()?.unix_timestamp;
    let eight_fish = ctx.accounts.eight_fish.deref_mut();
    eight_fish.wasm_file = wasm_file;
    eight_fish.wasm_file_new_flag = true;

    emit!(WasmUpgrade {
        block_time,
        wasm_file_new_flag: eight_fish.wasm_file_new_flag
    });

    Ok(())
}
