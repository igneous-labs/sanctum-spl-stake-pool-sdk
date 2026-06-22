use expect_test::expect;
use proptest::prelude::*;
use sanctum_spl_stake_pool_core::{Fee, QuoteRevDepositStakeArgs, StakeAccountLamports, StakePool};
use sanctum_u64_ratio::Ratio;

use crate::common::{
    proptest_utils::{ratio_gte_one, ratio_lte_one},
    quote::{NewPoolQuoteU64sBuilder, PoolQuoteU64Ds},
};

fn quote_rev_deposit_stake_round_trip(
    pool: PoolQuoteU64Ds,
    staked: u64,
    unstaked: u64,
    stake_deposit_fee: Fee,
    sol_deposit_fee: Fee,
) {
    let sp = StakePool {
        total_lamports: *pool.total_lamports(),
        pool_token_supply: *pool.pool_token_supply(),
        stake_deposit_fee,
        sol_deposit_fee,
        ..Default::default()
    };

    let Some(forward) = sp.quote_deposit_stake_unchecked(StakeAccountLamports { staked, unstaked })
    else {
        return;
    };

    let rev = sp
        .quote_rev_deposit_stake_unchecked(QuoteRevDepositStakeArgs {
            tokens_out: forward.tokens_out,
            unstaked_lamports: Some(unstaked),
        })
        .expect("reverse returned None but forward succeeded with valid fees");

    assert_eq!(rev.stake_account_lamports_in.unstaked, unstaked);
    assert_eq!(
        sp.quote_deposit_stake_unchecked(rev.stake_account_lamports_in),
        Some(rev)
    );
    assert!(
        rev.tokens_out >= forward.tokens_out,
        "{} < {}",
        rev.tokens_out,
        forward.tokens_out
    );
    // `sol_lo` conservative estimate can overshoot by 1 pool token.
    // For `ratio_lte_one` pools (supply > total), `rev_lamports_to_pool_tokens` can
    // additionally overshoot by floor(supply/total) pool tokens for skipped values.
    // Each extra pool token adds at most 1 to `staked_after_fee`, so total bound is:
    // `forward.tokens_out + floor(supply/total) + 1`
    let pool_token_overshoot = sp.pool_token_supply / sp.total_lamports.max(1);
    assert!(
        rev.tokens_out <= forward.tokens_out.saturating_add(pool_token_overshoot + 1),
        "rev.tokens_out={} > bound={} (forward.tokens_out={}, pool_token_overshoot={})",
        rev.tokens_out,
        forward.tokens_out.saturating_add(pool_token_overshoot + 1),
        forward.tokens_out,
        pool_token_overshoot,
    );

    // maps that 1 pool token gap bound to staked lamports after fees
    let fee_sub = stake_deposit_fee
        .denominator
        .saturating_sub(stake_deposit_fee.numerator);
    let fee_amplifier = if fee_sub == 0 {
        1u64
    } else {
        stake_deposit_fee.denominator.div_ceil(fee_sub)
    };
    let lamports_per_token = sp.total_lamports.div_ceil(sp.pool_token_supply.max(1));
    let staked_bound = forward
        .stake_account_lamports_in
        .staked
        .saturating_add(fee_amplifier.saturating_mul(lamports_per_token));
    assert!(
        rev.stake_account_lamports_in.staked <= staked_bound,
        "rev.staked={} > staked_bound={} (forward.staked={}",
        rev.stake_account_lamports_in.staked,
        staked_bound,
        forward.stake_account_lamports_in.staked,
    );
}

// verifies the floor arithmetic bound relied on by `quote_rev_deposit_stake_unchecked`:
// the token value of unstaked lamports in a combined deposit is at most unstaked_tokens_alone + 1
fn assert_unstaked_token_bucket_bound(
    pool: PoolQuoteU64Ds,
    staked: u64,
    unstaked: u64,
    sol_deposit_fee: Fee,
) {
    // mimics behaviour in quote_deposit_stake_unchecked
    // where StakeAccountLamports.total() overflows u64
    let Some(total) = staked.checked_add(unstaked) else {
        return;
    };

    let sp = StakePool {
        total_lamports: *pool.total_lamports(),
        pool_token_supply: *pool.pool_token_supply(),
        sol_deposit_fee,
        ..Default::default()
    };

    let Some(new_pool_tokens) = sp.lamports_to_pool_tokens(total) else {
        return;
    };
    let Some(new_pool_tokens_from_stake) = sp.lamports_to_pool_tokens(staked) else {
        return;
    };
    let Some(actual_unstaked_tokens) = new_pool_tokens.checked_sub(new_pool_tokens_from_stake)
    else {
        return;
    };
    let Some(unstaked_tokens_alone) = sp.lamports_to_pool_tokens(unstaked) else {
        return;
    };

    assert!(
        actual_unstaked_tokens == unstaked_tokens_alone
            || actual_unstaked_tokens == unstaked_tokens_alone + 1,
        "actual_unstaked_tokens={actual_unstaked_tokens}, unstaked_tokens_alone={unstaked_tokens_alone}"
    );

    let Some(fee) = sol_deposit_fee.to_fee_ceil() else {
        return;
    };
    let Some(actual_after_fee) = fee.apply(actual_unstaked_tokens).map(|x| x.rem()) else {
        return;
    };
    let Some(alone_after_fee) = fee.apply(unstaked_tokens_alone).map(|x| x.rem()) else {
        return;
    };
    assert!(actual_after_fee >= alone_after_fee);
    assert!(actual_after_fee <= alone_after_fee + 1);
}

proptest! {
    #[test]
    fn quote_rev_deposit_stake_round_trip_prop(
        (r_gte, r_lte) in (ratio_gte_one(), ratio_lte_one()),
        staked: u64,
        unstaked: u64,
        stake_fee_n: u64,
        stake_fee_d: u64,
        sol_fee_n: u64,
        sol_fee_d: u64,
    ) {
        let stake_fee = Fee { numerator: stake_fee_n, denominator: stake_fee_d };
        let sol_fee = Fee { numerator: sol_fee_n, denominator: sol_fee_d };
        for Ratio { n: total_lamports, d: pool_token_supply } in [r_gte, r_lte] {
            quote_rev_deposit_stake_round_trip(
                NewPoolQuoteU64sBuilder::start()
                    .with_total_lamports(total_lamports)
                    .with_pool_token_supply(pool_token_supply)
                    .build(),
                staked, unstaked, stake_fee, sol_fee,
            );
        }
    }

    #[test]
    fn deposit_stake_unstaked_token_bucket_bound(
        (r_gte, r_lte) in (ratio_gte_one(), ratio_lte_one()),
        staked: u64,
        unstaked: u64,
        sol_fee_n: u64,
        sol_fee_d: u64,
    ) {
        let sol_fee = Fee { numerator: sol_fee_n, denominator: sol_fee_d };
        for Ratio { n: total_lamports, d: pool_token_supply } in [r_gte, r_lte] {
            assert_unstaked_token_bucket_bound(
                NewPoolQuoteU64sBuilder::start()
                    .with_total_lamports(total_lamports)
                    .with_pool_token_supply(pool_token_supply)
                    .build(),
                staked, unstaked, sol_fee,
            );
        }
    }
}

#[test]
fn quote_rev_deposit_stake_when_unstaked_covers_target_returns_zero_staked() {
    let sp = StakePool {
        total_lamports: 1_000_000_000,
        pool_token_supply: 1_000_000_000,
        stake_deposit_fee: Fee {
            numerator: 1,
            denominator: 1,
        },
        sol_deposit_fee: Fee::ZERO,
        ..Default::default()
    };
    let quote = sp
        .quote_rev_deposit_stake_unchecked(QuoteRevDepositStakeArgs {
            tokens_out: 50,
            unstaked_lamports: Some(100),
        })
        .unwrap();

    expect![[r#"
        DepositStakeQuote {
            stake_account_lamports_in: StakeAccountLamports {
                staked: 0,
                unstaked: 100,
            },
            tokens_out: 100,
            manager_fee: 0,
            referral_fee: 0,
        }
    "#]]
    .assert_debug_eq(&quote);
}

#[test]
fn quote_rev_deposit_stake_full_stake_fee_without_unstaked_cover_returns_none() {
    let sp = StakePool {
        total_lamports: 1_000_000_000,
        pool_token_supply: 1_000_000_000,
        stake_deposit_fee: Fee {
            numerator: 1,
            denominator: 1,
        },
        sol_deposit_fee: Fee::ZERO,
        ..Default::default()
    };

    assert!(sp
        .quote_rev_deposit_stake_unchecked(QuoteRevDepositStakeArgs {
            tokens_out: 101,
            unstaked_lamports: Some(100),
        })
        .is_none());
}
