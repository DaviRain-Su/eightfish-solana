#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use spl_account_compression::cpi as spl_ac_cpi;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

declare_id!("33ERWC5kkcD3as36pQcfckTEBF4di9MMaveqYyxiLk1R");

pub mod constant;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod types;

use constant::*;
use errors::*;
use events::*;

use instructions::*;
use state::*;
use types::EightFishId;
use types::*;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "eightfish on solana core program",
    project_url: "https://github.com/eightfish-org",
    contacts: "email:davirian.yin@gmail.com",
    policy: "Please report (suspected) security vulnerabilities to email above.
You will receive a response from us within 48 hours.",
    source_code: "https://github.com/DaviRain-Su/eightfish-solana",
    source_revision: env!("GIT_HASH"),
    acknowledgements: "Everyone in the Solana community"
}

#[program]
pub mod eightfish_solana {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        max_depth: u32,
        max_buffer_size: u32,
    ) -> Result<()> {
        initialize_instruction(ctx, max_depth, max_buffer_size)
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
        act_instruction(ctx, model, action, payload)
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

        let payload = Payload::generate_payload(reqid, id);

        require!(
            payload.len() <= Payload::SIZE,
            EightFishError::PayloadTooLarge
        );

        emit!(IndexUpdated {
            model_name: model,
            action_name: "update_index".into(),
            payload,
            block_time,
        });

        Ok(())
    }

    /// Upload a new off-chain wasm runtime file to the on-chain storage, and once updated, set
    /// the new file flag.
    pub fn wasm_upgrade(ctx: Context<WasmUpgradeInstruction>, wasm_file: Vec<u8>) -> Result<()> {
        wasm_upgrade_instruction(ctx, wasm_file)
    }

    /// Once the offchain wasm worker retrieve the new wasm file, disable the wasm file flag.
    /// This is not a beautiful but easy and workable solution right now.
    pub fn disable_wasm_upgrade_flag(
        ctx: Context<DisableWasmUpgradeFlagInstruction>,
    ) -> Result<()> {
        disable_wasm_upgrade_flag_instruction(ctx)
    }
}

fn generate_random(mut seed: u64) -> u64 {
    seed ^= seed >> 12;
    seed ^= seed << 25;
    seed ^= seed >> 27;
    seed *= 0x2545F4914F6CDD1D;
    seed
}
