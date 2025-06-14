/*
 * Pipedrive API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetConvertResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// The description of the error
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// A message describing how to solve the problem
    #[serde(rename = "error_info", skip_serializing_if = "Option::is_none")]
    pub error_info: Option<String>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<serde_json::Value>>,
    #[serde(rename = "additional_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<Option<serde_json::Value>>,
}

impl GetConvertResponse {
    pub fn new() -> GetConvertResponse {
        GetConvertResponse {
            success: None,
            error: None,
            error_info: None,
            data: None,
            additional_data: None,
        }
    }
}

