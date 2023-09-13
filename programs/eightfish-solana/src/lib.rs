#![allow(clippy::result_large_err)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use anchor_lang::prelude::*;
use spl_account_compression::{cpi as spl_ac_cpi, program::SplAccountCompression, Node, Noop};

declare_id!("33ERWC5kkcD3as36pQcfckTEBF4di9MMaveqYyxiLk1R");

pub mod errors;
pub mod events;
pub mod state;
pub mod structions;
pub mod types;

use errors::*;
use events::*;
use state::*;
use structions::*;
use types::*;

use types::EightFishId;

#[program]
pub mod eightfish_solana {
    use std::ops::DerefMut;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
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

        let eight_fish = ctx.accounts.eight_fish.deref_mut();

        *eight_fish = EightfishStorage {
            nonce: 0,
            wasm_file: vec![],
            wasm_file_new_flag: false,
            authority: ctx.accounts.authority.key(),
        };

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

    /// This dispatchable is used to record the id-hash pair coresponding to the off-chain sql
    /// db table rows
    pub fn update_index(
        ctx: Context<UpdataIndexInstruction>,
        model: ModelName,
        reqid: Payload,
        id: EightFishId,
        hash: Hash,
    ) -> Result<()> {
        let block_time = Clock::get()?.unix_timestamp;

        let leaf = EightFishCompressionStorage {
            model_name: model.clone(),
            id: id.clone(),
            hash,
        };

        let node = leaf.to_node();

        let accounts = spl_ac_cpi::accounts::Modify {
            merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
            authority: ctx.accounts.tree_controller.to_account_info(),
            noop: ctx.accounts.noop_program.to_account_info(),
        };

        let bump = *ctx.bumps.get("tree_controller").unwrap();
        let signer_seeds: &[&[&[u8]]] = &[&[CONTROLLER_SEED, &[bump]]];

        let cpi_ctx = CpiContext::new(ctx.accounts.compression_program.to_account_info(), accounts)
            .with_signer(signer_seeds);

        spl_ac_cpi::append(cpi_ctx, node)?;

        // We need to pass back the `reqid` and instance `id` info for further uses.
        let mut payload: Vec<u8> = Vec::new();
        payload.extend_from_slice(&reqid.into_bytes());
        payload.push(b':');
        payload.extend_from_slice(&id.into_bytes());

        require!(
            payload.len() <= Payload::SIZE,
            EightFishError::PayloadTooLarge
        );

        emit!(IndexUpdated {
            model_name: model,
            action_name: "update_index".into(),
            payload: payload.into(),
            block_time,
        });

        Ok(())
    }

    /// Upload a new off-chain wasm runtime file to the on-chain storage, and once updated, set
    /// the new file flag.
    pub fn wasm_upgrade(ctx: Context<WasmUpgradeInstruction>, wasm_file: Vec<u8>) -> Result<()> {
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

    /// Once the offchain wasm worker retrieve the new wasm file, disable the wasm file flag.
    /// This is not a beautiful but easy and workable solution right now.
    pub fn disable_wasm_upgrade_flag(
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
}

fn generate_random(mut seed: u64) -> u64 {
    seed ^= seed >> 12;
    seed ^= seed << 25;
    seed ^= seed >> 27;
    seed *= 0x2545F4914F6CDD1D;
    seed
}
