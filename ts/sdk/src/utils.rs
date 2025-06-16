use bs58_fixed_wasm::Bs58Array;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[tsify_next::declare]
pub type B58PK = Bs58Array<32, 44>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AccountMeta {
    pub address: B58PK,

    /// Represents the role of an account in a transaction:
    /// - Readonly: 0
    /// - Writable: 1
    /// - ReadonlySigner: 2
    /// - WritableSigner: 3
    #[tsify(type = "0 | 1 | 2 | 3")]
    pub role: u8,
}

impl AccountMeta {
    pub(crate) const fn new(address: [u8; 32], role: Role) -> Self {
        Self {
            address: B58PK::new(address),
            role: role.as_u8(),
        }
    }
}

pub fn keys_signer_writer_to_account_metas<const N: usize>(
    keys: &[&[u8; 32]; N],
    signer: &[bool; N],
    writer: &[bool; N],
) -> [AccountMeta; N] {
    core::array::from_fn(|i| {
        let k = keys[i];
        AccountMeta::new(*k, Role::from_signer_writable(signer[i], writer[i]))
    })
}

pub enum Role {
    Readonly,
    Writable,
    ReadonlySigner,
    WritableSigner,
}

impl Role {
    pub const fn from_signer_writable(signer: bool, writable: bool) -> Self {
        match (signer, writable) {
            (true, true) => Self::WritableSigner,
            (true, false) => Self::ReadonlySigner,
            (false, true) => Self::Writable,
            (false, false) => Self::Readonly,
        }
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::Readonly => 0,
            Self::Writable => 1,
            Self::ReadonlySigner => 2,
            Self::WritableSigner => 3,
        }
    }
}
