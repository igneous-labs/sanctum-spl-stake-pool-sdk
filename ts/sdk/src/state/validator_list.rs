use sanctum_spl_stake_pool_core::ValidatorListHeader;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::ValidatorStakeInfo;

#[wasm_bindgen]
pub struct ValidatorListHandle(pub(crate) ValidatorListOwned);

#[wasm_bindgen(js_name = defaultValidatorList)]
pub fn default_validator_list() -> ValidatorListHandle {
    ValidatorListHandle(Default::default())
}

#[wasm_bindgen(js_name = getValidatorList)]
pub fn get_validator_list(this: &ValidatorListHandle) -> ValidatorList {
    ValidatorList::from_core(&this.0.as_borrowed())
}

/// @throws if `val` contains invalid vote account pubkeys
#[wasm_bindgen(js_name = setValidatorList)]
pub fn set_validator_list(
    this: &mut ValidatorListHandle,
    val: ValidatorList,
) -> Result<(), JsError> {
    this.0 = val.try_to_core()?;
    Ok(())
}

/// @throws if bytes do not make up a valid ValidatorList
#[wasm_bindgen(js_name = deserValidatorList)]
pub fn deser_validator_list(bytes: &[u8]) -> Result<ValidatorListHandle, JsError> {
    let val = sanctum_spl_stake_pool_core::ValidatorList::deserialize(bytes)?;
    Ok(ValidatorListHandle(ValidatorListOwned {
        header: val.header,
        validators: val.validators.to_vec(),
    }))
}

/// @throws if borsh serialization failed
#[wasm_bindgen(js_name = serValidatorList)]
pub fn ser_validator_list(
    ValidatorListHandle(val): &ValidatorListHandle,
) -> Result<Box<[u8]>, JsError> {
    let mut vec = Vec::new();
    val.as_borrowed().borsh_ser(&mut vec)?;
    Ok(vec.into())
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)]
pub struct ValidatorList {
    pub header: ValidatorListHeader,
    pub validators: Vec<ValidatorStakeInfo>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ValidatorListOwned {
    header: ValidatorListHeader,
    validators: Vec<sanctum_spl_stake_pool_core::ValidatorStakeInfo>,
}

impl Default for ValidatorListOwned {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl ValidatorListOwned {
    const DEFAULT: Self = Self {
        header: ValidatorListHeader {
            account_type: sanctum_spl_stake_pool_core::AccountType::ValidatorList,
            max_validators: 0,
        },
        validators: Vec::new(),
    };

    pub(crate) fn as_borrowed(&self) -> sanctum_spl_stake_pool_core::ValidatorList {
        sanctum_spl_stake_pool_core::ValidatorList {
            header: self.header,
            validators: &self.validators,
        }
    }
}

impl ValidatorList {
    fn try_to_core(&self) -> Result<ValidatorListOwned, JsError> {
        let Self { header, validators } = self;
        let validators: Result<Vec<_>, _> =
            validators.iter().map(|vsi| vsi.try_to_core()).collect();
        Ok(ValidatorListOwned {
            header: *header,
            validators: validators?,
        })
    }

    fn from_core(
        sanctum_spl_stake_pool_core::ValidatorList { header, validators }: &sanctum_spl_stake_pool_core::ValidatorList<
            '_,
        >,
    ) -> Self {
        Self {
            header: *header,
            validators: validators
                .iter()
                .map(ValidatorStakeInfo::from_core)
                .collect(),
        }
    }
}
