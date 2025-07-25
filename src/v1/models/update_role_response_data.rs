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

/// UpdateRoleResponseData : The response data
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRoleResponseData {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl UpdateRoleResponseData {
    /// The response data
    pub fn new() -> UpdateRoleResponseData {
        UpdateRoleResponseData {
            id: None,
        }
    }
}

