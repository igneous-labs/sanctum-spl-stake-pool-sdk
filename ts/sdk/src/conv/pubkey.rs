use wasm_bindgen::{intern, JsError};

pub(crate) fn pubkey_from_js(s: &str) -> Result<[u8; 32], JsError> {
    let mut res = [0u8; 32];
    let written = bs58::decode(s)
        .onto(res.as_mut_slice())
        .map_err(|e| JsError::new(intern(&e.to_string())))?;
    if written != 32 {
        Err(JsError::new(intern("Not a pubkey")))
    } else {
        Ok(res)
    }
}

pub(crate) fn pubkey_to_js(pubkey: &[u8; 32]) -> Box<str> {
    const_bs58_to_str::<32, 45>(pubkey)
}

fn const_bs58_to_str<const BUF: usize, const MAX_STR_LEN: usize>(buf: &[u8; BUF]) -> Box<str> {
    const {
        // formula: https://stackoverflow.com/a/59590236/5057425
        assert!(BUF == MAX_STR_LEN * 100 / 138);
    };

    let mut res = [0u8; MAX_STR_LEN];
    // unwrap-safety: buf sufficient length checked by assert above
    let len = unsafe {
        bs58::encode(buf)
            .onto(res.as_mut_slice())
            .unwrap_unchecked()
    };
    // safety: bs58 is valid ascii/utf8
    let s = unsafe { core::str::from_utf8_unchecked(&res[..len]) };
    intern(s);
    s.into()
}
