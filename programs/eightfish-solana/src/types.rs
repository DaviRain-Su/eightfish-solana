use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct EightFishId(String);

impl EightFishId {
    pub const SIZE: usize = 1024;

    pub fn into_bytes(self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes.copy_from_slice(&self.0.as_bytes());
        bytes
    }
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

impl From<&str> for ActionName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct Payload(Vec<u8>);

impl Payload {
    pub const SIZE: usize = 1024;

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    pub fn into_bytes(self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes.copy_from_slice(&self.0);
        bytes
    }
}

impl From<Vec<u8>> for Payload {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
