use generic_array_struct::generic_array_struct;

#[generic_array_struct(builder pub)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolQuoteU64s<T> {
    pub total_lamports: T,
    pub pool_token_supply: T,
}

pub type PoolQuoteU64Ds = PoolQuoteU64s<u64>;
