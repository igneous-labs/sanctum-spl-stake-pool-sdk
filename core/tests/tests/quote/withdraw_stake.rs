use expect_test::expect;
use proptest::prelude::*;
use sanctum_spl_stake_pool_core::{Fee, StakePool, WithdrawStakeQuote};
use sanctum_u64_ratio::Ratio;

use crate::common::{
    proptest_utils::{ratio_gte_one, ratio_lte_one},
    quote::{NewPoolQuoteU64sBuilder, PoolQuoteU64Ds},
};

#[test]
fn quoting_with_zero_fee_should_not_error() {
    let tokens = 126455611948;
    let sp = StakePool {
        total_lamports: 4072725611527686,
        pool_token_supply: 3727925207812268,
        stake_withdrawal_fee: Fee::ZERO,
        ..Default::default()
    };
    expect![[r#"
        WithdrawStakeQuote {
            tokens_in: 126455611948,
            lamports_staked: 138151647576,
            fee_amount: 0,
        }
    "#]]
    .assert_debug_eq(&sp.quote_withdraw_stake_unchecked(tokens).unwrap());
}

fn quote_rev_withdraw_stake_round_trip_x_gte_1(
    stake_withdrawal_fee: Fee,
    pool: PoolQuoteU64Ds,
    lamports_staked: u64,
) {
    let sp = StakePool {
        total_lamports: *pool.total_lamports(),
        pool_token_supply: *pool.pool_token_supply(),
        stake_withdrawal_fee,
        ..Default::default()
    };
    match sp.quote_rev_withdraw_stake_unchecked(lamports_staked) {
        Some(WithdrawStakeQuote {
            tokens_in: pool_tokens,
            lamports_staked: out_lamports,
            fee_amount: _,
        }) => {
            assert_eq!(out_lamports, lamports_staked);
            // diff can happen due to Ratio::reverse_est
            // but quote should always give more out lamports
            // for the same input pool tokens
            let quoted_lamports = sp
                .quote_withdraw_stake_unchecked(pool_tokens)
                .unwrap()
                .lamports_staked;
            assert!(
                quoted_lamports >= out_lamports,
                "{quoted_lamports}, {out_lamports}"
            );
            // despite difference, round-tripping should always give same reuslt
            let rt_in = sp
                .quote_rev_withdraw_stake_unchecked(quoted_lamports)
                .unwrap()
                .tokens_in;
            assert_eq!(rt_in, pool_tokens);
        }
        // since x-rate >= 1, quote_rev's
        // input (lamports) > output (tokens),
        // so if quote_rev overflows then
        // quote must also overflow for the same amount
        // only if fees are zero.
        // Otherwise fees might result in quote not overflowing
        None => {
            let should_check = stake_withdrawal_fee
                .to_fee_ceil()
                .map_or_else(|| true, |f| f.as_inner_ref().0.is_zero());
            if should_check {
                assert_eq!(sp.quote_withdraw_stake_unchecked(lamports_staked), None);
            }
        }
    }
}

proptest! {
    #[test]
    fn quote_rev_withdraw_stake_round_trip_x_gte_1_pt(
        numerator: u64,
        denominator: u64,
        Ratio {
            n: total_lamports,
            d: pool_token_supply,
        } in ratio_gte_one(),
        lamports_staked: u64,
    ) {
        quote_rev_withdraw_stake_round_trip_x_gte_1(
            Fee { numerator, denominator },
        NewPoolQuoteU64sBuilder::start()
            .with_total_lamports(total_lamports)
            .with_pool_token_supply(pool_token_supply)
            .build(),
            lamports_staked,
        );
    }
}

fn quote_rev_withdraw_stake_round_trip_x_lte_1(
    stake_withdrawal_fee: Fee,
    pool: PoolQuoteU64Ds,
    tokens_in: u64,
) {
    let sp = StakePool {
        total_lamports: *pool.total_lamports(),
        pool_token_supply: *pool.pool_token_supply(),
        stake_withdrawal_fee,
        ..Default::default()
    };
    match sp.quote_withdraw_stake_unchecked(tokens_in) {
        Some(WithdrawStakeQuote {
            tokens_in: pool_tokens,
            lamports_staked,
            fee_amount: _,
        }) => {
            assert_eq!(pool_tokens, tokens_in);
            let quoted_tokens = sp
                .quote_rev_withdraw_stake_unchecked(lamports_staked)
                .unwrap()
                .tokens_in;
            // diff can happen due to Ratio::reverse_est
            // but quote_rev should always give less input tokens
            // for the same output lamports
            assert!(quoted_tokens <= tokens_in, "{quoted_tokens}, {tokens_in}");
            // despite difference, round-tripping should always give same reuslt
            let rt_out = sp
                .quote_withdraw_stake_unchecked(quoted_tokens)
                .unwrap()
                .lamports_staked;
            assert_eq!(rt_out, lamports_staked);
        }
        // since x-rate <= 1, quote's
        // output (lamports) > input (tokens).
        // Plus, fees result in quote_rev out > quote out
        // So if quote overflows then
        // quote_rev must also overflow for the same amount
        None => assert_eq!(sp.quote_rev_withdraw_stake_unchecked(tokens_in), None),
    }
}

proptest! {
    #[test]
    fn quote_rev_withdraw_stake_round_trip_x_lte_1_pt(
        numerator: u64,
        denominator: u64,
        Ratio {
            n: total_lamports,
            d: pool_token_supply,
        } in ratio_lte_one(),
        tokens_in: u64,
    ) {
        quote_rev_withdraw_stake_round_trip_x_lte_1(
            Fee { numerator, denominator },
        NewPoolQuoteU64sBuilder::start()
            .with_total_lamports(total_lamports)
            .with_pool_token_supply(pool_token_supply)
            .build(),
            tokens_in,
        );
    }
}
