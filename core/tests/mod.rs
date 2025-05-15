#[cfg(test)]
mod tests {
    use const_crypto::bs58;
    use data_encoding::BASE64;
    use sanctum_spl_stake_pool_core::{self as stake_pool_sdk};

    const STAKE_POOL_DATA: &[u8] = include_bytes!("../../test-fixtures/jupsol-stake-pool.json");
    const VALIDATOR_LIST_DATA: &[u8] = include_bytes!("../../test-fixtures/validator-list.json");

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
        let stake_pool = stake_pool_sdk::StakePool::borsh_de(&mut &account_data[..]).unwrap();
        assert_eq!(
            stake_pool.account_type,
            stake_pool_sdk::AccountType::StakePool
        );
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
        let validator_list = stake_pool_sdk::ValidatorList::deserialize(&account_data).unwrap();
        assert_eq!(
            validator_list.header.account_type,
            stake_pool_sdk::AccountType::ValidatorList
        );
        assert_eq!(validator_list.header.max_validators, 10000);
        assert_eq!(validator_list.validators.len(), 3);

        let first_validator = &validator_list.validators[0];
        assert_eq!(first_validator.active_stake_lamports(), 2947319964963369);
        assert_eq!(first_validator.transient_stake_lamports(), 0u64);
        assert_eq!(first_validator.last_update_epoch(), 751);
        assert_eq!(
            first_validator.status(),
            stake_pool_sdk::StakeStatus::Active
        );

        // Serialization
        let mut serialized = Vec::new();
        validator_list.borsh_ser(&mut serialized).unwrap();
        assert_eq!(serialized, account_data);
    }

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
        let stake_pool = stake_pool_sdk::StakePool::borsh_de(&mut &account_data[..]).unwrap();

        let accounts = stake_pool_sdk::UpdateStakePoolBalanceIxKeysOwned::default()
            .with_keys_from_stake_pool(&stake_pool)
            .with_stake_pool(stake_pool.stake_deposit_authority)
            .with_withdraw_auth(stake_pool.stake_deposit_authority);

        let data = stake_pool_sdk::UpdateStakePoolBalanceIxData::new();

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
        let stake_pool = stake_pool_sdk::StakePool::borsh_de(&mut &account_data[..]).unwrap();
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
        let validator_list = stake_pool_sdk::ValidatorList::deserialize(&account_data).unwrap();

        let accounts = stake_pool_sdk::UpdateValidatorListBalanceIxPrefixKeysOwned::default()
            .with_keys_from_stake_pool(&stake_pool)
            .with_stake_pool(stake_pool.stake_deposit_authority)
            .with_withdraw_auth(stake_pool.stake_deposit_authority)
            .with_consts();

        let data = stake_pool_sdk::UpdateValidatorListBalanceIxData::new(0, false);

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
}
