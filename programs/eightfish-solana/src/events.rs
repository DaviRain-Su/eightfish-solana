use crate::types::*;
use anchor_lang::prelude::*;

#[event]
pub struct Action {
    pub model_name: ModelName,
    pub action_name: ActionName,
    pub payload: Payload,
    pub block_time: i64,
    pub random_output: u64,
    pub nonce: u64,
}

#[event]
pub struct IndexUpdated {
    pub model_name: ModelName,
    pub action_name: ActionName,
    pub payload: Payload,
    pub block_time: i64,
}

#[event]
pub struct Upgrade {
    pub wasm_file_new_flag: bool,
    pub block_time: i64,
}

#[event]
pub struct DisableUpgrade {
    pub wasm_file_new_flag: bool,
    pub block_time: i64,
}
