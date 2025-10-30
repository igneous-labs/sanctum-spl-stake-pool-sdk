use expect_test::expect;
use generic_array_struct::generic_array_struct;
use proptest::prelude::*;
use sanctum_spl_stake_pool_core::{Fee, StakePool, WithdrawSolQuote};
use sanctum_u64_ratio::Ratio;

use crate::common::proptest_utils::ratio_gte_one;

#[generic_array_struct(builder pub)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolQuoteU64s<T> {
    pub total_lamports: T,
    pub pool_token_supply: T,
}

pub type PoolQuoteU64Ds = PoolQuoteU64s<u64>;

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

fn quote_rev_withdraw_sol_round_trip_x_gte_1(
    sol_withdrawal_fee: Fee,
    pool: PoolQuoteU64Ds,
    lamports: u64,
) {
    let sp = StakePool {
        total_lamports: *pool.total_lamports(),
        pool_token_supply: *pool.pool_token_supply(),
        sol_withdrawal_fee,
        ..Default::default()
    };
    match sp.quote_rev_withdraw_sol_unchecked(lamports) {
        Some(WithdrawSolQuote {
            in_amount: pool_tokens,
            out_amount: out_lamports,
            manager_fee: _,
        }) => {
            assert_eq!(out_lamports, lamports);
            // diff can happen due to Ratio::reverse_est
            // but should always give more
            // (quote_rev_withdraw_sol_unchecked overestimates)
            let quoted_lamports = sp
                .quote_withdraw_sol_unchecked(pool_tokens)
                .unwrap()
                .out_amount;
            assert!(
                quoted_lamports >= out_lamports,
                "{quoted_lamports}, {out_lamports}"
            );
            // despite difference, round-tripping should always give same reuslt
            let rt_in = sp
                .quote_rev_withdraw_sol_unchecked(quoted_lamports)
                .unwrap()
                .in_amount;
            assert_eq!(rt_in, pool_tokens);
        }
        // since x-rate >= 1, quote_rev's
        // input (lamports) > output (tokens),
        // so if quote_rev overflows then
        // quote must also overflow for the same amount
        // only if fees are zero.
        // Otherwise fees might result in quote not overflowing
        None => {
            let should_check = sol_withdrawal_fee
                .to_fee_ceil()
                .map_or_else(|| true, |f| f.as_inner_ref().0.is_zero());
            if should_check {
                assert_eq!(sp.quote_withdraw_sol_unchecked(lamports), None);
            }
        }
    }
}

proptest! {
    #[test]
    fn quote_rev_withdraw_sol_round_trip_x_gte_1_pt(
        numerator: u64,
        denominator: u64,
        Ratio {
            n: total_lamports,
            d: pool_token_supply,
        } in ratio_gte_one(),
        lamports: u64,
    ) {
        quote_rev_withdraw_sol_round_trip_x_gte_1(
            Fee { numerator, denominator },
        NewPoolQuoteU64sBuilder::start()
            .with_total_lamports(total_lamports)
            .with_pool_token_supply(pool_token_supply)
            .build(),
            lamports,
        );
    }
}
