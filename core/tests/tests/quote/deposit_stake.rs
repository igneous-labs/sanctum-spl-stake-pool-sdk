use sanctum_spl_stake_pool_core::{DepositStakeQuote, Fee, StakeAccountLamports, StakePool};

fn assert_sufficient_and_minimal(sp: &StakePool, quote: DepositStakeQuote, requested: u64) {
    assert_eq!(
        sp.quote_deposit_stake_unchecked(quote.stake_account_lamports_in),
        Some(quote)
    );
    assert!(
        quote.tokens_out >= requested,
        "{} < {}",
        quote.tokens_out,
        requested
    );

    if quote.stake_account_lamports_in.staked == 0 {
        return;
    }

    let previous = StakeAccountLamports {
        staked: quote.stake_account_lamports_in.staked - 1,
        unstaked: quote.stake_account_lamports_in.unstaked,
    };
    if let Some(previous_quote) = sp.quote_deposit_stake_unchecked(previous) {
        assert!(
            previous_quote.tokens_out < requested,
            "previous staked lamports still satisfied target: {:?}",
            previous_quote
        );
    }
}

#[test]
fn quote_rev_deposit_stake_zero_fee_direct_path() {
    let sp = StakePool {
        total_lamports: 1_000_000_000_000,
        pool_token_supply: 900_000_000_000,
        stake_deposit_fee: Fee::ZERO,
        sol_deposit_fee: Fee::ZERO,
        ..Default::default()
    };
    let requested = 90_000_000;
    let unstaked = 2_282_880;

    let quote = sp
        .quote_rev_deposit_stake_unchecked(requested, unstaked)
        .unwrap();

    assert_eq!(quote.stake_account_lamports_in.unstaked, unstaked);
    assert_eq!(quote.manager_fee, 0);
    assert_eq!(quote.referral_fee, 0);
    assert_sufficient_and_minimal(&sp, quote, requested);
}

#[test]
fn quote_rev_deposit_stake_unstaked_lamports_already_cover_target() {
    let sp = StakePool {
        total_lamports: 1_000_000_000,
        pool_token_supply: 1_000_000_000,
        stake_deposit_fee: Fee::ZERO,
        sol_deposit_fee: Fee::ZERO,
        ..Default::default()
    };
    let requested = 1_000_000;
    let unstaked = 2_282_880;

    let quote = sp
        .quote_rev_deposit_stake_unchecked(requested, unstaked)
        .unwrap();

    assert_eq!(quote.stake_account_lamports_in.staked, 0);
    assert_eq!(quote.stake_account_lamports_in.unstaked, unstaked);
    assert_sufficient_and_minimal(&sp, quote, requested);
}

#[test]
fn quote_rev_deposit_stake_binary_search_path_with_nonzero_fees() {
    let sp = StakePool {
        total_lamports: 1_000_000_000_000,
        pool_token_supply: 900_000_000_000,
        stake_deposit_fee: Fee {
            numerator: 3,
            denominator: 1_000,
        },
        sol_deposit_fee: Fee {
            numerator: 7,
            denominator: 1_000,
        },
        stake_referral_fee: 20,
        ..Default::default()
    };
    let requested = 90_000_000;
    let unstaked = 2_282_880;

    let quote = sp
        .quote_rev_deposit_stake_unchecked(requested, unstaked)
        .unwrap();

    assert_eq!(quote.stake_account_lamports_in.unstaked, unstaked);
    assert!(quote.manager_fee > 0);
    assert_sufficient_and_minimal(&sp, quote, requested);
}
