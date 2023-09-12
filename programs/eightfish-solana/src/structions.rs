use crate::state::Controller;
use crate::state::EightfishStorage;

use anchor_lang::prelude::*;
use spl_account_compression::{cpi as spl_ac_cpi, program::SplAccountCompression, Node, Noop};

pub const CONTROLLER_SEED: &[u8] = b"controller";

/// Initialize the EightfishStorage account
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: first call to initialize is permissionless
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,

    #[account(
            init,
            space = 8 + 32 + 32,
            payer = payer,
            seeds = [CONTROLLER_SEED],
            bump,
    )]
    pub tree_controller: Account<'info, Controller>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub authority: Signer<'info>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub noop_program: Program<'info, Noop>,
    pub system_program: Program<'info, System>,
}

///
#[derive(Accounts)]
pub struct ActInstruction<'info> {
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

#[derive(Accounts)]
pub struct WasmUpgradeInstruction<'info> {
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

#[derive(Accounts)]
pub struct DisableWasmUpgradeFlagInstruction<'info> {
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
