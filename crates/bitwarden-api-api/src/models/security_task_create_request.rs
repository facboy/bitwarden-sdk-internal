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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityTaskCreateRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::SecurityTaskType>,
    #[serde(rename = "cipherId", skip_serializing_if = "Option::is_none")]
    pub cipher_id: Option<uuid::Uuid>,
}

impl SecurityTaskCreateRequest {
    pub fn new() -> SecurityTaskCreateRequest {
        SecurityTaskCreateRequest {
            r#type: None,
            cipher_id: None,
        }
    }
}
