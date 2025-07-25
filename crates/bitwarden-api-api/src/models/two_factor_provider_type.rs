/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models;
///
#[repr(i64)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum TwoFactorProviderType {
    Authenticator = 0,
    Email = 1,
    Duo = 2,
    YubiKey = 3,
    U2f = 4,
    Remember = 5,
    OrganizationDuo = 6,
    WebAuthn = 7,
    RecoveryCode = 8,
}

impl std::fmt::Display for TwoFactorProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Authenticator => "0",
                Self::Email => "1",
                Self::Duo => "2",
                Self::YubiKey => "3",
                Self::U2f => "4",
                Self::Remember => "5",
                Self::OrganizationDuo => "6",
                Self::WebAuthn => "7",
                Self::RecoveryCode => "8",
            }
        )
    }
}
impl Default for TwoFactorProviderType {
    fn default() -> TwoFactorProviderType {
        Self::Authenticator
    }
}
