use core::num::NonZeroU32;

use super::StakeStatus;
use borsh::{BorshDeserialize, BorshSerialize};

// Non pub fields, values should be accessed by getters and setters.
#[derive(Copy, Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct ValidatorStakeInfo {
    /// Amount of lamports on the validator stake account, including rent
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    active_stake_lamports: [u8; 8],

    /// Amount of transient stake delegated to this validator
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    transient_stake_lamports: [u8; 8],

    /// Last epoch the active and transient stake lamports fields were updated
    last_update_epoch: [u8; 8],

    /// Transient account seed suffix, used to derive the transient stake
    /// account address
    transient_seed_suffix: [u8; 8],

    /// Unused space, initially meant to specify the end of seed suffixes
    unused: [u8; 4],

    /// Validator account seed suffix
    validator_seed_suffix: [u8; 4], // really `Option<NonZeroU32>` so 0 is `None`

    /// Status of the validator stake account
    status: u8,

    /// Validator vote account address
    vote_account_address: [u8; 32],
}

impl ValidatorStakeInfo {
    inherent_borsh_serde!();
}

impl ValidatorStakeInfo {
    pub const DEFAULT: Self = Self {
        active_stake_lamports: [0u8; 8],
        transient_stake_lamports: [0u8; 8],
        last_update_epoch: [0u8; 8],
        transient_seed_suffix: [0u8; 8],
        unused: [0u8; 4],
        validator_seed_suffix: [0u8; 4],
        // safety: make sure 0 is a valid enum member of StakeStatus
        status: 0,
        vote_account_address: [0u8; 32],
    };
}

impl Default for ValidatorStakeInfo {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Getters that convert the bytes to the correct type
impl ValidatorStakeInfo {
    #[inline]
    pub fn status(&self) -> StakeStatus {
        // unwrap-safety: private fields means we enforce the invariant that
        // StakeStatus is always valid at construction time
        StakeStatus::borsh_de(core::slice::from_ref(&self.status)).unwrap()
    }

    #[inline]
    pub fn active_stake_lamports(&self) -> u64 {
        u64::from_le_bytes(self.active_stake_lamports)
    }

    #[inline]
    pub fn transient_stake_lamports(&self) -> u64 {
        u64::from_le_bytes(self.transient_stake_lamports)
    }

    #[inline]
    pub fn last_update_epoch(&self) -> u64 {
        u64::from_le_bytes(self.last_update_epoch)
    }

    #[inline]
    pub fn transient_seed_suffix(&self) -> u64 {
        u64::from_le_bytes(self.transient_seed_suffix)
    }

    #[inline]
    pub fn validator_seed_suffix(&self) -> Option<NonZeroU32> {
        NonZeroU32::new(u32::from_le_bytes(self.validator_seed_suffix))
    }

    #[inline]
    pub fn vote_account_address(&self) -> &[u8; 32] {
        &self.vote_account_address
    }
}

/// Setters that convert the correct type to bytes
impl ValidatorStakeInfo {
    #[inline]
    pub fn set_status(&mut self, value: StakeStatus) {
        self.status = value.as_byte();
    }

    #[inline]
    pub fn set_active_stake_lamports(&mut self, value: u64) {
        self.active_stake_lamports = value.to_le_bytes();
    }

    #[inline]
    pub fn set_transient_stake_lamports(&mut self, value: u64) {
        self.transient_stake_lamports = value.to_le_bytes();
    }

    #[inline]
    pub fn set_last_update_epoch(&mut self, value: u64) {
        self.last_update_epoch = value.to_le_bytes();
    }

    #[inline]
    pub fn set_transient_seed_suffix(&mut self, value: u64) {
        self.transient_seed_suffix = value.to_le_bytes();
    }

    #[inline]
    pub fn set_validator_seed_suffix(&mut self, value: Option<NonZeroU32>) {
        self.validator_seed_suffix = value.map_or_else(|| 0, |n| n.get()).to_le_bytes();
    }

    #[inline]
    pub fn set_vote_account_address(&mut self, value: [u8; 32]) {
        self.vote_account_address = value;
    }
}
