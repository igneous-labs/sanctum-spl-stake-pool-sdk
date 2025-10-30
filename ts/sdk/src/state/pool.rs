use sanctum_spl_stake_pool_core::{
    AccountType, DepositSolQuote, DepositStakeQuote, Fee, FutureEpoch, StakeAccountLamports,
    WithdrawSolQuote, WithdrawStakeQuote,
};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::{err::arithmetic_overflow_err, Lockup, B58PK};

#[wasm_bindgen]
pub struct StakePoolHandle(pub(crate) sanctum_spl_stake_pool_core::StakePool);

#[wasm_bindgen(js_name = defaultStakePool)]
pub fn default_stake_pool() -> StakePoolHandle {
    StakePoolHandle(Default::default())
}

#[wasm_bindgen(js_name = getStakePool)]
pub fn get_stake_pool(this: &StakePoolHandle) -> StakePool {
    StakePool::from_core(&this.0)
}

/// @throws if `val` contains invalid pubkeys
#[wasm_bindgen(js_name = setStakePool)]
pub fn set_stake_pool(this: &mut StakePoolHandle, val: StakePool) -> Result<(), JsError> {
    this.0 = val.try_to_core()?;
    Ok(())
}

/// @throws if bytes do not make up a valid StakePool
#[wasm_bindgen(js_name = deserStakePool)]
pub fn deser_stake_pool(bytes: &[u8]) -> Result<StakePoolHandle, JsError> {
    let sp = sanctum_spl_stake_pool_core::StakePool::borsh_de(bytes)?;
    Ok(StakePoolHandle(sp))
}

/// @throws if serialization failed
#[wasm_bindgen(js_name = serStakePool)]
pub fn ser_stake_pool(StakePoolHandle(sp): &StakePoolHandle) -> Result<Box<[u8]>, JsError> {
    let mut vec = Vec::new();
    sp.borsh_ser(&mut vec)?;
    Ok(vec.into())
}

/// @throws on arithmetic overflow
#[wasm_bindgen(js_name = quoteDepositSol)]
pub fn quote_deposit_sol(
    this: &StakePoolHandle,
    lamports: u64,
) -> Result<DepositSolQuote, JsError> {
    this.0
        .quote_deposit_sol_unchecked(lamports)
        .ok_or_else(arithmetic_overflow_err)
}

/// @throws on arithmetic overflow
#[wasm_bindgen(js_name = quoteDepositStake)]
pub fn quote_deposit_stake(
    this: &StakePoolHandle,
    stake_account_lamports: StakeAccountLamports,
) -> Result<DepositStakeQuote, JsError> {
    this.0
        .quote_deposit_stake_unchecked(stake_account_lamports)
        .ok_or_else(arithmetic_overflow_err)
}

/// @throws on arithmetic overflow
#[wasm_bindgen(js_name = quoteWithdrawSol)]
pub fn quote_withdraw_sol(
    this: &StakePoolHandle,
    pool_tokens: u64,
) -> Result<WithdrawSolQuote, JsError> {
    this.0
        .quote_withdraw_sol_unchecked(pool_tokens)
        .ok_or_else(arithmetic_overflow_err)
}

/// @throws on arithmetic overflow
#[wasm_bindgen(js_name = quoteRevWithdrawSol)]
pub fn quote_rev_withdraw_sol(
    this: &StakePoolHandle,
    lamports: u64,
) -> Result<WithdrawSolQuote, JsError> {
    this.0
        .quote_rev_withdraw_sol_unchecked(lamports)
        .ok_or_else(arithmetic_overflow_err)
}

/// @throws on arithmetic overflow
#[wasm_bindgen(js_name = quoteWithdrawStake)]
pub fn quote_withdraw_stake(
    this: &StakePoolHandle,
    pool_tokens: u64,
) -> Result<WithdrawStakeQuote, JsError> {
    this.0
        .quote_withdraw_stake_unchecked(pool_tokens)
        .ok_or_else(arithmetic_overflow_err)
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
#[serde(rename_all = "camelCase")]
pub struct StakePool {
    /// Account type, must be StakePool currently
    pub account_type: AccountType,

    /// Manager authority, allows for updating the staker, manager, and fee
    /// account
    pub manager: B58PK,

    /// Staker authority, allows for adding and removing validators, and
    /// managing stake distribution
    pub staker: B58PK,

    /// Stake deposit authority
    ///
    /// If a depositor pubkey is specified on initialization, then deposits must
    /// be signed by this authority. If no deposit authority is specified,
    /// then the stake pool will default to the result of:
    /// `Pubkey::find_program_address(
    ///     &[&stake_pool_address.as_ref(), b"deposit"],
    ///     program_id,
    /// )`
    pub stake_deposit_authority: B58PK,

    /// Stake withdrawal authority bump seed
    /// for `create_program_address(&[state::StakePool account, "withdrawal"])`
    pub stake_withdraw_bump_seed: u8,

    /// Validator stake list storage account
    pub validator_list: B58PK,

    /// Reserve stake account, holds deactivated stake
    pub reserve_stake: B58PK,

    /// Pool Mint
    pub pool_mint: B58PK,

    /// Manager fee account
    pub manager_fee_account: B58PK,

    /// Pool token program id
    pub token_program_id: B58PK,

    /// Total stake under management.
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub total_lamports: u64,

    /// Total supply of pool tokens (should always match the supply in the Pool
    /// Mint)
    pub pool_token_supply: u64,

    /// Last epoch the `total_lamports` field was updated
    pub last_update_epoch: u64,

    /// Lockup that all stakes in the pool must have
    pub lockup: Lockup,

    /// Fee taken as a proportion of rewards each epoch
    pub epoch_fee: Fee,

    /// Fee for next epoch
    pub next_epoch_fee: FutureEpoch<Fee>,

    /// Preferred deposit validator vote account pubkey
    #[tsify(optional)]
    pub preferred_deposit_validator_vote_address: Option<B58PK>,

    /// Preferred withdraw validator vote account pubkey
    #[tsify(optional)]
    pub preferred_withdraw_validator_vote_address: Option<B58PK>,

    /// Fee assessed on stake deposits
    pub stake_deposit_fee: Fee,

    /// Fee assessed on withdrawals
    pub stake_withdrawal_fee: Fee,

    /// Future stake withdrawal fee, to be set for the following epoch
    pub next_stake_withdrawal_fee: FutureEpoch<Fee>,

    /// Fees paid out to referrers on referred stake deposits.
    /// Expressed as a percentage (0 - 100) of deposit fees.
    /// i.e. `stake_deposit_fee`% of stake deposited is collected as deposit
    /// fees for every deposit and `stake_referral_fee`% of the collected
    /// stake deposit fees is paid out to the referrer
    pub stake_referral_fee: u8,

    /// Toggles whether the `DepositSol` instruction requires a signature from
    /// this `sol_deposit_authority`
    #[tsify(optional)]
    pub sol_deposit_authority: Option<B58PK>,

    /// Fee assessed on SOL deposits
    pub sol_deposit_fee: Fee,

    /// Fees paid out to referrers on referred SOL deposits.
    /// Expressed as a percentage (0 - 100) of SOL deposit fees.
    /// i.e. `sol_deposit_fee`% of SOL deposited is collected as deposit fees
    /// for every deposit and `sol_referral_fee`% of the collected SOL
    /// deposit fees is paid out to the referrer
    pub sol_referral_fee: u8,

    /// Toggles whether the `WithdrawSol` instruction requires a signature from
    /// the `deposit_authority`
    #[tsify(optional)]
    pub sol_withdraw_authority: Option<B58PK>,

    /// Fee assessed on SOL withdrawals
    pub sol_withdrawal_fee: Fee,

    /// Future SOL withdrawal fee, to be set for the following epoch
    pub next_sol_withdrawal_fee: FutureEpoch<Fee>,

    /// Last epoch's total pool tokens, used only for APR estimation
    pub last_epoch_pool_token_supply: u64,

    /// Last epoch's total lamports, used only for APR estimation
    pub last_epoch_total_lamports: u64,
}

impl StakePool {
    pub(crate) fn try_to_core(&self) -> Result<sanctum_spl_stake_pool_core::StakePool, JsError> {
        let Self {
            account_type,
            manager,
            staker,
            stake_deposit_authority,
            stake_withdraw_bump_seed,
            validator_list,
            reserve_stake,
            pool_mint,
            manager_fee_account,
            token_program_id,
            total_lamports,
            pool_token_supply,
            last_update_epoch,
            lockup,
            epoch_fee,
            next_epoch_fee,
            preferred_deposit_validator_vote_address,
            preferred_withdraw_validator_vote_address,
            stake_deposit_fee,
            stake_withdrawal_fee,
            next_stake_withdrawal_fee,
            stake_referral_fee,
            sol_deposit_authority,
            sol_deposit_fee,
            sol_referral_fee,
            sol_withdraw_authority,
            sol_withdrawal_fee,
            next_sol_withdrawal_fee,
            last_epoch_pool_token_supply,
            last_epoch_total_lamports,
        } = self;
        Ok(sanctum_spl_stake_pool_core::StakePool {
            account_type: *account_type,
            manager: manager.0,
            staker: staker.0,
            stake_deposit_authority: stake_deposit_authority.0,
            stake_withdraw_bump_seed: *stake_withdraw_bump_seed,
            validator_list: validator_list.0,
            reserve_stake: reserve_stake.0,
            pool_mint: pool_mint.0,
            manager_fee_account: manager_fee_account.0,
            token_program_id: token_program_id.0,
            total_lamports: *total_lamports,
            pool_token_supply: *pool_token_supply,
            last_update_epoch: *last_update_epoch,
            lockup: lockup.try_to_core()?,
            epoch_fee: *epoch_fee,
            next_epoch_fee: *next_epoch_fee,
            preferred_deposit_validator_vote_address: preferred_deposit_validator_vote_address
                .map(|b| b.0),
            preferred_withdraw_validator_vote_address: preferred_withdraw_validator_vote_address
                .map(|b| b.0),
            stake_deposit_fee: *stake_deposit_fee,
            stake_withdrawal_fee: *stake_withdrawal_fee,
            next_stake_withdrawal_fee: *next_stake_withdrawal_fee,
            stake_referral_fee: *stake_referral_fee,
            sol_deposit_authority: sol_deposit_authority.map(|b| b.0),
            sol_deposit_fee: *sol_deposit_fee,
            sol_referral_fee: *sol_referral_fee,
            sol_withdraw_authority: sol_withdraw_authority.map(|b| b.0),
            sol_withdrawal_fee: *sol_withdrawal_fee,
            next_sol_withdrawal_fee: *next_sol_withdrawal_fee,
            last_epoch_pool_token_supply: *last_epoch_pool_token_supply,
            last_epoch_total_lamports: *last_epoch_total_lamports,
        })
    }

    pub(crate) fn from_core(
        sanctum_spl_stake_pool_core::StakePool {
            account_type,
            manager,
            staker,
            stake_deposit_authority,
            stake_withdraw_bump_seed,
            validator_list,
            reserve_stake,
            pool_mint,
            manager_fee_account,
            token_program_id,
            total_lamports,
            pool_token_supply,
            last_update_epoch,
            lockup,
            epoch_fee,
            next_epoch_fee,
            preferred_deposit_validator_vote_address,
            preferred_withdraw_validator_vote_address,
            stake_deposit_fee,
            stake_withdrawal_fee,
            next_stake_withdrawal_fee,
            stake_referral_fee,
            sol_deposit_authority,
            sol_deposit_fee,
            sol_referral_fee,
            sol_withdraw_authority,
            sol_withdrawal_fee,
            next_sol_withdrawal_fee,
            last_epoch_pool_token_supply,
            last_epoch_total_lamports,
        }: &sanctum_spl_stake_pool_core::StakePool,
    ) -> Self {
        Self {
            account_type: *account_type,
            manager: B58PK::new(*manager),
            staker: B58PK::new(*staker),
            stake_deposit_authority: B58PK::new(*stake_deposit_authority),
            stake_withdraw_bump_seed: *stake_withdraw_bump_seed,
            validator_list: B58PK::new(*validator_list),
            reserve_stake: B58PK::new(*reserve_stake),
            pool_mint: B58PK::new(*pool_mint),
            manager_fee_account: B58PK::new(*manager_fee_account),
            token_program_id: B58PK::new(*token_program_id),
            total_lamports: *total_lamports,
            pool_token_supply: *pool_token_supply,
            last_update_epoch: *last_update_epoch,
            lockup: Lockup::from_core(lockup),
            epoch_fee: *epoch_fee,
            next_epoch_fee: *next_epoch_fee,
            preferred_deposit_validator_vote_address: preferred_deposit_validator_vote_address
                .map(B58PK::new),
            preferred_withdraw_validator_vote_address: preferred_withdraw_validator_vote_address
                .map(B58PK::new),
            stake_deposit_fee: *stake_deposit_fee,
            stake_withdrawal_fee: *stake_withdrawal_fee,
            next_stake_withdrawal_fee: *next_stake_withdrawal_fee,
            stake_referral_fee: *stake_referral_fee,
            sol_deposit_authority: sol_deposit_authority.map(B58PK::new),
            sol_deposit_fee: *sol_deposit_fee,
            sol_referral_fee: *sol_referral_fee,
            sol_withdraw_authority: sol_withdraw_authority.map(B58PK::new),
            sol_withdrawal_fee: *sol_withdrawal_fee,
            next_sol_withdrawal_fee: *next_sol_withdrawal_fee,
            last_epoch_pool_token_supply: *last_epoch_pool_token_supply,
            last_epoch_total_lamports: *last_epoch_total_lamports,
        }
    }
}
