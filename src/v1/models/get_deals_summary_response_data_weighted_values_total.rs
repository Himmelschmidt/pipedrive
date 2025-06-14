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

/// GetDealsSummaryResponseDataWeightedValuesTotal : The total weighted values of the deals grouped by deal currency. The weighted value is calculated as probability times deal value.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDealsSummaryResponseDataWeightedValuesTotal {
    /// The total weighted value of the deals in the deal currency group
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// The number of deals in the deal currency group
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The total weighted value of the deals formatted with deal currency. E.g. €50
    #[serde(rename = "value_formatted", skip_serializing_if = "Option::is_none")]
    pub value_formatted: Option<String>,
}

impl GetDealsSummaryResponseDataWeightedValuesTotal {
    /// The total weighted values of the deals grouped by deal currency. The weighted value is calculated as probability times deal value.
    pub fn new() -> GetDealsSummaryResponseDataWeightedValuesTotal {
        GetDealsSummaryResponseDataWeightedValuesTotal {
            value: None,
            count: None,
            value_formatted: None,
        }
    }
}

