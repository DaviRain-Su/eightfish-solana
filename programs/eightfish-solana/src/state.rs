use crate::types::*;
use crate::types::*;
use anchor_lang::{prelude::*, solana_program::keccak};
use spl_account_compression::Node;

#[account]
#[derive(Default)]
pub struct EightfishStorage {
    /// The nonce storage for helping generate on-chain deterministic randomness
    pub nonce: u64,
    /// On-chain blob for the off-chain executed wasm runtime file
    pub wasm_file: Vec<u8>,
    /// Is the wasm file fresh updated
    pub wasm_file_new_flag: bool,
    ///
    pub authority: Pubkey,
}

impl EightfishStorage {
    pub const SIZE: usize = 8 + 1024 + 1 + 32;
}

#[account]
pub struct EightFishCompressionStorage {
    pub model_name: ModelName,
    pub id_type: EightFishId,
    pub hash_type: Hash,
}

impl EightFishCompressionStorage {
    pub fn to_node(&self) -> Node {
        keccak::hashv(&[
            self.model_name.try_to_vec().unwrap_or_default().as_ref(),
            self.id_type.try_to_vec().unwrap_or_default().as_ref(),
            self.hash_type.try_to_vec().unwrap_or_default().as_ref(),
        ])
        .to_bytes()
    }
}

#[account]
pub struct Controller {
    pub authority: Pubkey,
    pub merkle_tree: Pubkey,
}
