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
pub struct ActivityDistributionDataWithAdditionalData {
    #[serde(rename = "activity_distribution", skip_serializing_if = "Option::is_none")]
    pub activity_distribution: Option<Box<models::ActivityDistribution>>,
    /// Pagination start
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    /// Items shown per page
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// If there are more list items in the collection than displayed or not
    #[serde(rename = "more_items_in_collection", skip_serializing_if = "Option::is_none")]
    pub more_items_in_collection: Option<bool>,
}

impl ActivityDistributionDataWithAdditionalData {
    pub fn new() -> ActivityDistributionDataWithAdditionalData {
        ActivityDistributionDataWithAdditionalData {
            activity_distribution: None,
            start: None,
            limit: None,
            more_items_in_collection: None,
        }
    }
}

