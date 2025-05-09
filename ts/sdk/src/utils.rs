use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::conv::pubkey_to_js;

#[allow(unused)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AccountMeta {
    pub address: Box<str>,

    /// Represents the role of an account in a transaction:
    /// - Readonly: 0
    /// - Writable: 1
    /// - ReadonlySigner: 2
    /// - WritableSigner: 3
    #[tsify(type = "0 | 1 | 2 | 3")]
    pub role: u8,
}

impl AccountMeta {
    pub(crate) const fn new(address: Box<str>, role: Role) -> Self {
        Self {
            address,
            role: role.as_u8(),
        }
    }
}

pub(crate) fn keys_signer_writer_to_account_metas<const N: usize>(
    keys: &[&[u8; 32]; N],
    signer: &[bool; N],
    writer: &[bool; N],
) -> [AccountMeta; N] {
    core::array::from_fn(|i| {
        let k = keys[i];
        AccountMeta::new(
            pubkey_to_js(k),
            Role::from_signer_writable(signer[i], writer[i]),
        )
    })
}

pub(crate) enum Role {
    Readonly,
    Writable,
    ReadonlySigner,
    WritableSigner,
}

impl Role {
    pub(crate) const fn from_signer_writable(signer: bool, writable: bool) -> Self {
        match (signer, writable) {
            (true, true) => Self::WritableSigner,
            (true, false) => Self::ReadonlySigner,
            (false, true) => Self::Writable,
            (false, false) => Self::Readonly,
        }
    }

    pub(crate) const fn as_u8(&self) -> u8 {
        match self {
            Self::Readonly => 0,
            Self::Writable => 1,
            Self::ReadonlySigner => 2,
            Self::WritableSigner => 3,
        }
    }
}
