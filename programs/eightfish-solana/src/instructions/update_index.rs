use crate::constant::CONTROLLER_SEED;
use crate::state::Controller;
use anchor_lang::prelude::*;

// use crate::errors::EightFishError;
// use crate::events::IndexUpdated;
// use crate::state::EightFishCompressionStorage;
// use crate::types::*;
// use spl_account_compression as spl_ac_cpi;
use spl_account_compression::{program::SplAccountCompression, Noop};

#[derive(Accounts)]
pub struct UpdataIndexInstruction<'info> {
    pub authority: Signer<'info>,

    /// CHECK: key is checked
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,

    #[account(
        seeds = [CONTROLLER_SEED],
        bump,
        has_one = merkle_tree,
    )]
    pub tree_controller: Account<'info, Controller>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub compression_program: Program<'info, SplAccountCompression>,
    pub noop_program: Program<'info, Noop>,
}
