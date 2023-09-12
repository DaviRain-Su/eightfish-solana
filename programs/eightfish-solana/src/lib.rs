#![allow(clippy::result_large_err)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use anchor_lang::prelude::*;
use spl_account_compression::{cpi as spl_ac_cpi, program::SplAccountCompression, Node, Noop};

declare_id!("33ERWC5kkcD3as36pQcfckTEBF4di9MMaveqYyxiLk1R");

pub mod errors;
pub mod state;
pub mod structions;
pub mod types;

use errors::ErrorCode;
use state::*;
use structions::*;
use types::*;

use types::Id as EightFishId;

#[program]
pub mod eightfish_solana {
    use std::ops::DerefMut;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        // const MAX_DEPTH: u32 = 30; // 1 billion possible entries
        // const MAX_BUFFER_SIZE: u32 = 2048; // tbd

        let accounts = spl_ac_cpi::accounts::Initialize {
            merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
            authority: ctx.accounts.tree_controller.to_account_info(),
            noop: ctx.accounts.noop_program.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            CONTROLLER_SEED,
            &[*ctx.bumps.get("tree_controller").unwrap()],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.compression_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        spl_ac_cpi::init_empty_merkle_tree(cpi_ctx, max_depth, max_buffer_size)?;

        ctx.accounts.tree_controller.set_inner(Controller {
            authority: ctx.accounts.authority.key(),
            merkle_tree: ctx.accounts.merkle_tree.key(),
        });

        Ok(())
    }

    /// This is a delegator dispatchable. Its main aim is to pass the validation of the
    /// incoming transactions (which contain the original user reqeust), and forwards the
    /// request to the off-chain listener for further process. By this design, we can think of
    /// the EightFish framework working as a batch processing system by intervals.
    /// Meanwhile, it provides three onchain parameters: time, nonce and a randomvec, these
    /// parameters are very important for a deterministic computation in the decentralized
    /// system.
    pub fn act(
        _ctx: Context<ActInstruction>,
        _model: ModelName,
        _action: ActionName,
        _payload: Payload,
    ) -> Result<()> {
        todo!()
    }

    /// This dispatchable is used to record the id-hash pair coresponding to the off-chain sql
    /// db table rows
    pub fn update_index(
        _ctx: Context<ActInstruction>,
        _model: ModelName,
        _reqid: Payload,
        _id: EightFishId,
        _hash: Hash,
    ) -> Result<()> {
        todo!()
    }

    /// Upload a new off-chain wasm runtime file to the on-chain storage, and once updated, set
    /// the new file flag.
    pub fn wasm_upgrade(_ctx: Context<WasmUpgradeInstruction>, _wasm_file: Vec<u8>) -> Result<()> {
        todo!()
    }

    /// Once the offchain wasm worker retrieve the new wasm file, disable the wasm file flag.
    /// This is not a beautiful but easy and workable solution right now.
    pub fn disable_wasm_upgrade_flag(
        _ctx: Context<DisableWasmUpgradeFlagInstruction>,
    ) -> Result<()> {
        todo!()
    }
}
