/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganizationUserUserDetailsResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::OrganizationUserType>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::OrganizationUserStatusType>,
    #[serde(rename = "accessAll", skip_serializing_if = "Option::is_none")]
    pub access_all: Option<bool>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(
        rename = "accessSecretsManager",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_secrets_manager: Option<bool>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::Permissions>>,
    #[serde(
        rename = "resetPasswordEnrolled",
        skip_serializing_if = "Option::is_none"
    )]
    pub reset_password_enrolled: Option<bool>,
    #[serde(rename = "usesKeyConnector", skip_serializing_if = "Option::is_none")]
    pub uses_key_connector: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "avatarColor", skip_serializing_if = "Option::is_none")]
    pub avatar_color: Option<String>,
    #[serde(rename = "twoFactorEnabled", skip_serializing_if = "Option::is_none")]
    pub two_factor_enabled: Option<bool>,
    #[serde(rename = "ssoBound", skip_serializing_if = "Option::is_none")]
    pub sso_bound: Option<bool>,
    #[serde(rename = "collections", skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<crate::models::SelectionReadOnlyResponseModel>>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<uuid::Uuid>>,
}

impl OrganizationUserUserDetailsResponseModel {
    pub fn new() -> OrganizationUserUserDetailsResponseModel {
        OrganizationUserUserDetailsResponseModel {
            object: None,
            id: None,
            user_id: None,
            r#type: None,
            status: None,
            access_all: None,
            external_id: None,
            access_secrets_manager: None,
            permissions: None,
            reset_password_enrolled: None,
            uses_key_connector: None,
            name: None,
            email: None,
            avatar_color: None,
            two_factor_enabled: None,
            sso_bound: None,
            collections: None,
            groups: None,
        }
    }
}
