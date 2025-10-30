use const_crypto::bs58;
use data_encoding::BASE64;
use sanctum_spl_stake_pool_core::{
    StakePool, UpdateStakePoolBalanceIxData, UpdateStakePoolBalanceIxKeysOwned,
    UpdateValidatorListBalanceIxData, UpdateValidatorListBalanceIxPrefixKeysOwned, ValidatorList,
};

use crate::common::consts::{STAKE_POOL_DATA, VALIDATOR_LIST_DATA};

#[test]
fn test_update_stake_pool_balance_ix() {
    let account_json: serde_json::Value = serde_json::from_slice(STAKE_POOL_DATA).unwrap();
    let account_data = BASE64
        .decode(
            account_json["account"]["data"][0]
                .as_str()
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    let stake_pool = StakePool::borsh_de(&mut &account_data[..]).unwrap();

    let accounts = UpdateStakePoolBalanceIxKeysOwned::default()
        .with_keys_from_stake_pool(&stake_pool)
        .with_stake_pool(stake_pool.stake_deposit_authority)
        .with_withdraw_auth(stake_pool.stake_deposit_authority);

    let data = UpdateStakePoolBalanceIxData::new();

    assert_eq!(accounts.0[0], stake_pool.stake_deposit_authority);
    assert_eq!(accounts.0[1], stake_pool.stake_deposit_authority);
    assert_eq!(data.to_buf(), [7]);
}

#[test]
fn test_update_validator_list_balance_ix() {
    let account_json: serde_json::Value = serde_json::from_slice(STAKE_POOL_DATA).unwrap();
    let account_data = BASE64
        .decode(
            account_json["account"]["data"][0]
                .as_str()
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    let stake_pool = StakePool::borsh_de(&mut &account_data[..]).unwrap();
    let stake_pool_addr = bs58::decode_pubkey("8VpRhuxa7sUUepdY3kQiTmX9rS5vx4WgaXiAnXq4KCtr");

    let account_json: serde_json::Value = serde_json::from_slice(VALIDATOR_LIST_DATA).unwrap();
    let account_data = BASE64
        .decode(
            account_json["account"]["data"][0]
                .as_str()
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    let validator_list = ValidatorList::deserialize(&account_data).unwrap();

    let accounts = UpdateValidatorListBalanceIxPrefixKeysOwned::default()
        .with_keys_from_stake_pool(&stake_pool)
        .with_stake_pool(stake_pool.stake_deposit_authority)
        .with_withdraw_auth(stake_pool.stake_deposit_authority)
        .with_consts();

    let data = UpdateValidatorListBalanceIxData::new(0, false);

    let validator_seeds = validator_list
        .validator_stake_account_seeds_itr(&stake_pool_addr)
        .collect::<Vec<_>>();

    let transient_seeds = validator_list
        .transient_stake_account_seeds_itr(&stake_pool_addr)
        .collect::<Vec<_>>();

    assert_eq!(accounts.0[0], stake_pool.stake_deposit_authority);
    assert_eq!(accounts.0[1], stake_pool.stake_deposit_authority);
    assert_eq!(data.to_buf(), [6, 0, 0, 0, 0, 0]);
    assert_eq!(validator_seeds.len(), 3);
    assert_eq!(transient_seeds.len(), 3);
}
