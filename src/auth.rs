use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    sub: String,
    exp: usize,
}

pub fn create(sub: String, exp: usize) -> Result<String, &'static str> {
    let jwt = encode(
        &Header::default(),
        &JWT {
            sub: sub.clone(),
            exp,
        },
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .ok();

    if let Some(token) = jwt {
        return Ok(token);
    } else {
        return Err("cannot create jwt");
    }
}

pub fn verify(jwt: String) -> Result<JWT, &'static str> {
    match decode::<JWT>(
        &jwt,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ) {
        Ok(token) => Ok(token.claims),
        Err(_) => Err("cannot verify jwt"),
    }
}
