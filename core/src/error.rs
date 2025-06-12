use core::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SplStakePoolError {
    CalculationFailure,
    IncorrectDepositVoteAddress,
    InvalidSolDepositAuthority,
    InvalidStakeDepositAuthority,
    InvalidState,
    SolWithdrawalTooLarge,
    StakeListAndPoolOutOfDate,
    ValidatorNotFound,
}

impl Display for SplStakePoolError {
    // Display=Debug, since this is just a simple str enum
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Error for SplStakePoolError {}
