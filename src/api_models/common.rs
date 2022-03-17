use axum::http::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// pub struct Response<T: for<'de> Deserialize<'de> + std::fmt::Debug> {
#[derive(Serialize)]
pub struct Res<T: Serialize> {
    data: Option<T>,
    // #[serde(flatten)]
    // code_message: CodeMessage,
    code: u16,
    message: String,
}

impl<T: Serialize> Res<T> {
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

    pub fn custom_fail(code: StatusCode, message: String) -> Self {
        Self {
            data: None,
            code: code.as_u16(),
            message,
        }
    }
}

pub fn today_to_next_month_hours(n_months: Option<i32>) -> i32 {
    use chrono::prelude::*;
    use chrono::Duration;

    let n_months = n_months.unwrap_or(1);

    let today = Utc::now();
    let to_next_month = if today.month() + n_months as u32 > 12 {
        NaiveDate::from_ymd(today.year() + n_months % 12, today.month() + 1, 1)
    } else {
        NaiveDate::from_ymd(today.year(), today.month() + n_months as u32, 1)
    }
    .signed_duration_since(NaiveDate::from_ymd(
        today.year(),
        today.month(),
        today.day(),
    ));

    to_next_month.num_hours() as i32
}

// #[derive(Serialize)]
// pub struct CodeMessage {
//     code: u16,
//     message: &'static str,
// }

/// copy from repo idp-note-rs common/model/src/consts/status.rs
// impl CodeMessage {
//     pub const SUCCESS: Self = Self {
//         code: 200,
//         message: "success",
//     };
//     pub const FAIL: Self = Self {
//         code: 500,
//         message: "fail",
//     };
// }

impl<T: Serialize> axum::response::IntoResponse for Res<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

/// postgres bigint/int8
#[derive(Debug, Clone, Copy)]
// #[serde(transparent)]
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
