/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::v1::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProviderLinkErrorResponse {
    /// Boolean that indicates whether the request was successful or not
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// The error message of the request
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl UserProviderLinkErrorResponse {
    pub fn new() -> UserProviderLinkErrorResponse {
        UserProviderLinkErrorResponse {
            success: None,
            message: None,
        }
    }
}

