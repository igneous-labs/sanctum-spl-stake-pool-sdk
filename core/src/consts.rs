use const_crypto::bs58;

pub const SYSVAR_RENT: [u8; 32] =
    bs58::decode_pubkey("SysvarRent111111111111111111111111111111111");

pub const SYSVAR_STAKE_HISTORY: [u8; 32] =
    bs58::decode_pubkey("SysvarStakeHistory1111111111111111111111111");

pub const SYSVAR_CLOCK: [u8; 32] =
    bs58::decode_pubkey("SysvarC1ock11111111111111111111111111111111");

pub const STAKE_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("Stake11111111111111111111111111111111111111");

pub const SYSVAR_STAKE_CONFIG: [u8; 32] =
    bs58::decode_pubkey("StakeConfig11111111111111111111111111111111");

pub const SYSTEM_PROGRAM: [u8; 32] = bs58::decode_pubkey("11111111111111111111111111111111");

pub const TOKEN_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub const ASSOCIATED_TOKEN_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

pub const STAKE_ACCOUNT_RENT_EXEMPT_LAMPORTS: u64 = 2282880;

/// Minimum amount of staked lamports required in a validator stake account
/// enforced by the spl stake pool program.
pub const MIN_ACTIVE_STAKE: u64 = 1_000_000;
