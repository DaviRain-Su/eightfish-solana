use crate::state::EightfishStorage;
use anchor_lang::prelude::*;

/// Initialize the EightfishStorage account
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + EightfishStorage::SIZE,
        seeds = [b"eightfish".as_ref()],
        bump
    )]
    pub eightfish_storage: Account<'info, EightfishStorage>,
    #[account(mut)]
    authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

///
#[derive(Accounts)]
pub struct ActInstruction<'info> {
    #[account(
        mut,
        seeds = [b"eightfish".as_ref()],
        bump
    )]
    pub eightfish_storage: Account<'info, EightfishStorage>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdataIndexInstruction<'info> {
    #[account(
        mut,
        seeds = [b"eightfish".as_ref()],
        bump
    )]
    pub eightfish_storage: Account<'info, EightfishStorage>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct WasmUpgradeInstruction<'info> {
    #[account(
        mut,
        seeds = [b"eightfish".as_ref()],
        bump
    )]
    pub eightfish_storage: Account<'info, EightfishStorage>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DisableWasmUpgradeFlagInstruction<'info> {
    #[account(
        mut,
        seeds = [b"eightfish".as_ref()],
        bump
    )]
    pub eightfish_storage: Account<'info, EightfishStorage>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
