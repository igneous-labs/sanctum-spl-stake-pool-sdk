use data_encoding::BASE64;
use sanctum_spl_stake_pool_core::{AccountType, StakePool, StakeStatus, ValidatorList};

use crate::common::consts::{STAKE_POOL_DATA, VALIDATOR_LIST_DATA};

#[test]
fn test_stake_pool_serde() {
    let account_json: serde_json::Value = serde_json::from_slice(STAKE_POOL_DATA).unwrap();
    let account_data = BASE64
        .decode(
            account_json["account"]["data"][0]
                .as_str()
                .unwrap()
                .as_bytes(),
        )
        .unwrap();

    // Deserialization
    let stake_pool = StakePool::borsh_de(&mut &account_data[..]).unwrap();
    assert_eq!(stake_pool.account_type, AccountType::StakePool);
    assert_eq!(stake_pool.pool_token_supply, 3792758591416065);
    assert_eq!(stake_pool.total_lamports, 4135211783809274);

    // Serialization
    let mut serialized = Vec::new();
    stake_pool.borsh_ser(&mut serialized).unwrap();

    // We don't have padding in the serialized data implemented for the pool
    assert_eq!(serialized, account_data[..435]);
}

#[test]
fn test_validator_list_serde() {
    let account_json: serde_json::Value = serde_json::from_slice(VALIDATOR_LIST_DATA).unwrap();
    let account_data = BASE64
        .decode(
            account_json["account"]["data"][0]
                .as_str()
                .unwrap()
                .as_bytes(),
        )
        .unwrap();

    // Deserialization
    let validator_list = ValidatorList::deserialize(&account_data).unwrap();
    assert_eq!(
        validator_list.header.account_type,
        AccountType::ValidatorList
    );
    assert_eq!(validator_list.header.max_validators, 10000);
    assert_eq!(validator_list.validators.len(), 3);

    let first_validator = &validator_list.validators[0];
    assert_eq!(first_validator.active_stake_lamports(), 2947319964963369);
    assert_eq!(first_validator.transient_stake_lamports(), 0u64);
    assert_eq!(first_validator.last_update_epoch(), 751);
    assert_eq!(first_validator.status(), StakeStatus::Active);

    // Serialization
    let mut serialized = Vec::new();
    validator_list.borsh_ser(&mut serialized).unwrap();
    assert_eq!(serialized, account_data);
}
