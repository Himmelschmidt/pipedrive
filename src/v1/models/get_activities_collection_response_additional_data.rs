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

/// GetActivitiesCollectionResponseAdditionalData : The additional data of the list
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActivitiesCollectionResponseAdditionalData {
    /// The first item on the next page. The value of the `next_cursor` field will be `null` if you have reached the end of the dataset and there’s no more pages to be returned.
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl GetActivitiesCollectionResponseAdditionalData {
    /// The additional data of the list
    pub fn new() -> GetActivitiesCollectionResponseAdditionalData {
        GetActivitiesCollectionResponseAdditionalData {
            next_cursor: None,
        }
    }
}

