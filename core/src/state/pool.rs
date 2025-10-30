use core::ops::RangeInclusive;

use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_u64_ratio::{Floor, Ratio};

use crate::{
    reserve_has_sufficient_lamports, AccountType, DepositSolQuote, DepositSolQuoteArgs,
    DepositStakeQuote, DepositStakeQuoteArgs, Fee, FutureEpoch, Lockup, ReferralFee,
    SplStakePoolError, StakeAccountLamports, StakeStatus, WithdrawSolQuote, WithdrawSolQuoteArgs,
    WithdrawStakeQuote, WithdrawStakeQuoteArgs,
};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakePool {
    /// Account type, must be StakePool currently
    pub account_type: AccountType,

    /// Manager authority, allows for updating the staker, manager, and fee
    /// account
    pub manager: [u8; 32],

    /// Staker authority, allows for adding and removing validators, and
    /// managing stake distribution
    pub staker: [u8; 32],

    /// Stake deposit authority
    ///
    /// If a depositor pubkey is specified on initialization, then deposits must
    /// be signed by this authority. If no deposit authority is specified,
    /// then the stake pool will default to the result of:
    /// `Pubkey::find_program_address(
    ///     &[&stake_pool_address.as_ref(), b"deposit"],
    ///     program_id,
    /// )`
    pub stake_deposit_authority: [u8; 32],

    /// Stake withdrawal authority bump seed
    /// for `create_program_address(&[state::StakePool account, "withdrawal"])`
    pub stake_withdraw_bump_seed: u8,

    /// Validator stake list storage account
    pub validator_list: [u8; 32],

    /// Reserve stake account, holds deactivated stake
    pub reserve_stake: [u8; 32],

    /// Pool Mint
    pub pool_mint: [u8; 32],

    /// Manager fee account
    pub manager_fee_account: [u8; 32],

    /// Pool token program id
    pub token_program_id: [u8; 32],

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
    pub preferred_deposit_validator_vote_address: Option<[u8; 32]>,

    /// Preferred withdraw validator vote account pubkey
    pub preferred_withdraw_validator_vote_address: Option<[u8; 32]>,

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
    pub sol_deposit_authority: Option<[u8; 32]>,

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
    pub sol_withdraw_authority: Option<[u8; 32]>,

    /// Fee assessed on SOL withdrawals
    pub sol_withdrawal_fee: Fee,

    /// Future SOL withdrawal fee, to be set for the following epoch
    pub next_sol_withdrawal_fee: FutureEpoch<Fee>,

    /// Last epoch's total pool tokens, used only for APR estimation
    pub last_epoch_pool_token_supply: u64,

    /// Last epoch's total lamports, used only for APR estimation
    pub last_epoch_total_lamports: u64,
}

impl Default for StakePool {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl StakePool {
    pub const DEFAULT: Self = Self {
        account_type: AccountType::StakePool,
        manager: [0u8; 32],
        staker: [0u8; 32],
        stake_deposit_authority: [0u8; 32],
        stake_withdraw_bump_seed: 0,
        validator_list: [0u8; 32],
        reserve_stake: [0u8; 32],
        pool_mint: [0u8; 32],
        manager_fee_account: [0u8; 32],
        token_program_id: [0u8; 32],
        total_lamports: 0,
        pool_token_supply: 0,
        last_update_epoch: 0,
        lockup: Lockup::DEFAULT,
        epoch_fee: Fee::ZERO,
        next_epoch_fee: FutureEpoch::None,
        preferred_deposit_validator_vote_address: None,
        preferred_withdraw_validator_vote_address: None,
        stake_deposit_fee: Fee::ZERO,
        stake_withdrawal_fee: Fee::ZERO,
        next_stake_withdrawal_fee: FutureEpoch::None,
        stake_referral_fee: 0,
        sol_deposit_authority: None,
        sol_deposit_fee: Fee::ZERO,
        sol_referral_fee: 0,
        sol_withdraw_authority: None,
        sol_withdrawal_fee: Fee::ZERO,
        next_sol_withdrawal_fee: FutureEpoch::None,
        last_epoch_pool_token_supply: 0,
        last_epoch_total_lamports: 0,
    };

    /// Must return `true` for the quote to be applicable
    #[inline]
    pub const fn is_updated_for_epoch(&self, current_epoch: u64) -> bool {
        self.last_update_epoch >= current_epoch
    }

    /// Must return `true` for the quote to be applicable
    #[inline]
    pub fn can_pk_deposit(&self, pubkey: &[u8; 32]) -> bool {
        self.sol_deposit_authority
            .is_none_or(|authority| &authority == pubkey)
    }

    /// Must return `true` for the quote to be applicable
    #[inline]
    pub fn can_deposit_stake_of(&self, vote: &[u8; 32]) -> bool {
        self.preferred_deposit_validator_vote_address
            .is_none_or(|preferred| vote == &preferred)
    }

    /// Performs the checks needed to be serviceable along with calculation logic
    #[inline]
    pub fn quote_deposit_sol(
        &self,
        lamports: u64,
        args: DepositSolQuoteArgs,
    ) -> Result<DepositSolQuote, SplStakePoolError> {
        if !self.can_pk_deposit(&args.depositor) {
            return Err(SplStakePoolError::InvalidSolDepositAuthority);
        }
        if !self.is_updated_for_epoch(args.current_epoch) {
            return Err(SplStakePoolError::StakeListAndPoolOutOfDate);
        }
        self.quote_deposit_sol_unchecked(lamports)
            .ok_or(SplStakePoolError::CalculationFailure)
    }

    /// Returns `None` on arithmetic overflow.
    ///
    /// NB: returned quote might not be applicable if pool has not been updated for the current epoch
    #[inline]
    pub fn quote_deposit_sol_unchecked(&self, lamports: u64) -> Option<DepositSolQuote> {
        let new_pool_tokens = self.lamports_to_pool_tokens(lamports)?;
        let after_sol_deposit_fee = self.sol_deposit_fee.to_fee_ceil()?.apply(new_pool_tokens)?;
        let out_amount = after_sol_deposit_fee.rem();
        let referral_fee_split = self
            .sol_referral_fee()?
            .0
            .apply(after_sol_deposit_fee.fee())?;
        let manager_fee = referral_fee_split.rem();
        let referral_fee = referral_fee_split.fee();

        Some(DepositSolQuote {
            in_amount: lamports,
            out_amount,
            referral_fee,
            manager_fee,
        })
    }

    /// Performs the checks needed to be serviceable along with calculation logic
    #[inline]
    pub fn quote_deposit_stake(
        &self,
        stake_account_lamports: StakeAccountLamports,
        DepositStakeQuoteArgs {
            validator_status,
            validator_vote,
            current_epoch,
        }: &DepositStakeQuoteArgs,
    ) -> Result<DepositStakeQuote, SplStakePoolError> {
        if !self.is_updated_for_epoch(*current_epoch) {
            return Err(SplStakePoolError::StakeListAndPoolOutOfDate);
        }

        if !self.can_deposit_stake_of(validator_vote) {
            return Err(SplStakePoolError::IncorrectDepositVoteAddress);
        }
        if *validator_status != StakeStatus::Active {
            return Err(SplStakePoolError::InvalidState);
        }

        self.quote_deposit_stake_unchecked(stake_account_lamports)
            .ok_or(SplStakePoolError::CalculationFailure)
    }

    // TODO: Might get refactored with `WithdrawStakeQuote`
    //
    /// Returns `None` on overflow
    ///
    /// NB: returned quote might not be applicable if:
    /// - pool has not been updated for the current epoch
    /// - selected validator is not active
    /// - selected validator has insufficient stake
    /// - selected validator is not in the validator list
    /// - selected validator is not the preferred validator
    #[inline]
    pub fn quote_deposit_stake_unchecked(
        &self,
        stake_account_lamports: StakeAccountLamports,
    ) -> Option<DepositStakeQuote> {
        let new_pool_tokens = self.lamports_to_pool_tokens(stake_account_lamports.total())?;
        let new_pool_tokens_from_stake =
            self.lamports_to_pool_tokens(stake_account_lamports.staked)?;
        let new_pool_tokens_from_sol = new_pool_tokens.checked_sub(new_pool_tokens_from_stake)?;

        let stake_deposit_fee = self
            .stake_deposit_fee
            .to_fee_ceil()?
            .apply(new_pool_tokens_from_stake)?
            .fee();
        let sol_deposit_fee = self
            .sol_deposit_fee
            .to_fee_ceil()?
            .apply(new_pool_tokens_from_sol)?
            .fee();
        let total_fee = stake_deposit_fee.checked_add(sol_deposit_fee)?;

        let tokens_out = new_pool_tokens.checked_sub(total_fee)?;

        let after_referral_fee = self.stake_referral_fee()?.0.apply(total_fee)?;

        Some(DepositStakeQuote {
            stake_account_lamports_in: stake_account_lamports,
            tokens_out,
            manager_fee: after_referral_fee.rem(),
            referral_fee: after_referral_fee.fee(),
        })
    }

    /// Performs the checks needed to be serviceable along with calculation logic
    #[inline]
    pub const fn quote_withdraw_sol(
        &self,
        pool_tokens: u64,
        args: WithdrawSolQuoteArgs,
    ) -> Result<WithdrawSolQuote, SplStakePoolError> {
        if !self.is_updated_for_epoch(args.current_epoch) {
            return Err(SplStakePoolError::StakeListAndPoolOutOfDate);
        }

        let quote = match self.quote_withdraw_sol_unchecked(pool_tokens) {
            None => return Err(SplStakePoolError::CalculationFailure),
            Some(x) => x,
        };

        if !reserve_has_sufficient_lamports(args.reserve_stake_lamports, quote.out_amount) {
            return Err(SplStakePoolError::SolWithdrawalTooLarge);
        }
        Ok(quote)
    }

    /// Performs the checks needed to be serviceable along with calculation logic
    #[inline]
    pub const fn quote_rev_withdraw_sol(
        &self,
        lamports: u64,
        args: WithdrawSolQuoteArgs,
    ) -> Result<WithdrawSolQuote, SplStakePoolError> {
        if !self.is_updated_for_epoch(args.current_epoch) {
            return Err(SplStakePoolError::StakeListAndPoolOutOfDate);
        }
        if !reserve_has_sufficient_lamports(args.reserve_stake_lamports, lamports) {
            return Err(SplStakePoolError::SolWithdrawalTooLarge);
        }

        match self.quote_rev_withdraw_sol_unchecked(lamports) {
            None => Err(SplStakePoolError::CalculationFailure),
            Some(x) => Ok(x),
        }
    }

    /// Returns `None` on arithmetic overflow.
    ///
    /// NB: returned quote might not be applicable if:
    /// - pool has not been updated for the current epoch
    /// - the reserve stake does not have enough SOL to service the withdrawal
    #[inline]
    pub const fn quote_withdraw_sol_unchecked(&self, pool_tokens: u64) -> Option<WithdrawSolQuote> {
        let fee = match self.sol_withdrawal_fee.to_fee_ceil() {
            None => return None,
            Some(x) => x,
        };
        let after_sol_withdrawal_fee = match fee.apply(pool_tokens) {
            None => return None,
            Some(x) => x,
        };
        let out_lamports = match self.pool_tokens_to_lamports(after_sol_withdrawal_fee.rem()) {
            None => return None,
            Some(x) => x,
        };
        Some(WithdrawSolQuote {
            in_amount: pool_tokens,
            out_amount: out_lamports,
            manager_fee: after_sol_withdrawal_fee.fee(),
        })
    }

    /// Reverse of [`Self::quote_withdraw_sol_unchecked`]: returns the smallest number
    /// of pool_tokens required for the withdrawal given the desired amount of SOL
    ///
    /// Returns `None` on arithmetic overflow.
    ///
    /// NB: returned quote might not be applicable if:
    /// - pool has not been updated for the current epoch
    /// - the reserve stake does not have enough SOL to service the withdrawal
    #[inline]
    pub const fn quote_rev_withdraw_sol_unchecked(
        &self,
        lamports: u64,
    ) -> Option<WithdrawSolQuote> {
        let fee = match self.sol_withdrawal_fee.to_fee_ceil() {
            None => return None,
            Some(x) => x,
        };
        let after_sol_withdrawal_fee = match self.rev_pool_tokens_to_lamports(lamports) {
            None => return None,
            Some(x) => *x.start(),
        };
        let pool_tokens = match fee.reverse_from_rem(after_sol_withdrawal_fee) {
            None => return None,
            Some(x) => *x.start(),
        };
        // unchecked-arith: valid fee, so must be rem <= pool_tokens
        let manager_fee = pool_tokens - after_sol_withdrawal_fee;
        Some(WithdrawSolQuote {
            in_amount: pool_tokens,
            out_amount: lamports,
            manager_fee,
        })
    }

    /// Performs the checks needed to be serviceable along with calculation logic
    #[inline]
    pub fn quote_withdraw_stake(
        &self,
        pool_tokens: u64,
        args: WithdrawStakeQuoteArgs,
    ) -> Result<WithdrawStakeQuote, SplStakePoolError> {
        if !self.is_updated_for_epoch(args.current_epoch) {
            return Err(SplStakePoolError::StakeListAndPoolOutOfDate);
        }

        self.quote_withdraw_stake_unchecked(pool_tokens)
            .ok_or(SplStakePoolError::CalculationFailure)
    }

    /// Returns `None` on arithmetic overflow.
    ///
    /// NB: returned quote might not be applicable if:
    /// - pool has not been updated for the current epoch
    /// - stake account is not active
    /// - there are insufficient lamports in the stake account to cover the minimum leftover needed
    /// - the stake_to_receive account is not a rent exempt uninitialized stake account
    #[inline]
    pub fn quote_withdraw_stake_unchecked(&self, pool_tokens: u64) -> Option<WithdrawStakeQuote> {
        let after_stake_withdrawal_fee = self
            .stake_withdrawal_fee
            .to_fee_ceil()?
            .apply(pool_tokens)?;

        Some(WithdrawStakeQuote {
            tokens_in: pool_tokens,
            lamports_staked: self.pool_tokens_to_lamports(after_stake_withdrawal_fee.rem())?,
            fee_amount: after_stake_withdrawal_fee.fee(),
        })
    }
}

impl StakePool {
    /// Apply this to a `lamport` amount to determine the number of
    /// pool tokens it is equivalent to, before any fees. This is what
    /// [`Self::lamports_to_pool_tokens`] does.
    #[inline]
    pub const fn supply_over_lamports(&self) -> Floor<Ratio<u64, u64>> {
        Floor(Ratio {
            n: self.pool_token_supply,
            d: self.total_lamports,
        })
    }

    /// Apply this to a `pool_tokens` amount to determine the number of
    /// lamports it is equivalent to, after manager fees. This is what
    /// [`Self::pool_tokens_to_lamports`] does.
    #[inline]
    pub const fn lamports_over_supply(&self) -> Floor<Ratio<u64, u64>> {
        Floor(Ratio {
            n: self.total_lamports,
            d: self.pool_token_supply,
        })
    }

    /// Returns the number of pool tokens equivalent to `lamports`
    /// before any fees.
    ///
    /// ## Returns
    /// `lamports * self.pool_token_supply / self.total_lamports`
    ///
    /// Returns `lamports` if `self.pool_token_supply` or `self.total_lamports` is 0,
    /// which is the same as onchain program behaviour.
    ///
    /// Returns `None` on overflow
    #[inline]
    pub const fn lamports_to_pool_tokens(&self, lamports: u64) -> Option<u64> {
        let ratio = self.supply_over_lamports();
        if ratio.0.is_zero() {
            return Some(lamports);
        }
        ratio.apply(lamports)
    }

    /// Returns the number of lamports equivalent to `pool tokens`
    /// after manager fees.
    ///
    /// ## Returns
    /// `pool_tokens * self.total_lamports / self.pool_token_supply`
    ///
    /// Returns `pool_tokens` if `self.pool_token_supply` or `self.total_lamports` is 0,
    /// which is the same as onchain program behaviour.
    ///
    /// Returns `None` on overflow
    #[inline]
    pub const fn pool_tokens_to_lamports(&self, pool_tokens: u64) -> Option<u64> {
        let ratio = self.lamports_over_supply();
        if ratio.0.is_zero() {
            return Some(pool_tokens);
        }
        ratio.apply(pool_tokens)
    }

    /// Given output `lamports`, return range of `pool_tokens`
    /// that may have been fed into [`Self::pool_tokens_to_lamports`]
    /// This may yield a different result from [`Self::lamports_to_pool_tokens`]
    #[inline]
    pub const fn rev_pool_tokens_to_lamports(&self, lamports: u64) -> Option<RangeInclusive<u64>> {
        let ratio = self.lamports_over_supply();
        if ratio.0.is_zero() {
            return Some(lamports..=lamports);
        }
        ratio.reverse_est(lamports)
    }

    /// Returns None if self.sol_referral_fee > 100
    #[inline]
    pub const fn sol_referral_fee(&self) -> Option<ReferralFee> {
        ReferralFee::new(self.sol_referral_fee)
    }

    /// Returns None if self.stake_referral_fee > 100
    #[inline]
    pub const fn stake_referral_fee(&self) -> Option<ReferralFee> {
        ReferralFee::new(self.stake_referral_fee)
    }
}

impl StakePool {
    inherent_borsh_serde!();
}
