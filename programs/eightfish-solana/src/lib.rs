#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let eightfish_storage = ctx.accounts.eightfish_storage.deref_mut();
        let bump = *ctx.bumps.get("eightfish").ok_or(ErrorCode::CannotGetBump)?;

        *eightfish_storage = EightfishStorage {
            bump,
            ..EightfishStorage::default()
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
        _ctx: Context<ActInstruction>,
        _model: ModelName,
        _action: ActionName,
        _payload: Payload,
    ) -> Result<()> {
        // Block time.
        let _block_time = Clock::get()?.unix_timestamp;

        // Random value.
        // let (nonce, noncevec) = Self::get_and_increment_nonce();
        // let (nonc)
        // let (random_value, _) = T::MyRandomness::random(&noncevec);
        // let randomvec = random_value.as_bytes().to_vec();

        // In this call function, we do nothing now, excepting emitting the event back
        // This trick is to record the original requests from users to the blocks,
        // but not record it to the on-chain state storage.
        // Self::deposit_event(Event::Action(
        //     model, action, payload, block_time, randomvec, nonce,
        // ));

        Ok(())
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
