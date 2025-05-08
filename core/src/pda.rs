use core::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OptionalSeed<S> {
    None,
    Some(S),
}

impl<S: AsRef<[u8]>> OptionalSeed<S> {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::None => &[],
            Self::Some(s) => s.as_ref(),
        }
    }
}

impl<S: AsRef<[u8]>> AsRef<[u8]> for OptionalSeed<S> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

pub const AUTHORITY_WITHDRAW_SEED: [u8; 8] = *b"withdraw";
pub const AUTHORITY_DEPOSIT_SEED: [u8; 7] = *b"deposit";
pub const TRANSIENT_SEED: [u8; 9] = *b"transient";
pub const EPHEMERAL_SEED: [u8; 9] = *b"ephemeral";

#[inline]
pub const fn withdraw_auth_seeds(stake_pool: &[u8; 32]) -> (&[u8; 32], &[u8; 8]) {
    (stake_pool, &AUTHORITY_WITHDRAW_SEED)
}

#[inline]
pub const fn deposit_auth_seeds(stake_pool: &[u8; 32]) -> (&[u8; 32], &[u8; 7]) {
    (stake_pool, &AUTHORITY_DEPOSIT_SEED)
}

#[inline]
pub const fn validator_stake_seeds<'a>(
    vote_account: &'a [u8; 32],
    stake_pool: &'a [u8; 32],
    seed: Option<NonZeroU32>,
) -> (&'a [u8; 32], &'a [u8; 32], OptionalSeed<[u8; 4]>) {
    if let Some(seed) = seed {
        (
            vote_account,
            stake_pool,
            OptionalSeed::Some(seed.get().to_le_bytes()),
        )
    } else {
        (vote_account, stake_pool, OptionalSeed::None)
    }
}

#[inline]
pub const fn transient_stake_seeds<'a>(
    vote_account: &'a [u8; 32],
    stake_pool: &'a [u8; 32],
    seed: u64,
) -> (&'a [u8; 9], &'a [u8; 32], &'a [u8; 32], [u8; 8]) {
    (
        &TRANSIENT_SEED,
        vote_account,
        stake_pool,
        seed.to_le_bytes(),
    )
}

#[inline]
pub const fn ephemeral_stake_seeds(stake_pool: &[u8; 32]) -> (&[u8; 9], &[u8; 32], [u8; 8]) {
    (&EPHEMERAL_SEED, stake_pool, [0u8; 8])
}
