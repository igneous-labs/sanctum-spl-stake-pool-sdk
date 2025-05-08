use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_u64_ratio::{Floor, Ratio};

// TODO: derivation of Eq might be wrong since fraction equality is not necessarily bit equality,
// but this is how upstream does it
#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Fee {
    pub denominator: u64,
    pub numerator: u64,
}

type F = sanctum_fee_ratio::Fee<Floor<Ratio<u64, u64>>>;

impl Fee {
    pub const ZERO: Self = Self {
        denominator: 0,
        numerator: 0,
    };

    #[inline]
    pub const fn to_fee_floor(&self) -> Option<F> {
        F::new(Ratio {
            n: self.numerator,
            d: self.denominator,
        })
    }
}

impl Default for Fee {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Fee {
    inherent_borsh_serde!();
}

type RF = sanctum_fee_ratio::Fee<Floor<Ratio<u8, u8>>>;

/// Just use `self.0` for any fee operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReferralFee(pub RF);

impl ReferralFee {
    /// Returns None if `fee_pct > 100`
    #[inline]
    pub const fn new(fee_pct: u8) -> Option<Self> {
        match RF::new(Ratio { n: fee_pct, d: 100 }) {
            Some(f) => Some(Self(f)),
            None => None,
        }
    }
}
