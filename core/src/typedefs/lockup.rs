use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lockup {
    pub unix_timestamp: i64,

    pub epoch: u64,

    pub custodian: [u8; 32],
}

impl Default for Lockup {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Lockup {
    pub const DEFAULT: Self = Self {
        unix_timestamp: 0,
        epoch: 0,
        custodian: [0u8; 32],
    };
}

impl Lockup {
    inherent_borsh_serde!();
}
