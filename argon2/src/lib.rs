use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chacha20poly1305::aead::OsRng;
use extism_pdk::*;
use serde::Deserialize;

#[plugin_fn]
pub fn hash(password: String) -> FnResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            return Err(extism_pdk::WithReturnCode::new(
                extism_pdk::Error::msg(e.to_string()),
                0,
            ))
        }
    };

    Ok(hash)
}

#[derive(FromBytes, Deserialize, PartialEq, Debug)]
#[encoding(Json)]
struct Verify {
    password: String,
    hash: String,
}

#[plugin_fn]
pub fn verify(input: Verify) -> FnResult<String> {
    if input.hash.is_empty() || input.password.is_empty() {
        return Ok("false".to_string());
    }

    let parsed_hash = match PasswordHash::new(&input.hash) {
        Ok(hash) => hash,
        Err(_) => return Ok("false".to_string()),
    };

    let is_password_valid = Argon2::default()
        .verify_password(input.password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(if is_password_valid {
        "true".to_string()
    } else {
        "false".to_string()
    })
}
