use borsh::{BorshDeserialize, BorshSerialize};

use crate::{ValidatorStakeInfo, STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct DepositSolQuoteArgs {
    pub depositor: [u8; 32],
    pub current_epoch: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct DepositSolQuote {
    pub in_amount: u64,
    pub out_amount: u64,
    /// In terms of newly minted LSTs
    pub referral_fee: u64,
    /// In terms of newly minted LSTs
    pub manager_fee: u64,
}

impl DepositSolQuote {
    pub fn total_fees(&self) -> u64 {
        self.referral_fee + self.manager_fee
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct DepositStakeQuoteArgs {
    pub validator_stake_info: ValidatorStakeInfo,
    pub validator: [u8; 32],
    pub current_epoch: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct DepositStakeQuote {
    /// Staked and unstaked lamports, before subtracting fees
    pub stake_account_lamports_in: StakeAccountLamports,

    /// Output tokens, after subtracting fees
    pub tokens_out: u64,

    /// In terms of output tokens
    pub manager_fee: u64,

    /// In terms of output tokens
    pub referral_fee: u64,
}

impl DepositStakeQuote {
    pub fn total_fees(&self) -> u64 {
        self.referral_fee + self.manager_fee
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct WithdrawSolQuoteArgs {
    pub reserve_stake_lamports: u64,
    pub current_epoch: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct WithdrawSolQuote {
    /// Input tokens, before subtracting fees
    pub in_amount: u64,

    /// Output lamports, after subtracting fees
    pub out_amount: u64,

    /// In terms of pool tokens
    pub manager_fee: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct WithdrawStakeQuoteArgs {
    pub current_epoch: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct WithdrawStakeQuote {
    pub tokens_in: u64,
    pub lamports_staked: u64,
    /// fee is levied in pool tokens and transferred to the
    /// pool's manager fee destination
    pub fee_amount: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct StakeAccountLamports {
    pub staked: u64,
    pub unstaked: u64,
}

impl StakeAccountLamports {
    pub fn total(&self) -> u64 {
        self.staked + self.unstaked
    }
}

#[inline]
pub fn reserve_has_sufficient_lamports(reserve_stake_lamports: u64, lamports_out: u64) -> bool {
    reserve_stake_lamports
        .checked_sub(lamports_out)
        .map(|remaining| remaining > STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS)
        .unwrap_or(false)
}
