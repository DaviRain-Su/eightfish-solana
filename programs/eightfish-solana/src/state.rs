use crate::types::{Id as EightFishId, *};
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct EightfishStorage {
    /// The nonce storage for helping generate on-chain deterministic randomness
    pub nonce: u64,
    /// On-chain blob for the off-chain executed wasm runtime file
    pub wasm_file: Vec<u8>,
    /// Is the wasm file fresh updated
    pub wasm_file_new_flag: bool,
    pub id_hash_pair_map: IdHashPair,
    ///
    pub authority: Pubkey,
    pub bump: u8,
}

impl EightfishStorage {
    pub const SIZE: usize = 8 + 1024 + 1 + EightFishId::SIZE + Hash::SIZE + 32 + 1;
}

/// The Id-Hash pair map storage coresponding to the off-chain sql db table rows
#[account]
#[derive(Default, Debug)]
pub struct IdHashPair {
    pub id_type: EightFishId,
    pub hash_type: Hash,
}
