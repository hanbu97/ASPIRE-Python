use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// pub struct Response<T: for<'de> Deserialize<'de> + std::fmt::Debug> {
#[derive(Serialize)]
pub struct Resp<T: Serialize> {
    data: Option<T>,
    code: u16,
    message: String,
}

impl<T: Serialize> Resp<T> {
    const CODE_SUCCESS: u16 = 200;
    const CODE_FAIL: u16 = 500;
    const MESSAGE_SUCCESS: &'static str = "success";
    const MESSAGE_FAIL: &'static str = "fail";

    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            code: Self::CODE_SUCCESS,
            message: Self::MESSAGE_SUCCESS.to_string(),
        }
    }

    pub fn fail() -> Self {
        Self {
            data: None,
            code: Self::CODE_FAIL,
            message: Self::MESSAGE_FAIL.to_string(),
        }
    }

    pub fn custom_fail(message: String) -> Self {
        Self {
            data: None,
            code: Self::CODE_FAIL,
            message,
        }
    }
}

impl<T: Serialize> axum::response::IntoResponse for Resp<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

/// postgres bigint/int8
#[derive(Debug)]
pub struct I64String(i64);

impl From<i64> for I64String {
    fn from(val: i64) -> Self {
        Self(val)
    }
}

impl<'de> Deserialize<'de> for I64String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = String::deserialize(deserializer)?;
        match val.parse::<i64>() {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(serde::de::Error::custom(err.to_string())),
        }
    }
}

impl Serialize for I64String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl I64String {
    pub fn to_i64(self) -> i64 {
        self.0
    }
}

#[test]
fn feature() {
    let dt = chrono::Utc::now().naive_utc();
    dbg!(chrono::Utc::now().naive_utc().format(""));
}
