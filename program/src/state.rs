use {
    bonfida_utils::BorshSize,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{pubkey, pubkey::Pubkey},
};

pub mod mint_record;
pub mod registry;
pub mod schedule;
pub mod subdomain_record;

pub const ROOT_DOMAIN_ACCOUNT: Pubkey = sns_registrar::constants::ROOT_DOMAIN_ACCOUNT;

// 5% fee
pub const FEE_PCT: u64 = 5;
// Fee account
pub const FEE_ACC_OWNER: Pubkey = pubkey!("5D2zKog251d6KPCyFyLMt3KroWwXXPWSgTPyhV22K2gR");

#[derive(BorshSerialize, BorshDeserialize, BorshSize, PartialEq, Debug, Eq)]
#[allow(missing_docs)]
pub enum Tag {
    Uninitialized,
    Registrar,
    ClosedRegistrar,
    SubRecord,
    ClosedSubRecord,
    MintRecord,
    RevokedSubRecord,
}

impl Default for Tag {
    fn default() -> Self {
        Self::Uninitialized
    }
}
