use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct EightFishId(String);

impl EightFishId {
    pub const SIZE: usize = 1024;
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct Hash(String);

impl Hash {
    pub const SIZE: usize = 1024;
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct ModelName(String);

impl ModelName {
    pub const SIZE: usize = 1024;
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct ActionName(String);

impl ActionName {
    pub const SIZE: usize = 1024;
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct Payload(Vec<u8>);

impl Payload {
    pub const SIZE: usize = 1024;
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct RandomOutput(String);

impl RandomOutput {
    pub const SIZE: usize = 1024;
}
