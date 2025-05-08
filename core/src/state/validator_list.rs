use crate::{
    transient_stake_seeds, validator_stake_seeds, AccountType, OptionalSeed, ValidatorListHeader,
    ValidatorStakeInfo,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ValidatorList<'a> {
    /// Data outside of the validator list, separated out for cheaper
    /// deserialization
    pub header: ValidatorListHeader,

    // Removed the validator stake info list from here to be able to parse the vec without using std, utilizing how it's stored in the memory
    pub validators: &'a [ValidatorStakeInfo],
}

/// Includes `num_validators`. Used for borsh ser/de only
#[derive(BorshDeserialize, BorshSerialize)]
struct ValidatorListHeaderExt {
    account_type: AccountType,
    max_validators: u32,
    num_validators: u32,
}

impl<'a> ValidatorList<'a> {
    /// Deserializes the header and the validators
    #[inline]
    pub fn deserialize(data: &'a [u8]) -> borsh::io::Result<Self> {
        let mut remaining = data;
        let ValidatorListHeaderExt {
            account_type,
            max_validators,
            num_validators,
        } = ValidatorListHeaderExt::deserialize(&mut remaining)?;
        // CHECK: If data size matches expected size for num_validators
        let num_validators = num_validators as usize;
        let validator_size = core::mem::size_of::<ValidatorStakeInfo>();
        let expected_size = validator_size * num_validators;
        if remaining.len() < expected_size {
            return Err(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                "Data too small for validators",
            ));
        }

        const _: () = assert!(
            core::mem::align_of::<ValidatorStakeInfo>() == 1,
            "ValidatorStakeInfo must have alignment of 1"
        );

        // SAFETY: ValidatorStakeInfo has alignment of 1 (checked at compile time)
        let validators = unsafe {
            core::slice::from_raw_parts(
                remaining.as_ptr() as *const ValidatorStakeInfo,
                num_validators,
            )
        };

        Ok(ValidatorList {
            header: ValidatorListHeader {
                account_type,
                max_validators,
            },
            validators,
        })
    }

    /// Serializes the header and the validators with padding up to max_validators
    #[inline]
    pub fn borsh_ser<W: borsh::io::Write>(&self, mut writer: W) -> borsh::io::Result<()> {
        let Self {
            header:
                ValidatorListHeader {
                    account_type,
                    max_validators,
                },
            validators,
        } = self;
        let header_ext = ValidatorListHeaderExt {
            account_type: *account_type,
            max_validators: *max_validators,
            num_validators: validators.len().try_into().map_err(|_e| {
                // use static string instead of arg `e` here because borsh is broken between no-std and std:
                // https://github.com/near/borsh-rs/issues/342
                borsh::io::Error::new(
                    borsh::io::ErrorKind::InvalidData,
                    "validators len u32 overflow",
                )
            })?,
        };
        header_ext.serialize(&mut writer)?;

        for validator in validators.iter() {
            validator.borsh_ser(&mut writer)?;
        }

        // Pad remaining space up to max_validators
        let empty_validator = ValidatorStakeInfo::default();
        for _ in validators.len()..*max_validators as usize {
            empty_validator.borsh_ser(&mut writer)?;
        }

        Ok(())
    }

    pub fn validator_stake_account_seeds_itr(
        &'a self,
        stake_pool: &'a [u8; 32],
    ) -> impl Iterator<Item = (&'a [u8; 32], &'a [u8; 32], OptionalSeed<[u8; 4]>)> {
        self.validators.iter().map(move |v| {
            validator_stake_seeds(
                v.vote_account_address(),
                stake_pool,
                v.validator_seed_suffix(),
            )
        })
    }

    pub fn transient_stake_account_seeds_itr(
        &'a self,
        stake_pool: &'a [u8; 32],
    ) -> impl Iterator<Item = (&'a [u8; 9], &'a [u8; 32], &'a [u8; 32], [u8; 8])> {
        self.validators.iter().map(move |v| {
            transient_stake_seeds(
                v.vote_account_address(),
                stake_pool,
                v.transient_seed_suffix(),
            )
        })
    }

    /// Yields `(validator_stake_account_seeds, transient_stake_account_seeds)`
    /// for each validator on the list
    #[allow(clippy::type_complexity)]
    pub fn account_pair_seeds_itr(
        &'a self,
        stake_pool: &'a [u8; 32],
    ) -> impl Iterator<
        Item = (
            (&'a [u8; 32], &'a [u8; 32], OptionalSeed<[u8; 4]>),
            (&'a [u8; 9], &'a [u8; 32], &'a [u8; 32], [u8; 8]),
        ),
    > {
        self.validator_stake_account_seeds_itr(stake_pool)
            .zip(self.transient_stake_account_seeds_itr(stake_pool))
    }
}
