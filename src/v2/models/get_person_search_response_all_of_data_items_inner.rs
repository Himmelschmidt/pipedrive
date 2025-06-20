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
pub struct GetPersonSearchResponseAllOfDataItemsInner {
    /// Search result relevancy
    #[serde(rename = "result_score", skip_serializing_if = "Option::is_none")]
    pub result_score: Option<f64>,
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<models::GetPersonSearchResponseAllOfDataItemsInnerItem>>,
}

impl GetPersonSearchResponseAllOfDataItemsInner {
    pub fn new() -> GetPersonSearchResponseAllOfDataItemsInner {
        GetPersonSearchResponseAllOfDataItemsInner {
            result_score: None,
            item: None,
        }
    }
}

