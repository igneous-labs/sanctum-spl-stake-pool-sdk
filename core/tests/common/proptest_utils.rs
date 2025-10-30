use proptest::prelude::*;
use sanctum_u64_ratio::Ratio;

/// Includes `{ n: 0, d: 0 }`
pub fn ratio_gte_one() -> impl Strategy<Value = Ratio<u64, u64>> {
    (0..=u64::MAX)
        .prop_flat_map(|n| (Just(n), 0..=n))
        .prop_map(|(n, d)| Ratio { n, d })
}
