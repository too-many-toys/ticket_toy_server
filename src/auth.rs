use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::model::user::Claims;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    sub: String,
    exp: usize,
}

pub fn create(sub: String) -> Result<String, &'static str> {
    let jwt = encode(
        &Header::default(),
        &Claims {
            sub: sub.clone(),
            exp: usize::MAX,
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

pub fn verify(jwt: String) -> Result<Claims, String> {
    match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ) {
        Ok(token) => Ok(token.claims),
        Err(e) => Err(e.to_string()),
    }
}
