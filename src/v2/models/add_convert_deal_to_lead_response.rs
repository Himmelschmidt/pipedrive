/*
 * Pipedrive API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::v2::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddConvertDealToLeadResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::AddConvertDealToLeadResponseData>>,
    #[serde(rename = "additional_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<Option<serde_json::Value>>,
}

impl AddConvertDealToLeadResponse {
    pub fn new() -> AddConvertDealToLeadResponse {
        AddConvertDealToLeadResponse {
            success: None,
            data: None,
            additional_data: None,
        }
    }
}

