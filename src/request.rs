use serde::de::DeserializeOwned;

use crate::errors::AppError;

pub async fn get<T: DeserializeOwned>(url: String) -> Result<T, AppError> {
    let res = reqwest::get(url).await;
    if let Err(e) = res {
        return Err(AppError::Api(e.to_string()));
    }

    let res = res.unwrap().json::<T>().await;
    if let Err(e) = res {
        return Err(AppError::Api(e.to_string()));
    }

    Ok(res.unwrap())
}

pub async fn post() {}
