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
pub struct GetCurrenciesResponse {
    /// If the response is successful or not
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// The array of currencies
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::GetCurrenciesResponseDataInner>>,
}

impl GetCurrenciesResponse {
    pub fn new() -> GetCurrenciesResponse {
        GetCurrenciesResponse {
            success: None,
            data: None,
        }
    }
}

