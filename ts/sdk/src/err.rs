use wasm_bindgen::{intern, JsError};

pub fn arithmetic_overflow_err() -> JsError {
    JsError::new(intern("arithmetic overflow"))
}

pub fn no_valid_pda() -> JsError {
    JsError::new(intern("no valid PDA found"))
}

pub fn validator_idx_oob() -> JsError {
    JsError::new(intern("validator index out of bounds"))
}
