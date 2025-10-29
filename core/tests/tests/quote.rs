use expect_test::expect;
use sanctum_spl_stake_pool_core::{Fee, StakePool};

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
